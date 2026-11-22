import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { parseAnsiToHtml, stripAnsi } from "../../js/core/ansi.js";
import { applyIframeContextClass, bindDelegatedEvent, createEl } from "../../js/core/dom.js";
import { fuzzyFilterAndRank, fuzzyScore } from "../../js/core/fuzzy_search.js";
import { showConfirmModal } from "../../js/core/modal.js";
import { AsymptoticProgressBar, performActionWithProgress } from "../../js/core/progress.js";
import { setupPanelResizer } from "../../js/core/resizer.js";
import {
  createBrowserRuntimeContext,
  createMockRuntimeContext,
} from "../../js/core/runtime_context.js";
import { escapeHtml, sanitizeAuthor, sanitizeTextWithBlacklist } from "../../js/core/sanitize.js";
import { copyToClipboard, showToast } from "../../js/core/toast.js";
import { applyUiScale, initUiScaleControl } from "../../js/core/ui_scale.js";

describe("Shared Core Modules (Sanitize, ANSI, Fuzzy, UI Scale, Progress, DOM)", () => {
  it("redacts personal author names and email addresses per Rule 6", () => {
    assert.equal(sanitizeAuthor("Jane Doe <jane@example.com>"), "User <user@workspace>");
    assert.equal(sanitizeAuthor("jane@example.com"), "user@workspace");
    assert.equal(sanitizeAuthor("somehandle"), "User");
    assert.equal(sanitizeAuthor("User"), "User");
    assert.equal(sanitizeAuthor("system"), "system");
    assert.equal(sanitizeAuthor(""), "User");

    const scrubbed = sanitizeTextWithBlacklist(
      "Co-authored-by: SecretHandle and secrethandle@domain.org",
      ["secrethandle@domain.org", "SecretHandle"],
    );
    assert.equal(scrubbed, "Co-authored-by: User and user@workspace");
    assert.equal(escapeHtml('<script>"x"&\'y\'</script>'), "&lt;script&gt;&quot;x&quot;&amp;&#039;y&#039;&lt;/script&gt;");
  });

  it("parses ANSI SGR colors and OSC-8 clickable command hyperlinks without XSS", () => {
    const raw =
      "\x1b[32;1m[PASS]\x1b[0m Step 1 — \x1b]8;;prompt://P\x07[P]ass\x1b]8;;\x07 <unsafe>";
    const html = parseAnsiToHtml(raw);
    assert.ok(html.includes("color:#50fa7b;font-weight:700"));
    assert.ok(html.includes('data-osc-uri="prompt://P"'));
    assert.ok(html.includes("&lt;unsafe&gt;"));
    assert.ok(!html.includes("<unsafe>"));
    assert.equal(stripAnsi(raw), "[PASS] Step 1 — [P]ass <unsafe>");
  });

  it("ranks subsequence and path-boundary matches in fuzzyFilterAndRank", () => {
    const files = [
      "provider/scope/pipeline.py",
      "js/scope/pcb_renderer.js",
      "js/scope/pcb_spatial_index.js",
      "projects/carrier_board/carrier_board.py",
    ];
    const ranked = fuzzyFilterAndRank("pcb_ren", files);
    assert.equal(ranked[0].item, "js/scope/pcb_renderer.js");
    assert.equal(fuzzyScore("zzz", "js/scope/main.js"), null);
  });

  it("updates CSS --ui-scale custom property without setting document zoom", () => {
    const ctx = createMockRuntimeContext();
    const doc = ctx.getDocument();
    const select = createEl(ctx, "select", { id: "scale-sel" });
    select.value = "1.25";
    doc.body.appendChild(select);

    const applied = initUiScaleControl(ctx, "scale-sel", { storageKey: "test_scale" });
    assert.equal(applied, 1.25);
    assert.equal(doc.documentElement.style.getPropertyValue("--ui-scale"), "1.2500");
    assert.equal(doc.documentElement.style.zoom, "");

    applyUiScale(ctx, 80, { mode: "fontPercent", storageKey: "test_scale" });
    assert.equal(doc.documentElement.style.getPropertyValue("--ui-scale"), "1.0000");
  });

  it("drives AsymptoticProgressBar and performActionWithProgress deterministically", async () => {
    const ctx = createMockRuntimeContext();
    const doc = ctx.getDocument();
    const overlay = createEl(ctx, "div", { id: "actionProgressModalOverlay", style: { display: "none" } });
    const bar = createEl(ctx, "div", { id: "actionProgressBar" });
    const pct = createEl(ctx, "span", { id: "actionProgressPercent" });
    const status = createEl(ctx, "div", { id: "actionProgressStatus" });
    doc.body.appendChild(overlay);
    doc.body.appendChild(bar);
    doc.body.appendChild(pct);
    doc.body.appendChild(status);

    const tracker = new AsymptoticProgressBar(ctx, { barEl: bar, percentEl: pct, statusEl: status });
    tracker.start();
    ctx.advanceTime(1200);
    assert.ok(Number.parseInt(pct.textContent, 10) >= 35);
    tracker.stop();

    const result = await performActionWithProgress(
      ctx,
      async () => {
        ctx.advanceTime(1500);
        return "synced";
      },
      { title: "SYNC WORKSPACE", delayMs: 500 },
    );
    assert.equal(result, "synced");
    ctx.advanceTime(500);
    assert.equal(overlay.style.display, "none");
  });

  it("handles delegated events, panel resizing, toasts, clipboard, and confirm modals", async () => {
    const ctx = createMockRuntimeContext({ inIframe: true });
    const doc = ctx.getDocument();
    assert.equal(applyIframeContextClass(ctx), true);
    assert.equal(doc.documentElement.classList.contains("in-iframe"), true);

    const clickedIds = [];
    const list = createEl(ctx, "div", {
      children: [
        createEl(ctx, "button", {
          className: "item-btn",
          dataset: { itemId: "pad-42" },
          children: [createEl(ctx, "span", { text: "Pad 42" })],
        }),
      ],
    });
    doc.body.appendChild(list);
    bindDelegatedEvent(list, "click", ".item-btn", (_evt, btn) => {
      clickedIds.push(btn.dataset.itemId);
    });
    list.children[0].children[0].click();
    assert.deepEqual(clickedIds, ["pad-42"]);

    // WebKit / WKWebView Text-node click target (nodeType === 3, no .closest() method)
    list.dispatchEvent({
      type: "click",
      bubbles: true,
      target: { nodeType: 3, parentElement: list.children[0] },
    });
    assert.deepEqual(clickedIds, ["pad-42", "pad-42"]);

    const pane = createEl(ctx, "div", { id: "left-pane" });
    pane.setBoundingClientRect({ left: 0, top: 0, width: 240, height: 600 });
    const handle = createEl(ctx, "div", { id: "left-resizer" });
    doc.body.appendChild(pane);
    doc.body.appendChild(handle);
    setupPanelResizer(ctx, handle, pane, "pane_w", { minSize: 180, maxSize: 500 });
    handle.dispatchEvent({ type: "pointerdown", pointerId: 1, clientX: 240, clientY: 100 });
    handle.dispatchEvent({ type: "pointermove", pointerId: 1, clientX: 310, clientY: 100 });
    handle.dispatchEvent({ type: "pointerup", pointerId: 1, clientX: 310, clientY: 100 });
    assert.equal(pane.style.width, "310px");
    assert.equal(ctx.getStorage().getItem("pane_w"), "310");

    await copyToClipboard(ctx, "abc1234", "Hash copied");
    assert.equal(ctx.getCopiedText(), "abc1234");
    const toast = showToast(ctx, "Saved", "success", 500);
    ctx.flushAnimationFrames();
    assert.equal(toast.style.opacity, "1");
    ctx.advanceTime(800);

    const confirmPromise = showConfirmModal(ctx, { title: "Discard?", message: "Sure?" });
    ctx.emitViewportEvent("keydown", { key: "Enter" });
    assert.equal(await confirmPromise, true);

    const openedTrace = await ctx.openPerfettoTrace({
      buffer: '{"traceEvents":[]}',
      title: "Cat Detector Perf",
      fileName: "cat_detector.perfetto.json",
    });
    assert.equal(openedTrace, true);
    assert.equal(ctx.openedPerfettoTraces.length, 1);
    assert.equal(ctx.openedPerfettoTraces[0].title, "Cat Detector Perf");
    assert.equal(ctx.openedPerfettoTraces[0].fileName, "cat_detector.perfetto.json");
    assert.ok(ctx.openedPerfettoTraces[0].byteLength > 0);

    const mockIframe = createEl(ctx, "iframe", { id: "perfettoEmbedFrame" });
    const streamedMockIframe = await ctx.streamPerfettoToIframe(mockIframe, {
      loadBuffer: async () => '{"traceEvents":[{"name":"iframe_span"}]}',
      title: "Embedded Perfetto Trace",
      fileName: "embedded.perfetto.json",
    });
    assert.equal(streamedMockIframe, true);
    assert.equal(ctx.streamedPerfettoFrames.length, 1);
    assert.equal(ctx.streamedPerfettoFrames[0].title, "Embedded Perfetto Trace");
    assert.equal(ctx.streamedPerfettoFrames[0].fileName, "embedded.perfetto.json");
    assert.ok(ctx.streamedPerfettoFrames[0].byteLength > 0);

    // WORM-SEGMENTED-COCOON-284: Browser runtime returns false when popup is blocked (null)
    // and opens ui.perfetto.dev synchronously BEFORE awaiting loadBuffer().
    const blockedBrowserCtx = createBrowserRuntimeContext({
      open: () => null,
    });
    const blockedRes = await blockedBrowserCtx.openPerfettoTrace({
      buffer: '{"traceEvents":[]}',
    });
    assert.equal(blockedRes, false);

    const browserSeq = [];
    let messageHandler = null;
    const postedPayloads = [];
    const popupBrowserCtx = createBrowserRuntimeContext({
      open: (url) => {
        browserSeq.push(`open:${url}`);
        return {
          closed: false,
          postMessage(msg, origin) {
            postedPayloads.push({ msg, origin });
            if (msg === "PING" && messageHandler) {
              messageHandler({ origin: "https://ui.perfetto.dev", data: "PONG" });
            }
          },
        };
      },
      addEventListener: (evtType, fn) => {
        if (evtType === "message") {
          messageHandler = fn;
        }
      },
      removeEventListener: () => {},
      setInterval: () => 1,
      clearInterval: () => {},
      setTimeout: () => 2,
      clearTimeout: () => {},
    });
    const streamed = await popupBrowserCtx.openPerfettoTrace({
      loadBuffer: async () => {
        browserSeq.push("loadBuffer");
        return '{"traceEvents":[{"name":"sync_span"}]}';
      },
      title: "Live Perf Trace",
      fileName: "live_perf.perfetto.json",
    });
    assert.equal(streamed, true);
    assert.deepEqual(browserSeq, ["open:https://ui.perfetto.dev", "loadBuffer"]);
    assert.ok(
      postedPayloads.some(
        (entry) =>
          entry.origin === "https://ui.perfetto.dev" &&
          entry.msg &&
          entry.msg.perfetto &&
          entry.msg.perfetto.title === "Live Perf Trace" &&
          entry.msg.perfetto.buffer.byteLength > 0,
      ),
    );

    // WORM-SEGMENTED-COCOON-284 (Safari mixed-content fix): streamPerfettoToIframe performs
    // PING -> PONG handshake with iframeEl.contentWindow and posts { perfetto: { buffer } }.
    let iframeMsgHandler = null;
    const iframeMessages = [];
    const iframeBrowserCtx = createBrowserRuntimeContext({
      addEventListener: (evtType, fn) => {
        if (evtType === "message") {
          iframeMsgHandler = fn;
        }
      },
      removeEventListener: () => {},
      setInterval: () => 11,
      clearInterval: () => {},
      setTimeout: () => 12,
      clearTimeout: () => {},
    });
    const fakeIframeWin = {
      closed: false,
      postMessage(msg, origin) {
        iframeMessages.push({ msg, origin });
        if (msg === "PING" && iframeMsgHandler) {
          iframeMsgHandler({
            origin: "https://ui.perfetto.dev",
            source: fakeIframeWin,
            data: "PONG",
          });
        }
      },
    };
    const streamedToFrame = await iframeBrowserCtx.streamPerfettoToIframe(
      { contentWindow: fakeIframeWin },
      {
        loadBuffer: async () => '{"traceEvents":[{"name":"safari_bridge_span"}]}',
        title: "Safari Bridge Trace",
        fileName: "safari_bridge.perfetto.json",
      },
    );
    assert.equal(streamedToFrame, true);
    assert.ok(
      iframeMessages.some(
        (entry) =>
          entry.origin === "https://ui.perfetto.dev" &&
          entry.msg &&
          entry.msg.perfetto &&
          entry.msg.perfetto.title === "Safari Bridge Trace" &&
          entry.msg.perfetto.buffer.byteLength > 0,
      ),
    );
  });
});
