/**
 * Target Application Configuration, Flash Synchronization Matrix & Reset Controller.
 *
 * Manages target package/binary/chip/profile selection (`#targetConfigModal`),
 * `package_selection_mode` startup auto-prompting, session storage persistence,
 * `POST /api/sync` + `GET /api/sync/status` compile/flash progress polling,
 * cancellation (`POST /api/sync/cancel`), and target reset (`POST /api/reset`).
 */

import { clearChildren, createEl, resolveElement } from "../core/dom.js";
import { bindModalBackdropClose, hideModal, showModal } from "../core/modal.js";
import { showToast } from "../core/toast.js";
import { renderTerminalHtml, SHOW_SYSTEM_LOGS_STORAGE_KEY } from "./rtt_stream.js";

export const LAST_PACKAGE_STORAGE_KEY = "xerxes_last_target_package";
export const LAST_BIN_STORAGE_KEY = "xerxes_last_target_bin";
export const LAST_RELEASE_STORAGE_KEY = "xerxes_last_target_release";
export const LAST_TRACE_MODE_STORAGE_KEY = "xerxes_last_trace_mode";

export const DEFAULT_TRACE_MODE_OPTIONS = Object.freeze([
  Object.freeze({
    mode: "NONE",
    label: "NONE",
    summary: "Standard defmt logs; zero runtime trace overhead",
    cargo_features: [],
    external_viewer: "none",
    enabled_in_ui: true,
    badge: "default",
  }),
  Object.freeze({
    mode: "SENSOR",
    label: "SENSOR",
    summary: "Sensor reports & measured values only • External viewer: Rerun",
    cargo_features: ["trace-sensors"],
    external_viewer: "rerun",
    enabled_in_ui: true,
    badge: "trace-sensors",
  }),
  Object.freeze({
    mode: "PERF",
    label: "PERF",
    summary:
      "All CPU spans, CPU usage % & system telemetry (excl. sensors) • Viewer: Perfetto",
    cargo_features: ["trace-perf"],
    external_viewer: "perfetto",
    enabled_in_ui: true,
    badge: "trace-perf",
  }),
  Object.freeze({
    mode: "AUDIO",
    label: "AUDIO",
    summary: "PDM/I2S audio stream & DSP frame tracing • External viewer: Rerun",
    cargo_features: ["trace-audio"],
    external_viewer: "rerun",
    enabled_in_ui: false,
    badge: "PLANNED",
  }),
]);

const DEFAULT_PROBE_SPEED_KHZ = 1000;
const DEFAULT_GDB_CORE0_PORT = 3333;
const DEFAULT_GDB_CORE1_PORT = 3334;
const SYNC_POLL_INTERVAL_MS = 500;
const SYNC_AUTO_DISMISS_DELAY_MS = 1500;
const RESET_BUTTON_HOLD_MS = 1200;
const CONSOLE_SCROLL_SLACK_PX = 30;

/**
 * Infer a human-readable MCU architecture note for an ARM target triple.
 *
 * @param {string} targetTriple - Rust target triple.
 * @returns {string}
 */
export function describeTargetArchNote(targetTriple) {
  const triple = String(targetTriple || "");
  if (triple.includes("thumbv6m")) {
    return "(Auto-inferred: ARM Cortex-M0+ / RP2040)";
  }
  if (triple.includes("thumbv8m")) {
    return "(Auto-inferred: ARM Cortex-M33 / NXP MCX / RP2350)";
  }
  if (triple.includes("thumbv7em")) {
    return "(Auto-inferred: ARM Cortex-M4F/M7F)";
  }
  return "(Auto-inferred from hardware target)";
}

/**
 * Controller for Target Configuration modal, Sync/Flash progress modal, and Reset action.
 */
