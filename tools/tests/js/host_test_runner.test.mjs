import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import {
  createTestRunner,
  LEFT_PANEL_TABS,
  MANUAL_PROMPT_ACTIONS,
  MANUAL_PROMPT_KINDS,
  statusBadgeClass,
} from "../../js/daemon/test_runner.js";

function buildTestRunnerDom(ctx) {
  const doc = ctx.getDocument();
  const leftTabLogsBtn = createEl(ctx, "button", { id: "btnLeftTabLogs", text: "SYSTEM LOGS" });
  const leftTabTestsBtn = createEl(ctx, "button", { id: "btnLeftTabTests", text: "TEST SUITE" });
  const leftTabTestLogBtn = createEl(ctx, "button", {
    id: "btnLeftTabTestLog",
    text: "TEST EXECUTION LOG",
  });
  const logView = createEl(ctx, "div", { id: "logView", style: { display: "block" } });

  const projectSelect = createEl(ctx, "select", { id: "testFilterProject" });
  projectSelect.value = "all";
  const scenarioSelect = createEl(ctx, "select", { id: "testFilterScenario" });
  scenarioSelect.value = "all";
  const boardInput = createEl(ctx, "input", { id: "testFilterBoard" });
  boardInput.value = "*";
  const binarySelect = createEl(ctx, "select", { id: "testFilterBinary" });
  binarySelect.value = "all";
  const manualSelect = createEl(ctx, "select", { id: "testFilterManual" });
  manualSelect.value = "all";
  const searchInput = createEl(ctx, "input", { id: "testFilterSearch" });
  searchInput.value = "";
  const enableManualCb = createEl(ctx, "input", { id: "testEnableManualCb" });
  enableManualCb.checked = true;

  const scanBtn = createEl(ctx, "button", { id: "btnTestScan", text: "🔄 Scan" });
  const runSelectedBtn = createEl(ctx, "button", {
    id: "btnTestRunSelected",
    text: "▶ Run Selected (0)",
  });
  const runFilteredBtn = createEl(ctx, "button", {
    id: "btnTestRunFiltered",
    text: "▶ Run Filtered (0)",
  });
  const cancelBtn = createEl(ctx, "button", { id: "btnTestCancel", text: "⏹ Stop" });
  const selectAllCb = createEl(ctx, "input", { id: "testSelectAllCb" });

  const boardBadge = createEl(ctx, "span", { id: "testDetectedBoardBadge" });
  const summaryStats = createEl(ctx, "span", { id: "testSummaryStats" });
  const tableBody = createEl(ctx, "tbody", { id: "testTableBody" });
  const consoleOutput = createEl(ctx, "div", { id: "testConsoleOutput" });

  const manualTitle = createEl(ctx, "strong", { id: "testManualTitle" });
  const manualProcedure = createEl(ctx, "span", { id: "testManualProcedure" });
  const manualPromptText = createEl(ctx, "span", { id: "testManualPromptText" });
  const manualYesBtn = createEl(ctx, "button", { id: "btnManualYes", text: "YES" });
  const manualNoBtn = createEl(ctx, "button", { id: "btnManualNo", text: "NO" });
  const manualYesNoRow = createEl(ctx, "div", {
    id: "testManualYesNoRow",
    children: [manualYesBtn, manualNoBtn],
  });
  const manualValueInput = createEl(ctx, "input", { id: "testManualValueInput" });
  const manualUnitLabel = createEl(ctx, "span", { id: "testManualUnitLabel" });
  const manualSubmitValueBtn = createEl(ctx, "button", {
    id: "btnManualSubmitValue",
    text: "Submit Measurement",
  });
  const manualMeasurementRow = createEl(ctx, "div", {
    id: "testManualMeasurementRow",
    style: { display: "none" },
    children: [manualValueInput, manualUnitLabel, manualSubmitValueBtn],
  });
  const manualCommentInput = createEl(ctx, "input", { id: "testManualCommentInput" });
  const manualCancelBtn = createEl(ctx, "button", {
    id: "btnManualCancel",
    text: "Cancel Test",
  });
  const manualErrorMsg = createEl(ctx, "div", {
    id: "testManualErrorMsg",
    style: { display: "none" },
  });
  const manualCard = createEl(ctx, "div", {
    id: "testManualPromptCard",
    style: { display: "none" },
    children: [
      manualTitle,
      manualProcedure,
      manualPromptText,
      manualYesNoRow,
      manualMeasurementRow,
      manualCommentInput,
      manualCancelBtn,
      manualErrorMsg,
    ],
  });

  const sidebarView = createEl(ctx, "div", {
    id: "testSidebarView",
    style: { display: "none" },
    children: [
      projectSelect,
      scenarioSelect,
      boardInput,
      binarySelect,
      manualSelect,
      searchInput,
      enableManualCb,
      scanBtn,
      runSelectedBtn,
      runFilteredBtn,
      cancelBtn,
      selectAllCb,
      boardBadge,
      summaryStats,
      tableBody,
    ],
  });

  const viewEl = createEl(ctx, "div", {
    id: "testView",
    style: { display: "flex" },
    children: [manualCard],
  });
  doc.body.appendChild(leftTabLogsBtn);
  doc.body.appendChild(leftTabTestsBtn);
  doc.body.appendChild(leftTabTestLogBtn);
  doc.body.appendChild(logView);
  doc.body.appendChild(sidebarView);
  doc.body.appendChild(consoleOutput);
  doc.body.appendChild(viewEl);

  return {
    leftTabLogsBtn,
    leftTabTestsBtn,
    leftTabTestLogBtn,
    logView,
    sidebarView,
    viewEl,
    projectSelect,
    scenarioSelect,
    boardInput,
    binarySelect,
    manualSelect,
    searchInput,
    enableManualCb,
    scanBtn,
    runSelectedBtn,
    runFilteredBtn,
    cancelBtn,
    selectAllCb,
    boardBadge,
    summaryStats,
    tableBody,
    consoleOutput,
    manualCard,
    manualTitle,
    manualProcedure,
    manualPromptText,
    manualYesNoRow,
    manualYesBtn,
    manualNoBtn,
    manualMeasurementRow,
    manualValueInput,
    manualUnitLabel,
    manualSubmitValueBtn,
    manualCommentInput,
    manualCancelBtn,
    manualErrorMsg,
  };
}

