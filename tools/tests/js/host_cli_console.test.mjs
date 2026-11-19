import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createEl } from "../../js/core/dom.js";
import { createMockRuntimeContext } from "../../js/core/runtime_context.js";
import { createCliConsole, WORKSTATION_MODES } from "../../js/daemon/cli_console.js";

function buildCliDom(ctx) {
  const doc = ctx.getDocument();
  const modeSelect = createEl(ctx, "select", { id: "modeSelect" });
  modeSelect.value = "CLI";

  const panelTitle = createEl(ctx, "span", { id: "cliPanelTitle", text: "INTERACTIVE CLI CONSOLE" });
  const copyBtn = createEl(ctx, "button", { id: "btnCopyWorkstation", text: "📋 Copy" });

  const fsToolbar = createEl(ctx, "div", {
    id: "fsToolbar",
    style: { display: "none" },
    children: [
      createEl(ctx, "button", { className: "btn btn-cyan", dataset: { fsCmd: "ls" }, text: "ls" }),
      createEl(ctx, "button", { className: "btn", dataset: { fsCmd: "crash-log" }, text: "crash-log" }),
    ],
  });
  const cliView = createEl(ctx, "div", { id: "cliView", style: { display: "block" } });
  const fsView = createEl(ctx, "div", { id: "fsView", style: { display: "none" } });

  const fuzzyCloseBtn = createEl(ctx, "button", { id: "btnCloseFuzzyHistory", text: "✕" });
  const fuzzySearchInput = createEl(ctx, "input", { id: "fuzzySearchInput" });
  const fuzzyResultsList = createEl(ctx, "div", { id: "fuzzyResultsList" });
  const fuzzyContainer = createEl(ctx, "div", {
    id: "cliFuzzyHistory",
    style: { display: "none" },
    children: [fuzzySearchInput, fuzzyCloseBtn, fuzzyResultsList],
  });

  const promptLabel = createEl(ctx, "span", { id: "promptLabel", text: "shell> " });
  const cmdInput = createEl(ctx, "input", { id: "cmdInput" });
  const submitBtn = createEl(ctx, "button", { id: "btnSubmitCmd", text: "Send [↵]" });
  const footer = createEl(ctx, "footer", {
    className: "cli-input-bar",
    children: [fuzzyContainer, promptLabel, cmdInput, submitBtn],
  });

  doc.body.appendChild(modeSelect);
  doc.body.appendChild(panelTitle);
  doc.body.appendChild(copyBtn);
  doc.body.appendChild(fsToolbar);
  doc.body.appendChild(cliView);
  doc.body.appendChild(fsView);
  doc.body.appendChild(footer);

  return {
    modeSelect,
    panelTitle,
    copyBtn,
    fsToolbar,
    cliView,
    fsView,
    fuzzyContainer,
    fuzzySearchInput,
    fuzzyResultsList,
    fuzzyCloseBtn,
    promptLabel,
    cmdInput,
    submitBtn,
  };
}

async function flushMicrotasks(ticks = 12) {
  for (let i = 0; i < ticks; i += 1) {
    await Promise.resolve();
  }
}