export class TargetConfigController {
  /**
   * Construct a `TargetConfigController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Boot configuration and lifecycle callbacks.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    const boot = options.bootConfig || {};

    this.currentConfig = {
      package: boot.package || "",
      binName: boot.binName || boot.bin_name || boot.bin || "",
      target: boot.target || "",
      release: Boolean(boot.release),
      speed: Number(boot.speed) || DEFAULT_PROBE_SPEED_KHZ,
      chip: boot.chip || null,
      showSystemLogs: Boolean(boot.showSystemLogs ?? boot.show_system_logs),
      enableDebugServer: Boolean(boot.enableDebugServer ?? boot.enable_debug_server),
      haltOnReset: Boolean(boot.haltOnReset ?? boot.halt_on_reset),
      gdbCore0Port:
        Number(boot.gdbCore0Port ?? boot.gdb_core0_port) || DEFAULT_GDB_CORE0_PORT,
      gdbCore1Port:
        Number(boot.gdbCore1Port ?? boot.gdb_core1_port) || DEFAULT_GDB_CORE1_PORT,
      debugServerActive: Boolean(boot.debugServerActive ?? boot.debug_server_active),
      traceMode: String(boot.traceMode || boot.trace_mode || "NONE").toUpperCase(),
    };

    const rawTraceModes = boot.traceModes || boot.trace_modes;
    this.traceModeOptions =
      Array.isArray(rawTraceModes) && rawTraceModes.length > 0
        ? rawTraceModes
        : DEFAULT_TRACE_MODE_OPTIONS;
    this.isTraceDropdownOpen = false;

    this.discoveredPackages = Array.isArray(boot.targetPackages || boot.target_packages)
      ? [...(boot.targetPackages || boot.target_packages)]
      : [];
    this.lastSession = boot.lastSession || boot.last_session || null;
    this.packageSelectionMode = Boolean(
      boot.packageSelectionMode ?? boot.package_selection_mode,
    );

    this.onWorkstationDisabledChange = options.onWorkstationDisabledChange || null;
    this.onShowSystemLogsChange = options.onShowSystemLogsChange || null;
    this.onTraceModeChange = options.onTraceModeChange || null;
    this.onSyncComplete = options.onSyncComplete || null;
    this.shouldSuppressAutoSyncModal = options.shouldSuppressAutoSyncModal || null;

    this.syncPollTimer = null;
    this.syncAutoDismissTimer = null;
    this._unsubscribers = [];

    // Header elements
    this.targetBadge = resolveElement(ctx, options.targetBadge || "targetBadge");
    this.binBadge = resolveElement(ctx, options.binBadge || "binBadge");
    this.debugServerBadge = resolveElement(
      ctx,
      options.debugServerBadge || "debugServerBadge",
    );
    this.traceModeBadge = resolveElement(
      ctx,
      options.traceModeBadge || "traceModeBadge",
    );
    this.syncBtn = resolveElement(ctx, options.syncBtn || "btnSync");
    this.resetBtn = resolveElement(ctx, options.resetBtn || "btnReset");
    this.targetConfigBtn = resolveElement(ctx, options.targetConfigBtn || "btnTargetConfig");

    // Target Config Modal elements
    this.configModal = resolveElement(ctx, options.configModal || "targetConfigModal");
    this.cfgPackageSelect = resolveElement(ctx, options.cfgPackageSelect || "cfgPackageSelect");
    this.cfgBinSelect = resolveElement(ctx, options.cfgBinSelect || "cfgBinSelect");
    this.cfgReleaseCheckbox = resolveElement(
      ctx,
      options.cfgReleaseCheckbox || "cfgReleaseCheckbox",
    );
    this.cfgSpeedSelect = resolveElement(ctx, options.cfgSpeedSelect || "cfgSpeedSelect");
    this.cfgArchDisplay = resolveElement(
      ctx,
      options.cfgArchDisplay || "cfgInferredArchDisplay",
    );
    this.cfgArchNote = resolveElement(ctx, options.cfgArchNote || "cfgInferredArchNote");
    this.cfgArchInput = resolveElement(ctx, options.cfgArchInput || "cfgTargetArchInput");
    this.cfgTraceModeDropdown = resolveElement(
      ctx,
      options.cfgTraceModeDropdown || "cfgTraceModeDropdown",
    );
    this.cfgTraceModeTrigger = resolveElement(
      ctx,
      options.cfgTraceModeTrigger || "cfgTraceModeTrigger",
    );
    this.cfgTraceModeTriggerLabel = resolveElement(
      ctx,
      options.cfgTraceModeTriggerLabel || "cfgTraceModeTriggerLabel",
    );
    this.cfgTraceModeMenu = resolveElement(
      ctx,
      options.cfgTraceModeMenu || "cfgTraceModeMenu",
    );
    this.cfgTraceModeInput = resolveElement(
      ctx,
      options.cfgTraceModeInput || "cfgTraceModeInput",
    );
    this.cfgTraceModeFlags = resolveElement(
      ctx,
      options.cfgTraceModeFlags || "cfgTraceModeFlags",
    );
    this.cfgShowLogsCheckbox = resolveElement(
      ctx,
      options.cfgShowLogsCheckbox || "cfgShowLogsCheckbox",
    );
    this.cfgDebugServerCheckbox = resolveElement(
      ctx,
      options.cfgDebugServerCheckbox || "cfgDebugServerCheckbox",
    );
    this.cfgHaltOnResetCheckbox = resolveElement(
      ctx,
      options.cfgHaltOnResetCheckbox || "cfgHaltOnResetCheckbox",
    );
    this.cfgGdbCore0Port = resolveElement(
      ctx,
      options.cfgGdbCore0Port || "cfgGdbCore0Port",
    );
    this.cfgGdbCore1Port = resolveElement(
      ctx,
      options.cfgGdbCore1Port || "cfgGdbCore1Port",
    );
    this.cfgStatusMsg = resolveElement(ctx, options.cfgStatusMsg || "cfgStatusMsg");
    this.cfgCancelBtn = resolveElement(ctx, options.cfgCancelBtn || "btnCancelTargetConfig");
    this.cfgCloseBtn = resolveElement(ctx, options.cfgCloseBtn || "btnCloseTargetConfigModal");
    this.cfgConfirmBtn = resolveElement(ctx, options.cfgConfirmBtn || "btnConfirmTargetConfig");

    // Sync Modal elements
    this.syncModal = resolveElement(ctx, options.syncModal || "syncModal");
    this.syncCloseBtn = resolveElement(ctx, options.syncCloseBtn || "btnCloseSyncModal");
    this.syncCancelBtn = resolveElement(ctx, options.syncCancelBtn || "btnSyncCancel");
    this.syncDismissBtn = resolveElement(ctx, options.syncDismissBtn || "btnSyncDismiss");
    this.syncInfoTarget = resolveElement(ctx, options.syncInfoTarget || "syncInfoTarget");
    this.syncInfoPackage = resolveElement(ctx, options.syncInfoPackage || "syncInfoPackage");
    this.syncInfoProfile = resolveElement(ctx, options.syncInfoProfile || "syncInfoProfile");
    this.syncInfoSpeed = resolveElement(ctx, options.syncInfoSpeed || "syncInfoSpeed");
    this.syncInfoTraceMode = resolveElement(
      ctx,
      options.syncInfoTraceMode || "syncInfoTraceMode",
    );
    this.syncStatusDot = resolveElement(ctx, options.syncStatusDot || "syncStatusDot");
    this.syncProgressMsg = resolveElement(ctx, options.syncProgressMsg || "syncProgressMsg");
    this.syncProgressBar = resolveElement(ctx, options.syncProgressBar || "syncProgressBar");
    this.syncProgressPct = resolveElement(ctx, options.syncProgressPct || "syncProgressPct");
    this.syncConsoleLog = resolveElement(ctx, options.syncConsoleLog || "syncConsoleLog");
  }

  /**
   * Bind event listeners, populate target binary options, and auto-open config/sync modal as needed.
   *
   * @returns {Promise<void>}
   */
  async init() {
    this._bindDomListeners();
    this._populatePackageOptions();

    const initialPkg =
      (this.cfgPackageSelect && this.cfgPackageSelect.value) || this.currentConfig.package;
    this.populateTargetBinaries(initialPkg, this.currentConfig.binName);
    this._renderTraceModeDropdown(this.currentConfig.traceMode);
    this.updateTraceBadge(this.currentConfig.traceMode);
    this.updateDebugBadge({
      enabled: this.currentConfig.enableDebugServer,
      active: this.currentConfig.debugServerActive,
      halt_on_reset: this.currentConfig.haltOnReset,
      gdb_core0_port: this.currentConfig.gdbCore0Port,
      gdb_core1_port: this.currentConfig.gdbCore1Port,
    });

    if (this.packageSelectionMode) {
      this.openConfigModal();
      return;
    }

    await this.checkInitialSync();
  }

