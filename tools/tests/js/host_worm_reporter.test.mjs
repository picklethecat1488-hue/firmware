import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import {
  buildReproSteps,
  bytesToBase64,
  createWormReporter,
  utf8ToBytes,
} from "../../js/daemon/worm_reporter.js";

function buildWormDom(ctx) {
  const doc = ctx.getDocument();
  const openBtn = createEl(ctx, "button", { id: "btnTargetBug", text: "🐞 Target Bug" });
  const closeBtn = createEl(ctx, "button", { id: "btnCloseTargetBugModal", text: "✕" });
  const cancelBtn = createEl(ctx, "button", { id: "btnCancelTargetBug", text: "Cancel" });
  const submitBtn = createEl(ctx, "button", {
    id: "btnSubmitTargetBug",
    text: "⚡ File Target Bug",
  });

  const titleInput = createEl(ctx, "input", { id: "targetBugTitle" });
  const severitySelect = createEl(ctx, "select", { id: "targetBugSeverity" });
  severitySelect.value = "HIGH";
  const categorySelect = createEl(ctx, "select", { id: "targetBugCategory" });
  categorySelect.value = "DRIVER";
  const componentInput = createEl(ctx, "input", { id: "targetBugComponent" });
  componentInput.value = "tmc6300";
  const descInput = createEl(ctx, "textarea", { id: "targetBugDescription" });
  const stepsInput = createEl(ctx, "textarea", { id: "targetBugSteps" });
  const expectedInput = createEl(ctx, "textarea", { id: "targetBugExpected" });
  const actualInput = createEl(ctx, "textarea", { id: "targetBugActual" });

  const attachSystemLogs = createEl(ctx, "input", { id: "attachSystemLogs" });
  attachSystemLogs.checked = true;
  const attachCrashLog = createEl(ctx, "input", { id: "attachCrashLog" });
  attachCrashLog.checked = true;
  const attachTelemetry = createEl(ctx, "input", { id: "attachTelemetry" });
  attachTelemetry.checked = false;
  const attachMetadata = createEl(ctx, "input", { id: "attachMetadata" });
  attachMetadata.checked = true;

  const pasteBtn = createEl(ctx, "button", { id: "btnPasteTargetBugClipboard" });
  const dropzone = createEl(ctx, "div", { id: "targetBugDropzone", className: "dropzone" });
  const fileInput = createEl(ctx, "input", { id: "targetBugFileInput" });
  const attachmentsGrid = createEl(ctx, "div", { id: "targetBugAttachmentsGrid" });

  const progressBar = createEl(ctx, "div", { id: "targetBugProgressBar" });
  const progressMsg = createEl(ctx, "span", { id: "targetBugProgressMsg" });
  const progressPct = createEl(ctx, "span", { id: "targetBugProgressPct" });
  const progressContainer = createEl(ctx, "div", {
    id: "targetBugProgressContainer",
    style: { display: "none" },
    children: [progressMsg, progressPct, progressBar],
  });
  const statusMsg = createEl(ctx, "div", { id: "targetBugStatusMsg", style: { display: "none" } });

  const modal = createEl(ctx, "div", {
    id: "targetBugModal",
    style: { display: "none" },
    children: [
      closeBtn,
      cancelBtn,
      submitBtn,
      titleInput,
      severitySelect,
      categorySelect,
      componentInput,
      descInput,
      stepsInput,
      expectedInput,
      actualInput,
      attachSystemLogs,
      attachCrashLog,
      attachTelemetry,
      attachMetadata,
      pasteBtn,
      dropzone,
      fileInput,
      attachmentsGrid,
      progressContainer,
      statusMsg,
    ],
  });

  doc.body.appendChild(openBtn);
  doc.body.appendChild(modal);

  return {
    openBtn,
    modal,
    submitBtn,
    titleInput,
    descInput,
    stepsInput,
    expectedInput,
    actualInput,
    dropzone,
    attachmentsGrid,
    statusMsg,
  };
}

