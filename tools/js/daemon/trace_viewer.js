/**
 * Live Firmware Trace & Sensor Telemetry Workstation Controller (`TRACE` Mode).
 *
 * Renders simplified multi-track span timelines (`PERF` mode) and real-time scalar
 * sensor curves (`SENSOR` / `AUDIO` modes) on a Retina-scaled HTML5 `<canvas>`,
 * drives the hovering bottom playback/zoom/filter trackbar (`#traceBottomTrackbar`),
 * exports `.csv` and `.perfetto.json` files to disk, and launches the mode-specific
 * external viewer (`ui.perfetto.dev` for `PERF`; native `Rerun` viewer for `SENSOR` / `AUDIO`).
 */

import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { showToast } from "../core/toast.js";

export const TRACE_MODES = Object.freeze({
  NONE: "NONE",
  SENSOR: "SENSOR",
  PERF: "PERF",
  AUDIO: "AUDIO",
});

export const EXTERNAL_VIEWERS = Object.freeze({
  NONE: "NONE",
  PERFETTO: "PERFETTO",
  RERUN: "RERUN",
});

const DEFAULT_CANVAS_WIDTH_PX = 960;
const MIN_CANVAS_HEIGHT_PX = 260;
const BOTTOM_TRACKBAR_CLEARANCE_PX = 56;
const RULER_HEIGHT_PX = 24;
const TRACK_LABEL_WIDTH_PX = 185;
const SPAN_ROW_HEIGHT_PX = 20;
const SPAN_TRACK_PADDING_PX = 10;
const SCALAR_TRACK_HEIGHT_PX = 58;
const MARKER_TRACK_HEIGHT_PX = 38;
const DEFAULT_VIEWPORT_SPAN_US = 2_000_000;
const MIN_VIEWPORT_SPAN_US = 10;
const MAX_VIEWPORT_SPAN_US = 3_600_000_000;
const ZOOM_STEP_FACTOR = 1.5;
const PAN_STEP_FRACTION = 0.2;
const WHEEL_ZOOM_IN_FACTOR = 0.8;
const WHEEL_ZOOM_OUT_FACTOR = 1.25;
const DRAG_CLICK_THRESHOLD_PX = 4;
const HIT_TEST_RADIUS_PX = 8;
const MAX_TABLE_ROWS = 120;
const POLL_BATCH_LIMIT = 4000;
const RULER_TICK_COUNT = 6;
const US_PER_SECOND = 1_000_000;
const US_PER_MILLISECOND = 1_000;

const SPAN_PALETTE = Object.freeze([
  "#00d8ff",
  "#00ffb4",
  "#ffb000",
  "#d68ff0",
  "#5ce1e6",
  "#73b8ff",
]);

/**
 * Resolve the canonical external viewer (`"PERFETTO"` or `"RERUN"`) for a firmware trace mode.
 *
 * @param {string} traceMode - Active trace mode (`"NONE"`, `"SENSOR"`, `"PERF"`, `"AUDIO"`).
 * @returns {string} External viewer identifier.
 */
export function resolveExternalViewerForMode(traceMode) {
  const normalized = String(traceMode || TRACE_MODES.NONE).toUpperCase();
  if (normalized === TRACE_MODES.SENSOR || normalized === TRACE_MODES.AUDIO) {
    return EXTERNAL_VIEWERS.RERUN;
  }
  return EXTERNAL_VIEWERS.PERFETTO;
}

/**
 * Format a duration or timestamp delta in microseconds into compact units (`µs`, `ms`, `s`).
 *
 * @param {number} durationUs - Microsecond value.
 * @returns {string} Human-readable duration string.
 */
export function formatDurationUs(durationUs) {
  const val = Number(durationUs) || 0;
  const absVal = Math.abs(val);
  if (absVal >= US_PER_SECOND) {
    return `${(val / US_PER_SECOND).toFixed(3)}s`;
  }
  if (absVal >= US_PER_MILLISECOND) {
    return `${(val / US_PER_MILLISECOND).toFixed(2)}ms`;
  }
  return `${Math.round(val)}µs`;
}

/**
 * Group normalized trace events into ordered swimlane track descriptors for rendering and hit-testing.
 *
 * @param {Array<object>} events - Ordered trace events from `/api/trace/events`.
 * @param {string} [filterQuery] - Optional case-insensitive filter substring.
 * @returns {Array<object>} Computed track layout models with vertical offsets (`topPx`, `heightPx`).
 */
export function buildTraceTracks(events, filterQuery = "") {
  const needle = String(filterQuery || "").trim().toLowerCase();
  const trackMap = new Map();

  for (const ev of events) {
    if (!ev || ev.ph === "E") {
      continue;
    }
    const trackName = String(ev.track || "Default");
    const nameStr = String(ev.name || "");
    const catStr = String(ev.category || "");
    if (
      needle &&
      !trackName.toLowerCase().includes(needle) &&
      !nameStr.toLowerCase().includes(needle) &&
      !catStr.toLowerCase().includes(needle)
    ) {
      continue;
    }

    let bucket = trackMap.get(trackName);
    if (!bucket) {
      bucket = {
        name: trackName,
        kind: ev.category === "span" ? "spans" : ev.ph === "C" ? "scalar" : "markers",
        items: [],
        maxDepth: 0,
        minValue: Infinity,
        maxValue: -Infinity,
        unit: ev.unit || "",
      };
      trackMap.set(trackName, bucket);
    }

    if (ev.category === "span") {
      bucket.kind = "spans";
      const depth = Math.max(0, Number(ev.depth) || 0);
      if (depth > bucket.maxDepth) {
        bucket.maxDepth = depth;
      }
    } else if (ev.ph === "C" && bucket.kind !== "spans") {
      bucket.kind = "scalar";
      const numeric = Number(ev.value);
      if (Number.isFinite(numeric)) {
        if (numeric < bucket.minValue) {
          bucket.minValue = numeric;
        }
        if (numeric > bucket.maxValue) {
          bucket.maxValue = numeric;
        }
      }
      if (!bucket.unit && ev.unit) {
        bucket.unit = String(ev.unit);
      }
    }

    bucket.items.push(ev);
  }

  const ordered = Array.from(trackMap.values()).sort((a, b) => {
    const rank = (t) => {
      if (t.name.includes("CPU Usage")) return 0;
      if (t.name.startsWith("CPU Core")) return 1;
      if (t.name.startsWith("Proximity")) return 2;
      if (t.name.includes("Thermal")) return 3;
      return 4;
    };
    const diff = rank(a) - rank(b);
    return diff !== 0 ? diff : a.name.localeCompare(b.name);
  });

  let cursorY = RULER_HEIGHT_PX;
  for (const track of ordered) {
    track.topPx = cursorY;
    if (track.kind === "spans") {
      track.heightPx =
        (track.maxDepth + 1) * SPAN_ROW_HEIGHT_PX + SPAN_TRACK_PADDING_PX * 2;
    } else if (track.kind === "scalar") {
      track.heightPx = SCALAR_TRACK_HEIGHT_PX;
      if (!Number.isFinite(track.minValue) || !Number.isFinite(track.maxValue)) {
        track.minValue = 0;
        track.maxValue = 100;
      }
    } else {
      track.heightPx = MARKER_TRACK_HEIGHT_PX;
    }
    cursorY += track.heightPx;
  }

  return ordered;
}

