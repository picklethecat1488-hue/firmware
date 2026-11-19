/**
 * On-Device Test Mode UI Controller (`tools/host.py` — `ACTIVE MODE: TEST`).
 *
 * Supersedes the legacy bringup workstation with `@device_test` discovery,
 * attribute-based filtering (`project`, `scenarios`, `boards` glob, `target_binary`,
 * `manual`, text search), multi-test selection, top-to-bottom `pytest` execution,
 * live status/console streaming, and interactive manual operator prompt responses
 * (`YES`, `NO`, numeric `measurement`, and `cancel` with required comment).
 */

import { stripAnsi } from "../core/ansi.js";
import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { showToast } from "../core/toast.js";
import { renderTerminalHtml } from "./rtt_stream.js";

export const TEST_RUN_STEPS = Object.freeze({
  IDLE: "idle",
  RUNNING: "running",
  COMPLETE: "complete",
  FAILED: "failed",
  CANCELLED: "cancelled",
});

export const MANUAL_PROMPT_ACTIONS = Object.freeze({
  YES: "yes",
  NO: "no",
  MEASUREMENT: "measurement",
  CANCEL: "cancel",
});

export const MANUAL_PROMPT_KINDS = Object.freeze({
  YES_NO: "yes_no",
  MEASUREMENT: "measurement",
});

const CONSOLE_SCROLL_SLACK_PX = 30;
const COPY_FEEDBACK_DURATION_MS = 1500;

/**
 * Map a per-test status string to a Xerxes badge CSS class.
 *
 * @param {string} status - Test outcome or lifecycle label.
 * @returns {string}
 */
export function statusBadgeClass(status) {
  const upper = String(status || "IDLE").toUpperCase();
  if (upper === "PASSED") {
    return "badge badge-green";
  }
  if (upper === "FAILED") {
    return "badge test-badge-red";
  }
  if (upper === "RUNNING" || upper === "SKIPPED" || upper === "CANCELLED") {
    return "badge badge-amber";
  }
  return "badge";
}

/**
 * Controller for the `#testView` On-Device Test Runner workstation mode.
 */
