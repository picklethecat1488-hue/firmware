/**
 * Interactive Target CLI (`host_cli`) & Filesystem (`host_fs`) Workstation Console.
 *
 * Manages separate CLI and FS output streams, mode switching (`CLI` / `FS` / `BRINGUP`),
 * command execution lockouts while in-flight, target-offline workstation disabling,
 * Up/Down command history, single-click command replay, and `Ctrl+R` fuzzy history search.
 */

import { stripAnsi } from "../core/ansi.js";
import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { fuzzyFilterAndRank } from "../core/fuzzy_search.js";
import { extractOscCommand, isSelectionInside, renderTerminalHtml } from "./rtt_stream.js";

export const WORKSTATION_MODES = Object.freeze({
  CLI: "CLI",
  FS: "FS",
});

const FOLLOWUP_POLL_SHORT_MS = 100;
const FOLLOWUP_POLL_LONG_MS = 300;
const COPY_FEEDBACK_DURATION_MS = 1500;
const PROMPT_PREFIX_RE = /^(?:shell>|fs>|>)\s*/i;

const DEFAULT_CLI_PLACEHOLDER =
  "Type shell command (e.g. 'motor speed 100', 'gpio status', 'battery status')...";
const DEFAULT_FS_PLACEHOLDER =
  "Type FS command (e.g. 'ls', 'cat metadata', 'crash-log', 'format')...";
const DEFAULT_DISABLED_PLACEHOLDER =
  "CLI disabled until target synchronization succeeds. Click ⚡ Sync.";

/**
 * Controller for the right-hand Interactive CLI / Filesystem Workstation & command bar.
 */