  /**
   * Attach all button, dropdown, and modal backdrop listeners.
   */
  _bindDomListeners() {
    const bindClick = (el, handler) => {
      if (!el) {
        return;
      }
      el.addEventListener("click", handler);
      this._unsubscribers.push(() => el.removeEventListener("click", handler));
    };

    bindClick(this.syncBtn, () => this.triggerSync());
    bindClick(this.resetBtn, () => this.triggerReset());
    bindClick(this.targetConfigBtn, () => this.openConfigModal());

    bindClick(this.cfgCancelBtn, () => this.closeConfigModal());
    bindClick(this.cfgCloseBtn, () => this.closeConfigModal());
    bindClick(this.cfgConfirmBtn, () => this.submitConfig());

    if (this.cfgTraceModeTrigger) {
      bindClick(this.cfgTraceModeTrigger, (evt) => {
        if (evt && typeof evt.stopPropagation === "function") {
          evt.stopPropagation();
        }
        this.toggleTraceModeDropdown();
      });
    }

    const doc = this.ctx.getDocument();
    if (doc && this.cfgTraceModeMenu) {
      const onDocClick = (evt) => {
        if (!this.isTraceDropdownOpen) {
          return;
        }
        const tgt = evt && evt.target;
        if (
          tgt &&
          this.cfgTraceModeDropdown &&
          typeof tgt.closest === "function" &&
          tgt.closest("#cfgTraceModeDropdown")
        ) {
          return;
        }
        this.toggleTraceModeDropdown(false);
      };
      doc.addEventListener("click", onDocClick);
      this._unsubscribers.push(() => doc.removeEventListener("click", onDocClick));
    }

    bindClick(this.syncCloseBtn, () => this.closeSyncModal());
    bindClick(this.syncDismissBtn, () => this.closeSyncModal());
    bindClick(this.syncCancelBtn, () => this.cancelSync());

    if (this.cfgPackageSelect) {
      const onPkgChange = () => {
        this.populateTargetBinaries(this.cfgPackageSelect.value);
      };
      this.cfgPackageSelect.addEventListener("change", onPkgChange);
      this._unsubscribers.push(() =>
        this.cfgPackageSelect.removeEventListener("change", onPkgChange),
      );
    }

    if (this.cfgBinSelect) {
      const onBinChange = () => {
        const pkgName = this.cfgPackageSelect
          ? this.cfgPackageSelect.value
          : this.currentConfig.package;
        this.updateInferredArchDisplay(pkgName);
      };
      this.cfgBinSelect.addEventListener("change", onBinChange);
      this._unsubscribers.push(() =>
        this.cfgBinSelect.removeEventListener("change", onBinChange),
      );
    }

    bindModalBackdropClose(this.ctx, this.configModal, () => this.closeConfigModal());
  }

  /**
   * Return a snapshot of the active target configuration.
   *
   * @returns {object}
   */
  getCurrentConfig() {
    return { ...this.currentConfig };
  }

  /**
   * Populate `#cfgPackageSelect` when packages are provided dynamically.
   */
  _populatePackageOptions() {
    if (!this.cfgPackageSelect || this.discoveredPackages.length === 0) {
      return;
    }
    if (this.cfgPackageSelect.children.length > 0) {
      return;
    }
    for (const pkg of this.discoveredPackages) {
      const opt = createEl(this.ctx, "option", {
        text: pkg.package,
        attrs: { value: pkg.package },
      });
      opt.value = pkg.package;
      if (pkg.package === this.currentConfig.package) {
        opt.selected = true;
      }
      this.cfgPackageSelect.appendChild(opt);
    }
    this.cfgPackageSelect.value = this.currentConfig.package;
  }

  /**
   * Refresh discovered packages and last session from `GET /api/targets`.
   *
   * @returns {Promise<object|null>}
   */
  async refreshTargets() {
    try {
      const data = await this.ctx.fetchJson("/api/targets");
      if (data && Array.isArray(data.packages)) {
        this.discoveredPackages = data.packages;
      }
      if (data && data.last_session) {
        this.lastSession = data.last_session;
      }
      if (data && data.current) {
        this.currentConfig = {
          ...this.currentConfig,
          package: data.current.package || this.currentConfig.package,
          binName: data.current.bin || this.currentConfig.binName,
          release:
            typeof data.current.release === "boolean"
              ? data.current.release
              : this.currentConfig.release,
          target: data.current.target || this.currentConfig.target,
          speed: Number(data.current.speed) || this.currentConfig.speed,
          chip: data.current.chip ?? this.currentConfig.chip,
          showSystemLogs: Boolean(data.current.show_system_logs),
          enableDebugServer:
            typeof data.current.enable_debug_server === "boolean"
              ? data.current.enable_debug_server
              : this.currentConfig.enableDebugServer,
          haltOnReset:
            typeof data.current.halt_on_reset === "boolean"
              ? data.current.halt_on_reset
              : this.currentConfig.haltOnReset,
          gdbCore0Port:
            Number(data.current.gdb_core0_port) || this.currentConfig.gdbCore0Port,
          gdbCore1Port:
            Number(data.current.gdb_core1_port) || this.currentConfig.gdbCore1Port,
          traceMode: String(
            data.current.trace_mode || this.currentConfig.traceMode || "NONE",
          ).toUpperCase(),
        };
        if (Array.isArray(data.trace_modes) && data.trace_modes.length > 0) {
          this.traceModeOptions = data.trace_modes;
        }
        this.updateTraceBadge(this.currentConfig.traceMode);
        if (data.current.debug_server) {
          this.updateDebugBadge(data.current.debug_server);
        }
      }
      return data;
    } catch {
      return null;
    }
  }