/**
 * Deterministic color lookup for a span or track name.
 *
 * @param {string} label - Span or track identifier.
 * @returns {string} Hex color string.
 */
function pickSpanColor(label) {
  const str = String(label || "");
  let hash = 0;
  for (let i = 0; i < str.length; i += 1) {
    hash = (hash * 31 + str.charCodeAt(i)) | 0;
  }
  return SPAN_PALETTE[Math.abs(hash) % SPAN_PALETTE.length];
}

/**
 * Interactive controller for `#traceView` and `#traceBottomTrackbar`.
 */
export class TraceViewerController {
  /**
   * Construct a `TraceViewerController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Optional boot metadata and callbacks.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.traceMode = String(options.initialTraceMode || TRACE_MODES.NONE).toUpperCase();
    this.binName = String(options.initialBinName || "firmware");
    this.onOpenTargetConfig = options.onOpenTargetConfig || null;

    this.eventsById = new Map();
    this.orderedEvents = [];
    this.afterSeq = 0;
    this.recording = true;
    this.minTsUs = null;
    this.maxTsUs = null;
    this.viewStartUs = 0;
    this.viewEndUs = DEFAULT_VIEWPORT_SPAN_US;
    this.autoFollow = true;
    this.filterQuery = "";
    this.selectedEventId = null;

    this._hitBoxes = [];
    this._tracks = [];
    this._dragState = null;
    this._unsubscribers = [];

    this.traceView = resolveElement(ctx, options.traceView || "traceView");
    this.disabledBanner = resolveElement(
      ctx,
      options.disabledBanner || "traceDisabledBanner",
    );
    this.configureShortcutBtn = resolveElement(
      ctx,
      options.configureShortcutBtn || "btnTraceConfigureShortcut",
    );
    this.exportPathInput = resolveElement(ctx, options.exportPathInput || "traceExportPath");
    this.exportCsvBtn = resolveElement(ctx, options.exportCsvBtn || "btnExportTraceCsv");
    this.exportPerfettoBtn = resolveElement(
      ctx,
      options.exportPerfettoBtn || "btnExportTracePerfetto",
    );
    this.openExternalBtn = resolveElement(
      ctx,
      options.openExternalBtn || "btnOpenExternalViewer",
    );

    this.canvasScroller = resolveElement(
      ctx,
      options.canvasScroller || "traceCanvasScroller",
    );
    this.canvas = resolveElement(ctx, options.canvas || "traceTimelineCanvas");

    // Hovering bottom trackbar elements
    this.bottomTrackbar = resolveElement(
      ctx,
      options.bottomTrackbar || "traceBottomTrackbar",
    );
    this.recordingBadge = resolveElement(
      ctx,
      options.recordingBadge || "traceRecordingBadge",
    );
    this.pauseResumeBtn = resolveElement(
      ctx,
      options.pauseResumeBtn || "btnTracePauseResume",
    );
    this.clearBtn = resolveElement(ctx, options.clearBtn || "btnTraceClear");
    this.zoomOutBtn = resolveElement(ctx, options.zoomOutBtn || "btnTraceZoomOut");
    this.zoomInBtn = resolveElement(ctx, options.zoomInBtn || "btnTraceZoomIn");
    this.fitBtn = resolveElement(ctx, options.fitBtn || "btnTraceFit");
    this.viewportBadge = resolveElement(
      ctx,
      options.viewportBadge || "traceViewportBadge",
    );
    this.filterInput = resolveElement(ctx, options.filterInput || "traceFilterInput");

    // Bottom inspector split elements
    this.sliceBadge = resolveElement(ctx, options.sliceBadge || "traceSliceBadge");
    this.sliceDetails = resolveElement(ctx, options.sliceDetails || "traceSliceDetails");
    this.eventsTableBody = resolveElement(
      ctx,
      options.eventsTableBody || "traceEventsTableBody",
    );
  }

  /**
   * Bind UI controls, canvas pointer/wheel handlers, and initial mode state.
   */
  init() {
    const bindClick = (el, fn) => {
      if (!el) {
        return;
      }
      el.addEventListener("click", fn);
      this._unsubscribers.push(() => el.removeEventListener("click", fn));
    };

    bindClick(this.configureShortcutBtn, () => {
      if (typeof this.onOpenTargetConfig === "function") {
        this.onOpenTargetConfig();
      }
    });
    bindClick(this.pauseResumeBtn, () => this.toggleRecording());
    bindClick(this.clearBtn, () => this.clearTrace());
    bindClick(this.zoomInBtn, () => this.zoomViewport(1 / ZOOM_STEP_FACTOR));
    bindClick(this.zoomOutBtn, () => this.zoomViewport(ZOOM_STEP_FACTOR));
    bindClick(this.fitBtn, () => this.fitViewport());
    bindClick(this.exportCsvBtn, () => this.exportTrace("csv"));
    bindClick(this.exportPerfettoBtn, () => this.exportTrace("perfetto"));
    bindClick(this.openExternalBtn, () => this.openExternalViewer());

    if (this.filterInput) {
      const onFilter = () => {
        this.filterQuery = this.filterInput.value || "";
        this.render();
      };
      this.filterInput.addEventListener("input", onFilter);
      this._unsubscribers.push(() => this.filterInput.removeEventListener("input", onFilter));
    }

    if (this.eventsTableBody) {
      this._unsubscribers.push(
        bindDelegatedEvent(
          this.eventsTableBody,
          "click",
          "[data-event-id]",
          (_evt, rowEl) => {
            const idNum = Number.parseInt(rowEl.getAttribute("data-event-id") || "", 10);
            if (Number.isFinite(idNum)) {
              this.selectEvent(idNum, { centerIfOutside: true });
            }
          },
        ),
      );
    }

    this._bindCanvasInteractions();
    this.setTraceMode(this.traceMode, { binName: this.binName });
    this.render();
  }

