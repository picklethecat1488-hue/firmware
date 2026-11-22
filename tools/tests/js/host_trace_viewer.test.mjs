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
    assert.equal(tracks[0].minValue, 0);
    assert.equal(tracks[0].maxValue, 100);
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

  it("animates swimlanes smoothly at 30fps+ via requestAnimationFrame and 120ms fast polling while active", async () => {
    let pollCount = 0;
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/trace/events?after_seq=0&limit=4000": () => {
          pollCount += 1;
          return {
            status: 200,
            body: {
              trace_mode: "PERF",
              recording: true,
              events: [
                {
                  id: 1,
                  seq: 1,
                  ts_us: 2_000_000,
                  track: "Core 0 CPU Usage (%)",
                  ph: "C",
                  category: "cpu_usage",
                  name: "Core 0 CPU Usage (%)",
                  value: 18.5,
                  unit: "%",
                },
                {
                  id: 2,
                  seq: 2,
                  ts_us: 2_050_000,
                  track: "Core 1 CPU Usage (%)",
                  ph: "C",
                  category: "cpu_usage",
                  name: "Core 1 CPU Usage (%)",
                  value: 44.2,
                  unit: "%",
                },
              ],
              cursor: 2,
              total_events: 2,
              min_ts_us: 2_000_000,
              max_ts_us: 2_050_000,
            },
          };
        },
        "GET /api/trace/events?after_seq=2&limit=4000": () => {
          pollCount += 1;
          return {
            status: 200,
            body: {
              trace_mode: "PERF",
              recording: true,
              events: [],
              cursor: 2,
              total_events: 2,
              min_ts_us: 2_000_000,
              max_ts_us: 2_050_000,
            },
          };
        },
        "POST /api/trace/control": { status: 200, body: { success: true, recording: false } },
      },
    });

    const dom = buildTraceViewerDom(ctx);
    const viewer = createTraceViewer(ctx, {
      initialTraceMode: "PERF",
      initialBinName: "cat_detector_shell",
    });
    viewer.init();
    viewer.setActiveView(true);

    // Initial poll anchors live playhead at 2,050,000 µs and schedules rAF loop
    await viewer.pollEvents();
    assert.equal(pollCount, 1);
    assert.equal(ctx.rafQueue.size, 1);
    const initialPlayhead = viewer.getLivePlayheadUs();
    const initialViewEnd = viewer.viewEndUs;
    const initialBadgeText = dom.viewportBadge.textContent;
    assert.equal(initialPlayhead, 2_050_000);

    // Advance 6 animation frames (~200ms @ 30fps) -> playhead, viewport & badge glide smoothly forward without waiting 1s!
    for (let i = 0; i < 6; i += 1) {
      ctx.flushAnimationFrames(33);
    }
    const advancedPlayhead = viewer.getLivePlayheadUs();
    assert.ok(advancedPlayhead > initialPlayhead + 150_000);
    assert.ok(viewer.viewEndUs > initialViewEnd);
    assert.notEqual(dom.viewportBadge.textContent, initialBadgeText);
    assert.equal(ctx.rafQueue.size, 1);

    // Fast 120ms poll timer triggers incremental fetch automatically
    ctx.advanceTime(130);
    await Promise.resolve();
    assert.ok(pollCount >= 2);

    // Pausing recording stops rAF loop and fast polling immediately
    await viewer.toggleRecording();
    assert.equal(ctx.rafQueue.size, 0);

    viewer.destroy();
  });

  it("purges stale SENSOR tracks and resets cursor when switching to PERF mode (WORM-RADIANT-VINEGARWORM-918)", async () => {
    let phase = "SENSOR";
    const ctx = createMockRuntimeContext({
      fetchHandler: async (url) => {
        const body =
          phase === "SENSOR"
            ? {
                trace_mode: "SENSOR",
                session_epoch: 1,
                recording: true,
                cursor: 12,
                total_events: 1,
                min_ts_us: 57_109_376,
                max_ts_us: 57_109_376,
                events: url.includes("after_seq=0")
                  ? [
                      {
                        id: 5,
                        seq: 12,
                        ts_us: 57_109_376,
                        track: "Proximity (West)",
                        ph: "C",
                        category: "sensor",
                        name: "Proximity (West)",
                        value: 79,
                        unit: "mm",
                      },
                    ]
                  : [],
              }
            : {
                trace_mode: "PERF",
                session_epoch: 2,
                recording: true,
                cursor: 2,
                total_events: 2,
                min_ts_us: 27_632,
                max_ts_us: 40_887,
                events: [
                  {
                    id: 1,
                    seq: 1,
                    ts_us: 27_632,
                    end_ts_us: 30_750,
                    dur_us: 3_118,
                    depth: 0,
                    track: "CPU Core 0",
                    ph: "X",
                    category: "span",
                    name: "bootstrap_task",
                  },
                  {
                    id: 2,
                    seq: 2,
                    ts_us: 40_887,
                    track: "Core 0 CPU Usage (%)",
                    ph: "C",
                    category: "cpu_usage",
                    name: "Core 0 CPU Usage (%)",
                    value: 22.4,
                    unit: "%",
                  },
                ],
              };
        return {
          ok: true,
          status: 200,
          async text() {
            return JSON.stringify(body);
          },
          async json() {
            return body;
          },
        };
      },
    });

    const dom = buildTraceViewerDom(ctx);
    const viewer = createTraceViewer(ctx, {
      initialTraceMode: "SENSOR",
      initialBinName: "cat_detector_app",
    });
    viewer.init();

    await viewer.pollEvents();
    assert.ok(dom.eventsTableBody.textContent.includes("Proximity (West)"));
    assert.equal(viewer.afterSeq, 12);

    // Switch daemon from SENSOR -> PERF (with non-zero total_events right after sync!)
    phase = "PERF";
    await viewer.pollEvents();

    assert.equal(viewer.traceMode, "PERF");
    assert.ok(!dom.eventsTableBody.textContent.includes("Proximity (West)"));
    assert.ok(dom.eventsTableBody.textContent.includes("bootstrap_task"));
    assert.equal(viewer.eventsById.size, 2);
    assert.equal(viewer.maxTsUs, 40_887);

    viewer.destroy();
  });

  it("terminates 0-us closed ph='X' configure_alerts slices instead of stretching across Core 0 idle gaps (WORM-DIRT-PUPA-301)", async () => {
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/trace/events?after_seq=0&limit=4000": {
          status: 200,
          body: {
            trace_mode: "PERF",
            session_epoch: 1,
            recording: true,
            cursor: 4,
            total_events: 4,
            min_ts_us: 33_050,
            max_ts_us: 2_000_000,
            events: [
              {
                id: 1,
                seq: 1,
                ts_us: 33_050,
                end_ts_us: 33_978,
                dur_us: 928,
                depth: 0,
                track: "CPU Core 0",
                ph: "X",
                category: "span",
                name: "BATTERY_CHANNEL",
              },
              {
                id: 2,
                seq: 2,
                ts_us: 33_211,
                end_ts_us: 33_978,
                dur_us: 767,
                depth: 1,
                track: "CPU Core 0",
                ph: "X",
                category: "span",
                name: "configure_alerts",
              },
              {
                id: 3,
                seq: 3,
                ts_us: 42_003,
                end_ts_us: 42_003,
                dur_us: 0,
                depth: 1,
                track: "CPU Core 0",
                ph: "X",
                category: "span",
                name: "configure_alerts",
                args: { core: 0, resumed: true },
              },
              {
                id: 4,
                seq: 4,
                ts_us: 2_000_000,
                track: "Core 0 CPU Usage (%)",
                ph: "C",
                category: "cpu_usage",
                name: "Core 0 CPU Usage (%)",
                value: 1.2,
                unit: "%",
              },
            ],
          },
        },
      },
    });

    const dom = buildTraceViewerDom(ctx);
    const viewer = createTraceViewer(ctx, {
      initialTraceMode: "PERF",
      initialBinName: "cat_detector_app",
    });
    viewer.init();
    await viewer.pollEvents();
    dom.fitBtn.click();

    const zeroDurBox = viewer._hitBoxes.find((b) => b.eventId === 3);
    assert.ok(zeroDurBox, "expected configure_alerts slice hitbox to exist at 42,003 us");
    assert.ok(
      zeroDurBox.w <= 6,
      `expected terminated 0-us configure_alerts slice width <= 6px, got ${zeroDurBox.w}px stretching across Core 0 idle window`,
    );
    assert.equal(viewer.hitTestCanvas(550, zeroDurBox.y + 4), null);
    assert.ok(!dom.canvas.id.includes("unused"));

    viewer.destroy();
  });

  it("opens ui.perfetto.dev via POST /api/trace/open_perfetto in pywebview or popup-blocked browsers and opens popup synchronously before fetch in browser tabs (WORM-SEGMENTED-COCOON-284)", async () => {
    // 1. Desktop pywebview mode (`python tools/host.py` default): delegates directly to backend system browser launcher
    const backendOpenCalls = [];
    const pywebviewCtx = createMockRuntimeContext({
      pywebview: { api: {} },
      fetchRoutes: {
        "POST /api/trace/open_perfetto": (_url, opts) => {
          backendOpenCalls.push(JSON.parse(opts.body || "{}"));
          return {
            status: 200,
            body: {
              success: true,
              url: "http://127.0.0.1:8899/perfetto?ts=1700000000000",
              event_count: 5,
            },
          };
        },
      },
    });
    buildTraceViewerDom(pywebviewCtx);
    const pywebviewViewer = createTraceViewer(pywebviewCtx, {
      initialTraceMode: "PERF",
      initialBinName: "cat_detector_app",
    });
    pywebviewViewer.init();

    const pywebviewOpened = await pywebviewViewer.openExternalViewer();
    assert.equal(pywebviewOpened, true);
    assert.equal(backendOpenCalls.length, 1);
    assert.equal(pywebviewCtx.openedPerfettoTraces.length, 0);
    pywebviewViewer.destroy();

    // 2. Standard browser tab: calls openPerfettoTrace synchronously BEFORE fetching /api/trace/perfetto.json
    const callSequence = [];
    const browserCtx = createMockRuntimeContext({
      onOpenPerfettoTraceStart: () => {
        callSequence.push("open_popup_sync");
      },
      fetchHandler: async (url) => {
        callSequence.push(`fetch:${url}`);
        return {
          ok: true,
          status: 200,
          async text() {
            return '{"traceEvents":[{"name":"main"}]}';
          },
          async json() {
            return { traceEvents: [{ name: "main" }] };
          },
        };
      },
    });
    buildTraceViewerDom(browserCtx);
    const browserViewer = createTraceViewer(browserCtx, {
      initialTraceMode: "PERF",
      initialBinName: "cat_detector_app",
    });
    browserViewer.init();

    const browserOpened = await browserViewer.openExternalViewer();
    assert.equal(browserOpened, true);
    assert.deepEqual(callSequence, ["open_popup_sync", "fetch:/api/trace/perfetto.json"]);
    assert.equal(browserCtx.openedPerfettoTraces.length, 1);
    assert.ok(browserCtx.openedPerfettoTraces[0].byteLength > 0);
    browserViewer.destroy();

    // 3. Popup-blocked browser (`perfettoOpenResult: false`): falls back to POST /api/trace/open_perfetto
    const fallbackCalls = [];
    const blockedCtx = createMockRuntimeContext({
      perfettoOpenResult: false,
      fetchRoutes: {
        "GET /api/trace/perfetto.json": {
          status: 200,
          body: { traceEvents: [] },
        },
        "POST /api/trace/open_perfetto": () => {
          fallbackCalls.push(true);
          return {
            status: 200,
            body: {
              success: true,
              url: "http://127.0.0.1:8899/perfetto?ts=1700000000000",
            },
          };
        },
      },
    });
    buildTraceViewerDom(blockedCtx);
    const blockedViewer = createTraceViewer(blockedCtx, {
      initialTraceMode: "PERF",
      initialBinName: "cat_detector_app",
    });
    blockedViewer.init();

    const fallbackOpened = await blockedViewer.openExternalViewer();
    assert.equal(fallbackOpened, true);
    assert.equal(fallbackCalls.length, 1);
    blockedViewer.destroy();
  });

  it("streams same-origin /api/trace/perfetto.json into embedded ui.perfetto.dev iframe without mixed content (WORM-SEGMENTED-COCOON-284)", async () => {
    const { createPerfettoBridgeApp } = await import("../../js/daemon/perfetto_bridge.js");

    const ctx = createMockRuntimeContext({
      bootConfig: {
        title: "cat_detector_app (PERF)",
        fileName: "cat_detector_app_perf.perfetto.json",
        traceUrl: "/api/trace/perfetto.json?ts=12345",
        eventCount: 2,
      },
      fetchRoutes: {
        "GET /api/trace/perfetto.json?ts=12345": {
          status: 200,
          body: { traceEvents: [{ name: "span_a" }, { name: "span_b" }] },
        },
      },
    });
    const doc = ctx.getDocument();
    const banner = createEl(ctx, "div", { id: "perfettoBridgeBanner" });
    const statusEl = createEl(ctx, "span", { id: "perfettoBridgeStatus", text: "Connecting..." });
    const openTabBtn = createEl(ctx, "button", { id: "perfettoBridgeOpenTabBtn", text: "Open Tab" });
    const dismissBtn = createEl(ctx, "button", { id: "perfettoBridgeDismissBtn", text: "×" });
    const iframeEl = createEl(ctx, "iframe", {
      id: "perfettoEmbedFrame",
      attrs: { src: "https://ui.perfetto.dev" },
    });
    banner.appendChild(statusEl);
    banner.appendChild(openTabBtn);
    banner.appendChild(dismissBtn);
    doc.body.appendChild(banner);
    doc.body.appendChild(iframeEl);

    const bridgeApp = createPerfettoBridgeApp(ctx);
    const loaded = await bridgeApp.start();
    assert.equal(loaded, true);
    assert.equal(ctx.streamedPerfettoFrames.length, 1);
    assert.equal(ctx.streamedPerfettoFrames[0].title, "cat_detector_app (PERF)");
    assert.equal(ctx.streamedPerfettoFrames[0].fileName, "cat_detector_app_perf.perfetto.json");
    assert.ok(statusEl.textContent.includes("Loaded"));

    // Clicking "Open Tab" pushes the cached same-origin ArrayBuffer via postMessage without re-fetching mixed HTTP URLs.
    openTabBtn.click();
    await Promise.resolve();
    assert.equal(ctx.openedPerfettoTraces.length, 1);
    assert.equal(ctx.openedPerfettoTraces[0].fileName, "cat_detector_app_perf.perfetto.json");

    ctx.advanceTime(4000);
    assert.equal(banner.style.display, "none");
  });
});
