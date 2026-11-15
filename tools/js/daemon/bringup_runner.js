/**
 * Interactive Hardware Bringup Checklist Runner & Markdown Report Exporter.
 *
 * Drives YAML bringup verification checklists (`/api/bringup/*`), step state
 * transitions (`pending` / `running` / `passed` / `failed` / `skipped`),
 * hardware test-point / control-pin inspection tables, auto-pass regex prompts,
 * and markdown verification report export.
 */

import { stripAnsi } from "../core/ansi.js";
import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { showToast } from "../core/toast.js";
import { extractOscCommand, renderTerminalHtml } from "./rtt_stream.js";

export const BRINGUP_STEP_STATES = Object.freeze({
  PENDING: "pending",
  RUNNING: "running",
  PASSED: "passed",
  FAILED: "failed",
  SKIPPED: "skipped",
});

const DEFAULT_REPORT_PATH = "bringup_report.md";
const AUTO_PASS_DELAY_MS = 200;
const AUTO_STEP_DELAY_MS = 250;
const COPY_FEEDBACK_DURATION_MS = 1500;
const PROMPT_ACTIONS = new Set(["pass", "fail", "skip"]);

/**
 * Resolve badge CSS class and icon glyph for a bringup step status.
 *
 * @param {string} status - Step status string.
 * @returns {{badgeClass: string, icon: string}}
 */
function resolveStepVisuals(status) {
  switch (String(status || "").toLowerCase()) {
    case BRINGUP_STEP_STATES.PASSED:
      return { badgeClass: "badge badge-green", icon: "✔" };
    case BRINGUP_STEP_STATES.FAILED:
      return { badgeClass: "badge badge-amber", icon: "✘" };
    case BRINGUP_STEP_STATES.SKIPPED:
      return { badgeClass: "badge", icon: "⏭" };
    case BRINGUP_STEP_STATES.RUNNING:
      return { badgeClass: "badge badge-cyan", icon: "⚙" };
    default:
      return { badgeClass: "badge", icon: "⏳" };
  }
}

/**
 * Construct a two-column key/value table element for `test_points` or `control_pins`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {string} col1Header - First column header text.
 * @param {string} col2Header - Second column header text.
 * @param {Record<string, string>} entriesMap - Mapping of signal/pin names to expected values.
 * @returns {HTMLElement}
 */
function buildSignalTable(ctx, col1Header, col2Header, entriesMap) {
  const thead = createEl(ctx, "thead", {
    children: [
      createEl(ctx, "tr", {
        children: [
          createEl(ctx, "th", { text: col1Header }),
          createEl(ctx, "th", { text: col2Header }),
        ],
      }),
    ],
  });
  const rows = Object.entries(entriesMap).map(([key, val]) =>
    createEl(ctx, "tr", {
      children: [
        createEl(ctx, "td", {
          children: [createEl(ctx, "strong", { text: key })],
        }),
        createEl(ctx, "td", { text: String(val ?? "") }),
      ],
    }),
  );
  const tbody = createEl(ctx, "tbody", { children: rows });
  return createEl(ctx, "table", {
    className: "bringup-table",
    children: [thead, tbody],
  });
}

/**
 * Controller for the Bringup Verification Checklist workstation view.
 */