  /**
   * Bind mouse-wheel zoom, click-drag horizontal pan, and slice/sample click selection on `#traceTimelineCanvas`.
   */
  _bindCanvasInteractions() {
    if (!this.canvas) {
      return;
    }

    const onWheel = (evt) => {
      evt.preventDefault();
      const rect = this.canvas.getBoundingClientRect();
      const width = Math.max(1, (rect.width || DEFAULT_CANVAS_WIDTH_PX) - TRACK_LABEL_WIDTH_PX);
      const localX = (evt.clientX ?? TRACK_LABEL_WIDTH_PX) - rect.left - TRACK_LABEL_WIDTH_PX;
      const anchorRatio = Math.max(0, Math.min(1, localX / width));
      const factor = (evt.deltaY || 0) < 0 ? WHEEL_ZOOM_IN_FACTOR : WHEEL_ZOOM_OUT_FACTOR;
      this.zoomViewport(factor, anchorRatio);
    };

    const onPointerDown = (evt) => {
      this._dragState = {
        pointerId: evt.pointerId,
        startX: evt.clientX ?? 0,
        startY: evt.clientY ?? 0,
        initialStartUs: this.viewStartUs,
        initialEndUs: this.viewEndUs,
        moved: false,
      };
      if (evt.pointerId !== undefined && typeof this.canvas.setPointerCapture === "function") {
        this.canvas.setPointerCapture(evt.pointerId);
      }
    };

    const onPointerMove = (evt) => {
      if (!this._dragState) {
        return;
      }
      const dx = (evt.clientX ?? 0) - this._dragState.startX;
      const dy = (evt.clientY ?? 0) - this._dragState.startY;
      if (Math.hypot(dx, dy) >= DRAG_CLICK_THRESHOLD_PX) {
        this._dragState.moved = true;
      }
      if (!this._dragState.moved) {
        return;
      }
      const rect = this.canvas.getBoundingClientRect();
      const plotWidth = Math.max(1, (rect.width || DEFAULT_CANVAS_WIDTH_PX) - TRACK_LABEL_WIDTH_PX);
      const spanUs = this._dragState.initialEndUs - this._dragState.initialStartUs;
      const deltaUs = -(dx / plotWidth) * spanUs;
      this.autoFollow = false;
      this.viewStartUs = this._dragState.initialStartUs + deltaUs;
      this.viewEndUs = this._dragState.initialEndUs + deltaUs;
      this.render();
    };

    const onPointerUp = (evt) => {
      if (!this._dragState) {
        return;
      }
      const wasDrag = this._dragState.moved;
      if (
        this._dragState.pointerId !== undefined &&
        typeof this.canvas.releasePointerCapture === "function"
      ) {
        this.canvas.releasePointerCapture(this._dragState.pointerId);
      }
      this._dragState = null;

      if (!wasDrag) {
        const rect = this.canvas.getBoundingClientRect();
        const x = (evt.clientX ?? 0) - rect.left;
        const y = (evt.clientY ?? 0) - rect.top;
        const hit = this.hitTestCanvas(x, y);
        if (hit !== null) {
          this.selectEvent(hit, { centerIfOutside: false });
        }
      }
    };

    this.canvas.addEventListener("wheel", onWheel);
    this.canvas.addEventListener("pointerdown", onPointerDown);
    this.canvas.addEventListener("pointermove", onPointerMove);
    this.canvas.addEventListener("pointerup", onPointerUp);
    this._unsubscribers.push(() => {
      this.canvas.removeEventListener("wheel", onWheel);
      this.canvas.removeEventListener("pointerdown", onPointerDown);
      this.canvas.removeEventListener("pointermove", onPointerMove);
      this.canvas.removeEventListener("pointerup", onPointerUp);
    });

    const onResize = () => this.render();
    this.ctx.addViewportListener("resize", onResize);
    this._unsubscribers.push(() => this.ctx.removeViewportListener("resize", onResize));
  }

  /**
   * Synchronize trace mode metadata (`NONE` / `SENSOR` / `PERF` / `AUDIO`) and external viewer CTA.
   *
   * @param {string} mode - Active firmware trace mode.
   * @param {{binName?: string}} [meta] - Optional binary metadata.
   */
  setTraceMode(mode, meta = {}) {
    const normalized = String(mode || TRACE_MODES.NONE).toUpperCase();
    this.traceMode = normalized;
    if (meta.binName) {
      this.binName = String(meta.binName);
    }

    const viewer = resolveExternalViewerForMode(normalized);
    if (this.openExternalBtn) {
      if (viewer === EXTERNAL_VIEWERS.RERUN) {
        this.openExternalBtn.textContent = "🚀 Open in Rerun";
        this.openExternalBtn.className = "btn btn-rerun";
        this.openExternalBtn.title =
          "Stream sensor/audio time-series directly into native Rerun Viewer";
      } else {
        this.openExternalBtn.textContent = "🚀 Open in Perfetto";
        this.openExternalBtn.className = "btn btn-perfetto";
        this.openExternalBtn.title =
          "Open execution span & CPU timeline in ui.perfetto.dev";
      }
    }

    if (this.exportPathInput && !this.exportPathInput.value) {
      const suffix = normalized.toLowerCase();
      this.exportPathInput.placeholder = `recordings/traces/${this.binName}_${suffix}`;
    }

    this._syncStatusDom();
  }

