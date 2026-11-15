import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import { SHOW_SYSTEM_LOGS_STORAGE_KEY } from "../../js/daemon/rtt_stream.js";
import {
  createTargetConfig,
  describeTargetArchNote,
  LAST_BIN_STORAGE_KEY,
  LAST_PACKAGE_STORAGE_KEY,
  LAST_RELEASE_STORAGE_KEY,
} from "../../js/daemon/target_config.js";

function buildTargetConfigDom(ctx) {
  const doc = ctx.getDocument();
  const targetBadge = createEl(ctx, "span", { id: "targetBadge", text: "thumbv6m-none-eabi" });
  const binBadge = createEl(ctx, "span", { id: "binBadge", text: "cat_detector_shell" });
  const syncBtn = createEl(ctx, "button", { id: "btnSync", text: "⚡ Sync" });
  const resetBtn = createEl(ctx, "button", { id: "btnReset", text: "⚡ Reset" });
  const targetConfigBtn = createEl(ctx, "button", { id: "btnTargetConfig", text: "🎯 Target Config" });

  const cfgPackageSelect = createEl(ctx, "select", { id: "cfgPackageSelect" });
  const cfgBinSelect = createEl(ctx, "select", { id: "cfgBinSelect" });
  const cfgReleaseCheckbox = createEl(ctx, "input", { id: "cfgReleaseCheckbox" });
  const cfgSpeedSelect = createEl(ctx, "select", { id: "cfgSpeedSelect" });
  cfgSpeedSelect.value = "4000";
  const cfgArchDisplay = createEl(ctx, "span", { id: "cfgInferredArchDisplay" });
  const cfgArchNote = createEl(ctx, "span", { id: "cfgInferredArchNote" });
  const cfgArchInput = createEl(ctx, "input", { id: "cfgTargetArchInput" });
  const cfgShowLogsCheckbox = createEl(ctx, "input", { id: "cfgShowLogsCheckbox" });
  const cfgStatusMsg = createEl(ctx, "div", { id: "cfgStatusMsg", style: { display: "none" } });
  const cfgCancelBtn = createEl(ctx, "button", { id: "btnCancelTargetConfig", text: "DISMISS" });
  const cfgCloseBtn = createEl(ctx, "button", { id: "btnCloseTargetConfigModal", text: "✕" });
  const cfgConfirmBtn = createEl(ctx, "button", { id: "btnConfirmTargetConfig", text: "FLASH" });

  const configModal = createEl(ctx, "div", {
    id: "targetConfigModal",
    style: { display: "none" },
    children: [
      cfgPackageSelect,
      cfgBinSelect,
      cfgReleaseCheckbox,
      cfgSpeedSelect,
      cfgArchDisplay,
      cfgArchNote,
      cfgArchInput,
      cfgShowLogsCheckbox,
      cfgStatusMsg,
      cfgCancelBtn,
      cfgCloseBtn,
      cfgConfirmBtn,
    ],
  });

  const syncCloseBtn = createEl(ctx, "button", { id: "btnCloseSyncModal", text: "✕" });
  const syncCancelBtn = createEl(ctx, "button", { id: "btnSyncCancel", text: "Cancel" });
  const syncDismissBtn = createEl(ctx, "button", { id: "btnSyncDismiss", text: "Dismiss" });
  const syncInfoTarget = createEl(ctx, "strong", { id: "syncInfoTarget" });
  const syncInfoPackage = createEl(ctx, "strong", { id: "syncInfoPackage" });
  const syncInfoProfile = createEl(ctx, "strong", { id: "syncInfoProfile" });
  const syncInfoSpeed = createEl(ctx, "strong", { id: "syncInfoSpeed" });
  const syncStatusDot = createEl(ctx, "span", { id: "syncStatusDot", className: "sync-status-dot pulse" });
  const syncProgressMsg = createEl(ctx, "span", { id: "syncProgressMsg" });
  const syncProgressBar = createEl(ctx, "div", { id: "syncProgressBar", style: { width: "0%" } });
  const syncProgressPct = createEl(ctx, "span", { id: "syncProgressPct", text: "0%" });
  const syncConsoleLog = createEl(ctx, "div", { id: "syncConsoleLog" });

  const syncModal = createEl(ctx, "div", {
    id: "syncModal",
    style: { display: "none" },
    children: [
      syncCloseBtn,
      syncCancelBtn,
      syncDismissBtn,
      syncInfoTarget,
      syncInfoPackage,
      syncInfoProfile,
      syncInfoSpeed,
      syncStatusDot,
      syncProgressMsg,
      syncProgressBar,
      syncProgressPct,
      syncConsoleLog,
    ],
  });

  doc.body.appendChild(targetBadge);
  doc.body.appendChild(binBadge);
  doc.body.appendChild(syncBtn);
  doc.body.appendChild(resetBtn);
  doc.body.appendChild(targetConfigBtn);
  doc.body.appendChild(configModal);
  doc.body.appendChild(syncModal);

  return {
    targetBadge,
    binBadge,
    syncBtn,
    resetBtn,
    targetConfigBtn,
    configModal,
    cfgPackageSelect,
    cfgBinSelect,
    cfgReleaseCheckbox,
    cfgSpeedSelect,
    cfgArchDisplay,
    cfgArchNote,
    cfgArchInput,
    cfgShowLogsCheckbox,
    cfgConfirmBtn,
    syncModal,
    syncCancelBtn,
    syncStatusDot,
    syncProgressMsg,
    syncProgressBar,
    syncProgressPct,
    syncConsoleLog,
  };
}

