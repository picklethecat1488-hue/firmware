/**
 * RTT / defmt Target System Log Stream & Terminal Hyperlink Renderer.
 *
 * Manages incremental `/api/logs` ingestion, system-channel filtering,
 * `host_show_system_logs` split-pane visibility persistence, ANSI SGR + OSC-8
 * clickable prompt/command links (`[P]ass`, `[F]ail`, `[S]kip`), clickable
 * `vscode://file` source path links, and user scroll-lock state transitions.
 */

import { parseAnsiToHtml, stripAnsi } from "../core/ansi.js";
import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { escapeHtml } from "../core/sanitize.js";

export const SHOW_SYSTEM_LOGS_STORAGE_KEY = "host_show_system_logs";
const AUTO_SCROLL_THRESHOLD_PX = 24;
const COPY_FEEDBACK_DURATION_MS = 1500;
const DEFAULT_LOG_PANEL_WIDTH = "50%";
const SYSTEM_LOG_CHANNELS = new Set(["daemon", "system", "build"]);

const COMMAND_URI_PREFIX_RE = /^(?:command|action|cmd|prompt|bringup|rtt):(?:\/\/)?/i;
const ORPHAN_OSC8_RE =
  /(?:\x1b\]|(?<![\w[\]])\])?8;;([^\s`"'<>)\x1b\x07\\]+)(?:\x1b\\|\x07|\\)?(.*?)(?:\x1b\]|(?<![\w[\]])\])?8;;(?:\x1b\\|\x07|\\)?/g;
const OSC8_BUTTON_TAG_RE =
  /<button type="button" class="osc8-link osc8-cmd-btn" data-osc-uri="([^"]*)">/g;
const HTML_TAG_SPLIT_RE = /(<[^>]+>)/g;
const URL_LINK_RE = /(https?:\/\/[^\s<>"')]+)/g;
const FILE_PATH_LINK_RE =
  /(?<![a-zA-Z0-9_./-])((?:\/(?:Users|home|root|workspace|[a-zA-Z0-9_.-]+)\/[a-zA-Z0-9_./-]+)|(?:(?:src|tools|board|platform|controller|peripheral|app|tests|docs|attachments|feedback|target)\/[a-zA-Z0-9_./-]+))(?::(\d+)(?::(\d+))?)?/g;
const TRAILING_PUNCT_CHARS = ".,;:?)'\"]";

/**
 * Extract the semantic command or prompt choice from an OSC-8 URI.
 *
 * @param {string} oscUri - Raw OSC-8 URI (e.g. `"command:pass"`, `"prompt://P"`).
 * @returns {string} Extracted command payload, or `""` when not a command URI.
 */
export function extractOscCommand(oscUri) {
  const raw = String(oscUri || "").trim();
  if (!raw) {
    return "";
  }
  if (
    raw.startsWith("http://") ||
    raw.startsWith("https://") ||
    raw.startsWith("file://") ||
    raw.startsWith("vscode://")
  ) {
    return "";
  }
  return raw.replace(COMMAND_URI_PREFIX_RE, "").trim();
}

/**
 * Collapse intra-line `\r` carriage-return progress overwrites while preserving `\n` newlines.
 *
 * @param {string} text - Raw terminal stream text.
 * @returns {string}
 */
function normalizeCarriageReturns(text) {
  const unified = text.replace(/\r\n/g, "\n");
  if (!unified.includes("\r")) {
    return unified;
  }
  return unified
    .split("\n")
    .map((line) => {
      if (!line.includes("\r")) {
        return line;
      }
      const segments = line.split("\r").filter(Boolean);
      return segments.length > 0 ? segments[segments.length - 1] : "";
    })
    .join("\n");
}

/**
 * Linkify plain-text `http(s)://` URLs and repository/absolute file paths inside an already HTML-escaped text node.
 *
 * @param {string} escapedSegment - HTML-escaped plain text outside interactive tags.
 * @returns {string} HTML string with safe `<a>` elements.
 */
function linkifyEscapedPlainSegment(escapedSegment) {
  const withUrls = escapedSegment.replace(
    URL_LINK_RE,
    (url) => `<a href="${url}" target="_blank" rel="noopener noreferrer" class="term-link">${url}</a>`,
  );

  const parts = withUrls.split(HTML_TAG_SPLIT_RE);
  let insideAnchor = false;
  for (let idx = 0; idx < parts.length; idx += 1) {
    const token = parts[idx];
    if (token.startsWith("<")) {
      const lower = token.toLowerCase();
      if (lower.startsWith("<a ")) {
        insideAnchor = true;
      } else if (lower.startsWith("</a")) {
        insideAnchor = false;
      }
      continue;
    }
    if (insideAnchor || !token) {
      continue;
    }
    parts[idx] = token.replace(FILE_PATH_LINK_RE, (fullMatch, rawPath, lineNum, colNum) => {
      let cleanPath = rawPath;
      let trailing = "";
      while (cleanPath && TRAILING_PUNCT_CHARS.includes(cleanPath[cleanPath.length - 1])) {
        trailing = cleanPath[cleanPath.length - 1] + trailing;
        cleanPath = cleanPath.slice(0, -1);
      }
      if (!cleanPath) {
        return fullMatch;
      }
      const absLike = cleanPath.startsWith("/") ? cleanPath : `/${cleanPath}`;
      const lineSuffix = lineNum ? `:${lineNum}${colNum ? `:${colNum}` : ""}` : "";
      const displayLabel = `${cleanPath}${lineSuffix}`;
      const vscodeHref = `vscode://file${absLike}${lineSuffix}`;
      return `<a href="${vscodeHref}" class="term-link term-file-link" data-path="${cleanPath}">${displayLabel}</a>${trailing}`;
    });
  }
  return parts.join("");
}

/**
 * Convert raw terminal output containing ANSI SGR colors, OSC-8 hyperlinks
 * (`\x1b]8;;command:pass\x1b\\[P]ass\x1b]8;;\x1b\\`), URLs, and file paths into safe HTML.
 *
 * @param {string} rawText - Raw terminal line or multi-line chunk.
 * @returns {string} Safe HTML fragment.
 */
export function renderTerminalHtml(rawText) {
  if (!rawText) {
    return "";
  }
  const crNormalized = normalizeCarriageReturns(String(rawText));
  const oscNormalized = crNormalized.replace(
    ORPHAN_OSC8_RE,
    (_m, url, label) => `\x1b]8;;${url}\x1b\\${label}\x1b]8;;\x1b\\`,
  );

  const ansiHtml = parseAnsiToHtml(oscNormalized).replace(
    OSC8_BUTTON_TAG_RE,
    (_tag, escapedUri) => {
      const unescapedUri = escapedUri
        .replace(/&quot;/g, '"')
        .replace(/&#039;/g, "'")
        .replace(/&gt;/g, ">")
        .replace(/&lt;/g, "<")
        .replace(/&amp;/g, "&");
      const cmd = extractOscCommand(unescapedUri);
      const safeCmd = escapeHtml(cmd);
      return (
        `<button type="button" class="osc8-link osc8-cmd-btn term-link term-cmd-link" ` +
        `data-osc-uri="${escapedUri}" data-command="${safeCmd}" title="Click to select ${safeCmd}">`
      );
    },
  );

  const tokens = ansiHtml.split(HTML_TAG_SPLIT_RE);
  let interactiveDepth = 0;
  for (let idx = 0; idx < tokens.length; idx += 1) {
    const tok = tokens[idx];
    if (tok.startsWith("<")) {
      const lower = tok.toLowerCase();
      if (lower.startsWith("<a ") || lower.startsWith("<button ")) {
        interactiveDepth += 1;
      } else if (lower.startsWith("</a") || lower.startsWith("</button")) {
        interactiveDepth = Math.max(0, interactiveDepth - 1);
      }
      continue;
    }
    if (interactiveDepth === 0 && tok) {
      tokens[idx] = linkifyEscapedPlainSegment(tok);
    }
  }
  return tokens.join("");
}

/**
 * Determine whether the user currently holds an active non-collapsed text selection inside `element`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {Element|null} element - Container element to check.
 * @returns {boolean}
 */
export function isSelectionInside(ctx, element) {
  if (!element) {
    return false;
  }
  const doc = ctx.getDocument();
  if (!doc || typeof doc.getSelection !== "function") {
    return false;
  }
  const sel = doc.getSelection();
  if (!sel || sel.isCollapsed || sel.rangeCount === 0) {
    return false;
  }
  if (typeof element.contains === "function") {
    if (element.contains(sel.anchorNode) || element.contains(sel.focusNode)) {
      return true;
    }
    if (typeof sel.getRangeAt === "function") {
      const range = sel.getRangeAt(0);
      if (range && element.contains(range.commonAncestorContainer)) {
        return true;
      }
    }
  }
  return false;
}

/**
 * Controller for the System / Target RTT & defmt Logging Monitor panel.
 */
export class RttStreamController {
  /**
   * Construct an `RttStreamController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Optional element overrides and callbacks.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.onCommandClick = options.onCommandClick || null;
    this.entries = [];
    this.lastLogId = 0;
    this.clearedUpToId = 0;
    this.includeSystemLogs = options.includeSystemLogs !== undefined ? Boolean(options.includeSystemLogs) : true;
    this.autoScrollLocked = true;
    this.isLogHidden = true;
    this.prevLogWidth = DEFAULT_LOG_PANEL_WIDTH;
    this._unsubscribers = [];

    this.logPanel = resolveElement(ctx, options.logPanel || "logPanel");
    this.splitter = resolveElement(ctx, options.splitter || "vSplitter");
    this.logView = resolveElement(ctx, options.logView || "logView");
    this.toggleBtn = resolveElement(ctx, options.toggleBtn || "btnToggleLog");
    this.collapseBtn = resolveElement(ctx, options.collapseBtn || "btnCollapseLog");
    this.copyBtn = resolveElement(ctx, options.copyBtn || "btnCopyLog");
    this.autoScrollBadge = resolveElement(ctx, options.autoScrollBadge || "logAutoScrollBadge");
  }

  /**
   * Bind DOM event listeners and initialize panel visibility from boot config & storage.
   *
   * @param {object} [initOptions] - `{ initialShowLogs }`.
   */
  init(initOptions = {}) {
    if (this.logView) {
      const onScroll = () => {
        const distanceToBottom =
          this.logView.scrollHeight - (this.logView.scrollTop + this.logView.clientHeight);
        const atBottom = distanceToBottom <= AUTO_SCROLL_THRESHOLD_PX;
        if (atBottom !== this.autoScrollLocked) {
          this.setAutoScroll(atBottom, { scrollNow: false });
        }
      };
      this.logView.addEventListener("scroll", onScroll);
      this._unsubscribers.push(() => this.logView.removeEventListener("scroll", onScroll));

      this._unsubscribers.push(
        bindDelegatedEvent(
          this.logView,
          "click",
          ".osc8-cmd-btn",
          (event, btn) => {
            event.preventDefault();
            const cmd =
              btn.getAttribute("data-command") ||
              extractOscCommand(btn.getAttribute("data-osc-uri") || "");
            if (cmd && typeof this.onCommandClick === "function") {
              this.onCommandClick(cmd);
            }
          },
        ),
      );

      this._unsubscribers.push(
        bindDelegatedEvent(this.logView, "click", ".term-file-link", (event, anchor) => {
          const href = anchor.getAttribute("href");
          if (href && href.startsWith("vscode://")) {
            event.preventDefault();
            this.ctx.openExternalUrl(href, "_self");
          }
        }),
      );
    }

    if (this.autoScrollBadge) {
      const onBadgeClick = () => this.setAutoScroll(!this.autoScrollLocked);
      this.autoScrollBadge.addEventListener("click", onBadgeClick);
      this._unsubscribers.push(() =>
        this.autoScrollBadge.removeEventListener("click", onBadgeClick),
      );
    }

    const storedShow = this.ctx.getStorage().getItem(SHOW_SYSTEM_LOGS_STORAGE_KEY);
    const shouldShow =
      storedShow !== null ? storedShow === "true" : Boolean(initOptions.initialShowLogs);
    if (shouldShow) {
      this.togglePanel(true);
    }
  }

  /**
   * Return whether a log entry passes active channel filtering.
   *
   * @param {object} entry - Log entry `{ channel, level, message }`.
   * @returns {boolean}
   */
  _matchesFilter(entry) {
    if (this.includeSystemLogs) {
      return true;
    }
    const ch = String(entry.channel || "daemon").toLowerCase();
    return !SYSTEM_LOG_CHANNELS.has(ch);
  }

  /**
   * Enable or disable inclusion of host daemon/system/build logs alongside target RTT/defmt logs.
   *
   * @param {boolean} include - True to show all channels; false for target/RTT channels only.
   */
  setIncludeSystemLogs(include) {
    const next = Boolean(include);
    if (this.includeSystemLogs === next) {
      return;
    }
    this.includeSystemLogs = next;
    this._rerenderAll();
  }

  /**
   * Return currently visible log entries after filtering.
   *
   * @returns {Array<object>}
   */
  getVisibleEntries() {
    return this.entries.filter((entry) => this._matchesFilter(entry));
  }

  /**
   * Update auto-scroll lock state and synchronize header indicator text.
   *
   * @param {boolean} locked - True to lock scroll to bottom on incoming logs.
   * @param {object} [options] - `{ scrollNow: boolean }`.
   */
  setAutoScroll(locked, options = {}) {
    this.autoScrollLocked = Boolean(locked);
    if (this.autoScrollBadge) {
      this.autoScrollBadge.textContent = this.autoScrollLocked
        ? "AUTO-SCROLL: ON"
        : "AUTO-SCROLL: PAUSED";
    }
    if (this.autoScrollLocked && options.scrollNow !== false && this.logView) {
      this.logView.scrollTop = this.logView.scrollHeight;
    }
  }

  /**
   * Return whether auto-scroll is currently locked to bottom.
   *
   * @returns {boolean}
   */
  isAutoScrollLocked() {
    return this.autoScrollLocked;
  }

  /**
   * Show or hide the left `#logPanel` and `#vSplitter`, persisting preference to storage.
   *
   * @param {boolean} [forceShow] - Optional explicit visibility boolean.
   * @returns {boolean} Resulting visibility (`true` when shown).
   */
  togglePanel(forceShow) {
    if (typeof forceShow === "boolean") {
      this.isLogHidden = !forceShow;
    } else {
      this.isLogHidden = !this.isLogHidden;
    }

    if (this.logPanel) {
      if (this.isLogHidden) {
        this.prevLogWidth = this.logPanel.style.width || DEFAULT_LOG_PANEL_WIDTH;
        this.logPanel.style.display = "none";
      } else {
        this.logPanel.style.display = "flex";
        this.logPanel.style.width = this.prevLogWidth;
      }
    }

    if (this.splitter) {
      this.splitter.style.display = this.isLogHidden ? "none" : "flex";
    }

    if (this.toggleBtn) {
      this.toggleBtn.textContent = this.isLogHidden ? "Show Log" : "Hide Log";
      this.toggleBtn.classList.toggle("btn-amber", this.isLogHidden);
    }

    if (this.collapseBtn) {
      this.collapseBtn.textContent = this.isLogHidden ? "▶ Show" : "◀ Hide";
    }

    this.ctx.getStorage().setItem(SHOW_SYSTEM_LOGS_STORAGE_KEY, String(!this.isLogHidden));
    return !this.isLogHidden;
  }

  /**
   * Return whether the system logging panel is currently visible.
   *
   * @returns {boolean}
   */
  isPanelVisible() {
    return !this.isLogHidden;
  }

  /**
   * Construct a single `.log-line` DOM element for `entry`.
   *
   * @param {object} entry - `{ id, level, channel, message }`.
   * @returns {HTMLElement}
   */
  _buildLogLineElement(entry) {
    const channel = String(entry.channel || "daemon");
    const levelLower = String(entry.level || "info").toLowerCase();
    const cssSuffix =
      channel === "rtt" || channel === "defmt" || channel === "target"
        ? levelLower === "error" || levelLower === "warn"
          ? levelLower
          : "target"
        : levelLower;
    const lineEl = createEl(this.ctx, "div", {
      className: `log-line log-line-${cssSuffix}`,
      dataset: {
        logId: entry.id ?? "",
        channel,
      },
    });
    lineEl.innerHTML = renderTerminalHtml(`[${channel}] ${entry.message ?? ""}`);
    return lineEl;
  }

  /**
   * Rebuild `#logView` children from `this.entries`.
   */
  _rerenderAll() {
    if (!this.logView) {
      return;
    }
    clearChildren(this.logView);
    for (const entry of this.entries) {
      if (this._matchesFilter(entry)) {
        this.logView.appendChild(this._buildLogLineElement(entry));
      }
    }
    if (this.autoScrollLocked) {
      this.logView.scrollTop = this.logView.scrollHeight;
    }
  }

  /**
   * Ingest a snapshot array of log entries from `/api/logs` incrementally.
   *
   * @param {Array<object>} logs - Array of `{ id, timestamp, level, channel, message }`.
   */
  ingestLogs(logs) {
    if (!Array.isArray(logs)) {
      return;
    }
    if (this.logView && isSelectionInside(this.ctx, this.logView)) {
      return;
    }

    if (logs.length === 0) {
      if (this.lastLogId !== 0 || this.entries.length > 0) {
        this.entries = [];
        this.lastLogId = 0;
        this.clearedUpToId = 0;
        clearChildren(this.logView);
      }
      return;
    }

    const normalized = logs.map((item, idx) => ({
      ...item,
      id: item.id ?? idx + 1,
    }));
    const firstId = normalized[0].id;
    const lastId = normalized[normalized.length - 1].id;

    // Detect server restart (sequence ID reset below previous high-water mark)
    if (lastId < this.lastLogId) {
      this.lastLogId = 0;
      this.clearedUpToId = 0;
      this.entries = [];
    }

    const postClearLogs =
      this.clearedUpToId > 0 ? normalized.filter((item) => item.id > this.clearedUpToId) : normalized;

    if (postClearLogs.length === 0) {
      this.lastLogId = lastId;
      return;
    }

    if (this.lastLogId === 0 || firstId > this.lastLogId) {
      this.entries = postClearLogs;
      this.lastLogId = lastId;
      this._rerenderAll();
      return;
    }

    if (lastId > this.lastLogId) {
      const appended = postClearLogs.filter((item) => item.id > this.lastLogId);
      this.entries.push(...appended);
      this.lastLogId = lastId;
      if (this.logView) {
        for (const item of appended) {
          if (this._matchesFilter(item)) {
            this.logView.appendChild(this._buildLogLineElement(item));
          }
        }
        if (this.autoScrollLocked) {
          this.logView.scrollTop = this.logView.scrollHeight;
        }
      }
    }
  }

  /**
   * Poll `GET /api/logs` and update the streaming log panel.
   *
   * @returns {Promise<void>}
   */
  async pollLogs() {
    try {
      const logs = await this.ctx.fetchJson("/api/logs");
      this.ingestLogs(logs);
    } catch {
      // Transient background poll errors are non-fatal while daemon restarts.
    }
  }

  /**
   * Clear buffered log lines from the UI and suppress already-seen log IDs.
   */
  clear() {
    this.clearedUpToId = this.lastLogId;
    this.entries = [];
    clearChildren(this.logView);
  }

  /**
   * Return plain-text representation of buffered visible logs (ANSI stripped).
   *
   * @returns {string}
   */
  getPlainTextLogs() {
    return this.getVisibleEntries()
      .map((entry) => stripAnsi(`[${entry.channel || "daemon"}] ${entry.message || ""}`))
      .join("\n");
  }

  /**
   * Copy visible log text to the system clipboard and display button feedback.
   *
   * @param {Element|null} [buttonEl] - Optional trigger button for visual feedback.
   * @returns {Promise<boolean>}
   */
  async copyLogs(buttonEl = this.copyBtn) {
    const text =
      this.entries.length > 0
        ? this.getPlainTextLogs()
        : (this.logView && this.logView.textContent) || "";
    const ok = await this.ctx.copyTextToClipboard(text);
    if (buttonEl) {
      const orig = buttonEl.textContent;
      buttonEl.textContent = ok ? "✓ Copied!" : "Error";
      this.ctx.setTimeout(() => {
        buttonEl.textContent = orig;
      }, COPY_FEEDBACK_DURATION_MS);
    }
    return ok;
  }

  /**
   * Unbind all registered DOM listeners.
   */
  destroy() {
    for (const unsub of this._unsubscribers) {
      unsub();
    }
    this._unsubscribers = [];
  }
}

/**
 * Factory creating an `RttStreamController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {RttStreamController}
 */
export function createRttStream(ctx, options = {}) {
  return new RttStreamController(ctx, options);
}