export class BringupRunnerController {
  /**
   * Construct a `BringupRunnerController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Optional element overrides.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.scripts = [];
    this.activeScriptPath = "";
    this.lastStatus = null;
    this.lastLogCount = 0;
    this.isRunningAll = false;
    this._runTimer = null;
    this._unsubscribers = [];

    this.scriptSelect = resolveElement(ctx, options.scriptSelect || "bringupScriptSelect");
    this.stepBtn = resolveElement(ctx, options.stepBtn || "btnBringupStep");
    this.runBtn = resolveElement(ctx, options.runBtn || "btnBringupRun");
    this.resetBtn = resolveElement(ctx, options.resetBtn || "btnBringupReset");
    this.reportBtn = resolveElement(ctx, options.reportBtn || "btnBringupReport");
    this.summaryBadge = resolveElement(ctx, options.summaryBadge || "bringupSummaryBadge");
    this.stepList = resolveElement(ctx, options.stepList || "bringupStepList");
    this.cardTitle = resolveElement(ctx, options.cardTitle || "bringupCardTitle");
    this.cardType = resolveElement(ctx, options.cardType || "bringupCardType");
    this.cardDesc = resolveElement(ctx, options.cardDesc || "bringupCardDesc");
    this.cardDetails = resolveElement(ctx, options.cardDetails || "bringupCardDetails");
    this.promptActionArea = resolveElement(
      ctx,
      options.promptActionArea || "bringupPromptActionArea",
    );
    this.promptText = resolveElement(ctx, options.promptText || "bringupPromptText");
    this.promptNotes = resolveElement(ctx, options.promptNotes || "bringupPromptNotes");
    this.consoleOutput = resolveElement(ctx, options.consoleOutput || "bringupConsoleOutput");
    this.copyLogBtn = resolveElement(ctx, options.copyLogBtn || "btnCopyBringupLog");
  }

  /**
   * Bind toolbar, prompt card, and console hyperlink event listeners.
   */
  init() {
    if (this.scriptSelect) {
      const onChange = () => {
        if (this.scriptSelect.value) {
          this.selectScript(this.scriptSelect.value);
        }
      };
      this.scriptSelect.addEventListener("change", onChange);
      this._unsubscribers.push(() =>
        this.scriptSelect.removeEventListener("change", onChange),
      );
    }

    if (this.stepBtn) {
      const onStep = () => this.step();
      this.stepBtn.addEventListener("click", onStep);
      this._unsubscribers.push(() => this.stepBtn.removeEventListener("click", onStep));
    }

    if (this.runBtn) {
      const onRun = () => this.runAll();
      this.runBtn.addEventListener("click", onRun);
      this._unsubscribers.push(() => this.runBtn.removeEventListener("click", onRun));
    }

    if (this.resetBtn) {
      const onReset = () => this.reset();
      this.resetBtn.addEventListener("click", onReset);
      this._unsubscribers.push(() => this.resetBtn.removeEventListener("click", onReset));
    }

    if (this.reportBtn) {
      const onReport = () => this.exportReport();
      this.reportBtn.addEventListener("click", onReport);
      this._unsubscribers.push(() => this.reportBtn.removeEventListener("click", onReport));
    }

    if (this.promptActionArea) {
      this._unsubscribers.push(
        bindDelegatedEvent(
          this.promptActionArea,
          "click",
          "[data-bringup-action]",
          (_evt, btn) => {
            const action = btn.getAttribute("data-bringup-action");
            if (action) {
              this.respond(action);
            }
          },
        ),
      );
    }

    if (this.consoleOutput) {
      this._unsubscribers.push(
        bindDelegatedEvent(
          this.consoleOutput,
          "click",
          ".osc8-cmd-btn",
          (event, btn) => {
            event.preventDefault();
            const cmd = (
              btn.getAttribute("data-command") ||
              extractOscCommand(btn.getAttribute("data-osc-uri") || "")
            ).toLowerCase();
            if (PROMPT_ACTIONS.has(cmd)) {
              this.respond(cmd);
            }
          },
        ),
      );
    }

    if (this.copyLogBtn) {
      const onCopy = () => this.copyConsole(this.copyLogBtn);
      this.copyLogBtn.addEventListener("click", onCopy);
      this._unsubscribers.push(() => this.copyLogBtn.removeEventListener("click", onCopy));
    }
  }

  /**
   * Discover available YAML bringup scripts via `GET /api/bringup/scripts` and populate selector.
   *
   * @returns {Promise<Array<object>>}
   */
  async loadScripts() {
    try {
      const scripts = await this.ctx.fetchJson("/api/bringup/scripts");
      if (!Array.isArray(scripts)) {
        return [];
      }
      this.scripts = scripts;
      if (this.scriptSelect) {
        const previousVal = this.activeScriptPath || this.scriptSelect.value;
        clearChildren(this.scriptSelect);
        for (const script of scripts) {
          const opt = createEl(this.ctx, "option", {
            text: `${script.name} (${script.chip}) - ${script.path}`,
            attrs: { value: script.path },
          });
          opt.value = script.path;
          if (previousVal && script.path === previousVal) {
            opt.selected = true;
          }
          this.scriptSelect.appendChild(opt);
        }
        if (!previousVal && scripts.length > 0) {
          this.scriptSelect.value = scripts[0].path;
          await this.selectScript(scripts[0].path);
        } else if (previousVal) {
          this.scriptSelect.value = previousVal;
        }
      }
      return scripts;
    } catch {
      return [];
    }
  }