describe("Host Daemon Test Mode UI Controller (TestRunnerController)", () => {
  it("scans @device_test suites with attribute filters, manages selection, runs ordered node IDs, and stops runs", async () => {
    assert.equal(statusBadgeClass("PASSED"), "badge badge-green");
    assert.equal(statusBadgeClass("FAILED"), "badge test-badge-red");
    assert.equal(statusBadgeClass("SKIPPED"), "badge badge-amber");

    const sampleTests = [
      {
        order: 1,
        node_id: "tools/device_tests/suites/test_cat_detector_smoke.py::test_battery_fuel_gauge_and_alert",
        name: "test_battery_fuel_gauge_and_alert",
        project: "cat_detector",
        boards: ["rp2040*"],
        scenarios: ["smoke", "bringup"],
        target_binary: "cat_detector_shell",
        manual: false,
        description: "Verify MAX17048 fuel gauge telemetry.",
      },
      {
        order: 2,
        node_id: "tools/device_tests/suites/test_carrier_board_bringup.py::test_power_rails_and_protection",
        name: "test_power_rails_and_protection",
        project: "carrier_board",
        boards: ["carrier_board*"],
        scenarios: ["bringup"],
        target_binary: null,
        manual: true,
        description: "Measure TP1..TP4 rail voltages.",
      },
    ];

    const runPayloads = [];
    const scanPayloads = [];
    let cancelCalls = 0;
    let statusSnapshot = {
      active: false,
      step: "idle",
      total: 0,
      completed: 0,
      passed: 0,
      failed: 0,
      skipped: 0,
      results_map: {},
      console: [],
      manual_prompt: { active: false },
    };

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "POST /api/tests/scan": (_url, opts) => {
          const req = JSON.parse(opts.body);
          scanPayloads.push(req);
          const matching =
            req.project && req.project !== "all"
              ? sampleTests.filter((t) => t.project === req.project)
              : sampleTests;
          return {
            status: 200,
            body: {
              success: true,
              tests: matching,
              detected_board: { board_type: "rp2040_flash_2mb", chip: "rp2040" },
              facets: {
                projects: ["carrier_board", "cat_detector"],
                scenarios: ["smoke", "integration", "bringup"],
                binaries: ["carrier_board_shell", "cat_detector_shell", "none"],
                boards: ["carrier_board*", "rp2040*"],
              },
            },
          };
        },
        "GET /api/tests/status": () => ({ status: 200, body: statusSnapshot }),
        "POST /api/tests/run": (_url, opts) => {
          const parsed = JSON.parse(opts.body);
          runPayloads.push(parsed);
          statusSnapshot = {
            active: true,
            step: "running",
            total: parsed.node_ids.length,
            completed: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            results_map: {
              [parsed.node_ids[0]]: {
                node_id: parsed.node_ids[0],
                status: "RUNNING",
                duration_s: null,
              },
            },
            console: ["\x1b[1;36mcollecting ... collected 1 item\x1b[0m"],
            manual_prompt: { active: false },
          };
          return { status: 200, body: { success: true, status: statusSnapshot } };
        },
        "POST /api/tests/cancel": () => {
          cancelCalls += 1;
          statusSnapshot = {
            ...statusSnapshot,
            active: false,
            step: "cancelled",
          };
          return { status: 200, body: { success: true, status: statusSnapshot } };
        },
      },
    });

    const dom = buildTestRunnerDom(ctx);
    const controller = createTestRunner(ctx);
    controller.init();
    await controller.ensureInitialized();

    assert.equal(dom.boardBadge.textContent, "BOARD: rp2040_flash_2mb (rp2040)");
    assert.equal(dom.tableBody.children.length, 2);
    assert.equal(dom.projectSelect.children.length, 3); // All + 2 projects

    // Regression WORM-LITHE-NYMPH-529: Left panel switches cleanly between TEST SUITE and SYSTEM LOGS tabs
    controller.setTestModeActive(true);
    assert.equal(controller.getActiveLeftTab(), LEFT_PANEL_TABS.TESTS);
    assert.equal(dom.sidebarView.style.display, "flex");
    assert.equal(dom.logView.style.display, "none");

    dom.leftTabLogsBtn.click();
    assert.equal(controller.getActiveLeftTab(), LEFT_PANEL_TABS.LOGS);
    assert.equal(dom.sidebarView.style.display, "none");
    assert.equal(dom.consoleOutput.style.display, "none");
    assert.equal(dom.logView.style.display, "block");

    // Regression WORM-ROOTED-FIREWORM-570: Left panel switches to TEST EXECUTION LOG tab
    dom.leftTabTestLogBtn.click();
    assert.equal(controller.getActiveLeftTab(), LEFT_PANEL_TABS.TEST_LOG);
    assert.equal(dom.sidebarView.style.display, "none");
    assert.equal(dom.logView.style.display, "none");
    assert.equal(dom.consoleOutput.style.display, "block");

    dom.leftTabTestsBtn.click();
    assert.equal(controller.getActiveLeftTab(), LEFT_PANEL_TABS.TESTS);
    assert.equal(dom.sidebarView.style.display, "flex");
    assert.equal(dom.consoleOutput.style.display, "none");

    // Regression WORM-FOSSORIAL-NIGHTCRAWLER-861: Test cell uses flex-wrap .test-badge-group
    // with discrete scenario/attribute pills so wrapped items never overlap badge borders.
    const firstRowCells = dom.tableBody.children[0].children;
    const modeScenarioGroup = firstRowCells[2].querySelector(".test-badge-group");
    assert.ok(modeScenarioGroup, "Expected flex badge group wrapper inside left-panel Test cell");
    assert.equal(modeScenarioGroup.children[0].textContent, "AUTO");
    assert.equal(modeScenarioGroup.children[1].textContent, "smoke");
    assert.equal(modeScenarioGroup.children[2].textContent, "bringup");

    // Select all visible tests and execute `runSelected()`
    dom.selectAllCb.checked = true;
    dom.selectAllCb.dispatchEvent({ type: "change", bubbles: true });
    assert.equal(controller.selectedNodeIds.size, 2);
    assert.equal(dom.runSelectedBtn.textContent, "▶ Run Selected (2)");

    await controller.runSelected();
    assert.equal(runPayloads.length, 1);
    assert.deepEqual(runPayloads[0].node_ids, [
      sampleTests[0].node_id,
      sampleTests[1].node_id,
    ]);
    assert.equal(runPayloads[0].enable_manual, true);
    assert.equal(controller.isRunning, true);
    assert.ok(dom.consoleOutput.innerHTML.includes("collected 1 item"));

    // Cancel active run
    await controller.cancelRun();
    assert.equal(cancelCalls, 1);
    assert.equal(controller.isRunning, false);
    assert.equal(controller.activeStep, "cancelled");

    // Apply project filter -> triggers /api/tests/scan (probe_board=false) and prunes hidden selections
    dom.projectSelect.value = "cat_detector";
    await controller.scanTests({ probeBoard: false });
    assert.equal(scanPayloads[scanPayloads.length - 1].project, "cat_detector");
    assert.equal(scanPayloads[scanPayloads.length - 1].probe_board, false);
    assert.equal(dom.tableBody.children.length, 1);
    assert.equal(controller.selectedNodeIds.size, 1);

    controller.destroy();
  });

  it("renders interactive manual prompts for YES/NO, numeric measurements, and cancellation with required comment", async () => {
    const submittedResponses = [];
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "POST /api/manual_prompt/respond": (_url, opts) => {
          const body = JSON.parse(opts.body);
          submittedResponses.push(body);
          return {
            status: 200,
            body: {
              success: true,
              manual_prompt: { active: false, response: body },
            },
          };
        },
      },
    });

    const dom = buildTestRunnerDom(ctx);
    const controller = createTestRunner(ctx);
    controller.init();

    // 1. Numeric measurement prompt
    controller.renderManualPrompt({
      active: true,
      prompt_id: "meas-001",
      kind: MANUAL_PROMPT_KINDS.MEASUREMENT,
      unit: "V",
      project: "carrier_board",
      test_name: "test_power_rails_and_protection",
      procedure: "Apply bench supply to USB_VBUS.",
      prompt: "Measure voltage on TP1 (USB_VBUS)",
    });

    assert.equal(dom.manualCard.style.display, "flex");
    assert.equal(dom.manualYesNoRow.style.display, "none");
    assert.equal(dom.manualMeasurementRow.style.display, "flex");
    assert.equal(dom.manualUnitLabel.textContent, "(V)");

    // Submitting empty value displays validation error without calling server
    await controller.submitManualResponse(MANUAL_PROMPT_ACTIONS.MEASUREMENT);
    assert.equal(submittedResponses.length, 0);
    assert.equal(dom.manualErrorMsg.style.display, "block");

    dom.manualValueInput.value = "5.02";
    await controller.submitManualResponse(MANUAL_PROMPT_ACTIONS.MEASUREMENT);
    assert.equal(submittedResponses.length, 1);
    assert.deepEqual(submittedResponses[0], {
      prompt_id: "meas-001",
      action: "measurement",
      value: "5.02",
      comment: "",
    });
    assert.equal(dom.manualCard.style.display, "none");

    // 2. YES/NO prompt + cancel validation requiring comment
    controller.renderManualPrompt({
      active: true,
      prompt_id: "yn-002",
      kind: MANUAL_PROMPT_KINDS.YES_NO,
      project: "cat_detector",
      test_name: "test_motor_calibration_routine",
      procedure: "Observe pump ramp.",
      prompt: "Did the pump ramp smoothly?",
    });
    assert.equal(dom.manualYesNoRow.style.display, "flex");
    assert.equal(dom.manualMeasurementRow.style.display, "none");

    // Cancel without comment is blocked
    await controller.submitManualResponse(MANUAL_PROMPT_ACTIONS.CANCEL);
    assert.equal(submittedResponses.length, 1);
    assert.ok(dom.manualErrorMsg.textContent.includes("comment"));

    // Cancel with comment succeeds
    dom.manualCommentInput.value = "Reservoir drained for bench inspection";
    await controller.submitManualResponse(MANUAL_PROMPT_ACTIONS.CANCEL);
    assert.equal(submittedResponses.length, 2);
    assert.equal(submittedResponses[1].action, "cancel");
    assert.equal(
      submittedResponses[1].comment,
      "Reservoir drained for bench inspection",
    );

    // Re-open YES/NO prompt and click YES
    controller.renderManualPrompt({
      active: true,
      prompt_id: "yn-003",
      kind: MANUAL_PROMPT_KINDS.YES_NO,
      project: "cat_detector",
      test_name: "test_motor_calibration_routine",
      procedure: "Observe pump ramp.",
      prompt: "Did the pump ramp smoothly?",
    });
    dom.manualYesBtn.click();
    await Promise.resolve();
    assert.equal(submittedResponses.length, 3);
    assert.equal(submittedResponses[2].action, "yes");

    controller.destroy();
  });
});
