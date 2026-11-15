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

const DEFAULT_FALLBACK_BINARIES = Object.freeze([
  "cat_detector_shell",
  "cat_detector_app",
  "carrier_board_shell",
]);
const DEFAULT_ARM_TARGET = "thumbv6m-none-eabi";
const DEFAULT_PROBE_SPEED_KHZ = 1000;
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
      package: boot.package || "cat_detector",
      binName: boot.binName || boot.bin_name || boot.bin || "cat_detector_shell",
      target: boot.target || DEFAULT_ARM_TARGET,
      release: Boolean(boot.release),
      speed: Number(boot.speed) || DEFAULT_PROBE_SPEED_KHZ,
      chip: boot.chip || null,
      showSystemLogs: Boolean(boot.showSystemLogs ?? boot.show_system_logs),
    };

    this.discoveredPackages = Array.isArray(boot.targetPackages || boot.target_packages)
      ? [...(boot.targetPackages || boot.target_packages)]
      : [];
    this.lastSession = boot.lastSession || boot.last_session || null;
    this.packageSelectionMode = Boolean(
      boot.packageSelectionMode ?? boot.package_selection_mode,
    );

    this.onWorkstationDisabledChange = options.onWorkstationDisabledChange || null;
    this.onShowSystemLogsChange = options.onShowSystemLogsChange || null;
    this.onSyncComplete = options.onSyncComplete || null;

    this.syncPollTimer = null;
    this.syncAutoDismissTimer = null;
    this._unsubscribers = [];

    // Header elements
    this.targetBadge = resolveElement(ctx, options.targetBadge || "targetBadge");
    this.binBadge = resolveElement(ctx, options.binBadge || "binBadge");
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
    this.cfgShowLogsCheckbox = resolveElement(
      ctx,
      options.cfgShowLogsCheckbox || "cfgShowLogsCheckbox",
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
        };
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
        : DEFAULT_FALLBACK_BINARIES;

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
   * Update inferred ARM target triple readout for `pkgName`.
   *
   * @param {string} pkgName - Selected Cargo package name.
   */
  updateInferredArchDisplay(pkgName) {
    const pkgData = this.discoveredPackages.find((p) => p.package === pkgName) || null;
    const target =
      (pkgData && pkgData.inferred_target) || this.currentConfig.target || DEFAULT_ARM_TARGET;

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
    hideModal(this.ctx, this.configModal);
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
      });

      if (data && data.success) {
        const storage = this.ctx.getStorage();
        storage.setItem(LAST_PACKAGE_STORAGE_KEY, pkg);
        storage.setItem(LAST_BIN_STORAGE_KEY, bin);
        storage.setItem(LAST_RELEASE_STORAGE_KEY, String(release));
        storage.setItem(SHOW_SYSTEM_LOGS_STORAGE_KEY, String(showLogs));

        this.currentConfig = {
          ...this.currentConfig,
          package: pkg,
          binName: bin,
          release,
          speed,
          target: data.target || target,
          showSystemLogs: showLogs,
        };
        this.lastSession = {
          package: pkg,
          bin,
          release,
          target: this.currentConfig.target,
          speed,
        };
        this.packageSelectionMode = false;

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
        this.openSyncModal();
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