export class CliConsoleController {
  /**
   * Construct a `CliConsoleController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Optional element overrides, initial mode, and callbacks.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.currentMode = String(options.initialMode || WORKSTATION_MODES.CLI).toUpperCase();
    this.onModeChange = options.onModeChange || null;

    this.commandHistory = Array.isArray(options.initialHistory)
      ? [...options.initialHistory]
      : [];
    this.historyIdx = this.commandHistory.length;
    this.isFuzzyOpen = false;
    this.fuzzySelectedIndex = 0;
    this.currentFuzzyMatches = [];
    this.workstationDisabled = false;

    this.cliEntries = [];
    this.fsEntries = [];
    this.lastCliId = 0;
    this.lastFsId = 0;
    this.clearedCliUpToId = 0;
    this.clearedFsUpToId = 0;
    this._unsubscribers = [];

    this.modeSelect = resolveElement(ctx, options.modeSelect || "modeSelect");
    this.cliView = resolveElement(ctx, options.cliView || "cliView");
    this.fsView = resolveElement(ctx, options.fsView || "fsView");
    this.fsToolbar = resolveElement(ctx, options.fsToolbar || "fsToolbar");
    this.promptLabel = resolveElement(ctx, options.promptLabel || "promptLabel");
    this.cmdInput = resolveElement(ctx, options.cmdInput || "cmdInput");
    this.submitBtn = resolveElement(ctx, options.submitBtn || "btnSubmitCmd");
    this.panelTitle = resolveElement(ctx, options.panelTitle || "cliPanelTitle");
    this.copyBtn = resolveElement(ctx, options.copyBtn || "btnCopyWorkstation");

    this.fuzzyContainer = resolveElement(ctx, options.fuzzyContainer || "cliFuzzyHistory");
    this.fuzzySearchInput = resolveElement(ctx, options.fuzzySearchInput || "fuzzySearchInput");
    this.fuzzyResultsList = resolveElement(ctx, options.fuzzyResultsList || "fuzzyResultsList");
    this.fuzzyCloseBtn = resolveElement(ctx, options.fuzzyCloseBtn || "btnCloseFuzzyHistory");
  }

  /**
   * Bind DOM listeners, apply initial mode layout, and fetch existing command history.
   *
   * @returns {Promise<void>}
   */
  async init() {
    this._applyModeDom(this.currentMode);

    if (this.modeSelect) {
      const onModeSelectChange = () => {
        this.setMode(this.modeSelect.value);
      };
      this.modeSelect.addEventListener("change", onModeSelectChange);
      this._unsubscribers.push(() =>
        this.modeSelect.removeEventListener("change", onModeSelectChange),
      );
    }

    if (this.submitBtn) {
      const onSubmitClick = () => {
        this.submitCommand();
      };
      this.submitBtn.addEventListener("click", onSubmitClick);
      this._unsubscribers.push(() =>
        this.submitBtn.removeEventListener("click", onSubmitClick),
      );
    }

    if (this.fsToolbar) {
      this._unsubscribers.push(
        bindDelegatedEvent(this.fsToolbar, "click", "[data-fs-cmd]", (_evt, btn) => {
          const fsCmd = btn.getAttribute("data-fs-cmd");
          if (fsCmd) {
            this.runFsQuick(fsCmd);
          }
        }),
      );
    }

    if (this.cmdInput) {
      const onCmdKeyDown = (event) => this._handleCmdInputKeyDown(event);
      const onCmdClick = () => {
        if (!this.cmdInput.disabled) {
          this.cmdInput.select();
        }
      };
      this.cmdInput.addEventListener("keydown", onCmdKeyDown);
      this.cmdInput.addEventListener("click", onCmdClick);
      this._unsubscribers.push(() => {
        this.cmdInput.removeEventListener("keydown", onCmdKeyDown);
        this.cmdInput.removeEventListener("click", onCmdClick);
      });
    }

    if (this.promptLabel) {
      const onPromptClick = () => {
        if (this.cmdInput && !this.cmdInput.disabled) {
          this.cmdInput.focus();
          this.cmdInput.select();
        }
      };
      this.promptLabel.addEventListener("click", onPromptClick);
      this._unsubscribers.push(() =>
        this.promptLabel.removeEventListener("click", onPromptClick),
      );
    }

    const doc = this.ctx.getDocument();
    const footerEl = doc ? doc.querySelector(".cli-input-bar") : null;
    if (footerEl) {
      const onFooterClick = (event) => {
        const tgt = event.target;
        if (
          tgt &&
          tgt.tagName !== "BUTTON" &&
          tgt.id !== "fuzzySearchInput" &&
          (!tgt.closest || !tgt.closest("#cliFuzzyHistory"))
        ) {
          if (this.cmdInput && !this.cmdInput.disabled) {
            this.cmdInput.focus();
            this.cmdInput.select();
          }
        }
      };
      footerEl.addEventListener("click", onFooterClick);
      this._unsubscribers.push(() => footerEl.removeEventListener("click", onFooterClick));
    }

    for (const viewEl of [this.cliView, this.fsView]) {
      if (!viewEl) {
        continue;
      }
      this._unsubscribers.push(
        bindDelegatedEvent(viewEl, "click", ".cli-line-input", (_evt, lineEl) => {
          const raw = String(
            lineEl.getAttribute("data-raw-text") || lineEl.textContent || "",
          ).trim();
          const cleaned = raw.replace(PROMPT_PREFIX_RE, "").trim();
          if (cleaned && this.cmdInput && !this.cmdInput.disabled) {
            this.cmdInput.value = cleaned;
            this.cmdInput.focus();
            this.cmdInput.select();
          }
        }),
      );

      this._unsubscribers.push(
        bindDelegatedEvent(
          viewEl,
          "click",
          ".osc8-cmd-btn",
          (event, btn) => {
            event.preventDefault();
            event.stopPropagation();
            const cmd =
              btn.getAttribute("data-command") ||
              extractOscCommand(btn.getAttribute("data-osc-uri") || "");
            if (cmd) {
              this.handleCommandClick(cmd);
            }
          },
        ),
      );

      this._unsubscribers.push(
        bindDelegatedEvent(viewEl, "click", ".term-file-link", (event, anchor) => {
          const href = anchor.getAttribute("href");
          if (href && href.startsWith("vscode://")) {
            event.preventDefault();
            this.ctx.openExternalUrl(href, "_self");
          }
        }),
      );
    }

    this._setupFuzzyHistoryListeners();
    await this.loadHistory();
  }

