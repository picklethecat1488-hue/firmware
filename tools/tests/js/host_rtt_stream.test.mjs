import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import {
  createRttStream,
  extractOscCommand,
  renderTerminalHtml,
  SHOW_SYSTEM_LOGS_STORAGE_KEY,
} from "../../js/daemon/rtt_stream.js";

function buildRttDom(ctx) {
  const doc = ctx.getDocument();
  const logPanel = createEl(ctx, "div", {
    id: "logPanel",
    style: { display: "none", width: "50%" },
  });
  const splitter = createEl(ctx, "div", {
    id: "vSplitter",
    style: { display: "none" },
  });
  const logView = createEl(ctx, "div", { id: "logView" });
  logView.clientHeight = 200;
  logView.scrollHeight = 200;
  logView.scrollTop = 0;

  const toggleBtn = createEl(ctx, "button", {
    id: "btnToggleLog",
    className: "btn btn-amber",
    text: "Show Log",
  });
  const collapseBtn = createEl(ctx, "button", {
    id: "btnCollapseLog",
    text: "▶ Show",
  });
  const copyBtn = createEl(ctx, "button", {
    id: "btnCopyLog",
    text: "📋 Copy",
  });
  const autoScrollBadge = createEl(ctx, "span", {
    id: "logAutoScrollBadge",
    text: "AUTO-SCROLL: ON",
  });

  logPanel.appendChild(logView);
  doc.body.appendChild(logPanel);
  doc.body.appendChild(splitter);
  doc.body.appendChild(toggleBtn);
  doc.body.appendChild(collapseBtn);
  doc.body.appendChild(copyBtn);
  doc.body.appendChild(autoScrollBadge);

  return {
    logPanel,
    splitter,
    logView,
    toggleBtn,
    collapseBtn,
    copyBtn,
    autoScrollBadge,
  };
}