  /**
   * Populate `#cfgBinSelect` with binaries belonging to `selectedPkgName` and update architecture info.
   *
   * @param {string} selectedPkgName - Cargo package name.
   * @param {string} [preferredBinName] - Optional preferred binary name.
   */
  populateTargetBinaries(selectedPkgName, preferredBinName) {
    const pkgData = this.discoveredPackages.find((p) => p.package === selectedPkgName) || null;
    const rawBinaries = pkgData ? pkgData.binaries || pkgData.bins : null;
    const binaries =
      Array.isArray(rawBinaries) && rawBinaries.length > 0
        ? rawBinaries
        : preferredBinName
          ? [preferredBinName]
          : [];

    if (this.cfgBinSelect) {
      clearChildren(this.cfgBinSelect);
      let matchedValue = "";
      for (const bin of binaries) {
        const opt = createEl(this.ctx, "option", {
          text: bin,
          attrs: { value: bin },
        });
        opt.value = bin;
        if (
          bin === preferredBinName ||
          (!preferredBinName && pkgData && bin === pkgData.default_bin)
        ) {
          opt.selected = true;
          matchedValue = bin;
        }
        this.cfgBinSelect.appendChild(opt);
      }
      this.cfgBinSelect.value = matchedValue || binaries[0] || "";
    }

    this.updateInferredArchDisplay(selectedPkgName);
  }

  /**
   * Update inferred ARM target triple readout for `pkgName` and active binary selection.
   *
   * @param {string} pkgName - Selected Cargo package name.
   */
  updateInferredArchDisplay(pkgName) {
    const pkgData = this.discoveredPackages.find((p) => p.package === pkgName) || null;
    const selectedBin = (this.cfgBinSelect && this.cfgBinSelect.value) || "";
    const binCfg =
      pkgData && pkgData.binary_configs && selectedBin
        ? pkgData.binary_configs[selectedBin]
        : null;
    const target =
      (binCfg && binCfg.target) ||
      (pkgData && pkgData.inferred_target) ||
      this.currentConfig.target ||
      "";

    if (this.cfgArchDisplay) {
      this.cfgArchDisplay.textContent = target;
    }
    if (this.cfgArchInput) {
      this.cfgArchInput.value = target;
    }
    if (this.cfgArchNote) {
      this.cfgArchNote.textContent = describeTargetArchNote(target);
    }
  }

  /**
   * Open the Target Configuration modal populated with last-session or active runner settings.
   */
  openConfigModal() {
    if (!this.configModal) {
      return;
    }
    const pkg =
      (this.lastSession && this.lastSession.package) || this.currentConfig.package;
    const bin =
      (this.lastSession && (this.lastSession.bin || this.lastSession.bin_name)) ||
      this.currentConfig.binName;

    if (this.cfgPackageSelect && pkg) {
      this.cfgPackageSelect.value = pkg;
    }
    this.populateTargetBinaries(pkg, bin);

    if (this.cfgReleaseCheckbox) {
      this.cfgReleaseCheckbox.checked =
        this.lastSession && typeof this.lastSession.release === "boolean"
          ? this.lastSession.release
          : this.currentConfig.release;
    }

    if (this.cfgSpeedSelect) {
      const spd = (this.lastSession && this.lastSession.speed) || this.currentConfig.speed;
      this.cfgSpeedSelect.value = String(spd);
    }

    if (this.cfgShowLogsCheckbox) {
      const storedShow = this.ctx.getStorage().getItem(SHOW_SYSTEM_LOGS_STORAGE_KEY);
      this.cfgShowLogsCheckbox.checked =
        storedShow !== null ? storedShow === "true" : this.currentConfig.showSystemLogs;
    }

    if (this.cfgDebugServerCheckbox) {
      this.cfgDebugServerCheckbox.checked =
        this.lastSession && typeof this.lastSession.enable_debug_server === "boolean"
          ? this.lastSession.enable_debug_server
          : this.currentConfig.enableDebugServer;
    }

    if (this.cfgHaltOnResetCheckbox) {
      this.cfgHaltOnResetCheckbox.checked =
        this.lastSession && typeof this.lastSession.halt_on_reset === "boolean"
          ? this.lastSession.halt_on_reset
          : this.currentConfig.haltOnReset;
    }

    if (this.cfgGdbCore0Port) {
      const c0Port =
        (this.lastSession && this.lastSession.gdb_core0_port) ||
        this.currentConfig.gdbCore0Port;
      this.cfgGdbCore0Port.value = String(c0Port);
    }

    if (this.cfgGdbCore1Port) {
      const c1Port =
        (this.lastSession && this.lastSession.gdb_core1_port) ||
        this.currentConfig.gdbCore1Port;
      this.cfgGdbCore1Port.value = String(c1Port);
    }

    const initialTraceMode = String(
      (this.lastSession && this.lastSession.trace_mode) ||
        this.ctx.getStorage().getItem(LAST_TRACE_MODE_STORAGE_KEY) ||
        this.currentConfig.traceMode ||
        "NONE",
    ).toUpperCase();
    this.selectTraceModeOption(initialTraceMode);
    this.toggleTraceModeDropdown(false);

    if (this.cfgStatusMsg) {
      this.cfgStatusMsg.style.display = "none";
      this.cfgStatusMsg.textContent = "";
    }

    showModal(this.ctx, this.configModal, "flex");
  }