  /**
   * Bind listeners for the `Ctrl+R` reverse fuzzy search popup.
   */
  _setupFuzzyHistoryListeners() {
    if (this.fuzzyCloseBtn) {
      const onCloseClick = () => this.closeFuzzyHistory();
      this.fuzzyCloseBtn.addEventListener("click", onCloseClick);
      this._unsubscribers.push(() =>
        this.fuzzyCloseBtn.removeEventListener("click", onCloseClick),
      );
    }

    if (!this.fuzzySearchInput) {
      return;
    }

    const onFuzzyInput = () => {
      this.fuzzySelectedIndex = 0;
      this.updateFuzzyResults(this.fuzzySearchInput.value);
    };

    const onFuzzyKeyDown = (event) => {
      if ((event.ctrlKey || event.metaKey) && (event.key === "r" || event.key === "R")) {
        event.preventDefault();
        if (this.currentFuzzyMatches.length > 0) {
          this.fuzzySelectedIndex =
            (this.fuzzySelectedIndex + 1) % this.currentFuzzyMatches.length;
          this.updateFuzzyResults(this.fuzzySearchInput.value);
        }
        return;
      }

      if (event.key === "Escape") {
        event.preventDefault();
        this.closeFuzzyHistory();
      } else if (event.key === "Enter") {
        event.preventDefault();
        if (
          this.currentFuzzyMatches.length > 0 &&
          this.currentFuzzyMatches[this.fuzzySelectedIndex]
        ) {
          this.selectFuzzyCommand(this.currentFuzzyMatches[this.fuzzySelectedIndex]);
        } else {
          this.closeFuzzyHistory();
        }
      } else if (event.key === "ArrowUp") {
        event.preventDefault();
        if (this.currentFuzzyMatches.length > 0) {
          this.fuzzySelectedIndex =
            (this.fuzzySelectedIndex - 1 + this.currentFuzzyMatches.length) %
            this.currentFuzzyMatches.length;
          this.updateFuzzyResults(this.fuzzySearchInput.value);
        }
      } else if (event.key === "ArrowDown") {
        event.preventDefault();
        if (this.currentFuzzyMatches.length > 0) {
          this.fuzzySelectedIndex =
            (this.fuzzySelectedIndex + 1) % this.currentFuzzyMatches.length;
          this.updateFuzzyResults(this.fuzzySearchInput.value);
        }
      }
    };

    this.fuzzySearchInput.addEventListener("input", onFuzzyInput);
    this.fuzzySearchInput.addEventListener("keydown", onFuzzyKeyDown);
    this._unsubscribers.push(() => {
      this.fuzzySearchInput.removeEventListener("input", onFuzzyInput);
      this.fuzzySearchInput.removeEventListener("keydown", onFuzzyKeyDown);
    });
  }

  /**
   * Handle `Enter`, `Ctrl+R`, `ArrowUp`, and `ArrowDown` on `#cmdInput`.
   *
   * @param {KeyboardEvent} event - Keydown event.
   */
  _handleCmdInputKeyDown(event) {
    if ((event.ctrlKey || event.metaKey) && (event.key === "r" || event.key === "R")) {
      event.preventDefault();
      this.openFuzzyHistory();
      return;
    }
    if (event.key === "Enter") {
      event.preventDefault();
      this.submitCommand();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      if (this.historyIdx > 0) {
        this.historyIdx -= 1;
        this.cmdInput.value = this.commandHistory[this.historyIdx] || "";
      }
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      if (this.historyIdx < this.commandHistory.length - 1) {
        this.historyIdx += 1;
        this.cmdInput.value = this.commandHistory[this.historyIdx] || "";
      } else {
        this.historyIdx = this.commandHistory.length;
        this.cmdInput.value = "";
      }
    }
  }

