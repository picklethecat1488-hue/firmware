/**
 * Xerxes Host Daemon Workstation Root Application Controller (`tools/host.py`).
 *
 * Orchestrates `RttStreamController`, `CliConsoleController`,
 * `BringupRunnerController`, `TargetConfigController`, `WormReporterController`,
 * Retina-safe `--ui-scale` controls, split-pane resizing, and keyboard shortcuts
 * using an explicit `RuntimeContext` with zero browser globals.
 */

import { applyIframeContextClass, resolveElement } from "../core/dom.js";
import { setupPanelResizer } from "../core/resizer.js";
import { getRuntimeContext } from "../core/runtime_context.js";
import { applyUiScale } from "../core/ui_scale.js";
import { createCliConsole, WORKSTATION_MODES } from "./cli_console.js";
import { createRttStream } from "./rtt_stream.js";
import { createTargetConfig } from "./target_config.js";
import { createTestRunner, LEFT_PANEL_TABS } from "./test_runner.js";
import { createWormReporter } from "./worm_reporter.js";

export const UI_SCALES = Object.freeze([
  "75%",
  "80%",
  "90%",
  "100%",
  "110%",
  "125%",
  "150%",
]);
export const UI_SCALE_STORAGE_KEY = "ui_scale";
const LOG_PANEL_WIDTH_STORAGE_KEY = "host_log_panel_width_px";
const DEFAULT_UI_SCALE = "100%";
const OUTPUT_POLL_INTERVAL_MS = 1000;
const MIN_LOG_PANE_WIDTH_PX = 160;
const MAX_LOG_PANE_WIDTH_PX = 1100;

/**
 * Root workstation application wiring all `daemon/` and `core/` controllers.
 */
export class DaemonApp {
  /**
   * Construct a `DaemonApp`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   */
  constructor(ctx) {
    this.ctx = ctx;
    this.bootConfig = ctx.getBootConfig();
    this.pollTimer = null;
    this._openedLeftPanelForTestMode = false;
    this._unsubscribers = [];

    const initialMode = String(
      this.bootConfig.activeMode || this.bootConfig.active_mode || WORKSTATION_MODES.CLI,
    ).toUpperCase();

    this.rttStream = createRttStream(ctx, {
      onCommandClick: (cmd) => this.cliConsole.handleCommandClick(cmd),
    });

    this.testRunner = createTestRunner(ctx);

    this.cliConsole = createCliConsole(ctx, {
      initialMode,
      onModeChange: async (mode) => {
        await this._syncLeftPanelForMode(mode);
      },
    });

    this.targetConfig = createTargetConfig(ctx, {
      bootConfig: this.bootConfig,
      onWorkstationDisabledChange: (disabled, msg) =>
        this.cliConsole.setWorkstationDisabled(disabled, msg),
      onShowSystemLogsChange: (show) => {
        this.rttStream.togglePanel(Boolean(show));
      },
      onSyncComplete: () => {
        this.pollAll();
      },
    });

    this.wormReporter = createWormReporter(ctx, {
      getTargetConfig: () => ({
        ...this.targetConfig.getCurrentConfig(),
        mode: this.cliConsole.getMode(),
      }),
      getCommandHistory: () => this.cliConsole.getCommandHistory(),
    });

    this.uiScaleSelect = resolveElement(ctx, "uiScaleSelect");
    this.toggleLogBtn = resolveElement(ctx, "btnToggleLog");
    this.collapseLogBtn = resolveElement(ctx, "btnCollapseLog");
    this.clearOutputBtn = resolveElement(ctx, "btnClearOutput");
    this.copyLogBtn = resolveElement(ctx, "btnCopyLog");
    this.copyWorkstationBtn = resolveElement(ctx, "btnCopyWorkstation");
    this.manualPromptBanner = resolveElement(ctx, "manualPromptBanner");
    this.manualPromptSummary = resolveElement(ctx, "manualPromptSummary");
  }

