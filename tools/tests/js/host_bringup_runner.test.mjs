import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import { BRINGUP_STEP_STATES, createBringupRunner } from "../../js/daemon/bringup_runner.js";

function buildBringupDom(ctx) {
  const doc = ctx.getDocument();
  const scriptSelect = createEl(ctx, "select", { id: "bringupScriptSelect" });
  const stepBtn = createEl(ctx, "button", { id: "btnBringupStep", text: "▶ Step" });
  const runBtn = createEl(ctx, "button", { id: "btnBringupRun", text: "⚡ Run All" });
  const resetBtn = createEl(ctx, "button", { id: "btnBringupReset", text: "↺ Reset" });
  const reportBtn = createEl(ctx, "button", { id: "btnBringupReport", text: "📄 Report" });
  const summaryBadge = createEl(ctx, "span", { id: "bringupSummaryBadge", text: "CHECKLIST: IDLE" });
  const stepList = createEl(ctx, "ul", { id: "bringupStepList" });

  const cardTitle = createEl(ctx, "span", { id: "bringupCardTitle" });
  const cardType = createEl(ctx, "span", { id: "bringupCardType" });
  const cardDesc = createEl(ctx, "div", { id: "bringupCardDesc" });
  const cardDetails = createEl(ctx, "div", { id: "bringupCardDetails" });

  const promptText = createEl(ctx, "div", { id: "bringupPromptText" });
  const promptNotes = createEl(ctx, "input", { id: "bringupPromptNotes" });
  const passBtn = createEl(ctx, "button", {
    id: "btnBringupPass",
    dataset: { bringupAction: "pass" },
    text: "✔ Pass",
  });
  const failBtn = createEl(ctx, "button", {
    id: "btnBringupFail",
    dataset: { bringupAction: "fail" },
    text: "✘ Fail",
  });
  const skipBtn = createEl(ctx, "button", {
    id: "btnBringupSkip",
    dataset: { bringupAction: "skip" },
    text: "⏭ Skip",
  });
  const promptActionArea = createEl(ctx, "div", {
    id: "bringupPromptActionArea",
    style: { display: "none" },
    children: [promptText, promptNotes, passBtn, failBtn, skipBtn],
  });

  const consoleOutput = createEl(ctx, "div", { id: "bringupConsoleOutput" });
  const copyLogBtn = createEl(ctx, "button", { id: "btnCopyBringupLog", text: "📋 Copy Log" });

  doc.body.appendChild(scriptSelect);
  doc.body.appendChild(stepBtn);
  doc.body.appendChild(runBtn);
  doc.body.appendChild(resetBtn);
  doc.body.appendChild(reportBtn);
  doc.body.appendChild(summaryBadge);
  doc.body.appendChild(stepList);
  doc.body.appendChild(cardTitle);
  doc.body.appendChild(cardType);
  doc.body.appendChild(cardDesc);
  doc.body.appendChild(cardDetails);
  doc.body.appendChild(promptActionArea);
  doc.body.appendChild(consoleOutput);
  doc.body.appendChild(copyLogBtn);

  return {
    scriptSelect,
    stepBtn,
    runBtn,
    resetBtn,
    reportBtn,
    summaryBadge,
    stepList,
    cardTitle,
    cardType,
    cardDesc,
    cardDetails,
    promptActionArea,
    promptText,
    promptNotes,
    passBtn,
    failBtn,
    skipBtn,
    consoleOutput,
    copyLogBtn,
  };
}

async function flushMicrotasks(ticks = 15) {
  for (let i = 0; i < ticks; i += 1) {
    await Promise.resolve();
  }
}