  /**
   * Fetch recorded CLI history from `GET /api/cli/history`.
   *
   * @returns {Promise<Array<string>>}
   */
  async loadHistory() {
    try {
      const res = await this.ctx.fetchJson("/api/cli/history");
      if (res && Array.isArray(res.history) && res.history.length > 0) {
        if (this.commandHistory.length === 0) {
          this.commandHistory = res.history.filter(Boolean);
          this.historyIdx = this.commandHistory.length;
        }
      }
    } catch {
      // History endpoint optional during offline tests.
    }
    return this.commandHistory;
  }

  /**
   * Return a copy of recorded command history.
   *
   * @returns {Array<string>}
   */
  getCommandHistory() {
    return [...this.commandHistory];
  }

  /**
   * Return the active workstation mode (`"CLI"`, `"FS"`, or `"BRINGUP"`).
   *
   * @returns {string}
   */
  getMode() {
    return this.currentMode;
  }

  /**
   * Update DOM visibility and prompt copy for `mode`.
   *
   * @param {string} mode - Target workstation mode.
   */
  _applyModeDom(mode) {
    if (this.modeSelect && this.modeSelect.value !== mode) {
      this.modeSelect.value = mode;
    }

    let nextPlaceholder = DEFAULT_CLI_PLACEHOLDER;
    if (mode === WORKSTATION_MODES.FS) {
      if (this.cliView) this.cliView.style.display = "none";
      if (this.fsView) this.fsView.style.display = "block";
      if (this.fsToolbar) this.fsToolbar.style.display = "flex";
      if (this.promptLabel) this.promptLabel.textContent = "fs> ";
      if (this.panelTitle) {
        this.panelTitle.textContent = "FILESYSTEM COMMAND WORKSTATION (host_fs)";
      }
      nextPlaceholder = DEFAULT_FS_PLACEHOLDER;
    } else {
      if (this.cliView) this.cliView.style.display = "block";
      if (this.fsView) this.fsView.style.display = "none";
      if (this.fsToolbar) this.fsToolbar.style.display = "none";
      if (this.promptLabel) this.promptLabel.textContent = "shell> ";
      if (this.panelTitle) {
        this.panelTitle.textContent = "INTERACTIVE CLI CONSOLE (host_cli)";
      }
      nextPlaceholder = DEFAULT_CLI_PLACEHOLDER;
    }

    if (this.cmdInput) {
      if (this.workstationDisabled) {
        this.cmdInput.setAttribute("data-prev-placeholder", nextPlaceholder);
      } else {
        this.cmdInput.placeholder = nextPlaceholder;
      }
    }
  }

  /**
   * Switch workstation mode (`CLI`, `FS`, or `BRINGUP`) and notify `/api/mode`.
   *
   * @param {string} newMode - Desired mode.
   * @param {object} [options] - `{ notifyServer: boolean }`.
   * @returns {Promise<void>}
   */
  async setMode(newMode, options = {}) {
    const normalized = String(newMode || WORKSTATION_MODES.CLI).toUpperCase();
    this.currentMode = normalized;
    this._applyModeDom(normalized);

    if (options.notifyServer !== false) {
      try {
        await this.ctx.postJson("/api/mode", { mode: normalized });
        await this.pollOutputs();
      } catch {
        // Keep UI responsive if server mode switch fails transiently.
      }
    }

    if (typeof this.onModeChange === "function") {
      await this.onModeChange(normalized);
    }
  }

