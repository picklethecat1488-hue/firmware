import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import {
  buildTraceTracks,
  createTraceViewer,
  EXTERNAL_VIEWERS,
  formatDurationUs,
  resolveExternalViewerForMode,
  TRACE_MODES,
} from "../../js/daemon/trace_viewer.js";

function buildTraceViewerDom(ctx) {
  const doc = ctx.getDocument();

  const exportPathInput = createEl(ctx, "input", { id: "traceExportPath" });
  const exportCsvBtn = createEl(ctx, "button", { id: "btnExportTraceCsv", text: "💾 Export .CSV" });
  const exportPerfettoBtn = createEl(ctx, "button", {
    id: "btnExportTracePerfetto",
    text: "💾 Export Perfetto (.json)",
  });
  const openExternalBtn = createEl(ctx, "button", {
    id: "btnOpenExternalViewer",
    text: "🚀 Open in Perfetto",
  });

  const configureShortcutBtn = createEl(ctx, "button", {
    id: "btnTraceConfigureShortcut",
    text: "🎯 Configure Trace Mode",
  });
  const disabledBanner = createEl(ctx, "div", {
    id: "traceDisabledBanner",
    children: [configureShortcutBtn],
  });

  const canvas = ctx.createCanvas(960, 320);
  canvas.id = "traceTimelineCanvas";
  canvas.setBoundingClientRect({ left: 0, top: 0, width: 960, height: 320 });

  const canvasScroller = createEl(ctx, "div", {
    id: "traceCanvasScroller",
    children: [canvas],
  });
  canvasScroller.setBoundingClientRect({ left: 0, top: 0, width: 960, height: 320 });

  const recordingBadge = createEl(ctx, "span", { id: "traceRecordingBadge" });
  const pauseResumeBtn = createEl(ctx, "button", { id: "btnTracePauseResume" });
  const clearBtn = createEl(ctx, "button", { id: "btnTraceClear" });
  const zoomOutBtn = createEl(ctx, "button", { id: "btnTraceZoomOut" });
  const viewportBadge = createEl(ctx, "span", { id: "traceViewportBadge" });
  const zoomInBtn = createEl(ctx, "button", { id: "btnTraceZoomIn" });
  const fitBtn = createEl(ctx, "button", { id: "btnTraceFit" });
  const filterInput = createEl(ctx, "input", { id: "traceFilterInput" });

  const bottomTrackbar = createEl(ctx, "div", {
    id: "traceBottomTrackbar",
    children: [
      recordingBadge,
      pauseResumeBtn,
      clearBtn,
      zoomOutBtn,
      viewportBadge,
      zoomInBtn,
      fitBtn,
      filterInput,
    ],
  });

  const sliceBadge = createEl(ctx, "span", { id: "traceSliceBadge", text: "NONE" });
  const sliceDetails = createEl(ctx, "div", { id: "traceSliceDetails" });
  const eventsTableBody = createEl(ctx, "tbody", { id: "traceEventsTableBody" });

  const traceView = createEl(ctx, "div", {
    id: "traceView",
    children: [
      exportPathInput,
      exportCsvBtn,
      exportPerfettoBtn,
      openExternalBtn,
      disabledBanner,
      canvasScroller,
      bottomTrackbar,
      sliceBadge,
      sliceDetails,
      eventsTableBody,
    ],
  });

  doc.body.appendChild(traceView);
  return {
    traceView,
    exportPathInput,
    exportCsvBtn,
    exportPerfettoBtn,
    openExternalBtn,
    disabledBanner,
    configureShortcutBtn,
    canvas,
    canvasScroller,
    bottomTrackbar,
    recordingBadge,
    pauseResumeBtn,
    clearBtn,
    zoomOutBtn,
    viewportBadge,
    zoomInBtn,
    fitBtn,
    filterInput,
    sliceBadge,
    sliceDetails,
    eventsTableBody,
  };
}