  /**
   * Initialize all workstation subsystems, shortcuts, and background polling.
   *
   * @returns {Promise<DaemonApp>}
   */
  async init() {
    applyIframeContextClass(this.ctx);
    this._initUiScale();
    this._bindGlobalControls();

    const initialShowLogs = Boolean(
      this.bootConfig.showSystemLogs ?? this.bootConfig.show_system_logs,
    );
    this.rttStream.init({ initialShowLogs });
    if (this.rttStream.isPanelVisible() && this.targetConfig.cfgShowLogsCheckbox) {
      this.targetConfig.cfgShowLogsCheckbox.checked = true;
    }

    const cleanupResizer = setupPanelResizer(
      this.ctx,
      "vSplitter",
      "logPanel",
      LOG_PANEL_WIDTH_STORAGE_KEY,
      {
        minSize: MIN_LOG_PANE_WIDTH_PX,
        maxSize: MAX_LOG_PANE_WIDTH_PX,
      },
    );
    this._unsubscribers.push(cleanupResizer);

    this.wormReporter.init();
    this.testRunner.init();
    await this.cliConsole.init();
    await this._syncLeftPanelForMode(this.cliConsole.getMode());
    await this.targetConfig.init();

    await this.pollAll();

    this.pollTimer = this.ctx.setInterval(() => {
      this.pollAll();
    }, OUTPUT_POLL_INTERVAL_MS);

    return this;
  }

  /**
   * Initialize UI scale from storage and bind `#uiScaleSelect`.
   */
  _initUiScale() {
    const saved = this.ctx.getStorage().getItem(UI_SCALE_STORAGE_KEY) || DEFAULT_UI_SCALE;
    this.setUiScale(saved);

    if (this.uiScaleSelect) {
      const onChange = () => this.setUiScale(this.uiScaleSelect.value);
      this.uiScaleSelect.addEventListener("change", onChange);
      this._unsubscribers.push(() =>
        this.uiScaleSelect.removeEventListener("change", onChange),
      );
    }
  }

  /**
   * Apply a percentage UI scale label (`"75%"`..`"150%"`) via Retina-safe `--ui-scale`.
   *
   * @param {string} rawScaleLabel - Percentage label (e.g. `"125%"`).
   * @returns {number} Applied scale multiplier.
   */
  setUiScale(rawScaleLabel) {
    const normalizedLabel = UI_SCALES.includes(rawScaleLabel)
      ? rawScaleLabel
      : DEFAULT_UI_SCALE;
    const numericPercent = Number.parseInt(normalizedLabel, 10) || 100;
    const factor = applyUiScale(this.ctx, numericPercent, {
      storageKey: UI_SCALE_STORAGE_KEY,
      persistValue: normalizedLabel,
    });
    if (this.uiScaleSelect && this.uiScaleSelect.value !== normalizedLabel) {
      this.uiScaleSelect.value = normalizedLabel;
    }
    return factor;
  }

  /**
   * Step UI scale up or down by `delta` increments along `UI_SCALES`.
   *
   * @param {number} delta - Step offset (`+1` or `-1`).
   * @returns {number} Applied scale multiplier.
   */
  adjustUiScale(delta) {
    const current =
      this.ctx.getStorage().getItem(UI_SCALE_STORAGE_KEY) || DEFAULT_UI_SCALE;
    let idx = UI_SCALES.indexOf(current);
    if (idx === -1) {
      idx = UI_SCALES.indexOf(DEFAULT_UI_SCALE);
    }
    const nextIdx = Math.max(0, Math.min(UI_SCALES.length - 1, idx + delta));
    return this.setUiScale(UI_SCALES[nextIdx]);
  }

  /**
   * Reset UI scale to `100%`.
   *
   * @returns {number}
   */
  resetUiScale() {
    return this.setUiScale(DEFAULT_UI_SCALE);
  }