  /**
   * Load a specific YAML bringup checklist via `POST /api/bringup/load`.
   *
   * @param {string} scriptPath - Repository-relative YAML path.
   * @returns {Promise<object|null>}
   */
  async selectScript(scriptPath) {
    if (!scriptPath) {
      return null;
    }
    this.activeScriptPath = scriptPath;
    this.lastLogCount = 0;
    try {
      const res = await this.ctx.postJson("/api/bringup/load", { script_path: scriptPath });
      if (res && res.status) {
        this.renderStatus(res.status);
      } else {
        await this.pollStatus();
      }
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to load bringup script: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Poll `GET /api/bringup/status` and update the checklist UI.
   *
   * @returns {Promise<object|null>}
   */
  async pollStatus() {
    try {
      const status = await this.ctx.fetchJson("/api/bringup/status");
      this.renderStatus(status);
      return status;
    } catch {
      return null;
    }
  }

  /**
   * Render bringup checklist progress, sidebar items, active step card, and console logs.
   *
   * @param {object} data - Status payload from `/api/bringup/status`.
   */
  renderStatus(data) {
    if (!data) {
      return;
    }
    this.lastStatus = data;

    if (this.summaryBadge && data.summary) {
      const total = data.total_steps || 0;
      const stepNum = total > 0 ? Math.min(total, (data.current_step_idx ?? 0) + 1) : 0;
      this.summaryBadge.textContent =
        `STEP ${stepNum}/${total} | ` +
        `${data.summary.passed ?? 0} PASSED, ${data.summary.failed ?? 0} FAILED, ${data.summary.skipped ?? 0} SKIPPED`;
      this.summaryBadge.className =
        (data.summary.failed ?? 0) > 0 ? "badge badge-amber" : "badge badge-green";
    }

    if (this.stepList && Array.isArray(data.steps)) {
      clearChildren(this.stepList);
      data.steps.forEach((stepItem, idx) => {
        const isActive = idx === data.current_step_idx;
        const { badgeClass, icon } = resolveStepVisuals(stepItem.status);
        const li = createEl(this.ctx, "li", {
          className: isActive ? "bringup-step-item active" : "bringup-step-item",
          dataset: { stepId: String(stepItem.id), stepStatus: String(stepItem.status || "pending") },
          children: [
            createEl(this.ctx, "div", {
              style: {
                display: "flex",
                alignItems: "center",
                gap: "6px",
                overflow: "hidden",
              },
              children: [
                createEl(this.ctx, "span", {
                  text: `${icon} [${stepItem.id}]`,
                  style: { fontWeight: "bold", color: "var(--xerxes-cyan)" },
                }),
                createEl(this.ctx, "span", {
                  text: stepItem.name || "",
                  style: {
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    color: "var(--xerxes-text-light)",
                  },
                }),
              ],
            }),
            createEl(this.ctx, "span", {
              className: badgeClass,
              text: stepItem.type || "",
              style: { fontSize: "7.5px" },
            }),
          ],
        });
        this.stepList.appendChild(li);
      });
    }

    const currStep = data.current_step;
    if (currStep) {
      if (this.cardTitle) {
        this.cardTitle.textContent = `STEP ${currStep.id}: ${String(currStep.name || "").toUpperCase()}`;
      }
      if (this.cardType) {
        this.cardType.textContent = `TYPE: ${String(currStep.type || "--").toUpperCase()}`;
        this.cardType.className = "badge";
      }
      if (this.cardDesc) {
        this.cardDesc.textContent =
          currStep.description || "No description provided for this step.";
      }

      if (this.cardDetails) {
        clearChildren(this.cardDetails);
        const appendDetailRow = (labelText, valueText, codeColor = null, rowColor = null) => {
          const row = createEl(this.ctx, "div", {
            style: {
              marginTop: "4px",
              ...(rowColor ? { color: rowColor } : {}),
            },
            children: [
              createEl(this.ctx, "strong", { text: `${labelText}: ` }),
              codeColor
                ? createEl(this.ctx, "code", {
                    text: valueText,
                    style: { color: codeColor },
                  })
                : createEl(this.ctx, "span", { text: valueText }),
            ],
          });
          this.cardDetails.appendChild(row);
        };

        if (currStep.command) {
          appendDetailRow("Command", currStep.command, "var(--xerxes-cyan)");
        }
        if (currStep.expected_regex) {
          appendDetailRow("Expected Regex", currStep.expected_regex, "var(--xerxes-teal)");
        }
        if (currStep.matched_line) {
          appendDetailRow(
            "Matched Output",
            currStep.matched_line,
            "var(--xerxes-green)",
            "var(--xerxes-green)",
          );
        }
        if (currStep.procedure) {
          appendDetailRow("Procedure", currStep.procedure);
        }
        if (currStep.expected) {
          appendDetailRow("Expected Outcome", currStep.expected);
        }

        if (currStep.test_points && Object.keys(currStep.test_points).length > 0) {
          this.cardDetails.appendChild(
            buildSignalTable(
              this.ctx,
              "Test Point",
              "Expected Voltage / Signal",
              currStep.test_points,
            ),
          );
        }

        if (currStep.control_pins && Object.keys(currStep.control_pins).length > 0) {
          this.cardDetails.appendChild(
            buildSignalTable(
              this.ctx,
              "Control Signal",
              "Target Pin",
              currStep.control_pins,
            ),
          );
        }
      }

      if (data.active_prompt) {
        if (this.promptActionArea) {
          this.promptActionArea.style.display = "block";
        }
        const activePrompt = data.active_prompt;
        let promptMsg = `ACTION REQUIRED: Verify step outcome (Default: ${String(activePrompt.default_status || "pass").toUpperCase()})`;
        if (activePrompt.auto_pass === true) {
          promptMsg = `✔ AUTOMATIC MATCH CONFIRMED (${activePrompt.matched_line || "Pattern Matched"}). Click Pass to proceed.`;
        } else if (activePrompt.auto_pass === false) {
          promptMsg = `✘ AUTOMATIC MATCH FAILED: Expected pattern '${currStep.expected_regex || ""}' not found.`;
        }
        if (this.promptText) {
          this.promptText.textContent = promptMsg;
        }
      } else if (this.promptActionArea) {
        this.promptActionArea.style.display = "none";
      }
    } else if (this.promptActionArea) {
      this.promptActionArea.style.display = "none";
    }

    if (this.consoleOutput && Array.isArray(data.logs)) {
      if (data.logs.length !== this.lastLogCount) {
        clearChildren(this.consoleOutput);
        for (const logLine of data.logs) {
          const lineEl = createEl(this.ctx, "div", {
            className: "log-line log-line-info",
          });
          lineEl.innerHTML = renderTerminalHtml(logLine);
          this.consoleOutput.appendChild(lineEl);
        }
        this.lastLogCount = data.logs.length;
        this.consoleOutput.scrollTop = this.consoleOutput.scrollHeight;
      }
    }
  }

  /**
   * Execute the current bringup step via `POST /api/bringup/step`.
   *
   * @returns {Promise<object|null>}
   */
  async step() {
    try {
      const res = await this.ctx.postJson("/api/bringup/step", {});
      await this.pollStatus();
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Bringup step failed: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Submit an interactive `pass` / `fail` / `skip` verification verdict to `POST /api/bringup/respond`.
   *
   * @param {"pass"|"fail"|"skip"} action - Verification verdict.
   * @param {string} [overrideNotes] - Optional explicit notes overriding `#bringupPromptNotes`.
   * @returns {Promise<object|null>}
   */
  async respond(action, overrideNotes) {
    const normalizedAction = String(action || "pass").toLowerCase();
    const notes =
      overrideNotes !== undefined
        ? String(overrideNotes)
        : this.promptNotes
          ? this.promptNotes.value.trim()
          : "";
    try {
      const res = await this.ctx.postJson("/api/bringup/respond", {
        action: normalizedAction,
        notes,
      });
      if (this.promptNotes && overrideNotes === undefined) {
        this.promptNotes.value = "";
      }
      await this.pollStatus();
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to record bringup response: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Auto-run remaining bringup steps until completion or until a manual operator prompt is reached.
   *
   * @returns {Promise<void>}
   */
  async runAll() {
    this.stopRunAll();
    this.isRunningAll = true;

    const runNext = async () => {
      if (!this.isRunningAll) {
        return;
      }
      const status = await this.pollStatus();
      if (!status || !this.isRunningAll) {
        this.isRunningAll = false;
        return;
      }

      if (
        status.status === "complete" ||
        (status.total_steps > 0 && status.current_step_idx >= status.total_steps)
      ) {
        this.isRunningAll = false;
        return;
      }

      if (status.status === "waiting_prompt") {
        if (status.active_prompt && status.active_prompt.auto_pass === true) {
          await this.respond("pass");
          if (this.isRunningAll) {
            this._runTimer = this.ctx.setTimeout(runNext, AUTO_PASS_DELAY_MS);
          }
        } else {
          this.isRunningAll = false;
        }
        return;
      }

      await this.step();
      if (this.isRunningAll) {
        this._runTimer = this.ctx.setTimeout(runNext, AUTO_STEP_DELAY_MS);
      }
    };

    await runNext();
  }

  /**
   * Stop any active `runAll()` loop.
   */
  stopRunAll() {
    this.isRunningAll = false;
    if (this._runTimer !== null) {
      this.ctx.clearTimeout(this._runTimer);
      this._runTimer = null;
    }
  }

  /**
   * Reset checklist execution back to step 1 via `POST /api/bringup/reset`.
   *
   * @returns {Promise<object|null>}
   */
  async reset() {
    this.stopRunAll();
    this.lastLogCount = 0;
    try {
      const res = await this.ctx.postJson("/api/bringup/reset", {});
      await this.pollStatus();
      return res;
    } catch {
      return null;
    }
  }

  /**
   * Generate and export a Markdown verification report via `POST /api/bringup/report`.
   *
   * @param {string} [outputPath] - Destination report path on host.
   * @returns {Promise<object|null>}
   */
  async exportReport(outputPath = DEFAULT_REPORT_PATH) {
    try {
      const res = await this.ctx.postJson("/api/bringup/report", {
        output_path: outputPath,
      });
      if (res && res.success) {
        showToast(
          this.ctx,
          `Bringup report exported to ${res.output_path || outputPath}`,
          "success",
        );
        if (res.report) {
          const filename = String(res.output_path || outputPath).split("/").pop() || DEFAULT_REPORT_PATH;
          this.ctx.triggerDownload(filename, res.report, "text/markdown;charset=utf-8");
        }
      }
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to export bringup report: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Copy bringup live console logs to the system clipboard.
   *
   * @param {Element|null} [buttonEl] - Optional trigger button for visual feedback.
   * @returns {Promise<boolean>}
   */
  async copyConsole(buttonEl = this.copyLogBtn) {
    const rawLogs =
      this.lastStatus && Array.isArray(this.lastStatus.logs)
        ? this.lastStatus.logs.map((line) => stripAnsi(line)).join("\n")
        : (this.consoleOutput && this.consoleOutput.textContent) || "";
    const ok = await this.ctx.copyTextToClipboard(rawLogs);
    if (buttonEl) {
      const orig = buttonEl.textContent;
      buttonEl.textContent = ok ? "✔ Copied" : "Error";
      this.ctx.setTimeout(() => {
        buttonEl.textContent = orig;
      }, COPY_FEEDBACK_DURATION_MS);
    }
    return ok;
  }

  /**
   * Clean up timers and DOM listeners.
   */
  destroy() {
    this.stopRunAll();
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating a `BringupRunnerController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {BringupRunnerController}
 */
export function createBringupRunner(ctx, options = {}) {
  return new BringupRunnerController(ctx, options);
}