describe("Host Daemon Interactive CLI & Filesystem Console", () => {
  it("isolates CLI and FS command streams, switches modes, and disables workstation when RTT is offline", async () => {
    const cliStream = [];
    const fsStream = [];
    const executedModes = [];
    let simulateOffline = false;

    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/cli/history": {
          status: 200,
          body: { history: ["gpio status", "battery status"] },
        },
        "GET /api/cli/output": () => ({ status: 200, body: cliStream }),
        "GET /api/fs/output": () => ({ status: 200, body: fsStream }),
        "POST /api/mode": (_url, opts) => {
          const payload = JSON.parse(opts.body);
          executedModes.push(payload.mode);
          return { status: 200, body: { success: true, mode: payload.mode } };
        },
        "POST /api/cli/execute": (_url, opts) => {
          const { command } = JSON.parse(opts.body);
          if (simulateOffline) {
            return {
              status: 200,
              body: {
                success: true,
                output: "Error: Target CLI is not available (RTT disconnected).",
              },
            };
          }
          cliStream.push(
            { id: cliStream.length + 1, is_input: true, text: `shell> ${command}` },
            { id: cliStream.length + 2, is_input: false, text: `ACK: ${command}` },
          );
          return { status: 200, body: { success: true, output: `Dispatched: ${command}` } };
        },
        "POST /api/fs/execute": (_url, opts) => {
          const { command } = JSON.parse(opts.body);
          fsStream.push(
            { id: fsStream.length + 1, is_input: true, text: `fs> ${command}` },
            { id: fsStream.length + 2, is_input: false, text: "metadata (64 B)\ntelemetry.bin (512 B)" },
          );
          return { status: 200, body: { success: true, output: `Dispatched: ${command}` } };
        },
      },
    });

    const dom = buildCliDom(ctx);
    const cli = createCliConsole(ctx, {
      initialMode: WORKSTATION_MODES.CLI,
    });
    await cli.init();

    assert.deepEqual(cli.getCommandHistory(), ["gpio status", "battery status"]);

    // Submit CLI command via Enter key
    dom.cmdInput.value = "motor speed 100";
    dom.cmdInput.dispatchEvent({ type: "keydown", key: "Enter", bubbles: true });
    await flushMicrotasks();
    ctx.advanceTime(350);
    await flushMicrotasks();

    assert.equal(dom.cliView.children.length, 2);
    assert.equal(dom.fsView.children.length, 0);

    // Click echoed CLI input line to repopulate command bar without prefix
    dom.cliView.children[0].click();
    assert.equal(dom.cmdInput.value, "motor speed 100");

    // Switch to FS mode and click quick-action `ls` button
    dom.modeSelect.value = "FS";
    dom.modeSelect.dispatchEvent({ type: "change", bubbles: true });
    await flushMicrotasks();

    assert.equal(cli.getMode(), WORKSTATION_MODES.FS);
    assert.equal(dom.fsView.style.display, "block");
    assert.equal(dom.cliView.style.display, "none");
    assert.equal(dom.fsToolbar.style.display, "flex");
    assert.equal(dom.promptLabel.textContent, "fs> ");
    assert.deepEqual(executedModes, ["FS"]);

    dom.fsToolbar.children[0].click();
    await flushMicrotasks();
    ctx.advanceTime(350);
    await flushMicrotasks();

    assert.equal(dom.fsView.children.length, 2);
    assert.equal(dom.cliView.children.length, 2); // Strictly isolated!

    await cli.copyActiveOutput(dom.copyBtn);
    assert.ok(ctx.getCopiedText().includes("fs> ls"));
    assert.ok(ctx.getCopiedText().includes("telemetry.bin"));

    // Switch back to CLI and verify target offline disables workstation
    await cli.setMode(WORKSTATION_MODES.CLI);
    simulateOffline = true;
    await cli.submitCommand("help");
    assert.equal(dom.cmdInput.disabled, true);
    assert.equal(dom.submitBtn.disabled, true);
    assert.ok(dom.cmdInput.placeholder.includes("Target CLI is not available"));

    // Re-enable workstation after sync completes
    cli.setWorkstationDisabled(false);
    assert.equal(dom.cmdInput.disabled, false);
    assert.equal(dom.submitBtn.disabled, false);

    // Clickable command populates cmdInput
    cli.handleCommandClick("battery status");
    assert.equal(dom.cmdInput.value, "battery status");

    cli.destroy();
  });

  it("supports Up/Down arrow history navigation and Ctrl+R reverse fuzzy search popup", async () => {
    const ctx = createMockRuntimeContext({
      fetchRoutes: {
        "GET /api/cli/history": {
          status: 200,
          body: {
            history: [
              "gpio status",
              "motor speed 50",
              "battery status",
              "motor speed 120",
            ],
          },
        },
      },
    });
    const dom = buildCliDom(ctx);
    const cli = createCliConsole(ctx);
    await cli.init();

    // Up / Down arrow recall
    dom.cmdInput.dispatchEvent({ type: "keydown", key: "ArrowUp", bubbles: true });
    assert.equal(dom.cmdInput.value, "motor speed 120");
    dom.cmdInput.dispatchEvent({ type: "keydown", key: "ArrowUp", bubbles: true });
    assert.equal(dom.cmdInput.value, "battery status");
    dom.cmdInput.dispatchEvent({ type: "keydown", key: "ArrowDown", bubbles: true });
    assert.equal(dom.cmdInput.value, "motor speed 120");
    dom.cmdInput.dispatchEvent({ type: "keydown", key: "ArrowDown", bubbles: true });
    assert.equal(dom.cmdInput.value, "");

    // Open Ctrl+R fuzzy history search
    dom.cmdInput.dispatchEvent({
      type: "keydown",
      key: "r",
      ctrlKey: true,
      bubbles: true,
    });
    assert.equal(cli.isFuzzyOpen, true);
    assert.equal(dom.fuzzyContainer.style.display, "flex");
    assert.equal(dom.fuzzyResultsList.children.length, 4);

    // Filter with subsequence query "mtr 50"
    dom.fuzzySearchInput.value = "mtr 50";
    dom.fuzzySearchInput.dispatchEvent({ type: "input", bubbles: true });
    assert.equal(cli.currentFuzzyMatches[0], "motor speed 50");

    // Cycle with Ctrl+R and select with Enter
    dom.fuzzySearchInput.value = "status";
    dom.fuzzySearchInput.dispatchEvent({ type: "input", bubbles: true });
    assert.equal(cli.currentFuzzyMatches.length, 2);
    assert.equal(cli.fuzzySelectedIndex, 0);

    dom.fuzzySearchInput.dispatchEvent({
      type: "keydown",
      key: "r",
      ctrlKey: true,
      bubbles: true,
    });
    assert.equal(cli.fuzzySelectedIndex, 1);

    const expectedSelection = cli.currentFuzzyMatches[1];
    dom.fuzzySearchInput.dispatchEvent({
      type: "keydown",
      key: "Enter",
      bubbles: true,
    });
    assert.equal(cli.isFuzzyOpen, false);
    assert.equal(dom.fuzzyContainer.style.display, "none");
    assert.equal(dom.cmdInput.value, expectedSelection);

    // Re-open and dismiss with Escape
    cli.openFuzzyHistory();
    assert.equal(cli.isFuzzyOpen, true);
    dom.fuzzySearchInput.dispatchEvent({
      type: "keydown",
      key: "Escape",
      bubbles: true,
    });
    assert.equal(cli.isFuzzyOpen, false);

    cli.destroy();
  });
});