  /**
   * Synchronize left-panel visibility and active tab (`TESTS` vs `LOGS`) when workstation mode changes.
   *
   * @param {string} mode - Active workstation mode (`"CLI"`, `"FS"`, or `"TEST"`).
   * @returns {Promise<void>}
   */
  async _syncLeftPanelForMode(mode) {
    if (mode === WORKSTATION_MODES.TEST) {
      if (!this.rttStream.isPanelVisible()) {
        this._openedLeftPanelForTestMode = true;
        this.rttStream.togglePanel(true, { persist: false });
      }
      this.testRunner.setTestModeActive(true);
      await this.testRunner.ensureInitialized();
      return;
    }

    this.testRunner.setTestModeActive(false);
    if (this._openedLeftPanelForTestMode) {
      this._openedLeftPanelForTestMode = false;
      this.rttStream.togglePanel(false, { persist: false });
    }
  }

  /**
   * Bind top-bar buttons and global keyboard shortcuts (`Cmd/Ctrl +/-/0`, `Ctrl+R`, `Escape`).
   */
  _bindGlobalControls() {
    const bindClick = (el, fn) => {
      if (!el) {
        return;
      }
      el.addEventListener("click", fn);
      this._unsubscribers.push(() => el.removeEventListener("click", fn));
    };

    const handleToggleLogs = () => {
      if (
        this.cliConsole.getMode() === WORKSTATION_MODES.TEST &&
        this.rttStream.isPanelVisible() &&
        this.testRunner.getActiveLeftTab() === LEFT_PANEL_TABS.TESTS
      ) {
        this.testRunner.setActiveLeftTab(LEFT_PANEL_TABS.LOGS);
        return;
      }
      this._openedLeftPanelForTestMode = false;
      const visible = this.rttStream.togglePanel();
      if (this.targetConfig.cfgShowLogsCheckbox) {
        this.targetConfig.cfgShowLogsCheckbox.checked = visible;
      }
    };

    bindClick(this.toggleLogBtn, handleToggleLogs);
    bindClick(this.collapseLogBtn, handleToggleLogs);
    bindClick(this.clearOutputBtn, () => this.clearOutputs());
    bindClick(this.copyLogBtn, () => this.rttStream.copyLogs(this.copyLogBtn));
    bindClick(this.copyWorkstationBtn, () => {
      if (this.cliConsole.getMode() === WORKSTATION_MODES.TEST) {
        return this.testRunner.copyConsoleOutput(this.copyWorkstationBtn);
      }
      return this.cliConsole.copyActiveOutput(this.copyWorkstationBtn);
    });
    bindClick(this.manualPromptBanner, () => {
      this.cliConsole.setMode(WORKSTATION_MODES.TEST);
    });

    const onViewportKeyDown = (event) => this._handleGlobalKeyDown(event);
    this.ctx.addViewportListener("keydown", onViewportKeyDown);
    this._unsubscribers.push(() =>
      this.ctx.removeViewportListener("keydown", onViewportKeyDown),
    );
  }

