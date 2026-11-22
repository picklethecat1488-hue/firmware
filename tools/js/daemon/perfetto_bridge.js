/**
 * Same-Origin Perfetto `<iframe>` + `PING`/`PONG` `postMessage` Bridge (`GET /perfetto`).
 *
 * Fetches `/api/trace/perfetto.json` over same-origin HTTP from `host.py` and streams the
 * resulting `ArrayBuffer` directly into `<iframe src="https://ui.perfetto.dev">` via
 * `RuntimeContext.streamPerfettoToIframe`, avoiding WebKit/Safari `https -> http` Mixed-Content
 * `TypeError: Load failed` blocks (`WORM-SEGMENTED-COCOON-284`).
 */

import { resolveElement } from "../core/dom.js";
import { getRuntimeContext } from "../core/runtime_context.js";

const DEFAULT_TRACE_TITLE = "Firmware Perfetto Trace";
const DEFAULT_TRACE_FILENAME = "trace.perfetto.json";
const DEFAULT_TRACE_URL = "/api/trace/perfetto.json";
const BANNER_AUTOHIDE_DELAY_MS = 3500;

/**
 * Controller coordinating same-origin trace loading and `postMessage` delivery into `ui.perfetto.dev`.
 */
export class PerfettoBridgeController {
  /**
   * @param {object} ctx - Explicit `RuntimeContext`.
   */
  constructor(ctx) {
    this.ctx = ctx;
    const cfg = ctx.getBootConfig() || {};
    this.title = String(cfg.title || DEFAULT_TRACE_TITLE);
    this.fileName = String(cfg.fileName || DEFAULT_TRACE_FILENAME);
    this.traceUrl = String(cfg.traceUrl || DEFAULT_TRACE_URL);
    this.cachedTraceText = null;
    this.autoHideHandle = null;

    this.bannerEl = resolveElement(ctx, "perfettoBridgeBanner");
    this.statusEl = resolveElement(ctx, "perfettoBridgeStatus");
    this.openTabBtn = resolveElement(ctx, "perfettoBridgeOpenTabBtn");
    this.dismissBtn = resolveElement(ctx, "perfettoBridgeDismissBtn");
    this.iframeEl = resolveElement(ctx, "perfettoEmbedFrame");

    if (this.openTabBtn) {
      this.openTabBtn.addEventListener("click", () => {
        this.openStandaloneTab();
      });
    }
    if (this.dismissBtn) {
      this.dismissBtn.addEventListener("click", () => {
        this.dismissBanner();
      });
    }
  }

  /**
   * Fetch the Perfetto JSON trace payload over same-origin HTTP from `host.py`.
   *
   * @returns {Promise<string>}
   */
  async fetchSameOriginTraceText() {
    const resp = await this.ctx.fetchRaw(this.traceUrl);
    const rawJson = await resp.text();
    if (!resp.ok) {
      throw new Error(`HTTP ${resp.status}`);
    }
    this.cachedTraceText = rawJson;
    return rawJson;
  }

  /**
   * Update the compact bridge status banner text and state attribute.
   *
   * @param {string} message - Status text.
   * @param {"loading"|"ready"|"error"} state - Visual state indicator.
   */
  setStatus(message, state) {
    if (this.statusEl) {
      this.statusEl.textContent = message;
    }
    if (this.bannerEl) {
      this.bannerEl.setAttribute("data-state", state);
      this.bannerEl.style.display = "flex";
    }
  }

  /**
   * Hide the compact floating status banner once the trace is loaded.
   */
  dismissBanner() {
    if (this.autoHideHandle !== null) {
      this.ctx.clearTimeout(this.autoHideHandle);
      this.autoHideHandle = null;
    }
    if (this.bannerEl) {
      this.bannerEl.style.display = "none";
    }
  }

  /**
   * Fetch the trace from same-origin `/api/trace/perfetto.json` and push it into `#perfettoEmbedFrame`.
   *
   * @returns {Promise<boolean>}
   */
  async start() {
    if (!this.iframeEl) {
      this.setStatus("Perfetto embed frame missing", "error");
      return false;
    }
    this.setStatus(`Loading ${this.fileName} into Perfetto UI...`, "loading");
    try {
      const traceText = await this.fetchSameOriginTraceText();
      const streamed = await this.ctx.streamPerfettoToIframe(this.iframeEl, {
        buffer: traceText,
        title: this.title,
        fileName: this.fileName,
      });
      if (!streamed) {
        this.setStatus("Perfetto UI handshake timed out — click Open Standalone Tab", "error");
        return false;
      }
      this.setStatus(`Loaded ${this.fileName} in Perfetto UI`, "ready");
      this.autoHideHandle = this.ctx.setTimeout(() => {
        this.dismissBanner();
      }, BANNER_AUTOHIDE_DELAY_MS);
      return true;
    } catch (err) {
      this.setStatus(
        `Failed to load trace: ${(err && err.message) || String(err)}`,
        "error",
      );
      return false;
    }
  }

  /**
   * Open a standalone `https://ui.perfetto.dev` browser tab using a direct user-gesture popup
   * and push the cached same-origin trace buffer via `postMessage`.
   *
   * @returns {Promise<boolean>}
   */
  async openStandaloneTab() {
    const opened = await this.ctx.openPerfettoTrace({
      loadBuffer: async () => {
        if (this.cachedTraceText !== null) {
          return this.cachedTraceText;
        }
        return this.fetchSameOriginTraceText();
      },
      title: this.title,
      fileName: this.fileName,
    });
    if (!opened) {
      this.setStatus("Popup blocked — allow popups for 127.0.0.1 to open standalone tab", "error");
    }
    return opened;
  }
}

/**
 * Create a `PerfettoBridgeController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @returns {PerfettoBridgeController}
 */
export function createPerfettoBridgeApp(ctx) {
  return new PerfettoBridgeController(ctx);
}

// Auto-boot when loaded in the `/perfetto` browser document (`null` in headless `node --test`).
const browserCtx = getRuntimeContext();
const browserDoc = browserCtx.getDocument();
if (browserDoc !== null) {
  if (browserDoc.readyState === "loading") {
    browserDoc.addEventListener("DOMContentLoaded", () => {
      createPerfettoBridgeApp(browserCtx).start();
    });
  } else {
    createPerfettoBridgeApp(browserCtx).start();
  }
}