  /**
   * Close the Target Configuration modal.
   */
  closeConfigModal() {
    this.toggleTraceModeDropdown(false);
    hideModal(this.ctx, this.configModal);
  }

  /**
   * Toggle or explicitly set the open state of the scrollable `#cfgTraceModeMenu` dropdown.
   *
   * @param {boolean} [forceOpen] - Explicit open state.
   */
  toggleTraceModeDropdown(forceOpen) {
    this.isTraceDropdownOpen =
      typeof forceOpen === "boolean" ? forceOpen : !this.isTraceDropdownOpen;
    if (this.cfgTraceModeMenu) {
      if (this.isTraceDropdownOpen) {
        this.cfgTraceModeMenu.classList.add("open");
        this.cfgTraceModeMenu.style.display = "flex";
      } else {
        this.cfgTraceModeMenu.classList.remove("open");
        this.cfgTraceModeMenu.style.display = "none";
      }
    }
  }

  /**
   * Select a trace mode (`NONE`, `SENSOR`, `PERF`) inside the Target Config dropdown.
   *
   * @param {string} modeId - Trace mode identifier.
   */
  selectTraceModeOption(modeId) {
    const upper = String(modeId || "NONE").toUpperCase();
    const matched =
      this.traceModeOptions.find(
        (m) => String(m.mode || "").toUpperCase() === upper,
      ) || this.traceModeOptions[0];
    if (matched && matched.enabled_in_ui === false) {
      return;
    }
    const chosenId = matched ? String(matched.mode || "NONE").toUpperCase() : "NONE";
    if (this.cfgTraceModeInput) {
      this.cfgTraceModeInput.value = chosenId;
    }
    this._renderTraceModeDropdown(chosenId);
    this.toggleTraceModeDropdown(false);
  }

  /**
   * Render the scrollable `#cfgTraceModeMenu` items and update `#cfgTraceModeTriggerLabel` + `#cfgTraceModeFlags`.
   *
   * @param {string} selectedId - Selected trace mode ID.
   */
  _renderTraceModeDropdown(selectedId) {
    const activeId = String(selectedId || "NONE").toUpperCase();
    const selectedMeta =
      this.traceModeOptions.find(
        (m) => String(m.mode || "").toUpperCase() === activeId,
      ) || this.traceModeOptions[0];

    if (this.cfgTraceModeTriggerLabel && selectedMeta) {
      const modeName = String(selectedMeta.label || selectedMeta.mode || activeId);
      const summary = selectedMeta.summary ? ` — ${selectedMeta.summary}` : "";
      this.cfgTraceModeTriggerLabel.textContent = `${modeName}${summary}`;
    }

    if (this.cfgTraceModeFlags && selectedMeta) {
      const features = Array.isArray(selectedMeta.cargo_features)
        ? selectedMeta.cargo_features
        : [];
      const viewer = String(selectedMeta.external_viewer || "NONE").toUpperCase();
      if (features.length === 0) {
        this.cfgTraceModeFlags.textContent = "Cargo Flags: (none — standard build)";
      } else {
        const viewerNote =
          viewer && viewer !== "NONE" ? ` • External Viewer: ${viewer}` : "";
        this.cfgTraceModeFlags.textContent = `Cargo Flags: --features ${features.join(",")} (DEFMT_LOG=trace)${viewerNote}`;
      }
    }

    if (!this.cfgTraceModeMenu) {
      return;
    }
    clearChildren(this.cfgTraceModeMenu);

    for (const opt of this.traceModeOptions) {
      const optId = String(opt.mode || "").toUpperCase();
      const isDisabled = opt.enabled_in_ui === false;
      const isSelected = optId === activeId;
      const features = Array.isArray(opt.cargo_features) ? opt.cargo_features : [];
      const viewer = String(opt.external_viewer || "NONE").toUpperCase();
      const badgeSuffix =
        opt.badge === "PLANNED" ? " [PLANNED]" : "";
      const titleText = `${opt.label || optId}${badgeSuffix}${opt.summary ? ` — ${opt.summary}` : ""}`;
      const subText =
        features.length > 0
          ? `--features ${features.join(",")} • Viewer: ${viewer}`
          : "Zero runtime trace overhead";

      const itemEl = createEl(this.ctx, "div", {
        className: `trace-dropdown-item${isSelected ? " selected" : ""}${isDisabled ? " disabled" : ""}`,
        attrs: {
          "data-trace-mode": optId,
          "aria-disabled": isDisabled ? "true" : "false",
        },
        children: [
          createEl(this.ctx, "strong", {
            style: { color: isDisabled ? "var(--xerxes-text-dim)" : "var(--xerxes-cyan)", fontSize: "9.5px" },
            text: titleText,
          }),
          createEl(this.ctx, "span", {
            style: { color: "var(--xerxes-text-muted)", fontSize: "8px" },
            text: subText,
          }),
        ],
      });

      if (!isDisabled) {
        itemEl.addEventListener("click", (evt) => {
          if (evt && typeof evt.stopPropagation === "function") {
            evt.stopPropagation();
          }
          this.selectTraceModeOption(optId);
        });
      }
      this.cfgTraceModeMenu.appendChild(itemEl);
    }
  }