describe("Host Daemon Target Configuration, Sync Flash Matrix & Reset Controller", () => {
  it("infers ARM architecture notes, auto-opens in packageSelectionMode, persists target config, and drives sync completion", async () => {
    assert.ok(describeTargetArchNote("thumbv6m-none-eabi").includes("Cortex-M0+"));
    assert.ok(describeTargetArchNote("thumbv8m.main-none-eabihf").includes("Cortex-M33"));
    assert.ok(describeTargetArchNote("thumbv7em-none-eabihf").includes("Cortex-M4F"));

    let syncStatusPayload = {
      active: true,
      step: "compiling",
      progress: 45,
      message: "Compiling carrier_board_shell...",
      target: "thumbv8m.main-none-eabihf",
      package: "carrier_board",
      bin: "carrier_board_shell",
      profile: "release",
      speed: 4000,
      console: ["\x1b[1;32mCompiling\x1b[0m carrier_board v0.1.0"],
    };
    const configuredPayloads = [];
    const showLogsEvents = [];
    const disabledChanges = [];
    let syncCompletedCount = 0;

    const bootConfig = {
      package: "cat_detector",
      binName: "cat_detector_shell",
      target: "thumbv6m-none-eabi",
      release: false,
      speed: 1000,
      packageSelectionMode: true,
      targetPackages: [
        {
          package: "cat_detector",
          binaries: ["cat_detector_shell", "cat_detector_app"],
          default_bin: "cat_detector_shell",
          inferred_target: "thumbv6m-none-eabi",
        },
        {
          package: "carrier_board",
          binaries: ["carrier_board_shell"],
          default_bin: "carrier_board_shell",
          inferred_target: "thumbv8m.main-none-eabihf",
        },
      ],
      lastSession: {
        package: "carrier_board",
        bin: "carrier_board_shell",
        release: true,
        speed: 4000,
      },
    };

    const ctx = createMockRuntimeContext({
      bootConfig,
      fetchRoutes: {
        "POST /api/target/configure": (_url, opts) => {
          const parsed = JSON.parse(opts.body);
          configuredPayloads.push(parsed);
          return {
            status: 200,
            body: { success: true, target: parsed.target },
          };
        },
        "GET /api/sync/status": () => ({ status: 200, body: syncStatusPayload }),
      },
    });

    const dom = buildTargetConfigDom(ctx);
    const controller = createTargetConfig(ctx, {
      bootConfig,
      onShowSystemLogsChange: (show) => showLogsEvents.push(show),
      onWorkstationDisabledChange: (dis, msg) => disabledChanges.push({ dis, msg }),
      onSyncComplete: () => {
        syncCompletedCount += 1;
      },
    });

    await controller.init();

    // Auto-opened in packageSelectionMode and pre-populated from lastSession
    assert.equal(dom.configModal.style.display, "flex");
    assert.equal(dom.cfgPackageSelect.value, "carrier_board");
    assert.equal(dom.cfgBinSelect.value, "carrier_board_shell");
    assert.equal(dom.cfgReleaseCheckbox.checked, true);
    assert.equal(dom.cfgArchDisplay.textContent, "thumbv8m.main-none-eabihf");

    dom.cfgShowLogsCheckbox.checked = true;
    await controller.submitConfig();

    assert.equal(configuredPayloads.length, 1);
    assert.equal(configuredPayloads[0].package, "carrier_board");
    assert.equal(configuredPayloads[0].bin, "carrier_board_shell");
    assert.equal(configuredPayloads[0].release, true);
    assert.equal(configuredPayloads[0].speed, 4000);
    assert.equal(configuredPayloads[0].show_system_logs, true);

    assert.equal(ctx.getStorage().getItem(LAST_PACKAGE_STORAGE_KEY), "carrier_board");
    assert.equal(ctx.getStorage().getItem(LAST_BIN_STORAGE_KEY), "carrier_board_shell");
    assert.equal(ctx.getStorage().getItem(LAST_RELEASE_STORAGE_KEY), "true");
    assert.equal(ctx.getStorage().getItem(SHOW_SYSTEM_LOGS_STORAGE_KEY), "true");
    assert.deepEqual(showLogsEvents, [true]);

    // Config modal closed and Sync Matrix modal opened
    assert.equal(dom.configModal.style.display, "none");
    assert.equal(dom.syncModal.style.display, "flex");

    await controller.pollSyncStatus();
    assert.equal(dom.syncProgressBar.style.width, "45%");
    assert.equal(dom.syncProgressPct.textContent, "45%");
    assert.ok(dom.syncConsoleLog.innerHTML.includes("carrier_board"));

    // Transition sync to complete -> enables workstation and auto-dismisses after 1500ms
    syncStatusPayload = {
      ...syncStatusPayload,
      active: false,
      step: "complete",
      progress: 100,
      message: "Target flashed & RTT attached",
    };
    await controller.pollSyncStatus();
    assert.equal(dom.syncStatusDot.className, "sync-status-dot success");
    assert.equal(syncCompletedCount, 1);
    assert.equal(disabledChanges[disabledChanges.length - 1].dis, false);

    ctx.advanceTime(1600);
    assert.equal(dom.syncModal.style.display, "none");

    controller.destroy();
  });

  it("handles sync cancellation, sync failure workstation lockout, and hardware reset button lifecycle", async () => {
    let syncStep = "flashing";
    let resetCalls = 0;
    const disabledEvents = [];

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "POST /api/sync": { status: 200, body: { success: true } },
        "POST /api/sync/cancel": () => {
          syncStep = "cancelled";
          return { status: 200, body: { success: true, step: "cancelled" } };
        },
        "GET /api/sync/status": () => ({
          status: 200,
          body: {
            active: syncStep === "flashing",
            step: syncStep,
            progress: 60,
            message: syncStep === "cancelled" ? "Sync cancelled by user" : "Flashing...",
            console: [],
          },
        }),
        "POST /api/reset": () => {
          resetCalls += 1;
          return { status: 200, body: { success: true } };
        },
      },
    });

    const dom = buildTargetConfigDom(ctx);
    const controller = createTargetConfig(ctx, {
      onWorkstationDisabledChange: (dis, msg) => disabledEvents.push({ dis, msg }),
    });
    await controller.init();

    await controller.triggerSync();
    assert.equal(dom.syncModal.style.display, "flex");

    await controller.cancelSync();
    await controller.pollSyncStatus();
    assert.equal(dom.syncStatusDot.className, "sync-status-dot error");
    assert.equal(disabledEvents[disabledEvents.length - 1].dis, true);
    assert.ok(disabledEvents[disabledEvents.length - 1].msg.includes("cancelled"));

    // Trigger hardware reset button and verify temporary disabled state
    const resetPromise = controller.triggerReset();
    assert.equal(dom.resetBtn.disabled, true);
    assert.equal(dom.resetBtn.textContent, "⚡ Resetting...");
    await resetPromise;
    assert.equal(resetCalls, 1);

    ctx.advanceTime(1300);
    assert.equal(dom.resetBtn.disabled, false);
    assert.equal(dom.resetBtn.textContent, "⚡ Reset");

    controller.destroy();
  });
});