describe("Host Daemon Trace & Telemetry Workstation (TraceViewerController)", () => {
  it("maps PERF to Perfetto and SENSOR/AUDIO to Rerun, and builds swimlane tracks", () => {
    assert.equal(resolveExternalViewerForMode(TRACE_MODES.PERF), EXTERNAL_VIEWERS.PERFETTO);
    assert.equal(resolveExternalViewerForMode(TRACE_MODES.SENSOR), EXTERNAL_VIEWERS.RERUN);
    assert.equal(resolveExternalViewerForMode(TRACE_MODES.AUDIO), EXTERNAL_VIEWERS.RERUN);
    assert.equal(formatDurationUs(420), "420µs");
    assert.equal(formatDurationUs(36_000), "36.00ms");
    assert.equal(formatDurationUs(2_500_000), "2.500s");

    const tracks = buildTraceTracks([
      {
        id: 1,
        ts_us: 1000,
        end_us: 1500,
        dur_us: 500,
        depth: 0,
        track: "CPU Core 0",
        ph: "B",
        category: "span",
        name: "battery_controller::update",
      },
      {
        id: 2,
        ts_us: 1100,
        end_us: 1300,
        dur_us: 200,
        depth: 1,
        track: "CPU Core 0",
        ph: "B",
        category: "span",
        name: "max17048::read_voltage_mv",
      },
      {
        id: 3,
        ts_us: 1200,
        track: "Core 0 CPU Usage",
        ph: "C",
        category: "cpu_usage",
        name: "Core 0 CPU Usage (%)",
        value: 42.5,
        unit: "%",
      },
    ]);

    assert.equal(tracks.length, 2);
    assert.equal(tracks[0].name, "Core 0 CPU Usage");
    assert.equal(tracks[0].kind, "scalar");
    assert.equal(tracks[1].name, "CPU Core 0");
    assert.equal(tracks[1].kind, "spans");
    assert.equal(tracks[1].maxDepth, 1);

    const filtered = buildTraceTracks(tracks[1].items, "max17048");
    assert.equal(filtered.length, 1);
    assert.equal(filtered[0].items.length, 1);
  });

  it("renders PERF spans, drives hovering bottom trackbar controls & shortcuts, exports CSV/Perfetto, and opens Perfetto UI", async () => {
    let recordingState = true;
    const exportRequests = [];
    const controlActions = [];

    const perfEvents = [
      {
        id: 1,
        seq: 2,
        ts_us: 1_000_000,
        end_ts_us: 1_036_000,
        dur_us: 36_000,
        depth: 0,
        track: "CPU Core 1",
        ph: "X",
        category: "span",
        name: "motor_controller::update",
        module: "controller::motor_controller",
        args: { core: 1, speed: 50 },
      },
      {
        id: 2,
        seq: 3,
        ts_us: 1_010_000,
        track: "Core 1 CPU Usage",
        ph: "C",
        category: "cpu_usage",
        name: "Core 1 CPU Usage (%)",
        value: 61.0,
        unit: "%",
      },
    ];

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/trace/events?after_seq=0&limit=4000": () => ({
          status: 200,
          body: {
            trace_mode: "PERF",
            recording: recordingState,
            events: perfEvents,
            cursor: 3,
            total_events: perfEvents.length,
            min_ts_us: 1_000_000,
            max_ts_us: 1_036_000,
          },
        }),
        "POST /api/trace/control": (_url, opts) => {
          const { action } = JSON.parse(opts.body);
          controlActions.push(action);
          if (action === "pause") recordingState = false;
          if (action === "resume") recordingState = true;
          return { status: 200, body: { success: true, recording: recordingState } };
        },
        "POST /api/trace/export": (_url, opts) => {
          const payload = JSON.parse(opts.body);
          exportRequests.push(payload);
          return {
            status: 200,
            body: {
              success: true,
              format: payload.format,
              path: payload.path || `/tmp/session.${payload.format}`,
            },
          };
        },
        "GET /api/trace/perfetto.json": {
          status: 200,
          body: { displayTimeUnit: "ms", traceEvents: [{ name: "motor_controller::update" }] },
        },
      },
    });

    const dom = buildTraceViewerDom(ctx);
    let openedConfigCount = 0;
    const viewer = createTraceViewer(ctx, {
      initialTraceMode: "NONE",
      initialBinName: "cat_detector_app",
      onOpenTargetConfig: () => {
        openedConfigCount += 1;
      },
    });
    viewer.init();

    // Initially NONE with 0 events -> banner visible, shortcut button opens Target Config
    assert.equal(dom.disabledBanner.style.display, "flex");
    dom.configureShortcutBtn.click();
    assert.equal(openedConfigCount, 1);

    // Poll PERF events -> switches to PERF mode ("🚀 Open in Perfetto") & hides NONE banner
    await viewer.pollEvents();
    assert.equal(viewer.traceMode, "PERF");
    assert.equal(dom.disabledBanner.style.display, "none");
    assert.equal(dom.openExternalBtn.textContent, "🚀 Open in Perfetto");
    assert.ok(dom.recordingBadge.textContent.includes("RECORDING LIVE"));
    assert.equal(dom.eventsTableBody.children.length, 2);

    // Select event via table row click -> populates bottom-left #traceSliceInspector
    dom.eventsTableBody.children[1].click();
    assert.equal(viewer.selectedEventId, 1);
    assert.ok(dom.sliceBadge.textContent.includes("SPAN #1"));
    assert.ok(dom.sliceDetails.textContent.includes("motor_controller::update"));
    assert.ok(dom.sliceDetails.textContent.includes("36000 µs"));

    // Keyboard shortcuts: W zooms in, S zooms out, D pans right, F fits, Space pauses
    const spanBeforeZoom = viewer.viewEndUs - viewer.viewStartUs;
    assert.equal(
      viewer.handleKeyDown({ key: "w", preventDefault() {}, target: dom.traceView }),
      true,
    );
    assert.ok(viewer.viewEndUs - viewer.viewStartUs < spanBeforeZoom);

    // Mouse-wheel zoom on canvas
    const spanAfterKeyZoom = viewer.viewEndUs - viewer.viewStartUs;
    dom.canvas.dispatchEvent({
      type: "wheel",
      deltaY: 120,
      clientX: 400,
      preventDefault() {},
    });
    assert.ok(viewer.viewEndUs - viewer.viewStartUs > spanAfterKeyZoom);

    // Filter input narrows visible table rows & tracks
    dom.filterInput.value = "motor_controller";
    dom.filterInput.dispatchEvent({ type: "input", bubbles: true });
    assert.equal(dom.eventsTableBody.children.length, 1);

    // Pause via Space key -> updates hovering bottom trackbar badge to PAUSED
    viewer.handleKeyDown({ key: " ", preventDefault() {}, target: dom.traceView });
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(viewer.recording, false);
    assert.ok(dom.recordingBadge.textContent.includes("PAUSED"));

    // Export .csv & Perfetto (.json), then click "🚀 Open in Perfetto"
    dom.exportPathInput.value = "recordings/traces/custom_perf";
    await viewer.exportTrace("csv");
    await viewer.exportTrace("perfetto");
    assert.equal(exportRequests.length, 2);
    assert.deepEqual(
      exportRequests.map((r) => r.format),
      ["csv", "perfetto"],
    );

    await viewer.openExternalViewer();
    assert.equal(ctx.openedPerfettoTraces.length, 1);
    assert.equal(ctx.openedPerfettoTraces[0].fileName, "cat_detector_app_perf.perfetto.json");

    // Clear trace via bottom trackbar button
    await viewer.clearTrace();
    assert.equal(viewer.orderedEvents.length, 0);
    assert.ok(controlActions.includes("clear"));

    viewer.destroy();
  });

  it("renders SENSOR reports, switches external viewer button to Rerun for SENSOR and AUDIO modes, and invokes /api/trace/open_rerun", async () => {
    const rerunCalls = [];
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/trace/events?after_seq=0&limit=4000": {
          status: 200,
          body: {
            trace_mode: "SENSOR",
            recording: true,
            events: [
              {
                id: 10,
                seq: 1,
                ts_us: 5_204_110,
                dur_us: 0,
                track: "Proximity (North)",
                ph: "C",
                category: "sensor",
                name: "Proximity (North)",
                value: 118,
                unit: "mm",
                args: { record_type: "Proximity", direction: "North", valid: true, value: 118 },
              },
              {
                id: 11,
                seq: 2,
                ts_us: 5_250_000,
                dur_us: 0,
                track: "MCU / Board Thermal",
                ph: "C",
                category: "sensor",
                name: "MCU Temperature (mC)",
                value: 25_062,
                unit: "mC",
                args: { record_type: "Thermal", overheating: false, value: 25_062 },
              },
            ],
            cursor: 2,
            total_events: 2,
            min_ts_us: 5_204_110,
            max_ts_us: 5_250_000,
          },
        },
        "POST /api/trace/open_rerun": (_url, opts) => {
          rerunCalls.push(JSON.parse(opts.body));
          return { status: 200, body: { success: true, logged_samples: 2 } };
        },
      },
    });

    const dom = buildTraceViewerDom(ctx);
    const viewer = createTraceViewer(ctx, {
      initialTraceMode: "SENSOR",
      initialBinName: "cat_detector_shell",
    });
    viewer.init();

    assert.equal(dom.openExternalBtn.textContent, "🚀 Open in Rerun");
    assert.ok(dom.openExternalBtn.className.includes("btn-rerun"));

    await viewer.pollEvents();
    assert.ok(dom.recordingBadge.textContent.includes("RECORDING SENSOR VALUES • 2 samples"));
    assert.ok(dom.eventsTableBody.textContent.includes("25062 mC"));
    assert.ok(dom.eventsTableBody.textContent.includes("118 mm"));
    assert.ok(!dom.eventsTableBody.textContent.includes("0 µs"));

    dom.openExternalBtn.click();
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(rerunCalls.length, 1);
    assert.equal(rerunCalls[0].spawn, true);
    assert.equal(ctx.openedPerfettoTraces.length, 0);

    // AUDIO mode also maps to Rerun Viewer
    viewer.setTraceMode("AUDIO");
    assert.equal(dom.openExternalBtn.textContent, "🚀 Open in Rerun");

    viewer.destroy();
  });
});