export class TestRunnerController {
  /**
   * Construct a `TestRunnerController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Optional element overrides.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.tests = [];
    this.facets = { projects: [], scenarios: [], binaries: [], boards: [] };
    this.detectedBoard = null;
    this.selectedNodeIds = new Set();
    this.resultsMap = {};
    this.consoleLines = [];
    this.isRunning = false;
    this.activeStep = TEST_RUN_STEPS.IDLE;
    this.activePrompt = null;
    this._lastPromptId = "";
    this._initializedScan = false;
    this._unsubscribers = [];

    this.viewEl = resolveElement(ctx, options.viewEl || "testView");
    this.projectSelect = resolveElement(ctx, options.projectSelect || "testFilterProject");
    this.scenarioSelect = resolveElement(ctx, options.scenarioSelect || "testFilterScenario");
    this.boardInput = resolveElement(ctx, options.boardInput || "testFilterBoard");
    this.binarySelect = resolveElement(ctx, options.binarySelect || "testFilterBinary");
    this.manualSelect = resolveElement(ctx, options.manualSelect || "testFilterManual");
    this.searchInput = resolveElement(ctx, options.searchInput || "testFilterSearch");
    this.enableManualCb = resolveElement(ctx, options.enableManualCb || "testEnableManualCb");

    this.scanBtn = resolveElement(ctx, options.scanBtn || "btnTestScan");
    this.runSelectedBtn = resolveElement(ctx, options.runSelectedBtn || "btnTestRunSelected");
    this.runFilteredBtn = resolveElement(ctx, options.runFilteredBtn || "btnTestRunFiltered");
    this.cancelBtn = resolveElement(ctx, options.cancelBtn || "btnTestCancel");
    this.selectAllCb = resolveElement(ctx, options.selectAllCb || "testSelectAllCb");

    this.boardBadge = resolveElement(ctx, options.boardBadge || "testDetectedBoardBadge");
    this.summaryStats = resolveElement(ctx, options.summaryStats || "testSummaryStats");
    this.tableBody = resolveElement(ctx, options.tableBody || "testTableBody");
    this.consoleOutput = resolveElement(ctx, options.consoleOutput || "testConsoleOutput");

    // Interactive Manual Prompt Card
    this.manualCard = resolveElement(ctx, options.manualCard || "testManualPromptCard");
    this.manualTitle = resolveElement(ctx, options.manualTitle || "testManualTitle");
    this.manualProcedure = resolveElement(ctx, options.manualProcedure || "testManualProcedure");
    this.manualPromptText = resolveElement(ctx, options.manualPromptText || "testManualPromptText");
    this.manualYesNoRow = resolveElement(ctx, options.manualYesNoRow || "testManualYesNoRow");
    this.manualYesBtn = resolveElement(ctx, options.manualYesBtn || "btnManualYes");
    this.manualNoBtn = resolveElement(ctx, options.manualNoBtn || "btnManualNo");
    this.manualMeasurementRow = resolveElement(
      ctx,
      options.manualMeasurementRow || "testManualMeasurementRow",
    );
    this.manualValueInput = resolveElement(ctx, options.manualValueInput || "testManualValueInput");
    this.manualUnitLabel = resolveElement(ctx, options.manualUnitLabel || "testManualUnitLabel");
    this.manualSubmitValueBtn = resolveElement(
      ctx,
      options.manualSubmitValueBtn || "btnManualSubmitValue",
    );
    this.manualCommentInput = resolveElement(
      ctx,
      options.manualCommentInput || "testManualCommentInput",
    );
    this.manualCancelBtn = resolveElement(ctx, options.manualCancelBtn || "btnManualCancel");
    this.manualErrorMsg = resolveElement(ctx, options.manualErrorMsg || "testManualErrorMsg");
  }

  /**
   * Bind DOM event listeners for scanning, attribute filters, selection, execution, and manual prompts.
   */
  init() {
    const bindClick = (el, handler) => {
      if (!el) {
        return;
      }
      el.addEventListener("click", handler);
      this._unsubscribers.push(() => el.removeEventListener("click", handler));
    };

    const bindChange = (el, handler) => {
      if (!el) {
        return;
      }
      el.addEventListener("change", handler);
      this._unsubscribers.push(() => el.removeEventListener("change", handler));
    };

    bindClick(this.scanBtn, () => this.scanTests({ probeBoard: true }));
    bindClick(this.runSelectedBtn, () => this.runSelected());
    bindClick(this.runFilteredBtn, () => this.runFiltered());
    bindClick(this.cancelBtn, () => this.cancelRun());

    for (const sel of [this.projectSelect, this.scenarioSelect, this.binarySelect, this.manualSelect]) {
      bindChange(sel, () => this.scanTests({ probeBoard: false }));
    }

    if (this.boardInput) {
      const onBoardChange = () => this.scanTests({ probeBoard: false });
      this.boardInput.addEventListener("change", onBoardChange);
      this._unsubscribers.push(() =>
        this.boardInput.removeEventListener("change", onBoardChange),
      );
    }

    if (this.searchInput) {
      const onSearchInput = () => this.scanTests({ probeBoard: false });
      this.searchInput.addEventListener("input", onSearchInput);
      this._unsubscribers.push(() =>
        this.searchInput.removeEventListener("input", onSearchInput),
      );
    }

    if (this.selectAllCb) {
      bindChange(this.selectAllCb, () => {
        this.setSelectAllVisible(Boolean(this.selectAllCb.checked));
      });
    }

    if (this.tableBody) {
      this._unsubscribers.push(
        bindDelegatedEvent(this.tableBody, "change", "[data-test-select]", (_evt, cb) => {
          const nodeId = cb.getAttribute("data-test-select");
          if (nodeId) {
            this.toggleSelection(nodeId, Boolean(cb.checked));
          }
        }),
      );

      this._unsubscribers.push(
        bindDelegatedEvent(this.tableBody, "click", "[data-test-run-one]", (evt, btn) => {
          evt.preventDefault();
          const nodeId = btn.getAttribute("data-test-run-one");
          if (nodeId) {
            this.runNodes([nodeId]);
          }
        }),
      );
    }

    bindClick(this.manualYesBtn, () => this.submitManualResponse(MANUAL_PROMPT_ACTIONS.YES));
    bindClick(this.manualNoBtn, () => this.submitManualResponse(MANUAL_PROMPT_ACTIONS.NO));
    bindClick(this.manualSubmitValueBtn, () =>
      this.submitManualResponse(MANUAL_PROMPT_ACTIONS.MEASUREMENT),
    );
    bindClick(this.manualCancelBtn, () => this.submitManualResponse(MANUAL_PROMPT_ACTIONS.CANCEL));

    if (this.manualValueInput) {
      const onValueKeyDown = (event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          this.submitManualResponse(MANUAL_PROMPT_ACTIONS.MEASUREMENT);
        }
      };
      this.manualValueInput.addEventListener("keydown", onValueKeyDown);
      this._unsubscribers.push(() =>
        this.manualValueInput.removeEventListener("keydown", onValueKeyDown),
      );
    }