describe("Host Daemon WORM-* Defect Reporter & Attachment Bundler", () => {
  it("encodes UTF-8/Base64 in pure ES2024 and synthesizes python tools/host.py reproduction steps", () => {
    assert.equal(bytesToBase64(utf8ToBytes("Man")), "TWFu");
    assert.equal(bytesToBase64(utf8ToBytes("Ma")), "TWE=");
    assert.equal(bytesToBase64(utf8ToBytes("M")), "TQ==");
    assert.equal(bytesToBase64(new Uint8Array(0)), "");

    const steps = buildReproSteps(["motor speed 80", "motor status"], {
      package: "cat_detector",
      binName: "cat_detector_shell",
      release: true,
      mode: "CLI",
    });
    assert.ok(
      steps.includes(
        "1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell --release",
      ),
    );
    assert.ok(steps.includes("2. Execute command: python tools/host.py cli motor speed 80"));
    assert.ok(steps.includes("3. Execute command: python tools/host.py cli motor status"));
    assert.ok(steps.includes("4. Observe defect/failure on target"));
  });

  it("clears stale report fields on re-open, bundles binary/screenshot/clipboard attachments, and submits /api/target_bug", async () => {
    const submittedPayloads = [];
    let backendHistory = ["gpio read 14", "motor speed 100"];

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/cli/history": () => ({ status: 200, body: { history: backendHistory } }),
        "POST /api/target_bug": (_url, opts) => {
          const parsed = JSON.parse(opts.body);
          submittedPayloads.push(parsed);
          return {
            status: 200,
            body: {
              success: true,
              worm_id: "WORM-BRISTLY-GLOWWORM-390",
              attachments_count: 1 + (parsed.manual_attachments || []).length,
            },
          };
        },
      },
    });

    const dom = buildWormDom(ctx);
    const createdWorms = [];
    const reporter = createWormReporter(ctx, {
      getTargetConfig: () => ({
        package: "cat_detector",
        binName: "cat_detector_shell",
        release: false,
        mode: "CLI",
      }),
      getCommandHistory: () => ["local_cmd"],
      onWormCreated: (res) => createdWorms.push(res.worm_id),
    });
    reporter.init();

    // Seed stale values before opening modal -> openModal() must scrub them
    dom.descInput.value = "Old stale description";
    dom.expectedInput.value = "Old expected";
    dom.actualInput.value = "Old actual";

    await reporter.openModal();
    assert.equal(reporter.isModalOpen(), true);
    assert.equal(dom.descInput.value, "");
    assert.equal(dom.expectedInput.value, "");
    assert.equal(dom.actualInput.value, "");
    assert.ok(dom.stepsInput.value.includes("python tools/host.py cli motor speed 100"));

    // Reject empty title with validation banner
    const missingTitleResult = await reporter.submit();
    assert.equal(missingTitleResult, null);
    assert.ok(dom.statusMsg.textContent.includes("Bug title is required"));

    // Attach a PNG screenshot via binary arrayBuffer + a pasted text snippet + drag-dropped log
    const pngHeader = new Uint8Array([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
    await reporter.addFiles([
      {
        name: "scope_capture.png",
        type: "image/png",
        size: pngHeader.byteLength,
        arrayBuffer: async () => pngHeader.buffer,
      },
      {
        name: "fault_trace.log",
        type: "text/plain",
        text: async () => "FAULT: DIAG asserted on phase V",
      },
    ]);
    reporter.addTextSnippet("RTT clipboard trace line");

    assert.equal(reporter.getAttachments().length, 3);
    assert.equal(dom.attachmentsGrid.children.length, 3);
    assert.equal(reporter.getAttachments()[0].file_type, "screenshot");
    assert.ok(reporter.getAttachments()[0].content_base64.startsWith("data:image/png;base64,"));
    assert.equal(reporter.getAttachments()[1].file_type, "log");

    // Remove the middle attachment via delegated remove button click
    const removeButtons = dom.attachmentsGrid.querySelectorAll("[data-remove-attachment-idx]");
    assert.equal(removeButtons.length, 3);
    removeButtons[1].click();
    assert.equal(reporter.getAttachments().length, 2);

    // Fill report fields and submit
    dom.titleInput.value = "Phase V overcurrent trip at 100% duty";
    dom.descInput.value = "Observed DIAG pin low within 2ms of commutation.";
    dom.expectedInput.value = "Continuous rotation";
    dom.actualInput.value = "Immediate bridge shutdown";

    const res = await reporter.submit();
    assert.equal(res.worm_id, "WORM-BRISTLY-GLOWWORM-390");
    assert.deepEqual(createdWorms, ["WORM-BRISTLY-GLOWWORM-390"]);

    assert.equal(submittedPayloads.length, 1);
    const sent = submittedPayloads[0];
    assert.equal(sent.title, "Phase V overcurrent trip at 100% duty");
    assert.equal(sent.severity, "HIGH");
    assert.equal(sent.category, "DRIVER");
    assert.equal(sent.component, "tmc6300");
    assert.equal(sent.attach_system_logs, true);
    assert.equal(sent.attach_crash_log, true);
    assert.equal(sent.attach_telemetry, false);
    assert.equal(sent.attach_metadata, true);
    assert.equal(sent.manual_attachments.length, 2);

    // Modal auto-closes and clears attachments after 1500ms
    ctx.advanceTime(1600);
    assert.equal(reporter.isModalOpen(), false);
    assert.equal(reporter.getAttachments().length, 0);

    // Re-opening pulls updated backend CLI history unconditionally
    backendHistory = ["gpio read 14", "motor speed 100", "motor brake"];
    await reporter.openModal();
    assert.ok(dom.stepsInput.value.includes("python tools/host.py cli motor brake"));

    reporter.destroy();
  });
});