describe("Host Daemon RTT Stream & Terminal Hyperlink Renderer", () => {
  it("renders ANSI SGR colors, OSC-8 clickable command choices, URLs, and vscode:// source links safely", () => {
    const oscPass = "\x1b[1;32m\x1b]8;;command:pass\x1b\\[P]ass\x1b]8;;\x1b\\\x1b[0m";
    const oscFail = "\x1b[1;31m\x1b]8;;command:fail\x1b\\[F]ail\x1b]8;;\x1b\\\x1b[0m";
    const raw = `Prompt: ${oscPass} / ${oscFail} at src/main.rs:42:5 https://embassy.dev <script>x</script>`;

    const html = renderTerminalHtml(raw);
    assert.ok(html.includes('data-osc-uri="command:pass"'));
    assert.ok(html.includes('data-command="pass"'));
    assert.ok(html.includes('data-command="fail"'));
    assert.ok(html.includes("osc8-cmd-btn"));
    assert.ok(html.includes('href="vscode://file/src/main.rs:42:5"'));
    assert.ok(html.includes('href="https://embassy.dev"'));
    assert.ok(html.includes("&lt;script&gt;x&lt;/script&gt;"));
    assert.ok(!html.includes("<script>"));

    assert.equal(extractOscCommand("command:pass"), "pass");
    assert.equal(extractOscCommand("prompt://skip"), "skip");
    assert.equal(extractOscCommand("https://embassy.dev"), "");
    assert.equal(extractOscCommand("vscode://file/src/lib.rs:10"), "");

    // Verify intra-line carriage-return overwrite normalization
    const progressHtml = renderTerminalHtml("Erasing 10%\rErasing 50%\rErasing 100%\nDone");
    assert.ok(!progressHtml.includes("Erasing 10%"));
    assert.ok(progressHtml.includes("Erasing 100%\nDone"));
  });

  it("ingests incremental RTT/defmt logs, handles server ID resets, clears high-water marks, and filters channels", async () => {
    let logsPayload = [
      { id: 1, level: "INFO", channel: "daemon", message: "Host session attached" },
      { id: 2, level: "INFO", channel: "rtt", message: "\x1b[32mdefmt boot ok\x1b[0m" },
    ];
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/logs": () => ({ status: 200, body: logsPayload }),
      },
    });
    const dom = buildRttDom(ctx);
    const stream = createRttStream(ctx);
    stream.init({ initialShowLogs: false });

    assert.equal(stream.isPanelVisible(), false);
    assert.equal(dom.logPanel.style.display, "none");

    await stream.pollLogs();
    assert.equal(dom.logView.children.length, 2);
    assert.ok(dom.logView.children[1].classList.contains("log-line-target"));
    assert.equal(
      stream.getPlainTextLogs(),
      "[daemon] Host session attached\n[rtt] defmt boot ok",
    );

    // Append incremental entry id=3
    logsPayload = [
      ...logsPayload,
      { id: 3, level: "ERROR", channel: "defmt", message: "PANIC at src/motor.rs:88" },
    ];
    await stream.pollLogs();
    assert.equal(dom.logView.children.length, 3);
    assert.ok(dom.logView.children[2].classList.contains("log-line-error"));

    // Filter out system/daemon logs so only target/rtt/defmt channels remain
    stream.setIncludeSystemLogs(false);
    assert.equal(dom.logView.children.length, 2);
    assert.equal(
      stream.getPlainTextLogs(),
      "[rtt] defmt boot ok\n[defmt] PANIC at src/motor.rs:88",
    );
    stream.setIncludeSystemLogs(true);
    assert.equal(dom.logView.children.length, 3);

    // Clear suppresses IDs <= 3 while displaying newly arriving IDs > 3
    stream.clear();
    assert.equal(dom.logView.children.length, 0);
    await stream.pollLogs();
    assert.equal(dom.logView.children.length, 0);

    logsPayload = [
      ...logsPayload,
      { id: 4, level: "WARN", channel: "rtt", message: "Undervoltage warning" },
    ];
    await stream.pollLogs();
    assert.equal(dom.logView.children.length, 1);
    assert.equal(stream.getPlainTextLogs(), "[rtt] Undervoltage warning");

    // Server restart resets sequence IDs back to 1
    logsPayload = [{ id: 1, level: "INFO", channel: "rtt", message: "Rebooted target" }];
    await stream.pollLogs();
    assert.equal(dom.logView.children.length, 1);
    assert.equal(stream.getPlainTextLogs(), "[rtt] Rebooted target");

    // Copy plain text logs to clipboard
    await stream.copyLogs(dom.copyBtn);
    assert.equal(ctx.getCopiedText(), "[rtt] Rebooted target");
    assert.equal(dom.copyBtn.textContent, "✓ Copied!");
    ctx.advanceTime(1600);
    assert.equal(dom.copyBtn.textContent, "📋 Copy");

    stream.destroy();
  });

  it("persists panel visibility, tracks user scroll-lock state, and dispatches delegated OSC-8 & vscode clicks", () => {
    const ctx = createMockRuntimeContext();
    const dom = buildRttDom(ctx);
    const clickedCommands = [];
    const stream = createRttStream(ctx, {
      onCommandClick: (cmd) => clickedCommands.push(cmd),
    });
    stream.init({ initialShowLogs: true });

    assert.equal(stream.isPanelVisible(), true);
    assert.equal(dom.logPanel.style.display, "flex");
    assert.equal(dom.splitter.style.display, "flex");
    assert.equal(dom.toggleBtn.textContent, "Hide Log");
    assert.equal(ctx.getStorage().getItem(SHOW_SYSTEM_LOGS_STORAGE_KEY), "true");

    // Simulate user scrolling up away from bottom -> pauses auto-scroll
    dom.logView.scrollHeight = 1000;
    dom.logView.clientHeight = 200;
    dom.logView.scrollTop = 100;
    dom.logView.dispatchEvent({ type: "scroll", bubbles: false });
    assert.equal(stream.isAutoScrollLocked(), false);
    assert.equal(dom.autoScrollBadge.textContent, "AUTO-SCROLL: PAUSED");

    // Incoming log does not yank scrollTop when user has scrolled up
    stream.ingestLogs([{ id: 1, level: "INFO", channel: "rtt", message: "Line 1" }]);
    assert.equal(dom.logView.scrollTop, 100);

    // Clicking the auto-scroll badge re-locks and jumps to bottom
    dom.autoScrollBadge.click();
    assert.equal(stream.isAutoScrollLocked(), true);
    assert.equal(dom.autoScrollBadge.textContent, "AUTO-SCROLL: ON");
    assert.equal(dom.logView.scrollTop, 1000);

    // Delegated OSC-8 command button click inside #logView
    const cmdBtn = createEl(ctx, "button", {
      className: "osc8-link osc8-cmd-btn term-link term-cmd-link",
      dataset: { command: "pass", oscUri: "command:pass" },
      text: "[P]ass",
    });
    const fileLink = createEl(ctx, "a", {
      className: "term-link term-file-link",
      attrs: { href: "vscode://file/src/main.rs:24" },
      text: "src/main.rs:24",
    });
    dom.logView.appendChild(cmdBtn);
    dom.logView.appendChild(fileLink);

    cmdBtn.click();
    assert.deepEqual(clickedCommands, ["pass"]);

    fileLink.click();
    assert.equal(ctx.openedUrls.length, 1);
    assert.equal(ctx.openedUrls[0].url, "vscode://file/src/main.rs:24");

    stream.togglePanel(false);
    assert.equal(stream.isPanelVisible(), false);
    assert.equal(ctx.getStorage().getItem(SHOW_SYSTEM_LOGS_STORAGE_KEY), "false");

    stream.destroy();
  });
});