  /**
   * Poll incremental trace events from `GET /api/trace/events`.
   *
   * @returns {Promise<object|null>}
   */
  async pollEvents() {
    try {
      const data = await this.ctx.fetchJson(
        `/api/trace/events?after_seq=${this.afterSeq}&limit=${POLL_BATCH_LIMIT}`,
      );
      if (!data) {
        return null;
      }

      if (data.trace_mode && data.trace_mode !== this.traceMode) {
        this.setTraceMode(data.trace_mode);
      }
      if (typeof data.recording === "boolean") {
        this.recording = data.recording;
      }

      const nextCursor =
        typeof data.cursor === "number"
          ? data.cursor
          : typeof data.next_seq === "number"
            ? data.next_seq
            : 0;

      // Detect server-side clear/reset when total_events drops to 0 while local store is non-empty
      if (Number(data.total_events) === 0 && nextCursor === 0 && this.eventsById.size > 0) {
        this._resetLocalBuffer();
      }

      const incoming = Array.isArray(data.events) ? data.events : [];
      if (incoming.length > 0) {
        for (const item of incoming) {
          this.eventsById.set(item.id, item);
        }
        this.orderedEvents = Array.from(this.eventsById.values()).sort(
          (a, b) => (a.ts_us - b.ts_us) || (a.id - b.id),
        );
      }

      if (nextCursor > this.afterSeq) {
        this.afterSeq = nextCursor;
      }
      if (Number(data.total_events) > 0 || incoming.length > 0) {
        if (typeof data.min_ts_us === "number") {
          this.minTsUs = data.min_ts_us;
        }
        if (typeof data.max_ts_us === "number") {
          this.maxTsUs = data.max_ts_us;
        }
      }

      if (this.autoFollow && this.recording && this.maxTsUs !== null) {
        const currentSpan = Math.max(
          MIN_VIEWPORT_SPAN_US,
          this.viewEndUs - this.viewStartUs,
        );
        if (this.eventsById.size <= incoming.length && this.minTsUs !== null) {
          const totalSpan = Math.max(MIN_VIEWPORT_SPAN_US, this.maxTsUs - this.minTsUs);
          const span = Math.max(totalSpan * 1.1, 100_000);
          this.viewStartUs = Math.max(0, this.minTsUs - span * 0.05);
          this.viewEndUs = this.viewStartUs + span;
        } else if (this.maxTsUs > this.viewEndUs) {
          this.viewEndUs = this.maxTsUs + currentSpan * 0.05;
          this.viewStartUs = Math.max(0, this.viewEndUs - currentSpan);
        }
      }

      if (incoming.length > 0 || data.total_events === 0) {
        this.render();
      } else {
        this._syncStatusDom();
      }
      return data;
    } catch {
      return null;
    }
  }

  /**
   * Pause or resume live RTT trace capture via `POST /api/trace/control`.
   *
   * @returns {Promise<boolean>} Updated recording flag.
   */
  async toggleRecording() {
    const nextRecording = !this.recording;
    const action = nextRecording ? "resume" : "pause";
    this.recording = nextRecording;
    if (nextRecording) {
      this.autoFollow = true;
    }
    this._syncStatusDom();
    try {
      const res = await this.ctx.postJson("/api/trace/control", { action });
      if (res && typeof res.recording === "boolean") {
        this.recording = res.recording;
        this._syncStatusDom();
      }
    } catch {
      // Keep optimistic local toggle when server is unreachable in offline mode.
    }
    return this.recording;
  }

  /**
   * Clear buffered trace events both on the daemon and locally.
   *
   * @returns {Promise<void>}
   */
  async clearTrace() {
    try {
      await this.ctx.postJson("/api/trace/control", { action: "clear" });
    } catch {
      // Clear local buffer even if server is offline in isolated unit tests.
    }
    this._resetLocalBuffer();
    this.render();
  }

  /**
   * Reset in-memory trace state.
   */
  _resetLocalBuffer() {
    this.eventsById.clear();
    this.orderedEvents = [];
    this.afterSeq = 0;
    this.minTsUs = null;
    this.maxTsUs = null;
    this.selectedEventId = null;
    this.autoFollow = true;
    this.viewStartUs = 0;
    this.viewEndUs = DEFAULT_VIEWPORT_SPAN_US;
  }

  /**
   * Zoom the horizontal viewport around `anchorRatio` (`0.0`..`1.0`, default `0.5` center).
   *
   * @param {number} factor - Multiplier (`< 1` zooms in, `> 1` zooms out).
   * @param {number} [anchorRatio] - Anchor ratio within the plot window.
   */
  zoomViewport(factor, anchorRatio = 0.5) {
    const ratio = Math.max(0, Math.min(1, Number(anchorRatio) || 0.5));
    const oldSpan = Math.max(MIN_VIEWPORT_SPAN_US, this.viewEndUs - this.viewStartUs);
    const nextSpan = Math.max(
      MIN_VIEWPORT_SPAN_US,
      Math.min(MAX_VIEWPORT_SPAN_US, oldSpan * factor),
    );
    const anchorTs = this.viewStartUs + oldSpan * ratio;
    this.autoFollow = false;
    this.viewStartUs = Math.max(0, anchorTs - nextSpan * ratio);
    this.viewEndUs = this.viewStartUs + nextSpan;
    this.render();
  }