  /**
   * Enable or disable the CLI/FS command bar (e.g. when RTT is offline or sync cancelled).
   *
   * @param {boolean} disabled - True to disable input & quick-action buttons.
   * @param {string} [message] - Custom placeholder message when disabled.
   */
  setWorkstationDisabled(disabled, message) {
    this.workstationDisabled = Boolean(disabled);
    if (this.cmdInput) {
      this.cmdInput.disabled = this.workstationDisabled;
      if (this.workstationDisabled) {
        if (!this.cmdInput.hasAttribute("data-prev-placeholder")) {
          this.cmdInput.setAttribute("data-prev-placeholder", this.cmdInput.placeholder || "");
        }
        this.cmdInput.placeholder = message || DEFAULT_DISABLED_PLACEHOLDER;
        this.cmdInput.style.opacity = "0.5";
        this.cmdInput.style.cursor = "not-allowed";
      } else {
        const prev = this.cmdInput.getAttribute("data-prev-placeholder");
        if (prev) {
          this.cmdInput.placeholder = prev;
        }
        this.cmdInput.removeAttribute("data-prev-placeholder");
        this.cmdInput.style.opacity = "1";
        this.cmdInput.style.cursor = "text";
      }
    }

    if (this.submitBtn) {
      this.submitBtn.disabled = this.workstationDisabled;
    }

    if (this.fsToolbar) {
      const buttons = this.fsToolbar.querySelectorAll("button");
      for (const btn of buttons) {
        btn.disabled = this.workstationDisabled;
      }
    }
  }

  /**
   * Append an immediate line to `#cliView`.
   *
   * @param {string} text - Raw or ANSI text.
   * @param {boolean} [isInput] - True if echoed input command.
   */
  appendCliText(text, isInput = false) {
    if (!this.cliView) {
      return;
    }
    const lineEl = createEl(this.ctx, "div", {
      className: isInput ? "log-line cli-line-input" : "log-line cli-line-output",
      dataset: { rawText: stripAnsi(text) },
    });
    lineEl.innerHTML = renderTerminalHtml(text);
    this.cliView.appendChild(lineEl);
    this.cliView.scrollTop = this.cliView.scrollHeight;
  }

  /**
   * Append an immediate line to `#fsView`.
   *
   * @param {string} text - Raw or ANSI text.
   * @param {boolean} [isInput] - True if echoed input command.
   */
  appendFsText(text, isInput = false) {
    if (!this.fsView) {
      return;
    }
    const lineEl = createEl(this.ctx, "div", {
      className: isInput ? "log-line cli-line-input" : "log-line cli-line-output",
      dataset: { rawText: stripAnsi(text) },
    });
    lineEl.innerHTML = renderTerminalHtml(text);
    this.fsView.appendChild(lineEl);
    this.fsView.scrollTop = this.fsView.scrollHeight;
  }

  /**
   * Execute a quick-action `host_fs` command from the FS toolbar.
   *
   * @param {string} action - FS command string (e.g. `"ls"`, `"crash-log"`).
   * @returns {Promise<any>}
   */
  async runFsQuick(action) {
    if (this.cmdInput && this.cmdInput.disabled) {
      return null;
    }
    if (this.cmdInput) {
      this.cmdInput.value = action;
    }
    return this.submitCommand();
  }

