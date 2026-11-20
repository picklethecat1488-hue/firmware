import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import {
  createDaemonApp,
  UI_SCALE_STORAGE_KEY,
} from "../../js/daemon/daemon_app.js";

function mountFullDaemonDom(ctx) {
  const doc = ctx.getDocument();
  const ids = [
    ["span", "targetBadge"],
    ["span", "binBadge"],
    ["button", "btnSync"],
    ["button", "btnReset"],
    ["button", "btnTargetConfig"],
    ["button", "btnTargetBug"],
    ["select", "modeSelect"],
    ["select", "uiScaleSelect"],
    ["button", "btnToggleLog"],
    ["button", "btnClearOutput"],
    ["div", "syncModal"],
    ["button", "btnCloseSyncModal"],
    ["button", "btnSyncCancel"],
    ["button", "btnSyncDismiss"],
    ["strong", "syncInfoTarget"],
    ["strong", "syncInfoPackage"],
    ["strong", "syncInfoProfile"],
    ["strong", "syncInfoSpeed"],
    ["span", "syncStatusDot"],
    ["span", "syncProgressMsg"],
    ["div", "syncProgressBar"],
    ["span", "syncProgressPct"],
    ["div", "syncConsoleLog"],
    ["div", "targetBugModal"],
    ["button", "btnCloseTargetBugModal"],
    ["button", "btnCancelTargetBug"],
    ["button", "btnSubmitTargetBug"],
    ["input", "targetBugTitle"],
    ["select", "targetBugSeverity"],
    ["select", "targetBugCategory"],
    ["input", "targetBugComponent"],
    ["textarea", "targetBugDescription"],
    ["textarea", "targetBugSteps"],
    ["textarea", "targetBugExpected"],
    ["textarea", "targetBugActual"],
    ["input", "attachSystemLogs"],
    ["input", "attachCrashLog"],
    ["input", "attachTelemetry"],
    ["input", "attachMetadata"],
    ["button", "btnPasteTargetBugClipboard"],
    ["div", "targetBugDropzone"],
    ["input", "targetBugFileInput"],
    ["div", "targetBugAttachmentsGrid"],
    ["div", "targetBugProgressContainer"],
    ["div", "targetBugProgressBar"],
    ["span", "targetBugProgressMsg"],
    ["span", "targetBugProgressPct"],
    ["div", "targetBugStatusMsg"],
    ["div", "targetConfigModal"],
    ["button", "btnCloseTargetConfigModal"],
    ["select", "cfgPackageSelect"],
    ["select", "cfgBinSelect"],
    ["input", "cfgReleaseCheckbox"],
    ["select", "cfgSpeedSelect"],
    ["span", "cfgInferredArchDisplay"],
    ["span", "cfgInferredArchNote"],
    ["input", "cfgTargetArchInput"],
    ["input", "cfgShowLogsCheckbox"],
    ["div", "cfgStatusMsg"],
    ["button", "btnCancelTargetConfig"],
    ["button", "btnConfirmTargetConfig"],
    ["div", "logPanel"],
    ["button", "btnLeftTabTests"],
    ["button", "btnLeftTabLogs"],
    ["span", "leftPanelTestTools"],
    ["span", "leftPanelLogTools"],
    ["button", "btnCopyLog"],
    ["button", "btnCollapseLog"],
    ["span", "logAutoScrollBadge"],
    ["div", "logView"],
    ["div", "testSidebarView"],
    ["div", "vSplitter"],
    ["span", "cliPanelTitle"],
    ["button", "btnCopyWorkstation"],
    ["div", "fsToolbar"],
    ["div", "manualPromptBanner"],
    ["span", "manualPromptSummary"],
    ["div", "cliView"],
    ["div", "fsView"],
    ["div", "testView"],
    ["div", "testManualPromptCard"],
    ["tbody", "testTableBody"],
    ["div", "testConsoleOutput"],
    ["div", "cliFuzzyHistory"],
    ["input", "fuzzySearchInput"],
    ["button", "btnCloseFuzzyHistory"],
    ["div", "fuzzyResultsList"],
    ["span", "promptLabel"],
    ["input", "cmdInput"],
    ["button", "btnSubmitCmd"],
  ];

  const map = {};
  for (const [tag, id] of ids) {
    const el = createEl(ctx, tag, { id });
    if (id.endsWith("Modal") || id === "logPanel" || id === "vSplitter" || id === "cliFuzzyHistory") {
      el.style.display = "none";
    }
    doc.body.appendChild(el);
    map[id] = el;
  }
  return map;
}