describe("Host Daemon Hardware Bringup Checklist Runner", () => {
  it("discovers YAML scripts, renders test_points/control_pins tables, records pass/fail/skip responses, and exports Markdown reports", async () => {
    const scripts = [
      {
        name: "Carrier Board 2.0",
        chip: "MCXN947",
        path: "app/carrier_board_bringup.yaml",
      },
      {
        name: "Cat Detector",
        chip: "RP2040",
        path: "app/cat_detector_bringup.yaml",
      },
    ];

    const recordedResponses = [];
    let statusState = {
      status: "idle",
      current_step_idx: 0,
      total_steps: 2,
      summary: { passed: 0, failed: 0, skipped: 0 },
      steps: [
        { id: "1", name: "Power Rails", type: "hardware_check", status: BRINGUP_STEP_STATES.PENDING },
        { id: "2", name: "Battery ADC", type: "shell_command", status: BRINGUP_STEP_STATES.PENDING },
      ],
      current_step: {
        id: "1",
        name: "Power Rails",
        type: "hardware_check",
        description: "Verify 3.3V and 5V rails at test points.",
        procedure: "Probe TP1 and TP2 relative to GND.",
        expected: "3.30V +/- 2%",
        test_points: { TP1: "3.3V", TP2: "5.0V" },
        control_pins: { EN_3V3: "PIO0_12" },
      },
      active_prompt: null,
      logs: ["[BRINGUP] Loaded Carrier Board 2.0"],
    };

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/bringup/scripts": { status: 200, body: scripts },
        "POST /api/bringup/load": { status: 200, body: { success: true, status: statusState } },
        "GET /api/bringup/status": () => ({ status: 200, body: statusState }),
        "POST /api/bringup/step": () => {
          statusState = {
            ...statusState,
            status: "waiting_prompt",
            steps: [
              { id: "1", name: "Power Rails", type: "hardware_check", status: BRINGUP_STEP_STATES.RUNNING },
              { id: "2", name: "Battery ADC", type: "shell_command", status: BRINGUP_STEP_STATES.PENDING },
            ],
            active_prompt: {
              default_status: "pass",
              auto_pass: null,
            },
            logs: [...statusState.logs, "[STEP 1] Waiting for operator verification"],
          };
          return { status: 200, body: { status: "waiting_prompt" } };
        },
        "POST /api/bringup/respond": (_url, opts) => {
          const body = JSON.parse(opts.body);
          recordedResponses.push(body);
          statusState = {
            ...statusState,
            status: "idle",
            current_step_idx: 1,
            summary: {
              passed: body.action === "pass" ? 1 : 0,
              failed: body.action === "fail" ? 1 : 0,
              skipped: body.action === "skip" ? 1 : 0,
            },
            steps: [
              {
                id: "1",
                name: "Power Rails",
                type: "hardware_check",
                status: body.action === "pass" ? BRINGUP_STEP_STATES.PASSED : BRINGUP_STEP_STATES.FAILED,
              },
              { id: "2", name: "Battery ADC", type: "shell_command", status: BRINGUP_STEP_STATES.PENDING },
            ],
            current_step: {
              id: "2",
              name: "Battery ADC",
              type: "shell_command",
              command: "battery status",
              expected_regex: "\\d+\\s*mV",
              matched_line: "3980 mV (88%)",
            },
            active_prompt: null,
            logs: [...statusState.logs, `[STEP 1] Recorded ${body.action}: ${body.notes}`],
          };
          return { status: 200, body: { success: true } };
        },
        "POST /api/bringup/reset": () => {
          statusState = {
            ...statusState,
            current_step_idx: 0,
            summary: { passed: 0, failed: 0, skipped: 0 },
            active_prompt: null,
          };
          return { status: 200, body: { success: true } };
        },
        "POST /api/bringup/report": () => ({
          status: 200,
          body: {
            success: true,
            output_path: "build/bringup_report.md",
            report: "# Bringup Verification Report: Carrier Board 2.0\n- Step 1: PASSED\n",
          },
        }),
      },
    });

    const dom = buildBringupDom(ctx);
    const runner = createBringupRunner(ctx);
    runner.init();

    const loaded = await runner.loadScripts();
    assert.equal(loaded.length, 2);
    assert.equal(dom.scriptSelect.children.length, 2);
    assert.equal(runner.activeScriptPath, "app/carrier_board_bringup.yaml");
    assert.equal(dom.cardTitle.textContent, "STEP 1: POWER RAILS");

    // Verify hardware test_points and control_pins signal tables were rendered
    const tables = dom.cardDetails.querySelectorAll(".bringup-table");
    assert.equal(tables.length, 2);
    assert.ok(tables[0].textContent.includes("TP1"));
    assert.ok(tables[0].textContent.includes("3.3V"));
    assert.ok(tables[1].textContent.includes("EN_3V3"));

    // Step 1 -> opens prompt card
    dom.stepBtn.click();
    await Promise.resolve();
    await Promise.resolve();
    await runner.pollStatus();
    assert.equal(dom.promptActionArea.style.display, "block");
    assert.ok(dom.promptText.textContent.includes("ACTION REQUIRED"));

    // Submit Pass verdict with operator notes via delegated button click
    dom.promptNotes.value = "Measured 3.304V on TP1";
    dom.passBtn.click();
    await Promise.resolve();
    await Promise.resolve();
    await runner.pollStatus();

    assert.deepEqual(recordedResponses, [
      { action: "pass", notes: "Measured 3.304V on TP1" },
    ]);
    assert.equal(dom.promptNotes.value, "");
    assert.equal(dom.promptActionArea.style.display, "none");
    assert.ok(dom.summaryBadge.textContent.includes("1 PASSED"));
    assert.equal(dom.cardTitle.textContent, "STEP 2: BATTERY ADC");

    // Export Markdown report and verify client download trigger
    await runner.exportReport("build/bringup_report.md");
    assert.equal(ctx.downloads.length, 1);
    assert.equal(ctx.downloads[0].filename, "bringup_report.md");
    assert.ok(ctx.downloads[0].content.includes("Carrier Board 2.0"));

    // Copy bringup console output
    await runner.copyConsole(dom.copyLogBtn);
    assert.ok(ctx.getCopiedText().includes("Measured 3.304V on TP1"));

    await runner.reset();
    assert.equal(runner.lastStatus.current_step_idx, 0);

    runner.destroy();
  });

  it("auto-advances auto_pass regex steps in runAll() and pauses on manual operator prompts", async () => {
    let phase = 0;
    const actions = [];

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/bringup/status": () => {
          if (phase === 0) {
            return {
              status: 200,
              body: {
                status: "idle",
                current_step_idx: 0,
                total_steps: 2,
                summary: { passed: 0, failed: 0, skipped: 0 },
                steps: [],
                current_step: { id: "1", name: "Auto Step", type: "shell_command" },
                active_prompt: null,
                logs: [],
              },
            };
          }
          if (phase === 1) {
            return {
              status: 200,
              body: {
                status: "waiting_prompt",
                current_step_idx: 0,
                total_steps: 2,
                summary: { passed: 0, failed: 0, skipped: 0 },
                steps: [],
                current_step: { id: "1", name: "Auto Step", type: "shell_command" },
                active_prompt: { auto_pass: true, matched_line: "OK 3990 mV" },
                logs: [],
              },
            };
          }
          return {
            status: 200,
            body: {
              status: "waiting_prompt",
              current_step_idx: 1,
              total_steps: 2,
              summary: { passed: 1, failed: 0, skipped: 0 },
              steps: [],
              current_step: { id: "2", name: "Manual LED Check", type: "hardware_check" },
              active_prompt: { auto_pass: null, default_status: "pass" },
              logs: [],
            },
          };
        },
        "POST /api/bringup/step": () => {
          actions.push("step");
          phase = 1;
          return { status: 200, body: { status: "waiting_prompt" } };
        },
        "POST /api/bringup/respond": (_url, opts) => {
          const { action } = JSON.parse(opts.body);
          actions.push(`respond:${action}`);
          phase = 2;
          return { status: 200, body: { success: true } };
        },
      },
    });

    buildBringupDom(ctx);
    const runner = createBringupRunner(ctx);
    runner.init();

    await runner.runAll();
    // Advance timer for auto-pass check and flush chained async fetch promises
    ctx.advanceTime(300);
    await flushMicrotasks();
    ctx.advanceTime(300);
    await flushMicrotasks();

    assert.deepEqual(actions, ["step", "respond:pass"]);
    assert.equal(runner.isRunningAll, false);

    runner.destroy();
  });
});