  /**
   * Update the passive `#traceModeBadge` header indicator and `#syncInfoTraceMode`.
   *
   * @param {string} traceMode - Active firmware trace mode.
   */
  updateTraceBadge(traceMode) {
    const normalized = String(traceMode || "NONE").toUpperCase();
    this.currentConfig.traceMode = normalized;
    if (this.syncInfoTraceMode) {
      this.syncInfoTraceMode.textContent = normalized;
    }
    if (!this.traceModeBadge) {
      return;
    }
    if (normalized === "NONE") {
      this.traceModeBadge.style.display = "none";
      this.traceModeBadge.textContent = "TRACE: NONE";
    } else {
      this.traceModeBadge.style.display = "inline-block";
      this.traceModeBadge.textContent = `TRACE: ${normalized}`;
      this.traceModeBadge.className =
        normalized === "PERF" ? "badge badge-purple" : "badge badge-green";
    }
  }

  /**
   * Apply target configuration via `POST /api/target/configure` and start firmware sync/flash.
   *
   * @returns {Promise<object|null>}
   */
  async submitConfig() {
    const pkg =
      (this.cfgPackageSelect && this.cfgPackageSelect.value) || this.currentConfig.package;
    const bin =
      (this.cfgBinSelect && this.cfgBinSelect.value) || this.currentConfig.binName;
    const release = this.cfgReleaseCheckbox
      ? Boolean(this.cfgReleaseCheckbox.checked)
      : this.currentConfig.release;
    const speed =
      Number.parseInt(
        (this.cfgSpeedSelect && this.cfgSpeedSelect.value) || String(this.currentConfig.speed),
        10,
      ) || DEFAULT_PROBE_SPEED_KHZ;
    const target =
      (this.cfgArchInput && this.cfgArchInput.value) || this.currentConfig.target;
    const showLogs = this.cfgShowLogsCheckbox
      ? Boolean(this.cfgShowLogsCheckbox.checked)
      : false;
    const enableDebugServer = this.cfgDebugServerCheckbox
      ? Boolean(this.cfgDebugServerCheckbox.checked)
      : this.currentConfig.enableDebugServer;
    const haltOnReset = this.cfgHaltOnResetCheckbox
      ? Boolean(this.cfgHaltOnResetCheckbox.checked)
      : this.currentConfig.haltOnReset;
    const gdbCore0Port =
      Number.parseInt(
        (this.cfgGdbCore0Port && this.cfgGdbCore0Port.value) ||
          String(this.currentConfig.gdbCore0Port),
        10,
      ) || DEFAULT_GDB_CORE0_PORT;
    const gdbCore1Port =
      Number.parseInt(
        (this.cfgGdbCore1Port && this.cfgGdbCore1Port.value) ||
          String(this.currentConfig.gdbCore1Port),
        10,
      ) || DEFAULT_GDB_CORE1_PORT;
    const traceMode = String(
      (this.cfgTraceModeInput && this.cfgTraceModeInput.value) ||
        this.currentConfig.traceMode ||
        "NONE",
    ).toUpperCase();

    if (this.cfgConfirmBtn) {
      this.cfgConfirmBtn.disabled = true;
    }

    try {
      const data = await this.ctx.postJson("/api/target/configure", {
        package: pkg,
        bin,
        release,
        speed,
        target,
        show_system_logs: showLogs,
        trace_mode: traceMode,
        enable_debug_server: enableDebugServer,
        halt_on_reset: haltOnReset,
        gdb_core0_port: gdbCore0Port,
        gdb_core1_port: gdbCore1Port,
      });

      if (data && data.success) {
        const resolvedTraceMode = String(data.trace_mode || traceMode).toUpperCase();
        const storage = this.ctx.getStorage();
        storage.setItem(LAST_PACKAGE_STORAGE_KEY, pkg);
        storage.setItem(LAST_BIN_STORAGE_KEY, bin);
        storage.setItem(LAST_RELEASE_STORAGE_KEY, String(release));
        storage.setItem(LAST_TRACE_MODE_STORAGE_KEY, resolvedTraceMode);
        storage.setItem(SHOW_SYSTEM_LOGS_STORAGE_KEY, String(showLogs));

        this.currentConfig = {
          ...this.currentConfig,
          package: pkg,
          binName: bin,
          release,
          speed,
          target: data.target || target,
          showSystemLogs: showLogs,
          traceMode: resolvedTraceMode,
          enableDebugServer,
          haltOnReset,
          gdbCore0Port,
          gdbCore1Port,
        };
        this.lastSession = {
          package: pkg,
          bin,
          release,
          target: this.currentConfig.target,
          speed,
          trace_mode: resolvedTraceMode,
          enable_debug_server: enableDebugServer,
          halt_on_reset: haltOnReset,
          gdb_core0_port: gdbCore0Port,
          gdb_core1_port: gdbCore1Port,
        };
        this.packageSelectionMode = false;

        this.updateTraceBadge(resolvedTraceMode);
        if (typeof this.onTraceModeChange === "function") {
          this.onTraceModeChange(resolvedTraceMode, { binName: bin });
        }

        this.updateDebugBadge({
          enabled: enableDebugServer,
          active: enableDebugServer,
          halt_on_reset: haltOnReset,
          gdb_core0_port: gdbCore0Port,
          gdb_core1_port: gdbCore1Port,
        });

        this._updateHeaderAndSyncMetadata({
          target: this.currentConfig.target,
          package: pkg,
          bin,
          profile: release ? "release" : "debug",
          speed,
        });

        if (typeof this.onShowSystemLogsChange === "function") {
          this.onShowSystemLogsChange(showLogs);
        }

        this.closeConfigModal();
        this.openSyncModal();
        this._resetSyncProgressDom("Synchronizing target firmware...");
        this.startSyncPolling();
        return data;
      }

      const errMsg = (data && data.error) || "Failed to configure target";
      this._showConfigError(errMsg);
      return data;
    } catch (err) {
      const errMsg = (err && err.message) || String(err);
      this._showConfigError(`Error communicating with host daemon: ${errMsg}`);
      return null;
    } finally {
      if (this.cfgConfirmBtn) {
        this.cfgConfirmBtn.disabled = false;
      }
    }
  }