  /**
   * Dispatch a shell or filesystem command to `/api/cli/execute` or `/api/fs/execute`.
   *
   * @param {string} [overrideCommand] - Optional explicit command string.
   * @returns {Promise<object|null>}
   */
  async submitCommand(overrideCommand) {
    if (this.cmdInput && this.cmdInput.disabled && overrideCommand === undefined) {
      return null;
    }
    const rawCmd =
      overrideCommand !== undefined
        ? String(overrideCommand)
        : this.cmdInput
          ? this.cmdInput.value
          : "";
    const cmd = rawCmd.trim();
    if (!cmd) {
      return null;
    }

    this.commandHistory.push(cmd);
    this.historyIdx = this.commandHistory.length;
    if (this.cmdInput) {
      this.cmdInput.value = "";
    }

    const prevPlaceholder = this.cmdInput ? this.cmdInput.placeholder : "";
    if (this.cmdInput) {
      this.cmdInput.disabled = true;
      this.cmdInput.placeholder = `Running command: ${cmd}...`;
      this.cmdInput.style.opacity = "0.6";
      this.cmdInput.style.cursor = "wait";
    }
    if (this.submitBtn) {
      this.submitBtn.disabled = true;
    }
    if (this.fsToolbar) {
      for (const btn of this.fsToolbar.querySelectorAll("button")) {
        btn.disabled = true;
      }
    }

    const isFsMode = this.currentMode === WORKSTATION_MODES.FS;
    const endpoint = isFsMode ? "/api/fs/execute" : "/api/cli/execute";
    let result = null;

    try {
      result = await this.ctx.postJson(endpoint, { command: cmd });
      if (result && result.output && !String(result.output).startsWith("Dispatched:")) {
        if (isFsMode) {
          this.appendFsText(result.output, false);
        } else {
          this.appendCliText(result.output, false);
          if (String(result.output).includes("Target CLI is not available")) {
            this.setWorkstationDisabled(
              true,
              "Target CLI is not available. Click ⚡ Sync to connect.",
            );
          }
        }
      }
      await this.pollOutputs();
      this.ctx.setTimeout(() => this.pollOutputs(), FOLLOWUP_POLL_SHORT_MS);
      this.ctx.setTimeout(() => this.pollOutputs(), FOLLOWUP_POLL_LONG_MS);
    } catch (err) {
      const msg = `[ERROR] Command failed: ${(err && err.message) || String(err)}`;
      if (isFsMode) {
        this.appendFsText(msg, false);
      } else {
        this.appendCliText(msg, false);
      }
    } finally {
      if (!this.workstationDisabled) {
        if (this.cmdInput) {
          this.cmdInput.disabled = false;
          this.cmdInput.placeholder = prevPlaceholder;
          this.cmdInput.style.opacity = "1";
          this.cmdInput.style.cursor = "text";
          this.cmdInput.focus();
        }
        if (this.submitBtn) {
          this.submitBtn.disabled = false;
        }
        if (this.fsToolbar) {
          for (const btn of this.fsToolbar.querySelectorAll("button")) {
            btn.disabled = false;
          }
        }
      }
    }

    return result;
  }

  /**
   * Handle an OSC-8 clickable command/action choice (`pass`, `fail`, `skip`, or shell command).
   *
   * @param {string} cmd - Extracted command string.
   */
  handleCommandClick(cmd) {
    const clean = String(cmd || "").trim();
    if (!clean) {
      return;
    }
    if (this.cmdInput) {
      this.cmdInput.value = clean;
      this.cmdInput.focus();
      this.cmdInput.select();
    }
  }