    this._syncActionButtons();
  }

  /**
   * Ensure an initial scan has run when entering `TEST` mode.
   *
   * @returns {Promise<void>}
   */
  async ensureInitialized() {
    if (!this._initializedScan) {
      this._initializedScan = true;
      await this.scanTests({ probeBoard: true });
      await this.pollStatus();
    }
  }

  /**
   * Read the active attribute filter state from the toolbar controls.
   *
   * @returns {object}
   */
  getCurrentFilters() {
    return {
      project: (this.projectSelect && this.projectSelect.value) || "all",
      scenario: (this.scenarioSelect && this.scenarioSelect.value) || "all",
      board: ((this.boardInput && this.boardInput.value) || "auto").trim() || "auto",
      binary: (this.binarySelect && this.binarySelect.value) || "all",
      manual: (this.manualSelect && this.manualSelect.value) || "all",
      query: ((this.searchInput && this.searchInput.value) || "").trim(),
    };
  }

  /**
   * Scan `@device_test` suites via `/api/tests/scan` or `/api/tests` with active attribute filters.
   *
   * @param {object} [options] - `{ probeBoard: boolean }`.
   * @returns {Promise<object|null>}
   */
  async scanTests(options = {}) {
    const probeBoard = Boolean(options.probeBoard);
    const filters = this.getCurrentFilters();

    try {
      const data = await this.ctx.postJson("/api/tests/scan", {
        ...filters,
        probe_board: probeBoard,
      });

      if (!data) {
        return null;
      }

      this._initializedScan = true;
      this.tests = Array.isArray(data.tests) ? data.tests : [];
      if (data.facets && typeof data.facets === "object") {
        this.facets = data.facets;
        this._populateDynamicFacetSelects();
      }
      if ("detected_board" in data) {
        this.detectedBoard = data.detected_board;
      }

      for (const item of this.tests) {
        if (item.last_result && item.node_id && !this.resultsMap[item.node_id]) {
          this.resultsMap[item.node_id] = item.last_result;
        }
      }

      const visibleSet = new Set(this.tests.map((t) => t.node_id));
      for (const selectedId of this.selectedNodeIds) {
        if (!visibleSet.has(selectedId)) {
          this.selectedNodeIds.delete(selectedId);
        }
      }

      this._renderBoardBadge();
      this._renderTestTable();
      this._syncActionButtons();
      return data;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to scan device tests: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Populate `#testFilterProject` and `#testFilterBinary` options from discovered facets.
   */
  _populateDynamicFacetSelects() {
    this._syncFacetSelect(this.projectSelect, this.facets.projects || [], "All Projects");
    this._syncFacetSelect(this.binarySelect, this.facets.binaries || [], "All Binaries");
  }

  /**
   * Synchronize a `<select>` element's options while preserving its selected value.
   *
   * @param {Element|null} selectEl - Target `<select>` element.
   * @param {Array<string>} values - Discovered facet values.
   * @param {string} allLabel - Display label for `"all"`.
   */
  _syncFacetSelect(selectEl, values, allLabel) {
    if (!selectEl) {
      return;
    }
    const prevValue = selectEl.value || "all";
    clearChildren(selectEl);

    const allOpt = createEl(this.ctx, "option", {
      text: allLabel,
      attrs: { value: "all" },
    });
    allOpt.value = "all";
    selectEl.appendChild(allOpt);

    for (const val of values) {
      const opt = createEl(this.ctx, "option", {
        text: val,
        attrs: { value: val },
      });
      opt.value = val;
      if (val === prevValue) {
        opt.selected = true;
      }
      selectEl.appendChild(opt);
    }

    selectEl.value = values.includes(prevValue) ? prevValue : "all";
  }

  /**
   * Render the attached board badge (`#testDetectedBoardBadge`).
   */
  _renderBoardBadge() {
    if (!this.boardBadge) {
      return;
    }
    if (this.detectedBoard && this.detectedBoard.board_type) {
      this.boardBadge.textContent = `BOARD: ${this.detectedBoard.board_type} (${this.detectedBoard.chip})`;
      this.boardBadge.className = "badge badge-green";
    } else {
      this.boardBadge.textContent = "BOARD: NOT DETECTED";
      this.boardBadge.className = "badge badge-amber";
    }
  }

  /**
   * Toggle selection state of a single `nodeId`.
   *
   * @param {string} nodeId - Pytest node ID.
   * @param {boolean} selected - Desired selection flag.
   */
  toggleSelection(nodeId, selected) {
    if (selected) {
      this.selectedNodeIds.add(nodeId);
    } else {
      this.selectedNodeIds.delete(nodeId);
    }
    this._syncSelectAllCheckbox();
    this._syncActionButtons();
  }

  /**
   * Select or deselect all currently visible filtered tests.
   *
   * @param {boolean} selectAll - True to select every visible row.
   */
  setSelectAllVisible(selectAll) {
    if (selectAll) {
      for (const item of this.tests) {
        this.selectedNodeIds.add(item.node_id);
      }
    } else {
      for (const item of this.tests) {
        this.selectedNodeIds.delete(item.node_id);
      }
    }
    this._renderTestTable();
    this._syncActionButtons();
  }

  /**
   * Keep `#testSelectAllCb` synchronized with row selections.
   */
  _syncSelectAllCheckbox() {
    if (!this.selectAllCb) {
      return;
    }
    const allVisibleSelected =
      this.tests.length > 0 &&
      this.tests.every((item) => this.selectedNodeIds.has(item.node_id));
    this.selectAllCb.checked = allVisibleSelected;
  }

  /**
   * Render the `@device_test` discovery and status table (`#testTableBody`).
   */
  _renderTestTable() {
    if (!this.tableBody) {
      return;
    }
    clearChildren(this.tableBody);

    if (this.tests.length === 0) {
      const emptyRow = createEl(this.ctx, "tr", {
        children: [
          createEl(this.ctx, "td", {
            text: "No @device_test cases match the active attribute filters.",
            attrs: { colspan: "8" },
            style: {
              color: "var(--xerxes-text-dim)",
              textAlign: "center",
              padding: "14px",
            },
          }),
        ],
      });
      this.tableBody.appendChild(emptyRow);
      this._syncSelectAllCheckbox();
      return;
    }

    for (const item of this.tests) {
      const nodeId = String(item.node_id || "");
      const isSelected = this.selectedNodeIds.has(nodeId);
      const runInfo = this.resultsMap[nodeId] || item.last_result || null;
      const statusLabel = runInfo && runInfo.status ? String(runInfo.status) : "IDLE";
      const durationLabel =
        runInfo && typeof runInfo.duration_s === "number"
          ? ` (${runInfo.duration_s.toFixed(2)}s)`
          : "";

      const rowCb = createEl(this.ctx, "input", {
        attrs: {
          type: "checkbox",
          "data-test-select": nodeId,
        },
      });
      rowCb.checked = isSelected;

      const nameBlockChildren = [
        createEl(this.ctx, "div", {
          text: item.name,
          style: { fontWeight: "bold", color: "var(--xerxes-cyan)" },
        }),
      ];
      if (item.description) {
        nameBlockChildren.push(
          createEl(this.ctx, "div", {
            text: item.description,
            style: {
              color: "var(--xerxes-text-muted)",
              fontSize: "8px",
              marginTop: "2px",
              whiteSpace: "normal",
            },
          }),
        );
      }
      if (runInfo && runInfo.detail) {
        nameBlockChildren.push(
          createEl(this.ctx, "div", {
            text: runInfo.detail,
            style: {
              color: statusLabel === "FAILED" ? "var(--xerxes-red)" : "var(--xerxes-amber)",
              fontSize: "8px",
              marginTop: "2px",
              whiteSpace: "normal",
            },
          }),
        );
      }

      const scenarioBadges = Array.isArray(item.scenarios)
        ? item.scenarios.map((scen) =>
            createEl(this.ctx, "span", {
              className: "badge",
              text: String(scen),
            }),
          )
        : [];
      const boardsText = Array.isArray(item.boards) ? item.boards.join(", ") : "*";
      const binText = item.target_binary || "none";
      const modeBadge = createEl(this.ctx, "span", {
        className: item.manual ? "badge badge-amber" : "badge badge-cyan",
        text: item.manual ? "MANUAL" : "AUTO",
      });

      const statusBadge = createEl(this.ctx, "span", {
        className: statusBadgeClass(statusLabel),
        text: `${statusLabel}${durationLabel}`,
      });

      const quickRunBtn = createEl(this.ctx, "button", {
        className: "btn btn-cyan",
        text: "▶",
        attrs: {
          type: "button",
          title: `Run ${item.name}`,
          "data-test-run-one": nodeId,
        },
        style: { padding: "2px 6px", flexShrink: "0" },
      });
      quickRunBtn.disabled = this.isRunning;

      const rowEl = createEl(this.ctx, "tr", {
        dataset: { nodeId },
        children: [
          createEl(this.ctx, "td", { children: [rowCb] }),
          createEl(this.ctx, "td", { text: String(item.order || "") }),
          createEl(this.ctx, "td", { children: nameBlockChildren }),
          createEl(this.ctx, "td", { text: item.project || "" }),
          createEl(this.ctx, "td", {
            children: [
              createEl(this.ctx, "div", {
                className: "test-badge-group",
                children: [modeBadge, ...scenarioBadges],
              }),
            ],
          }),
          createEl(this.ctx, "td", { text: boardsText }),
          createEl(this.ctx, "td", { text: binText }),
          createEl(this.ctx, "td", {
            children: [
              createEl(this.ctx, "div", {
                className: "test-badge-group",
                style: { justifyContent: "space-between", flexWrap: "nowrap", gap: "6px" },
                children: [statusBadge, quickRunBtn],
              }),
            ],
          }),
        ],
      });
      this.tableBody.appendChild(rowEl);
    }

    this._syncSelectAllCheckbox();
  }

  /**
   * Update button labels/disabled flags and summary counters (`#testSummaryStats`).
   */
  _syncActionButtons() {
    const selectedCount = this.selectedNodeIds.size;
    if (this.runSelectedBtn) {
      this.runSelectedBtn.textContent = `▶ Run Selected (${selectedCount})`;
      this.runSelectedBtn.disabled = this.isRunning || selectedCount === 0;
    }
    if (this.runFilteredBtn) {
      this.runFilteredBtn.textContent = `▶ Run Filtered (${this.tests.length})`;
      this.runFilteredBtn.disabled = this.isRunning || this.tests.length === 0;
    }
    if (this.cancelBtn) {
      this.cancelBtn.disabled = !this.isRunning;
    }
    if (this.scanBtn) {
      this.scanBtn.disabled = this.isRunning;
    }

    if (this.summaryStats) {
      const results = Object.values(this.resultsMap);
      const passed = results.filter((r) => r.status === "PASSED").length;
      const failed = results.filter((r) => r.status === "FAILED").length;
      const skipped = results.filter((r) => r.status === "SKIPPED").length;
      this.summaryStats.textContent = `${this.tests.length} Shown | ${selectedCount} Selected | ${passed} Passed • ${failed} Failed • ${skipped} Skipped [${this.activeStep.toUpperCase()}]`;
    }
  }

  /**
   * Execute only the checked `@device_test` rows in top-to-bottom listing order.
   *
   * @returns {Promise<object|null>}
   */
  async runSelected() {
    const orderedIds = this.tests
      .map((item) => item.node_id)
      .filter((nid) => this.selectedNodeIds.has(nid));
    if (orderedIds.length === 0) {
      return null;
    }
    return this.runNodes(orderedIds);
  }

  /**
   * Execute all currently visible filtered `@device_test` rows in top-to-bottom listing order.
   *
   * @returns {Promise<object|null>}
   */
  async runFiltered() {
    const orderedIds = this.tests.map((item) => item.node_id);
    if (orderedIds.length === 0) {
      return null;
    }
    return this.runNodes(orderedIds);
  }

  /**
   * Start an asynchronous test run via `POST /api/tests/run`.
   *
   * @param {Array<string>} nodeIds - Ordered pytest node IDs to execute.
   * @returns {Promise<object|null>}
   */
  async runNodes(nodeIds) {
    if (!Array.isArray(nodeIds) || nodeIds.length === 0) {
      return null;
    }
    const filters = this.getCurrentFilters();
    const enableManual = this.enableManualCb ? Boolean(this.enableManualCb.checked) : true;
    const boardArg = filters.board === "*" ? "auto" : filters.board;

    try {
      const res = await this.ctx.postJson("/api/tests/run", {
        node_ids: nodeIds,
        board: boardArg,
        enable_manual: enableManual,
        filters,
      });
      if (res && res.status) {
        this._applyStatusSnapshot(res.status);
      } else {
        await this.pollStatus();
      }
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to start test run: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Cancel the active `pytest` run via `POST /api/tests/cancel`.
   *
   * @returns {Promise<object|null>}
   */
  async cancelRun() {
    try {
      const res = await this.ctx.postJson("/api/tests/cancel", {});
      if (res && res.status) {
        this._applyStatusSnapshot(res.status);
      } else {
        await this.pollStatus();
      }
      return res;
    } catch {
      return null;
    }
  }

  /**
   * Poll `GET /api/tests/status` and update live table badges, console output, and manual prompt UI.
   *
   * @returns {Promise<object|null>}
   */
  async pollStatus() {
    try {
      const snapshot = await this.ctx.fetchJson("/api/tests/status");
      if (snapshot && typeof snapshot === "object") {
        this._applyStatusSnapshot(snapshot);
      }
      return snapshot;
    } catch {
      return null;
    }
  }

  /**
   * Apply a `/api/tests/status` snapshot to local state and DOM elements.
   *
   * @param {object} snapshot - Status payload from `DeviceTestRunnerManager`.
   */
  _applyStatusSnapshot(snapshot) {
    this.isRunning = Boolean(snapshot.active);
    this.activeStep = String(snapshot.step || TEST_RUN_STEPS.IDLE);

    if (snapshot.detected_board) {
      this.detectedBoard = snapshot.detected_board;
      this._renderBoardBadge();
    }

    if (snapshot.results_map && typeof snapshot.results_map === "object") {
      this.resultsMap = {
        ...this.resultsMap,
        ...snapshot.results_map,
      };
    } else if (Array.isArray(snapshot.results)) {
      for (const rec of snapshot.results) {
        if (rec && rec.node_id) {
          this.resultsMap[rec.node_id] = rec;
        }
      }
    }

    if (Array.isArray(snapshot.console)) {
      this.consoleLines = snapshot.console;
      this._renderConsole();
    }

    this.renderManualPrompt(snapshot.manual_prompt || null);
    this._renderTestTable();
    this._syncActionButtons();
  }

  /**
   * Render live `pytest` output inside `#testConsoleOutput`.
   */
  _renderConsole() {
    if (!this.consoleOutput) {
      return;
    }
    const shouldScroll =
      this.consoleOutput.scrollTop + this.consoleOutput.clientHeight >=
      this.consoleOutput.scrollHeight - CONSOLE_SCROLL_SLACK_PX;

    if (this.consoleLines.length === 0) {
      this.consoleOutput.innerHTML =
        '<span class="term-dim">&gt; Select tests above and click ▶ Run Selected or ▶ Run Filtered.</span>';
      return;
    }

    this.consoleOutput.innerHTML = this.consoleLines
      .map((line) => renderTerminalHtml(line))
      .join("<br>");
    if (shouldScroll) {
      this.consoleOutput.scrollTop = this.consoleOutput.scrollHeight;
    }
  }

  /**
   * Render or hide the interactive Manual Test Prompt Card (`#testManualPromptCard`).
   *
   * @param {object|null} promptData - Active manual prompt dictionary from `host.py`.
   */
  renderManualPrompt(promptData) {
    this.activePrompt = promptData && promptData.active ? promptData : null;
    if (!this.manualCard) {
      return;
    }

    if (!this.activePrompt) {
      this.manualCard.style.display = "none";
      this._lastPromptId = "";
      return;
    }

    const promptId = String(this.activePrompt.prompt_id || "");
    const isNewPrompt = promptId !== this._lastPromptId;
    this._lastPromptId = promptId;

    this.manualCard.style.display = "flex";
    if (this.manualTitle) {
      this.manualTitle.textContent = `[${this.activePrompt.project || "device"}] ${this.activePrompt.test_name || "manual_test"}`;
    }
    if (this.manualProcedure) {
      this.manualProcedure.textContent = this.activePrompt.procedure || "";
    }
    if (this.manualPromptText) {
      this.manualPromptText.textContent = this.activePrompt.prompt || "";
    }

    const isMeasurement = this.activePrompt.kind === MANUAL_PROMPT_KINDS.MEASUREMENT;
    if (this.manualYesNoRow) {
      this.manualYesNoRow.style.display = isMeasurement ? "none" : "flex";
    }
    if (this.manualMeasurementRow) {
      this.manualMeasurementRow.style.display = isMeasurement ? "flex" : "none";
    }
    if (this.manualUnitLabel) {
      this.manualUnitLabel.textContent = this.activePrompt.unit ? `(${this.activePrompt.unit})` : "";
    }

    if (isNewPrompt) {
      if (this.manualValueInput) {
        this.manualValueInput.value = "";
      }
      if (this.manualCommentInput) {
        this.manualCommentInput.value = "";
      }
      if (this.manualErrorMsg) {
        this.manualErrorMsg.style.display = "none";
        this.manualErrorMsg.textContent = "";
      }
    }
  }

  /**
   * Show an inline validation error inside `#testManualPromptCard`.
   *
   * @param {string} message - Validation error text.
   */
  _showManualError(message) {
    if (this.manualErrorMsg) {
      this.manualErrorMsg.style.display = "block";
      this.manualErrorMsg.textContent = message;
    }
  }

  /**
   * Submit an operator decision (`yes`, `no`, `measurement`, or `cancel`) to `POST /api/manual_prompt/respond`.
   *
   * @param {string} action - One of `MANUAL_PROMPT_ACTIONS`.
   * @returns {Promise<object|null>}
   */
  async submitManualResponse(action) {
    if (!this.activePrompt) {
      return null;
    }
    const rawValue = this.manualValueInput ? String(this.manualValueInput.value || "").trim() : "";
    const comment = this.manualCommentInput
      ? String(this.manualCommentInput.value || "").trim()
      : "";

    if (action === MANUAL_PROMPT_ACTIONS.MEASUREMENT && !rawValue) {
      this._showManualError("Enter a numeric measured value before submitting.");
      return null;
    }
    if (action === MANUAL_PROMPT_ACTIONS.CANCEL && !comment) {
      this._showManualError("Enter a cancellation comment before cancelling the manual test.");
      return null;
    }

    try {
      const res = await this.ctx.postJson("/api/manual_prompt/respond", {
        prompt_id: this.activePrompt.prompt_id || "",
        action,
        value: rawValue,
        comment,
      });
      if (this.manualErrorMsg) {
        this.manualErrorMsg.style.display = "none";
        this.manualErrorMsg.textContent = "";
      }
      if (res && res.manual_prompt) {
        this.renderManualPrompt(res.manual_prompt);
      }
      return res;
    } catch (err) {
      this._showManualError((err && err.message) || String(err));
      return null;
    }
  }

  /**
   * Copy current test console output to clipboard.
   *
   * @param {Element|null} [buttonEl] - Optional trigger button for visual feedback.
   * @returns {Promise<boolean>}
   */
  async copyConsoleOutput(buttonEl = null) {
    const text = this.consoleLines.map((line) => stripAnsi(line)).join("\n");
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
   * Clear local test console output lines.
   */
  clearConsole() {
    this.consoleLines = [];
    this._renderConsole();
  }

  /**
   * Unbind all DOM event listeners.
   */
  destroy() {
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating a `TestRunnerController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {TestRunnerController}
 */
export function createTestRunner(ctx, options = {}) {
  return new TestRunnerController(ctx, options);
}