  /**
   * Pan the horizontal viewport left (`direction < 0`) or right (`direction > 0`).
   *
   * @param {number} direction - `-1` for left, `+1` for right.
   */
  panViewport(direction) {
    const span = Math.max(MIN_VIEWPORT_SPAN_US, this.viewEndUs - this.viewStartUs);
    const shift = span * PAN_STEP_FRACTION * Math.sign(direction || 0);
    this.autoFollow = false;
    this.viewStartUs = Math.max(0, this.viewStartUs + shift);
    this.viewEndUs = this.viewStartUs + span;
    this.render();
  }

  /**
   * Fit the viewport to the full buffered `[minTsUs, maxTsUs]` capture range.
   */
  fitViewport() {
    if (this.minTsUs === null || this.maxTsUs === null) {
      this.viewStartUs = 0;
      this.viewEndUs = DEFAULT_VIEWPORT_SPAN_US;
    } else {
      const rawSpan = Math.max(MIN_VIEWPORT_SPAN_US, this.maxTsUs - this.minTsUs);
      const pad = Math.max(10, rawSpan * 0.05);
      this.viewStartUs = Math.max(0, this.minTsUs - pad);
      this.viewEndUs = this.maxTsUs + pad;
    }
    this.autoFollow = true;
    this.render();
  }

  /**
   * Handle keyboard shortcuts while `TRACE` mode is active (`W`/`+`, `S`/`-`, `A`/`←`, `D`/`→`, `F`/`0`, `Space`).
   *
   * @param {KeyboardEvent|object} event - Keydown event.
   * @returns {boolean} True if handled.
   */
  handleKeyDown(event) {
    if (!event || event.metaKey || event.ctrlKey || event.altKey) {
      return false;
    }
    const tgt = event.target;
    const tag = tgt && tgt.tagName ? String(tgt.tagName).toUpperCase() : "";
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") {
      return false;
    }