  /**
   * Ingest incremental output lines for either `#cliView` or `#fsView`.
   *
   * @param {Array<object>} lines - Array of `{ id, is_input, text }`.
   * @param {"cli"|"fs"} streamKind - Target stream identifier.
   */
  ingestStreamLines(lines, streamKind) {
    if (!Array.isArray(lines)) {
      return;
    }
    const isFs = streamKind === "fs";
    const viewEl = isFs ? this.fsView : this.cliView;
    if (viewEl && isSelectionInside(this.ctx, viewEl)) {
      return;
    }

    const lastSeenId = isFs ? this.lastFsId : this.lastCliId;
    const clearedUpTo = isFs ? this.clearedFsUpToId : this.clearedCliUpToId;

    if (lines.length === 0) {
      if (lastSeenId !== 0) {
        if (isFs) {
          this.fsEntries = [];
          this.lastFsId = 0;
          this.clearedFsUpToId = 0;
        } else {
          this.cliEntries = [];
          this.lastCliId = 0;
          this.clearedCliUpToId = 0;
        }
        clearChildren(viewEl);
      }
      return;
    }

    const normalized = lines.map((item, idx) => ({
      ...item,
      id: item.id ?? idx + 1,
    }));
    const firstId = normalized[0].id;
    const lastId = normalized[normalized.length - 1].id;

    let effectiveLastId = lastSeenId;
    let effectiveClearedId = clearedUpTo;
    if (lastId < lastSeenId) {
      effectiveLastId = 0;
      effectiveClearedId = 0;
      if (isFs) {
        this.fsEntries = [];
        this.clearedFsUpToId = 0;
      } else {
        this.cliEntries = [];
        this.clearedCliUpToId = 0;
      }
    }

    const visibleLines =
      effectiveClearedId > 0
        ? normalized.filter((item) => item.id > effectiveClearedId)
        : normalized;

    if (visibleLines.length === 0) {
      if (isFs) {
        this.lastFsId = lastId;
      } else {
        this.lastCliId = lastId;
      }
      return;
    }

    const appendLineDom = (entry) => {
      if (!viewEl) {
        return;
      }
      const lineEl = createEl(this.ctx, "div", {
        className: entry.is_input ? "log-line cli-line-input" : "log-line cli-line-output",
        dataset: { rawText: stripAnsi(entry.text || "") },
      });
      lineEl.innerHTML = renderTerminalHtml(entry.text || "");
      viewEl.appendChild(lineEl);
    };

    if (effectiveLastId === 0 || firstId > effectiveLastId) {
      if (isFs) {
        this.fsEntries = visibleLines;
        this.lastFsId = lastId;
      } else {
        this.cliEntries = visibleLines;
        this.lastCliId = lastId;
      }
      clearChildren(viewEl);
      for (const entry of visibleLines) {
        appendLineDom(entry);
      }
      if (viewEl) {
        viewEl.scrollTop = viewEl.scrollHeight;
      }
    } else if (lastId > effectiveLastId) {
      const appended = visibleLines.filter((item) => item.id > effectiveLastId);
      if (isFs) {
        this.fsEntries.push(...appended);
        this.lastFsId = lastId;
      } else {
        this.cliEntries.push(...appended);
        this.lastCliId = lastId;
      }
      for (const entry of appended) {
        appendLineDom(entry);
      }
      if (viewEl) {
        viewEl.scrollTop = viewEl.scrollHeight;
      }
    }
  }

  /**
   * Poll `/api/cli/output` and `/api/fs/output` and render new lines.
   *
   * @returns {Promise<void>}
   */
  async pollOutputs() {
    try {
      const cliLines = await this.ctx.fetchJson("/api/cli/output");
      this.ingestStreamLines(cliLines, "cli");
    } catch {
      // Non-fatal background poll failure.
    }

    try {
      const fsLines = await this.ctx.fetchJson("/api/fs/output");
      this.ingestStreamLines(fsLines, "fs");
    } catch {
      // Non-fatal background poll failure.
    }
  }

  /**
   * Clear both CLI and FS output views and record high-water IDs.
   */
  clear() {
    this.clearedCliUpToId = this.lastCliId;
    this.clearedFsUpToId = this.lastFsId;
    this.cliEntries = [];
    this.fsEntries = [];
    clearChildren(this.cliView);
    clearChildren(this.fsView);
  }

  /**
   * Open the `Ctrl+R` reverse fuzzy command history search popup.
   */
  openFuzzyHistory() {
    if (!this.fuzzyContainer || !this.fuzzySearchInput) {
      return;
    }
    this.isFuzzyOpen = true;
    this.fuzzyContainer.style.display = "flex";
    this.fuzzySearchInput.value = "";
    this.fuzzySelectedIndex = 0;
    this.updateFuzzyResults("");
    this.fuzzySearchInput.focus();
  }

  /**
   * Close the `Ctrl+R` reverse fuzzy command history search popup and restore `#cmdInput` focus.
   */
  closeFuzzyHistory() {
    this.isFuzzyOpen = false;
    if (this.fuzzyContainer) {
      this.fuzzyContainer.style.display = "none";
    }
    if (this.cmdInput && !this.cmdInput.disabled) {
      this.cmdInput.focus();
    }
  }