  /**
   * Handle global keyboard shortcuts across the workstation.
   *
   * @param {KeyboardEvent|object} event - Keydown event.
   */
  _handleGlobalKeyDown(event) {
    if (event.metaKey || event.ctrlKey) {
      if (
        event.key === "=" ||
        event.key === "+" ||
        event.code === "Equal" ||
        event.code === "NumpadAdd"
      ) {
        event.preventDefault();
        this.adjustUiScale(1);
        return;
      }
      if (
        event.key === "-" ||
        event.key === "_" ||
        event.code === "Minus" ||
        event.code === "NumpadSubtract"
      ) {
        event.preventDefault();
        this.adjustUiScale(-1);
        return;
      }
      if (event.key === "0" || event.code === "Digit0" || event.code === "Numpad0") {
        event.preventDefault();
        this.resetUiScale();
        return;
      }
      if (event.key === "r" || event.key === "R") {
        const tgtId = event.target && event.target.id;
        if (tgtId !== "fuzzySearchInput") {
          event.preventDefault();
          this.cliConsole.openFuzzyHistory();
        }
        return;
      }
    }

    if (event.key === "Escape") {
      if (this.cliConsole.isFuzzyOpen) {
        event.preventDefault();
        this.cliConsole.closeFuzzyHistory();
      } else if (this.wormReporter.isModalOpen()) {
        event.preventDefault();
        this.wormReporter.closeModal();
      } else if (
        this.targetConfig.configModal &&
        this.targetConfig.configModal.style.display !== "none" &&
        this.targetConfig.configModal.style.display !== ""
      ) {
        event.preventDefault();
        this.targetConfig.closeConfigModal();
      } else if (
        this.targetConfig.syncModal &&
        this.targetConfig.syncModal.style.display !== "none" &&
        this.targetConfig.syncModal.style.display !== ""
      ) {
        event.preventDefault();
        this.targetConfig.closeSyncModal();
      }
    }
  }

  /**
   * Clear both RTT system logs and CLI/FS workstation output panels.
   */
  clearOutputs() {
    this.rttStream.clear();
    this.cliConsole.clear();
    this.testRunner.clearConsole();
  }

  /**
   * Poll `/api/manual_prompt` to display active manual pytest procedure & prompt context.
   *
   * @returns {Promise<void>}
   */
  async pollManualPrompt() {
    try {
      const data = await this.ctx.fetchJson("/api/manual_prompt");
      this.testRunner.renderManualPrompt(data);
      if (!this.manualPromptBanner || !this.manualPromptSummary) {
        return;
      }
      if (data && data.active && this.cliConsole.getMode() !== WORKSTATION_MODES.TEST) {
        this.manualPromptBanner.style.display = "flex";
        this.manualPromptSummary.textContent = `[${data.project || "device"}] ${data.test_name}: ${data.procedure} — ${data.prompt} (Click to respond in TEST mode)`;
      } else {
        this.manualPromptBanner.style.display = "none";
        this.manualPromptSummary.textContent = "";
      }
    } catch {
      if (this.manualPromptBanner) {
        this.manualPromptBanner.style.display = "none";
      }
    }
  }

  /**
   * Poll RTT system logs, CLI/FS outputs, Test Mode status, and active manual prompt context once.
   *
   * @returns {Promise<void>}
   */
  async pollAll() {
    const tasks = [
      this.rttStream.pollLogs(),
      this.cliConsole.pollOutputs(),
      this.pollManualPrompt(),
    ];
    if (
      this.cliConsole.getMode() === WORKSTATION_MODES.TEST ||
      this.testRunner.isRunning
    ) {
      tasks.push(this.testRunner.pollStatus());
    }
    await Promise.all(tasks);
  }

  /**
   * Tear down polling timers and all child controllers.
   */
  destroy() {
    if (this.pollTimer !== null) {
      this.ctx.clearInterval(this.pollTimer);
      this.pollTimer = null;
    }
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
    this.rttStream.destroy();
    this.cliConsole.destroy();
    this.testRunner.destroy();
    this.targetConfig.destroy();
    this.wormReporter.destroy();
  }
}

/**
 * Create a new `DaemonApp` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @returns {DaemonApp}
 */
export function createDaemonApp(ctx) {
  return new DaemonApp(ctx);
}

// Auto-boot when loaded in a browser Document (`null` in headless `node --test`).
const browserCtx = getRuntimeContext();
const browserDoc = browserCtx.getDocument();
if (browserDoc !== null) {
  if (browserDoc.readyState === "loading") {
    browserDoc.addEventListener("DOMContentLoaded", () => {
      createDaemonApp(browserCtx).init();
    });
  } else {
    createDaemonApp(browserCtx).init();
  }
}