  /**
   * Display an error banner inside `#targetConfigModal` and show a toast.
   *
   * @param {string} message - Error text.
   */
  _showConfigError(message) {
    if (this.cfgStatusMsg) {
      this.cfgStatusMsg.style.display = "block";
      this.cfgStatusMsg.style.background = "rgba(255, 42, 75, 0.15)";
      this.cfgStatusMsg.style.color = "var(--xerxes-red)";
      this.cfgStatusMsg.textContent = message;
    }
    showToast(this.ctx, message, "error");
  }

  /**
   * Update the passive `#debugServerBadge` header indicator from a `/api/debug/status` snapshot.
   *
   * @param {object|null} debugState - Debug server status snapshot.
   */
  updateDebugBadge(debugState) {
    if (!this.debugServerBadge || !debugState) {
      return;
    }
    const isActive = Boolean(debugState.active);
    const isEnabled = Boolean(debugState.enabled);
    const c0Port = Number(debugState.gdb_core0_port) || this.currentConfig.gdbCore0Port;
    const c1Port = debugState.gdb_core1_port ?? this.currentConfig.gdbCore1Port;
    this.currentConfig.debugServerActive = isActive;
    if (typeof debugState.enabled === "boolean") {
      this.currentConfig.enableDebugServer = isEnabled;
    }
    if (isActive || isEnabled) {
      const portLabel = c1Port ? `:${c0Port}/:${c1Port}` : `:${c0Port}`;
      const stateSuffix = !isActive
        ? " (SYNC)"
        : debugState.halted_on_reset || debugState.halt_on_reset
          ? " [HALTED]"
          : "";
      this.debugServerBadge.textContent = `GDB ${portLabel}${stateSuffix}`;
      this.debugServerBadge.style.display = "inline-block";
    } else {
      this.debugServerBadge.style.display = "none";
    }
  }

  /**
   * Poll `GET /api/debug/status` to keep `#debugServerBadge` synchronized with CLI/API changes.
   *
   * @returns {Promise<void>}
   */
  async pollDebugStatus() {
    if (!this.debugServerBadge) {
      return;
    }
    try {
      const data = await this.ctx.fetchJson("/api/debug/status");
      if (data) {
        this.updateDebugBadge(data);
      }
    } catch {
      // Ignore transient polling failures.
    }
  }

  /**
   * Synchronize header badges and sync modal metadata rows.
   *
   * @param {object} info - `{ target, package, bin, profile, speed }`.
   */
  _updateHeaderAndSyncMetadata(info) {
    if (info.target) {
      if (this.targetBadge) this.targetBadge.textContent = info.target;
      if (this.syncInfoTarget) this.syncInfoTarget.textContent = info.target;
    }
    if (info.bin && this.binBadge) {
      this.binBadge.textContent = info.bin;
    }
    if (info.package && info.bin && this.syncInfoPackage) {
      this.syncInfoPackage.textContent = `${info.package} (${info.bin})`;
    }
    if (info.profile && this.syncInfoProfile) {
      this.syncInfoProfile.textContent = info.profile;
    }
    if (info.speed && this.syncInfoSpeed) {
      this.syncInfoSpeed.textContent = `${info.speed} kHz`;
    }
  }

  /**
   * Initialize `#syncModal` progress bar and status dot to starting state.
   *
   * @param {string} message - Initial status copy.
   */
  _resetSyncProgressDom(message) {
    if (this.syncStatusDot) {
      this.syncStatusDot.className = "sync-status-dot pulse";
    }
    if (this.syncProgressMsg) {
      this.syncProgressMsg.textContent = message;
    }
    if (this.syncProgressBar) {
      this.syncProgressBar.style.width = "5%";
    }
    if (this.syncProgressPct) {
      this.syncProgressPct.textContent = "5%";
    }
    if (this.syncConsoleLog) {
      clearChildren(this.syncConsoleLog);
    }
  }

  /**
   * Open the Target Sync & Flash Matrix modal.
   */
  openSyncModal() {
    if (this.syncAutoDismissTimer !== null) {
      this.ctx.clearTimeout(this.syncAutoDismissTimer);
      this.syncAutoDismissTimer = null;
    }
    showModal(this.ctx, this.syncModal, "flex");
    if (this.syncCancelBtn) {
      this.syncCancelBtn.disabled = false;
      this.syncCancelBtn.textContent = "Cancel";
    }
  }

  /**
   * Close the Target Sync & Flash Matrix modal.
   */
  closeSyncModal() {
    if (this.syncAutoDismissTimer !== null) {
      this.ctx.clearTimeout(this.syncAutoDismissTimer);
      this.syncAutoDismissTimer = null;
    }
    hideModal(this.ctx, this.syncModal);
  }

  /**
   * Trigger incremental build, flash, and RTT attach via `POST /api/sync`.
   *
   * @returns {Promise<object|null>}
   */
  async triggerSync() {
    this.openSyncModal();
    this._resetSyncProgressDom("Triggering target synchronization...");

    try {
      const res = await this.ctx.postJson("/api/sync", {});
      this.startSyncPolling();
      return res;
    } catch (err) {
      if (this.syncStatusDot) {
        this.syncStatusDot.className = "sync-status-dot error";
      }
      if (this.syncProgressMsg) {
        this.syncProgressMsg.textContent = `Sync error: ${(err && err.message) || String(err)}`;
      }
      if (typeof this.onWorkstationDisabledChange === "function") {
        this.onWorkstationDisabledChange(true, "Sync failed to start. Click ⚡ Sync.");
      }
      return null;
    }
  }