    const key = String(event.key || "");
    if (key === "w" || key === "W" || key === "+" || key === "=") {
      event.preventDefault();
      this.zoomViewport(1 / ZOOM_STEP_FACTOR);
      return true;
    }
    if (key === "s" || key === "S" || key === "-" || key === "_") {
      event.preventDefault();
      this.zoomViewport(ZOOM_STEP_FACTOR);
      return true;
    }
    if (key === "a" || key === "A" || key === "ArrowLeft") {
      event.preventDefault();
      this.panViewport(-1);
      return true;
    }
    if (key === "d" || key === "D" || key === "ArrowRight") {
      event.preventDefault();
      this.panViewport(1);
      return true;
    }
    if (key === "f" || key === "F" || key === "0") {
      event.preventDefault();
      this.fitViewport();
      return true;
    }
    if (key === " ") {
      event.preventDefault();
      this.toggleRecording();
      return true;
    }
    return false;
  }

  /**
   * Export buffered trace session to disk via `POST /api/trace/export`.
   *
   * @param {"csv"|"perfetto"} format - Output file format.
   * @returns {Promise<object|null>}
   */
  async exportTrace(format) {
    const rawPath = this.exportPathInput ? this.exportPathInput.value.trim() : "";
    try {
      const res = await this.ctx.postJson("/api/trace/export", {
        format,
        path: rawPath || null,
      });
      if (res && res.success) {
        if (this.exportPathInput && !rawPath && res.path) {
          this.exportPathInput.placeholder = res.path;
        }
        showToast(
          this.ctx,
          `Exported ${format.toUpperCase()} trace -> ${res.path}`,
          "success",
        );
      }
      return res;
    } catch (err) {
      showToast(
        this.ctx,
        `Trace export failed: ${(err && err.message) || String(err)}`,
        "error",
      );
      return null;
    }
  }

  /**
   * Open the buffered trace in the mode-appropriate external viewer:
   * - `SENSOR` / `AUDIO` -> `POST /api/trace/open_rerun` (`rerun-sdk`)
   * - `PERF` (and `NONE` fallback) -> `ui.perfetto.dev` (`ctx.openPerfettoTrace`)
   *
   * @returns {Promise<object|boolean>}
   */
  async openExternalViewer() {
    const viewer = resolveExternalViewerForMode(this.traceMode);
    if (viewer === EXTERNAL_VIEWERS.RERUN) {
      try {
        const res = await this.ctx.postJson("/api/trace/open_rerun", { spawn: true });
        if (res && res.success) {
          showToast(
            this.ctx,
            `Streamed ${res.logged_samples} samples to Rerun Viewer`,
            "success",
          );
        }
        return res;
      } catch (err) {
        showToast(
          this.ctx,
          `Failed to open Rerun Viewer: ${(err && err.message) || String(err)}`,
          "error",
        );
        return false;
      }
    }

    try {
      const response = await this.ctx.fetchRaw("/api/trace/perfetto.json");
      const rawJson = await response.text();
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const loc = this.ctx.getLocation();
      const fallbackUrl = `${loc.origin || "http://127.0.0.1:8899"}/api/trace/perfetto.json`;
      const opened = await this.ctx.openPerfettoTrace({
        buffer: rawJson,
        title: `${this.binName} (${this.traceMode})`,
        fileName: `${this.binName}_${this.traceMode.toLowerCase()}.perfetto.json`,
        fallbackUrl,
      });
      showToast(
        this.ctx,
        opened ? "Opened trace in Perfetto UI" : "Opened Perfetto UI tab",
        "success",
      );
      return opened;
    } catch (err) {
      showToast(
        this.ctx,
        `Failed to open Perfetto UI: ${(err && err.message) || String(err)}`,
        "error",
      );
      return false;
    }
  }

  /**
   * Select a span or sensor sample by `eventId` and populate `#traceSliceInspector`.
   *
   * @param {number} eventId - Event ID.
   * @param {{centerIfOutside?: boolean}} [options] - Optional viewport recentering.
   */
  selectEvent(eventId, options = {}) {
    const ev = this.eventsById.get(eventId);
    if (!ev) {
      return;
    }
    this.selectedEventId = eventId;
    if (options.centerIfOutside) {
      const ts = Number(ev.ts_us) || 0;
      if (ts < this.viewStartUs || ts > this.viewEndUs) {
        const span = Math.max(MIN_VIEWPORT_SPAN_US, this.viewEndUs - this.viewStartUs);
        this.autoFollow = false;
        this.viewStartUs = Math.max(0, ts - span * 0.5);
        this.viewEndUs = this.viewStartUs + span;
      }
    }
    this.render();
  }

  /**
   * Hit-test CSS coordinates `(x, y)` on `#traceTimelineCanvas` and return matching `eventId` or `null`.
   *
   * @param {number} x - Canvas-local CSS X coordinate.
   * @param {number} y - Canvas-local CSS Y coordinate.
   * @returns {number|null}
   */
  hitTestCanvas(x, y) {
    for (let i = this._hitBoxes.length - 1; i >= 0; i -= 1) {
      const box = this._hitBoxes[i];
      if (x >= box.x && x <= box.x + box.w && y >= box.y && y <= box.y + box.h) {
        return box.eventId;
      }
    }
    return null;
  }

  /**
   * Synchronize playback badge, pause/resume button, viewport window badge, and `NONE` mode banner.
   */
  _syncStatusDom() {
    const count = this.orderedEvents.length;
    const unitNoun =
      this.traceMode === TRACE_MODES.SENSOR ? "samples" : "events";

    if (this.disabledBanner) {
      this.disabledBanner.style.display =
        this.traceMode === TRACE_MODES.NONE && count === 0 ? "flex" : "none";
    }

    if (this.recordingBadge) {
      if (this.recording) {
        const prefix =
          this.traceMode === TRACE_MODES.SENSOR
            ? "● RECORDING SENSOR VALUES"
            : "● RECORDING LIVE";
        this.recordingBadge.textContent = `${prefix} • ${count} ${unitNoun}`;
        this.recordingBadge.className = "badge badge-green";
      } else {
        this.recordingBadge.textContent = `⏸ PAUSED • ${count} ${unitNoun}`;
        this.recordingBadge.className = "badge badge-amber";
      }
    }

    if (this.pauseResumeBtn) {
      this.pauseResumeBtn.textContent = this.recording
        ? "⏸ Pause [Space]"
        : "⏺ Resume [Space]";
    }

    if (this.viewportBadge) {
      const startSec = (this.viewStartUs / US_PER_SECOND).toFixed(3);
      const endSec = (this.viewEndUs / US_PER_SECOND).toFixed(3);
      const spanLabel = formatDurationUs(this.viewEndUs - this.viewStartUs);
      this.viewportBadge.textContent = `${startSec}s – ${endSec}s (${spanLabel})`;
    }
  }

  /**
   * Render the Retina timeline canvas, selected slice/sample inspector, and recent events table.
   */
  render() {
    this._syncStatusDom();
    this._tracks = buildTraceTracks(this.orderedEvents, this.filterQuery);
    this._renderCanvas();
    this._renderInspector();
    this._renderTable();
  }

  /**
   * Draw the time ruler, swimlane backgrounds, span bars, and scalar curves onto `#traceTimelineCanvas`.
   */
  _renderCanvas() {
    if (!this.canvas) {
      return;
    }
    const g = this.canvas.getContext("2d");
    if (!g) {
      return;
    }

    const scrollerRect = this.canvasScroller
      ? this.canvasScroller.getBoundingClientRect()
      : { width: DEFAULT_CANVAS_WIDTH_PX, height: MIN_CANVAS_HEIGHT_PX };
    const cssWidth = Math.max(
      360,
      Math.floor(scrollerRect.width || this.canvas.clientWidth || DEFAULT_CANVAS_WIDTH_PX),
    );
    const tracksBottom =
      this._tracks.length > 0
        ? this._tracks[this._tracks.length - 1].topPx +
          this._tracks[this._tracks.length - 1].heightPx
        : RULER_HEIGHT_PX + 160;
    const cssHeight = Math.max(
      MIN_CANVAS_HEIGHT_PX,
      tracksBottom + BOTTOM_TRACKBAR_CLEARANCE_PX,
    );

    const dpr = this.ctx.getDevicePixelRatio();
    this.canvas.width = Math.floor(cssWidth * dpr);
    this.canvas.height = Math.floor(cssHeight * dpr);
    this.canvas.style.height = `${cssHeight}px`;

    g.save();
    g.scale(dpr, dpr);

    // Background
    g.fillStyle = "#03090d";
    g.fillRect(0, 0, cssWidth, cssHeight);

    const plotLeft = TRACK_LABEL_WIDTH_PX;
    const plotWidth = Math.max(40, cssWidth - plotLeft - 8);
    const spanUs = Math.max(MIN_VIEWPORT_SPAN_US, this.viewEndUs - this.viewStartUs);
    const tsToX = (ts) => plotLeft + ((ts - this.viewStartUs) / spanUs) * plotWidth;

    // Time Ruler Header
    g.fillStyle = "#071821";
    g.fillRect(0, 0, cssWidth, RULER_HEIGHT_PX);
    g.strokeStyle = "#0b5966";
    g.beginPath();
    g.moveTo(0, RULER_HEIGHT_PX);
    g.lineTo(cssWidth, RULER_HEIGHT_PX);
    g.moveTo(plotLeft, 0);
    g.lineTo(plotLeft, cssHeight);
    g.stroke();

    g.fillStyle = "#00f0ff";
    g.font = "bold 10px monospace";
    g.fillText(`TRACKS (${this._tracks.length})`, 8, 15);

    g.font = "10px monospace";
    for (let i = 0; i <= RULER_TICK_COUNT; i += 1) {
      const ratio = i / RULER_TICK_COUNT;
      const tickX = plotLeft + ratio * plotWidth;
      const tickUs = this.viewStartUs + ratio * spanUs;
      g.strokeStyle = "rgba(11, 89, 102, 0.35)";
      g.beginPath();
      g.moveTo(tickX, RULER_HEIGHT_PX);
      g.lineTo(tickX, cssHeight);
      g.stroke();

      g.fillStyle = "#7aa5b0";
      g.fillText(`+${(tickUs / US_PER_SECOND).toFixed(3)}s`, tickX + 3, 15);
    }

    this._hitBoxes = [];

    if (this._tracks.length === 0) {
      g.fillStyle = "#7aa5b0";
      g.font = "11px monospace";
      const emptyCopy =
        this.traceMode === TRACE_MODES.NONE
          ? "Trace mode is NONE — configure SENSOR or PERF in 🎯 Target Config to capture live events."
          : "Waiting for live RTT defmt trace events from target...";
      g.fillText(emptyCopy, plotLeft + 18, RULER_HEIGHT_PX + 48);
      g.restore();
      return;
    }

    for (const track of this._tracks) {
      // Track separator & label gutter
      g.fillStyle = "rgba(8, 22, 29, 0.72)";
      g.fillRect(0, track.topPx, plotLeft - 1, track.heightPx);
      g.strokeStyle = "rgba(11, 89, 102, 0.5)";
      g.beginPath();
      g.moveTo(0, track.topPx + track.heightPx);
      g.lineTo(cssWidth, track.topPx + track.heightPx);
      g.stroke();

      g.fillStyle = "#e6f9ff";
      g.font = "bold 10px monospace";
      g.fillText(track.name.slice(0, 24), 8, track.topPx + 16);

      g.fillStyle = "#7aa5b0";
      g.font = "9px monospace";
      const subLabel =
        track.kind === "spans"
          ? `Depth: ${track.maxDepth + 1} • ${track.items.length} spans`
          : track.kind === "scalar"
            ? `${Math.round(track.minValue)}..${Math.round(track.maxValue)} ${track.unit}`
            : `${track.items.length} events`;
      g.fillText(subLabel.slice(0, 27), 8, track.topPx + 30);

      if (track.kind === "spans") {
        for (const ev of track.items) {
          const startUs = Number(ev.ts_us) || 0;
          const rawEnd =
            typeof ev.end_ts_us === "number" && ev.end_ts_us > startUs
              ? ev.end_ts_us
              : typeof ev.end_us === "number"
                ? ev.end_us
                : null;
          const endUs =
            rawEnd !== null
              ? rawEnd
              : Math.max(startUs + 20, this.maxTsUs || startUs + 20);
          if (endUs < this.viewStartUs || startUs > this.viewEndUs) {
            continue;
          }
          const x0 = Math.max(plotLeft, tsToX(startUs));
          const x1 = Math.min(plotLeft + plotWidth, tsToX(endUs));
          const w = Math.max(3, x1 - x0);
          const depth = Math.max(0, Number(ev.depth) || 0);
          const y = track.topPx + SPAN_TRACK_PADDING_PX + depth * SPAN_ROW_HEIGHT_PX;
          const h = SPAN_ROW_HEIGHT_PX - 3;

          g.fillStyle = pickSpanColor(ev.name);
          g.fillRect(x0, y, w, h);

          if (ev.id === this.selectedEventId) {
            g.strokeStyle = "#ffffff";
            g.lineWidth = 2;
            g.strokeRect(x0, y, w, h);
            g.lineWidth = 1;
          }

          if (w > 34) {
            g.fillStyle = "#031015";
            g.font = "bold 9px monospace";
            const maxChars = Math.max(3, Math.floor((w - 6) / 5.5));
            g.fillText(String(ev.name || "").slice(0, maxChars), x0 + 3, y + 12);
          }

          this._hitBoxes.push({ x: x0, y, w, h, eventId: ev.id });
        }
      } else if (track.kind === "scalar") {
        const range = Math.max(1e-6, track.maxValue - track.minValue);
        const padY = 8;
        const innerH = Math.max(12, track.heightPx - padY * 2);
        const valToY = (val) =>
          track.topPx + padY + innerH * (1 - (val - track.minValue) / range);

        const curvePoints = [];
        for (const ev of track.items) {
          const ts = Number(ev.ts_us) || 0;
          if (ts < this.viewStartUs || ts > this.viewEndUs) {
            continue;
          }
          const x = tsToX(ts);
          if (ev.ph === "C" && Number.isFinite(Number(ev.value))) {
            const y = valToY(Number(ev.value));
            curvePoints.push({ x, y, ev });
          } else {
            const midY = track.topPx + track.heightPx * 0.5;
            g.fillStyle = "#ff2a4b";
            g.fillRect(x - 4, midY - 6, 8, 12);
            this._hitBoxes.push({
              x: x - HIT_TEST_RADIUS_PX,
              y: midY - HIT_TEST_RADIUS_PX,
              w: HIT_TEST_RADIUS_PX * 2,
              h: HIT_TEST_RADIUS_PX * 2,
              eventId: ev.id,
            });
          }
        }

        if (curvePoints.length > 0) {
          g.strokeStyle = track.name.includes("Thermal") ? "#ffb000" : "#00ffb4";
          g.lineWidth = 1.6;
          g.beginPath();
          curvePoints.forEach((pt, idx) => {
            if (idx === 0) {
              g.moveTo(pt.x, pt.y);
            } else {
              g.lineTo(pt.x, pt.y);
            }
          });
          g.stroke();
          g.lineWidth = 1;

          for (const pt of curvePoints) {
            const isSel = pt.ev.id === this.selectedEventId;
            g.fillStyle = isSel ? "#ffffff" : "#00f0ff";
            g.beginPath();
            g.arc(pt.x, pt.y, isSel ? 4.5 : 3, 0, Math.PI * 2);
            g.fill();
            this._hitBoxes.push({
              x: pt.x - HIT_TEST_RADIUS_PX,
              y: pt.y - HIT_TEST_RADIUS_PX,
              w: HIT_TEST_RADIUS_PX * 2,
              h: HIT_TEST_RADIUS_PX * 2,
              eventId: pt.ev.id,
            });
          }
        }
      } else {
        const midY = track.topPx + track.heightPx * 0.5;
        for (const ev of track.items) {
          const ts = Number(ev.ts_us) || 0;
          if (ts < this.viewStartUs || ts > this.viewEndUs) {
            continue;
          }
          const x = tsToX(ts);
          const isInvalid = String(ev.name || "").toLowerCase().includes("invalid");
          g.fillStyle = isInvalid ? "#ff2a4b" : "#d68ff0";
          g.fillRect(x - 4, midY - 6, 8, 12);
          this._hitBoxes.push({
            x: x - HIT_TEST_RADIUS_PX,
            y: midY - HIT_TEST_RADIUS_PX,
            w: HIT_TEST_RADIUS_PX * 2,
            h: HIT_TEST_RADIUS_PX * 2,
            eventId: ev.id,
          });
        }
      }
    }

    // Selected timestamp vertical cursor
    if (this.selectedEventId !== null) {
      const selEv = this.eventsById.get(this.selectedEventId);
      if (selEv && selEv.ts_us >= this.viewStartUs && selEv.ts_us <= this.viewEndUs) {
        const cursorX = tsToX(selEv.ts_us);
        g.strokeStyle = "#ffb000";
        g.beginPath();
        g.moveTo(cursorX, RULER_HEIGHT_PX);
        g.lineTo(cursorX, cssHeight);
        g.stroke();
      }
    }

    g.restore();
  }

  /**
   * Populate the bottom-left `#traceSliceInspector` card for the selected slice or sensor sample.
   */
  _renderInspector() {
    if (!this.sliceDetails) {
      return;
    }
    const ev =
      this.selectedEventId !== null ? this.eventsById.get(this.selectedEventId) : null;
    if (!ev) {
      if (this.sliceBadge) {
        this.sliceBadge.textContent = "NONE";
      }
      this.sliceDetails.textContent =
        "Click any span bar, sensor sample point, or event row to inspect timing and measured values.";
      return;
    }

    if (this.sliceBadge) {
      this.sliceBadge.textContent = `${String(ev.category || "EVENT").toUpperCase()} #${ev.id}`;
    }

    clearChildren(this.sliceDetails);
    const pairs = [
      ["Name:", String(ev.name || "")],
      ["Track:", String(ev.track || "")],
      ["Timestamp:", `${ev.ts_us} µs (+${(ev.ts_us / US_PER_SECOND).toFixed(4)}s)`],
    ];
    const isSpanEvent =
      ev.category === "span" || ev.ph === "X" || ev.ph === "B" || Number(ev.dur_us) > 0;
    if (isSpanEvent && typeof ev.dur_us === "number") {
      pairs.push(["Duration:", `${ev.dur_us} µs (${formatDurationUs(ev.dur_us)})`]);
    }
    if (ev.value !== null && ev.value !== undefined) {
      pairs.push(["Value:", `${ev.value}${ev.unit ? ` ${ev.unit}` : ""}`]);
    }
    if (ev.module) {
      pairs.push(["Module:", String(ev.module)]);
    }
    if (ev.args && typeof ev.args === "object") {
      const argSummary = Object.entries(ev.args)
        .filter(([k]) => k !== "csv")
        .map(([k, v]) => `${k}=${v}`)
        .join(", ");
      if (argSummary) {
        pairs.push(["Details:", argSummary]);
      }
    }

    for (const [label, val] of pairs) {
      this.sliceDetails.appendChild(
        createEl(this.ctx, "div", {
          className: "trace-kv-row",
          children: [
            createEl(this.ctx, "span", { text: label }),
            createEl(this.ctx, "span", { className: "trace-kv-val", text: val }),
          ],
        }),
      );
    }
  }

  /**
   * Render the most recent filtered trace events into `#traceEventsTableBody`.
   */
  _renderTable() {
    if (!this.eventsTableBody) {
      return;
    }
    clearChildren(this.eventsTableBody);

    const needle = String(this.filterQuery || "").trim().toLowerCase();
    const visible = [];
    for (let i = this.orderedEvents.length - 1; i >= 0 && visible.length < MAX_TABLE_ROWS; i -= 1) {
      const ev = this.orderedEvents[i];
      if (!ev || ev.ph === "E") {
        continue;
      }
      if (
        needle &&
        !String(ev.track || "").toLowerCase().includes(needle) &&
        !String(ev.name || "").toLowerCase().includes(needle) &&
        !String(ev.category || "").toLowerCase().includes(needle)
      ) {
        continue;
      }
      visible.push(ev);
    }

    for (const ev of visible) {
      const isSpanEvent =
        ev.category === "span" || ev.ph === "X" || ev.ph === "B" || Number(ev.dur_us) > 0;
      const metric =
        isSpanEvent && typeof ev.dur_us === "number"
          ? `${ev.dur_us} µs`
          : ev.value !== null && ev.value !== undefined
            ? `${ev.value}${ev.unit ? ` ${ev.unit}` : ""}`
            : ev.args && ev.args.valid === false
              ? "INVALID"
              : "—";

      const tr = createEl(this.ctx, "tr", {
        className: ev.id === this.selectedEventId ? "selected" : "",
        attrs: { "data-event-id": String(ev.id) },
        children: [
          createEl(this.ctx, "td", { text: String(ev.ts_us) }),
          createEl(this.ctx, "td", { text: String(ev.track || "") }),
          createEl(this.ctx, "td", { text: String(ev.category || "").toUpperCase() }),
          createEl(this.ctx, "td", { text: String(ev.name || "") }),
          createEl(this.ctx, "td", { text: metric }),
        ],
      });
      this.eventsTableBody.appendChild(tr);
    }
  }

  /**
   * Unbind all DOM and viewport listeners.
   */
  destroy() {
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating a `TraceViewerController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {TraceViewerController}
 */
export function createTraceViewer(ctx, options = {}) {
  return new TraceViewerController(ctx, options);
}