  /**
   * Recompute fuzzy history matches for `query` and update `#fuzzyResultsList`.
   *
   * @param {string} query - Subsequence filter query.
   * @returns {Array<string>} Matching commands in ranked order.
   */
  updateFuzzyResults(query) {
    const historyReversed = [];
    const seen = new Set();
    for (let idx = this.commandHistory.length - 1; idx >= 0; idx -= 1) {
      const cmd = this.commandHistory[idx];
      if (cmd && !seen.has(cmd)) {
        seen.add(cmd);
        historyReversed.push(cmd);
      }
    }

    const ranked = fuzzyFilterAndRank(query, historyReversed);
    this.currentFuzzyMatches = ranked.map((entry) => entry.item);

    if (this.fuzzySelectedIndex >= this.currentFuzzyMatches.length) {
      this.fuzzySelectedIndex = Math.max(0, this.currentFuzzyMatches.length - 1);
    }

    if (!this.fuzzyResultsList) {
      return this.currentFuzzyMatches;
    }

    clearChildren(this.fuzzyResultsList);
    if (this.currentFuzzyMatches.length === 0) {
      const emptyEl = createEl(this.ctx, "div", {
        className: "fuzzy-item",
        text: "No matching commands",
        style: { color: "var(--xerxes-text-dim, #3a626c)" },
      });
      this.fuzzyResultsList.appendChild(emptyEl);
      return this.currentFuzzyMatches;
    }

    this.currentFuzzyMatches.forEach((cmd, idx) => {
      const itemEl = createEl(this.ctx, "div", {
        className: idx === this.fuzzySelectedIndex ? "fuzzy-item active" : "fuzzy-item",
        text: cmd,
        dataset: { cmdIndex: String(idx) },
        on: {
          click: () => this.selectFuzzyCommand(cmd),
        },
      });
      this.fuzzyResultsList.appendChild(itemEl);
    });

    const activeEl = this.fuzzyResultsList.children[this.fuzzySelectedIndex];
    if (activeEl && typeof activeEl.scrollIntoView === "function") {
      activeEl.scrollIntoView({ block: "nearest" });
    }

    return this.currentFuzzyMatches;
  }

  /**
   * Populate `#cmdInput` with `cmd` and close the fuzzy history popup.
   *
   * @param {string} cmd - Selected command.
   */
  selectFuzzyCommand(cmd) {
    if (this.cmdInput) {
      this.cmdInput.value = String(cmd || "");
    }
    this.closeFuzzyHistory();
  }

  /**
   * Copy active workstation text (`#fsView` in FS mode, else `#cliView`) to clipboard.
   *
   * @param {Element|null} [buttonEl] - Optional trigger button for visual feedback.
   * @returns {Promise<boolean>}
   */
  async copyActiveOutput(buttonEl = this.copyBtn) {
    const isFs = this.currentMode === WORKSTATION_MODES.FS;
    const entries = isFs ? this.fsEntries : this.cliEntries;
    const viewEl = isFs ? this.fsView : this.cliView;
    const text =
      entries.length > 0
        ? entries.map((e) => stripAnsi(e.text || "")).join("\n")
        : (viewEl && viewEl.textContent) || "";
    const ok = await this.ctx.copyTextToClipboard(text);
    if (buttonEl) {
      const orig = buttonEl.textContent;
      buttonEl.textContent = ok ? "✓ Copied!" : "Error";
      this.ctx.setTimeout(() => {
        buttonEl.textContent = orig;
      }, COPY_FEEDBACK_DURATION_MS);
    }
    return ok;
  }

  /**
   * Unbind all DOM listeners.
   */
  destroy() {
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating a `CliConsoleController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {CliConsoleController}
 */
export function createCliConsole(ctx, options = {}) {
  return new CliConsoleController(ctx, options);
}