describe("Host Daemon Root Workstation Application (DaemonApp)", () => {
  it("boots from #app-boot-config JSON, coordinates all child controllers, scales UI via --ui-scale, and handles global shortcuts", async () => {
    let logsPollCount = 0;
    let activePromptState = { active: false };

    const ctx = createMockRuntimeContext({
      inIframe: true,
      bootConfig: {
        package: "cat_detector",
        binName: "cat_detector_shell",
        target: "thumbv6m-none-eabi",
        activeMode: "CLI",
        release: true,
        speed: 4000,
        showSystemLogs: false,
        packageSelectionMode: false,
        targetPackages: [
          {
            package: "cat_detector",
            binaries: ["cat_detector_shell"],
            inferred_target: "thumbv6m-none-eabi",
          },
        ],
      },
      fetchRoutes: {
        "GET /api/logs": () => {
          logsPollCount += 1;
          return {
            status: 200,
            body: [{ id: 1, level: "INFO", channel: "rtt", message: "Boot ready" }],
          };
        },
        "GET /api/cli/output": {
          status: 200,
          body: [{ id: 1, is_input: false, text: "shell ready" }],
        },
        "GET /api/fs/output": { status: 200, body: [] },
        "GET /api/cli/history": { status: 200, body: { history: ["help", "gpio status"] } },
        "GET /api/sync/status": { status: 200, body: { active: false, step: "idle", progress: 0 } },
        "POST /api/mode": { status: 200, body: { success: true, mode: "TEST" } },
        "GET /api/manual_prompt": () => ({ status: 200, body: activePromptState }),
        "POST /api/tests/scan": {
          status: 200,
          body: { success: true, tests: [], facets: {}, detected_board: null },
        },
        "GET /api/tests/status": () => ({
          status: 200,
          body: {
            active: false,
            step: "idle",
            results_map: {},
            console: [],
            manual_prompt: activePromptState,
          },
        }),
      },
    });

    const dom = mountFullDaemonDom(ctx);
    const doc = ctx.getDocument();

    const app = await createDaemonApp(ctx).init();

    // Iframe context class & initial Retina-safe --ui-scale
    assert.equal(doc.documentElement.classList.contains("in-iframe"), true);
    assert.equal(doc.documentElement.style.getPropertyValue("--ui-scale"), "1.0000");
    assert.equal(doc.documentElement.style.zoom, "");

    // Keyboard zoom shortcuts (Cmd/Ctrl + '+', '-', '0') update --ui-scale & storage
    ctx.emitViewportEvent("keydown", { key: "+", ctrlKey: true });
    assert.equal(ctx.getStorage().getItem(UI_SCALE_STORAGE_KEY), "110%");
    assert.equal(doc.documentElement.style.getPropertyValue("--ui-scale"), "1.1000");

    ctx.emitViewportEvent("keydown", { key: "0", metaKey: true });
    assert.equal(ctx.getStorage().getItem(UI_SCALE_STORAGE_KEY), "100%");
    assert.equal(doc.documentElement.style.getPropertyValue("--ui-scale"), "1.0000");

    // Toggle system logs via #btnToggleLog and sync checkbox in target config
    assert.equal(app.rttStream.isPanelVisible(), false);
    dom.btnToggleLog.click();
    assert.equal(app.rttStream.isPanelVisible(), true);
    assert.equal(dom.cfgShowLogsCheckbox.checked, true);

    // Global Ctrl+R opens fuzzy history search; Escape dismisses it
    ctx.emitViewportEvent("keydown", { key: "r", ctrlKey: true, target: dom.cmdInput });
    assert.equal(app.cliConsole.isFuzzyOpen, true);
    ctx.emitViewportEvent("keydown", { key: "Escape" });
    assert.equal(app.cliConsole.isFuzzyOpen, false);

    // Global Escape also dismisses open Target Bug modal
    await app.wormReporter.openModal();
    assert.equal(app.wormReporter.isModalOpen(), true);
    ctx.emitViewportEvent("keydown", { key: "Escape" });
    assert.equal(app.wormReporter.isModalOpen(), false);

    // Manual pytest prompt banner updates from /api/manual_prompt
    activePromptState = {
      active: true,
      project: "cat_detector",
      test_name: "test_motor_calibration_routine",
      procedure: "Fill reservoir above minimum waterline.",
      prompt: "Confirm smooth pump flow.",
    };
    await app.pollManualPrompt();
    assert.equal(dom.manualPromptBanner.style.display, "flex");
    assert.ok(dom.manualPromptSummary.textContent.includes("test_motor_calibration_routine"));

    // Clear button clears both RTT and CLI views
    assert.equal(dom.logView.children.length, 1);
    assert.equal(dom.cliView.children.length, 1);
    dom.btnClearOutput.click();
    assert.equal(dom.logView.children.length, 0);
    assert.equal(dom.cliView.children.length, 0);

    // Clicking #manualPromptBanner transitions workstation directly into TEST mode and opens left TEST SUITE tab
    dom.manualPromptBanner.click();
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(app.cliConsole.getMode(), "TEST");
    assert.equal(dom.testView.style.display, "flex");
    assert.equal(app.rttStream.isPanelVisible(), true);
    assert.equal(app.testRunner.getActiveLeftTab(), "TESTS");
    assert.equal(dom.testSidebarView.style.display, "flex");

    // Periodic 1000ms background poll fires while active and stops after destroy()
    const beforeTick = logsPollCount;
    ctx.advanceTime(1100);
    await Promise.resolve();
    assert.ok(logsPollCount > beforeTick);

    const afterTick = logsPollCount;
    app.destroy();
    ctx.advanceTime(3000);
    assert.equal(logsPollCount, afterTick);
  });
});