  /**
   * Cancel an in-progress firmware synchronization via `POST /api/sync/cancel`.
   *
   * @returns {Promise<object|null>}
   */
  async cancelSync() {
    if (this.syncCancelBtn) {
      this.syncCancelBtn.disabled = true;
      this.syncCancelBtn.textContent = "Cancelling...";
    }
    try {
      const res = await this.ctx.postJson("/api/sync/cancel", {});
      if (typeof this.onWorkstationDisabledChange === "function") {
        this.onWorkstationDisabledChange(
          true,
          "Target sync cancelled. Click ⚡ Sync to re-synchronize before using CLI.",
        );
      }
      return res;
    } catch {
      return null;
    }
  }

  /**
   * Start polling `GET /api/sync/status` every `500ms`.
   */
  startSyncPolling() {
    this.stopSyncPolling();
    this.syncPollTimer = this.ctx.setInterval(
      () => this.pollSyncStatus(),
      SYNC_POLL_INTERVAL_MS,
    );
    this.pollSyncStatus();
  }

  /**
   * Stop the active sync status polling interval.
   */
  stopSyncPolling() {
    if (this.syncPollTimer !== null) {
      this.ctx.clearInterval(this.syncPollTimer);
      this.syncPollTimer = null;
    }
  }

  /**
   * Poll `GET /api/sync/status` once and transition the Sync Matrix state machine.
   *
   * @returns {Promise<object|null>}
   */
  async pollSyncStatus() {
    try {
      const data = await this.ctx.fetchJson("/api/sync/status");
      if (!data) {
        return null;
      }

      const pctVal = Number(data.progress) || 0;
      if (this.syncProgressBar) {
        this.syncProgressBar.style.width = `${pctVal}%`;
      }
      if (this.syncProgressPct) {
        this.syncProgressPct.textContent = `${pctVal}%`;
      }
      if (this.syncProgressMsg) {
        this.syncProgressMsg.textContent = data.message || "Synchronizing...";
      }

      this._updateHeaderAndSyncMetadata(data);

      if (this.syncConsoleLog && Array.isArray(data.console)) {
        const shouldScroll =
          this.syncConsoleLog.scrollTop + this.syncConsoleLog.clientHeight >=
          this.syncConsoleLog.scrollHeight - CONSOLE_SCROLL_SLACK_PX;
        this.syncConsoleLog.innerHTML = data.console
          .map((line) => renderTerminalHtml(line))
          .join("<br>");
        if (shouldScroll) {
          this.syncConsoleLog.scrollTop = this.syncConsoleLog.scrollHeight;
        }
      }

      if (data.step === "complete") {
        if (this.syncStatusDot) {
          this.syncStatusDot.className = "sync-status-dot success";
        }
        this.stopSyncPolling();
        if (typeof this.onWorkstationDisabledChange === "function") {
          this.onWorkstationDisabledChange(false);
        }
        if (typeof this.onSyncComplete === "function") {
          this.onSyncComplete();
        }
        if (this.syncAutoDismissTimer === null) {
          this.syncAutoDismissTimer = this.ctx.setTimeout(() => {
            this.closeSyncModal();
            this.syncAutoDismissTimer = null;
          }, SYNC_AUTO_DISMISS_DELAY_MS);
        }
      } else if (data.step === "cancelled") {
        if (this.syncStatusDot) {
          this.syncStatusDot.className = "sync-status-dot error";
        }
        this.stopSyncPolling();
        if (typeof this.onWorkstationDisabledChange === "function") {
          this.onWorkstationDisabledChange(
            true,
            "Target sync cancelled. Click ⚡ Sync to re-synchronize before using CLI.",
          );
        }
      } else if (data.step === "failed") {
        if (this.syncStatusDot) {
          this.syncStatusDot.className = "sync-status-dot error";
        }
        this.stopSyncPolling();
        if (typeof this.onWorkstationDisabledChange === "function") {
          this.onWorkstationDisabledChange(
            true,
            "Target sync failed. Click ⚡ Sync to retry.",
          );
        }
      } else if (this.syncStatusDot) {
        this.syncStatusDot.className = "sync-status-dot pulse";
      }

      return data;
    } catch {
      return null;
    }
  }

  /**
   * Check `GET /api/sync/status` on startup and attach the progress modal if a sync is already running.
   *
   * @returns {Promise<void>}
   */
  async checkInitialSync() {
    try {
      const data = await this.ctx.fetchJson("/api/sync/status");
      if (data && (data.active || (data.progress > 0 && data.progress < 100))) {
        const suppressModal =
          typeof this.shouldSuppressAutoSyncModal === "function" &&
          Boolean(this.shouldSuppressAutoSyncModal());
        if (!suppressModal) {
          this.openSyncModal();
        }
        this.startSyncPolling();
      }
    } catch {
      // Non-fatal if sync endpoint is unreachable on startup.
    }
  }

  /**
   * Trigger a hardware reset via `POST /api/reset`.
   *
   * @returns {Promise<object|null>}
   */
  async triggerReset() {
    if (this.resetBtn) {
      this.resetBtn.disabled = true;
      this.resetBtn.textContent = "⚡ Resetting...";
    }
    try {
      return await this.ctx.postJson("/api/reset", {});
    } catch (err) {
      showToast(
        this.ctx,
        `Target reset failed: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    } finally {
      this.ctx.setTimeout(() => {
        if (this.resetBtn) {
          this.resetBtn.disabled = false;
          this.resetBtn.textContent = "⚡ Reset";
        }
      }, RESET_BUTTON_HOLD_MS);
    }
  }

  /**
   * Stop timers and unbind listeners.
   */
  destroy() {
    this.stopSyncPolling();
    if (this.syncAutoDismissTimer !== null) {
      this.ctx.clearTimeout(this.syncAutoDismissTimer);
      this.syncAutoDismissTimer = null;
    }
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating a `TargetConfigController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {TargetConfigController}
 */
export function createTargetConfig(ctx, options = {}) {
  return new TargetConfigController(ctx, options);
}
