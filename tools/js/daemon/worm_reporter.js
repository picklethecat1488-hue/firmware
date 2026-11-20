/**
 * Target Defect Discovery (`WORM-*`) Bug Reporter & Attachment Bundler.
 *
 * Synthesizes executable `python tools/host.py ...` reproduction steps from
 * recent target CLI/FS session history, bundles RTT/defmt system logs,
 * `host_fs` crash logs, telemetry, and metadata flags, and encodes drag-drop,
 * file-input, and clipboard screenshot/log attachments in pure ES2024 without
 * browser globals.
 */

import { bindDelegatedEvent, clearChildren, createEl, resolveElement } from "../core/dom.js";
import { bindModalBackdropClose, hideModal, showModal } from "../core/modal.js";
import { showToast } from "../core/toast.js";

const BASE64_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const MAX_REPRO_COMMANDS = 6;
const FOCUS_DELAY_MS = 50;
const AUTO_CLOSE_SUCCESS_MS = 1500;
const BYTES_PER_KILOBYTE = 1024;
const IMAGE_EXT_RE = /\.(?:png|jpe?g|gif|webp|svg|bmp)$/i;
const LOG_EXT_RE = /\.(?:log|txt|trace)$/i;

/**
 * Encode a `Uint8Array` into a standard RFC 4648 Base64 string using pure ES2024 operations.
 *
 * @param {Uint8Array} bytes - Raw binary payload.
 * @returns {string} Base64-encoded string.
 */
export function bytesToBase64(bytes) {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0) {
    return "";
  }
  const chunks = [];
  const len = bytes.length;
  let idx = 0;
  while (idx + 2 < len) {
    const triplet = (bytes[idx] << 16) | (bytes[idx + 1] << 8) | bytes[idx + 2];
    chunks.push(
      BASE64_ALPHABET[(triplet >> 18) & 63] +
        BASE64_ALPHABET[(triplet >> 12) & 63] +
        BASE64_ALPHABET[(triplet >> 6) & 63] +
        BASE64_ALPHABET[triplet & 63],
    );
    idx += 3;
  }
  const rem = len - idx;
  if (rem === 1) {
    const pair = bytes[idx] << 16;
    chunks.push(
      `${BASE64_ALPHABET[(pair >> 18) & 63]}${BASE64_ALPHABET[(pair >> 12) & 63]}==`,
    );
  } else if (rem === 2) {
    const pair = (bytes[idx] << 16) | (bytes[idx + 1] << 8);
    chunks.push(
      `${BASE64_ALPHABET[(pair >> 18) & 63]}${BASE64_ALPHABET[(pair >> 12) & 63]}${BASE64_ALPHABET[(pair >> 6) & 63]}=`,
    );
  }
  return chunks.join("");
}

/**
 * Encode a UTF-8 JavaScript string into a `Uint8Array` using pure ES2024 code-point math.
 *
 * @param {string} text - Input string.
 * @returns {Uint8Array}
 */
export function utf8ToBytes(text) {
  const str = String(text ?? "");
  const out = [];
  for (let idx = 0; idx < str.length; idx += 1) {
    let cp = str.codePointAt(idx);
    if (cp > 0xffff) {
      idx += 1;
    }
    if (cp <= 0x7f) {
      out.push(cp);
    } else if (cp <= 0x7ff) {
      out.push(0xc0 | (cp >> 6), 0x80 | (cp & 0x3f));
    } else if (cp <= 0xffff) {
      out.push(0xe0 | (cp >> 12), 0x80 | ((cp >> 6) & 0x3f), 0x80 | (cp & 0x3f));
    } else {
      out.push(
        0xf0 | (cp >> 18),
        0x80 | ((cp >> 12) & 0x3f),
        0x80 | ((cp >> 6) & 0x3f),
        0x80 | (cp & 0x3f),
      );
    }
  }
  return new Uint8Array(out);
}

/**
 * Synthesize numbered `python tools/host.py ...` reproduction steps from session history and target config.
 *
 * @param {Array<string>} [commands] - Recent CLI/FS commands executed against target.
 * @param {object} [targetInfo] - `{ package, binName, bin, release, mode }`.
 * @returns {string} Multi-line numbered reproduction steps.
 */
export function buildReproSteps(commands = [], targetInfo = {}) {
  const pkg = targetInfo.package || "cat_detector";
  const bin = targetInfo.binName || targetInfo.bin || "cat_detector_shell";
  const releaseFlag = targetInfo.release ? " --release" : "";
  const subcommand = String(targetInfo.mode || "CLI").toUpperCase() === "FS" ? "fs" : "cli";

  const lines = [
    `1. Launch host daemon & synchronize firmware: python tools/host.py --package ${pkg} --bin ${bin}${releaseFlag}`,
  ];

  const cleanCmds = Array.isArray(commands)
    ? commands
        .map((c) => String(c || "").trim())
        .filter(Boolean)
        .slice(-MAX_REPRO_COMMANDS)
    : [];

  if (cleanCmds.length > 0) {
    cleanCmds.forEach((cmd, idx) => {
      lines.push(`${idx + 2}. Execute command: python tools/host.py ${subcommand} ${cmd}`);
    });
    lines.push(`${cleanCmds.length + 2}. Observe defect/failure on target`);
  } else {
    lines.push(`2. Execute target command via python tools/host.py ${subcommand} <command>`);
    lines.push("3. Observe defect/failure on target");
  }

  return lines.join("\n");
}

/**
 * Classify attachment `file_type` (`"screenshot"`, `"log"`, or `"document"`) from filename and MIME type.
 *
 * @param {string} filename - File basename.
 * @param {string} mimeType - MIME type string.
 * @returns {"screenshot"|"log"|"document"}
 */
function classifyAttachmentType(filename, mimeType) {
  const mime = String(mimeType || "").toLowerCase();
  const name = String(filename || "");
  if (mime.startsWith("image/") || IMAGE_EXT_RE.test(name)) {
    return "screenshot";
  }
  if (mime === "text/plain" || LOG_EXT_RE.test(name)) {
    return "log";
  }
  return "document";
}

/**
 * Convert a browser/mock `File` or `Blob`-like object into a `{ dataUrl, sizeBytes }` pair.
 *
 * @param {object} fileObj - `File`, `Blob`, or mock file descriptor.
 * @returns {Promise<{dataUrl: string, sizeBytes: number}>}
 */
async function encodeFileToDataUrl(fileObj) {
  if (typeof fileObj.dataUrl === "string" && fileObj.dataUrl) {
    return {
      dataUrl: fileObj.dataUrl,
      sizeBytes: Number(fileObj.size) || fileObj.dataUrl.length,
    };
  }
  if (typeof fileObj.content_base64 === "string" && fileObj.content_base64) {
    const raw = fileObj.content_base64;
    const mime = fileObj.type || "application/octet-stream";
    const dataUrl = raw.startsWith("data:") ? raw : `data:${mime};base64,${raw}`;
    return {
      dataUrl,
      sizeBytes: Number(fileObj.size) || raw.length,
    };
  }
  if (typeof fileObj.arrayBuffer === "function") {
    const buf = await fileObj.arrayBuffer();
    const bytes = new Uint8Array(buf);
    const mime = fileObj.type || "application/octet-stream";
    return {
      dataUrl: `data:${mime};base64,${bytesToBase64(bytes)}`,
      sizeBytes: bytes.byteLength,
    };
  }
  if (typeof fileObj.text === "function") {
    const text = await fileObj.text();
    const bytes = utf8ToBytes(text);
    const mime = fileObj.type || "text/plain";
    return {
      dataUrl: `data:${mime};base64,${bytesToBase64(bytes)}`,
      sizeBytes: bytes.byteLength,
    };
  }
  return { dataUrl: "", sizeBytes: 0 };
}

/**
 * Controller for the Target Bug (`WORM-*`) Discovery modal.
 */
export class WormReporterController {
  /**
   * Construct a `WormReporterController`.
   *
   * @param {object} ctx - Explicit `RuntimeContext`.
   * @param {object} [options] - Accessors for target config and CLI command history.
   */
  constructor(ctx, options = {}) {
    this.ctx = ctx;
    this.getTargetConfig =
      options.getTargetConfig ||
      (() => ({
        package: "cat_detector",
        binName: "cat_detector_shell",
        release: false,
        mode: "CLI",
      }));
    this.getCommandHistory = options.getCommandHistory || (() => []);
    this.onWormCreated = options.onWormCreated || null;

    this.attachments = [];
    this._unsubscribers = [];

    this.openBtn = resolveElement(ctx, options.openBtn || "btnTargetBug");
    this.modal = resolveElement(ctx, options.modal || "targetBugModal");
    this.closeBtn = resolveElement(ctx, options.closeBtn || "btnCloseTargetBugModal");
    this.cancelBtn = resolveElement(ctx, options.cancelBtn || "btnCancelTargetBug");
    this.submitBtn = resolveElement(ctx, options.submitBtn || "btnSubmitTargetBug");

    this.titleInput = resolveElement(ctx, options.titleInput || "targetBugTitle");
    this.severitySelect = resolveElement(ctx, options.severitySelect || "targetBugSeverity");
    this.categorySelect = resolveElement(ctx, options.categorySelect || "targetBugCategory");
    this.componentInput = resolveElement(ctx, options.componentInput || "targetBugComponent");
    this.descInput = resolveElement(ctx, options.descInput || "targetBugDescription");
    this.stepsInput = resolveElement(ctx, options.stepsInput || "targetBugSteps");
    this.expectedInput = resolveElement(ctx, options.expectedInput || "targetBugExpected");
    this.actualInput = resolveElement(ctx, options.actualInput || "targetBugActual");

    this.attachSystemLogs = resolveElement(ctx, options.attachSystemLogs || "attachSystemLogs");
    this.attachCrashLog = resolveElement(ctx, options.attachCrashLog || "attachCrashLog");
    this.attachTelemetry = resolveElement(ctx, options.attachTelemetry || "attachTelemetry");
    this.attachMetadata = resolveElement(ctx, options.attachMetadata || "attachMetadata");
    this.attachTestExecLogsRow = resolveElement(
      ctx,
      options.attachTestExecLogsRow || "attachTestExecLogsRow",
    );
    this.attachTestExecutionLogs = resolveElement(
      ctx,
      options.attachTestExecutionLogs || "attachTestExecutionLogs",
    );

    this.pasteBtn = resolveElement(ctx, options.pasteBtn || "btnPasteTargetBugClipboard");
    this.dropzone = resolveElement(ctx, options.dropzone || "targetBugDropzone");
    this.fileInput = resolveElement(ctx, options.fileInput || "targetBugFileInput");
    this.attachmentsGrid = resolveElement(
      ctx,
      options.attachmentsGrid || "targetBugAttachmentsGrid",
    );

    this.progressContainer = resolveElement(
      ctx,
      options.progressContainer || "targetBugProgressContainer",
    );
    this.progressBar = resolveElement(ctx, options.progressBar || "targetBugProgressBar");
    this.progressMsg = resolveElement(ctx, options.progressMsg || "targetBugProgressMsg");
    this.progressPct = resolveElement(ctx, options.progressPct || "targetBugProgressPct");
    this.statusMsg = resolveElement(ctx, options.statusMsg || "targetBugStatusMsg");
  }

  /**
   * Bind modal buttons, drag-and-drop upload listeners, and clipboard paste handling.
   */
  init() {
    const bindClick = (el, fn) => {
      if (!el) {
        return;
      }
      el.addEventListener("click", fn);
      this._unsubscribers.push(() => el.removeEventListener("click", fn));
    };

    bindClick(this.openBtn, () => this.openModal());
    bindClick(this.closeBtn, () => this.closeModal());
    bindClick(this.cancelBtn, () => this.closeModal());
    bindClick(this.submitBtn, () => this.submit());
    bindClick(this.pasteBtn, () => this.pasteClipboard());

    if (this.dropzone) {
      const onDropzoneClick = () => {
        if (this.fileInput) {
          this.fileInput.click();
        }
      };
      const onDragOver = (event) => {
        event.preventDefault();
        this.dropzone.classList.add("dragover");
      };
      const onDragLeave = (event) => {
        event.preventDefault();
        this.dropzone.classList.remove("dragover");
      };
      const onDrop = (event) => {
        event.preventDefault();
        this.dropzone.classList.remove("dragover");
        if (event.dataTransfer && event.dataTransfer.files) {
          this.addFiles(event.dataTransfer.files);
        }
      };

      this.dropzone.addEventListener("click", onDropzoneClick);
      this.dropzone.addEventListener("dragover", onDragOver);
      this.dropzone.addEventListener("dragleave", onDragLeave);
      this.dropzone.addEventListener("drop", onDrop);
      this._unsubscribers.push(() => {
        this.dropzone.removeEventListener("click", onDropzoneClick);
        this.dropzone.removeEventListener("dragover", onDragOver);
        this.dropzone.removeEventListener("dragleave", onDragLeave);
        this.dropzone.removeEventListener("drop", onDrop);
      });
    }

    if (this.fileInput) {
      const onFileChange = () => {
        if (this.fileInput.files && this.fileInput.files.length > 0) {
          this.addFiles(this.fileInput.files);
          this.fileInput.value = "";
        }
      };
      this.fileInput.addEventListener("change", onFileChange);
      this._unsubscribers.push(() =>
        this.fileInput.removeEventListener("change", onFileChange),
      );
    }

    if (this.attachmentsGrid) {
      this._unsubscribers.push(
        bindDelegatedEvent(
          this.attachmentsGrid,
          "click",
          "[data-remove-attachment-idx]",
          (_evt, btn) => {
            const idx = Number.parseInt(btn.getAttribute("data-remove-attachment-idx"), 10);
            if (Number.isFinite(idx)) {
              this.removeAttachment(idx);
            }
          },
        ),
      );
    }

    const onViewportPaste = (event) => {
      if (!this.isModalOpen()) {
        return;
      }
      this.handlePasteEvent(event);
    };
    this.ctx.addViewportListener("paste", onViewportPaste);
    this._unsubscribers.push(() =>
      this.ctx.removeViewportListener("paste", onViewportPaste),
    );

    bindModalBackdropClose(this.ctx, this.modal, () => this.closeModal());
  }

  /**
   * Return whether `#targetBugModal` is currently open.
   *
   * @returns {boolean}
   */
  isModalOpen() {
    return Boolean(this.modal && this.modal.style.display !== "none" && this.modal.style.display !== "");
  }

  /**
   * Open `#targetBugModal`, clear prior report fields, and synthesize fresh `python tools/host.py` repro steps.
   *
   * @returns {Promise<void>}
   */
  async openModal() {
    if (!this.modal) {
      return;
    }
    this.attachments = [];
    this.renderAttachments();

    if (this.titleInput) this.titleInput.value = "";
    if (this.descInput) this.descInput.value = "";
    if (this.expectedInput) this.expectedInput.value = "";
    if (this.actualInput) this.actualInput.value = "";

    const targetInfo = this.getTargetConfig();
    const isTestMode = String(targetInfo.mode || "CLI").toUpperCase() === "TEST";
    if (this.attachTestExecLogsRow) {
      this.attachTestExecLogsRow.style.display = isTestMode ? "flex" : "none";
    }
    if (this.attachTestExecutionLogs) {
      this.attachTestExecutionLogs.checked = isTestMode;
    }

    const localCmds = this.getCommandHistory();
    if (this.stepsInput) {
      this.stepsInput.value = buildReproSteps(localCmds, targetInfo);
    }

    if (this.statusMsg) {
      this.statusMsg.style.display = "none";
      this.statusMsg.textContent = "";
    }
    if (this.progressContainer) {
      this.progressContainer.style.display = "none";
    }
    if (this.progressBar) {
      this.progressBar.style.width = "0%";
      this.progressBar.style.background =
        "linear-gradient(90deg, var(--xerxes-teal), var(--xerxes-cyan))";
    }

    showModal(this.ctx, this.modal, "flex");

    if (this.titleInput) {
      this.ctx.setTimeout(() => this.titleInput.focus(), FOCUS_DELAY_MS);
    }

    try {
      const data = await this.ctx.fetchJson("/api/cli/history");
      if (data && Array.isArray(data.history) && data.history.length > 0 && this.stepsInput) {
        this.stepsInput.value = buildReproSteps(data.history, this.getTargetConfig());
      }
    } catch {
      // Retain locally synthesized steps when backend history is unavailable.
    }
  }

  /**
   * Close `#targetBugModal`.
   */
  closeModal() {
    hideModal(this.ctx, this.modal);
  }

  /**
   * Attach one or more `File` / `Blob` items and update the preview grid.
   *
   * @param {Iterable<object>} files - File list from dropzone or `<input type="file">`.
   * @returns {Promise<Array<object>>}
   */
  async addFiles(files) {
    if (!files) {
      return this.attachments;
    }
    const items = Array.from(files);
    for (const file of items) {
      if (!file) {
        continue;
      }
      const filename = file.name || `attachment_${Math.round(this.ctx.nowMs())}.bin`;
      const fileType = classifyAttachmentType(filename, file.type || "");
      const { dataUrl, sizeBytes } = await encodeFileToDataUrl(file);
      this.attachments.push({
        filename,
        file_type: fileType,
        content_base64: dataUrl,
        size_bytes: sizeBytes,
        description: file.description || `Attached ${fileType} (${filename})`,
      });
    }
    this.renderAttachments();
    return this.attachments;
  }

  /**
   * Attach a plain-text log/clipboard snippet as a `.txt` log attachment.
   *
   * @param {string} text - Text snippet content.
   * @param {string} [filenamePrefix] - Filename prefix (`"clipboard"` default).
   * @returns {object|null} Created attachment descriptor.
   */
  addTextSnippet(text, filenamePrefix = "clipboard") {
    const trimmed = String(text || "").trim();
    if (!trimmed) {
      return null;
    }
    const bytes = utf8ToBytes(text);
    const dataUrl = `data:text/plain;base64,${bytesToBase64(bytes)}`;
    const filename = `${filenamePrefix}_${Math.round(this.ctx.nowMs())}.txt`;
    const entry = {
      filename,
      file_type: "log",
      content_base64: dataUrl,
      content_text: String(text),
      size_bytes: bytes.byteLength,
      description: "Pasted clipboard text snippet",
    };
    this.attachments.push(entry);
    this.renderAttachments();
    return entry;
  }

  /**
   * Handle a DOM `paste` event while `#targetBugModal` is open.
   *
   * @param {ClipboardEvent|object} event - Paste event with `clipboardData`.
   * @returns {Promise<boolean>} True if an image or text attachment was captured.
   */
  async handlePasteEvent(event) {
    const clip = event && event.clipboardData;
    if (!clip) {
      return false;
    }

    if (clip.items) {
      for (const item of Array.from(clip.items)) {
        if (item && typeof item.type === "string" && item.type.startsWith("image/")) {
          if (typeof event.preventDefault === "function") {
            event.preventDefault();
          }
          const blob = typeof item.getAsFile === "function" ? item.getAsFile() : null;
          if (blob) {
            const namedFile = {
              name: blob.name || `screenshot_${Math.round(this.ctx.nowMs())}.png`,
              type: item.type,
              size: blob.size,
              arrayBuffer: blob.arrayBuffer ? () => blob.arrayBuffer() : undefined,
              dataUrl: blob.dataUrl,
              description: "Pasted screenshot",
            };
            await this.addFiles([namedFile]);
            return true;
          }
        }
      }
    }

    return false;
  }

  /**
   * Read image or text from the system clipboard when clicking `#btnPasteTargetBugClipboard`.
   *
   * @returns {Promise<boolean>}
   */
  async pasteClipboard() {
    if (typeof this.ctx.readClipboardAttachment === "function") {
      const custom = await this.ctx.readClipboardAttachment();
      if (custom) {
        if (custom.text) {
          this.addTextSnippet(custom.text);
        } else {
          await this.addFiles([custom]);
        }
        return true;
      }
    }

    const doc = this.ctx.getDocument();
    const view = doc && doc.defaultView ? doc.defaultView : null;
    const clipboard = view && view.navigator ? view.navigator.clipboard : null;
    if (!clipboard) {
      showToast(this.ctx, "Use Cmd/Ctrl+V to paste screenshots into this modal.", "info");
      return false;
    }

    try {
      if (typeof clipboard.read === "function") {
        const items = await clipboard.read();
        for (const item of items) {
          const imgType = (item.types || []).find((t) => t.startsWith("image/"));
          if (imgType) {
            const blob = await item.getType(imgType);
            await this.addFiles([
              {
                name: `screenshot_${Math.round(this.ctx.nowMs())}.png`,
                type: imgType,
                size: blob.size,
                arrayBuffer: () => blob.arrayBuffer(),
                description: "Pasted clipboard screenshot",
              },
            ]);
            return true;
          }
        }
      }

      if (typeof clipboard.readText === "function") {
        const text = await clipboard.readText();
        if (this.addTextSnippet(text) !== null) {
          return true;
        }
      }
    } catch {
      showToast(this.ctx, "Clipboard permission denied — press Cmd/Ctrl+V instead.", "warning");
    }
    return false;
  }

  /**
   * Remove an attachment card by index.
   *
   * @param {number} index - Zero-based attachment index.
   */
  removeAttachment(index) {
    if (index >= 0 && index < this.attachments.length) {
      this.attachments.splice(index, 1);
      this.renderAttachments();
    }
  }

  /**
   * Return a copy of queued manual attachments.
   *
   * @returns {Array<object>}
   */
  getAttachments() {
    return [...this.attachments];
  }

  /**
   * Render attachment preview cards inside `#targetBugAttachmentsGrid`.
   */
  renderAttachments() {
    if (!this.attachmentsGrid) {
      return;
    }
    clearChildren(this.attachmentsGrid);
    this.attachments.forEach((att, idx) => {
      const isImage =
        att.file_type === "screenshot" ||
        (typeof att.content_base64 === "string" && att.content_base64.startsWith("data:image/"));

      const previewNode = isImage
        ? createEl(this.ctx, "img", {
            attrs: { src: att.content_base64 || "", alt: att.filename || "screenshot" },
            style: {
              width: "100%",
              height: "65px",
              objectFit: "cover",
              borderRadius: "2px",
              border: "1px solid #092430",
              background: "#02070a",
            },
          })
        : createEl(this.ctx, "div", {
            style: {
              width: "100%",
              height: "65px",
              borderRadius: "2px",
              border: "1px solid #092430",
              background: "#02070a",
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              gap: "2px",
            },
            children: [
              createEl(this.ctx, "span", { text: "📄", style: { fontSize: "16px" } }),
              createEl(this.ctx, "span", {
                text: String(att.file_type || "DOCUMENT").toUpperCase(),
                style: {
                  fontSize: "7px",
                  color: "var(--xerxes-text-muted)",
                  fontWeight: "bold",
                },
              }),
            ],
          });

      const sizeKb = ((Number(att.size_bytes) || 0) / BYTES_PER_KILOBYTE).toFixed(1);
      const card = createEl(this.ctx, "div", {
        style: {
          background: "#040e14",
          border: "1px solid var(--xerxes-border-mid)",
          borderRadius: "3px",
          padding: "6px",
          display: "flex",
          flexDirection: "column",
          gap: "4px",
        },
        children: [
          previewNode,
          createEl(this.ctx, "div", {
            text: att.filename || "",
            attrs: { title: att.filename || "" },
            style: {
              fontSize: "8.5px",
              color: "#e0f8ff",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            },
          }),
          createEl(this.ctx, "div", {
            style: {
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
              fontSize: "7.5px",
              color: "var(--xerxes-text-dim)",
            },
            children: [
              createEl(this.ctx, "span", { text: `${sizeKb} KB` }),
              createEl(this.ctx, "button", {
                className: "btn",
                text: "✕",
                attrs: { type: "button" },
                dataset: { removeAttachmentIdx: String(idx) },
                style: {
                  fontSize: "7.5px",
                  padding: "1px 4px",
                  color: "var(--xerxes-red)",
                  borderColor: "rgba(255,42,75,0.4)",
                },
              }),
            ],
          }),
        ],
      });

      this.attachmentsGrid.appendChild(card);
    });
  }

  /**
   * Construct the `/api/target_bug` submission payload from current form state.
   *
   * @returns {object}
   */
  buildPayload() {
    const targetInfo = this.getTargetConfig();
    const currentMode = String(targetInfo.mode || "CLI").toUpperCase();
    const isTestMode = currentMode === "TEST";
    const includeTestExecLogs = isTestMode
      ? this.attachTestExecutionLogs
        ? Boolean(this.attachTestExecutionLogs.checked)
        : true
      : false;

    return {
      title: this.titleInput ? this.titleInput.value.trim() : "",
      severity: (this.severitySelect && this.severitySelect.value) || "MEDIUM",
      category: (this.categorySelect && this.categorySelect.value) || "FIRMWARE",
      component: (this.componentInput && this.componentInput.value.trim()) || "target",
      description: (this.descInput && this.descInput.value.trim()) || "",
      reproduction_steps: (this.stepsInput && this.stepsInput.value.trim()) || "",
      expected_behavior: (this.expectedInput && this.expectedInput.value.trim()) || "",
      actual_behavior: (this.actualInput && this.actualInput.value.trim()) || "",
      mode: currentMode,
      attach_system_logs: this.attachSystemLogs ? Boolean(this.attachSystemLogs.checked) : true,
      attach_test_execution_logs: includeTestExecLogs,
      attach_crash_log: this.attachCrashLog ? Boolean(this.attachCrashLog.checked) : false,
      attach_telemetry: this.attachTelemetry ? Boolean(this.attachTelemetry.checked) : false,
      attach_metadata: this.attachMetadata ? Boolean(this.attachMetadata.checked) : false,
      manual_attachments: [...this.attachments],
    };
  }

  /**
   * Update progress indicator inside `#targetBugModal`.
   *
   * @param {number} pct - Progress percentage `[0..100]`.
   * @param {string} message - Status text.
   */
  _setProgress(pct, message) {
    if (this.progressContainer) {
      this.progressContainer.style.display = "block";
    }
    if (this.progressBar) {
      this.progressBar.style.width = `${pct}%`;
    }
    if (this.progressPct) {
      this.progressPct.textContent = `${pct}%`;
    }
    if (this.progressMsg) {
      this.progressMsg.textContent = message;
    }
  }

  /**
   * Display a status/error banner in `#targetBugStatusMsg`.
   *
   * @param {string} text - Banner message.
   * @param {boolean} isError - True for red error styling, false for teal success styling.
   */
  _showStatusBanner(text, isError) {
    if (!this.statusMsg) {
      return;
    }
    this.statusMsg.style.display = "block";
    this.statusMsg.style.background = isError
      ? "rgba(255, 42, 75, 0.15)"
      : "rgba(20, 217, 196, 0.15)";
    this.statusMsg.style.color = isError ? "var(--xerxes-red)" : "var(--xerxes-teal)";
    this.statusMsg.textContent = text;
  }

  /**
   * Validate fields and submit the `WORM-*` defect report to `POST /api/target_bug`.
   *
   * @returns {Promise<object|null>}
   */
  async submit() {
    const payload = this.buildPayload();
    if (!payload.title) {
      this._showStatusBanner("Error: Bug title is required.", true);
      return null;
    }

    if (this.submitBtn) {
      this.submitBtn.disabled = true;
      this.submitBtn.textContent = "⏳ Filing Bug Report...";
    }

    this._setProgress(20, "Collecting system logs & target hardware context...");

    try {
      this._setProgress(45, "Packaging defect attachments & telemetry data...");
      const res = await this.ctx.postJson("/api/target_bug", payload);
      this._setProgress(85, "Registering worm in tracking matrix...");

      if (res && res.success) {
        const successText = `✓ Created defect report ${res.worm_id} with ${res.attachments_count} attachments.`;
        this._setProgress(100, successText);
        this._showStatusBanner(successText, false);
        showToast(this.ctx, successText, "success");

        if (typeof this.onWormCreated === "function") {
          this.onWormCreated(res);
        }

        this.ctx.setTimeout(() => {
          this.closeModal();
          if (this.titleInput) this.titleInput.value = "";
          if (this.descInput) this.descInput.value = "";
          if (this.stepsInput) this.stepsInput.value = "";
          if (this.expectedInput) this.expectedInput.value = "";
          if (this.actualInput) this.actualInput.value = "";
          this.attachments = [];
          this.renderAttachments();
          if (this.submitBtn) {
            this.submitBtn.disabled = false;
            this.submitBtn.textContent = "⚡ File Target Bug";
          }
        }, AUTO_CLOSE_SUCCESS_MS);

        return res;
      }

      const errMsg = `Error: ${(res && res.error) || "Failed to create bug report"}`;
      this._setProgress(100, "Defect submission failed");
      if (this.progressBar) {
        this.progressBar.style.background = "var(--xerxes-red)";
      }
      this._showStatusBanner(errMsg, true);
      if (this.submitBtn) {
        this.submitBtn.disabled = false;
        this.submitBtn.textContent = "⚡ File Target Bug";
      }
      return res;
    } catch (err) {
      const errMsg = `Error: ${(err && err.message) || String(err)}`;
      this._setProgress(100, "Defect submission failed");
      if (this.progressBar) {
        this.progressBar.style.background = "var(--xerxes-red)";
      }
      this._showStatusBanner(errMsg, true);
      if (this.submitBtn) {
        this.submitBtn.disabled = false;
        this.submitBtn.textContent = "⚡ File Target Bug";
      }
      return null;
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
 * Factory creating a `WormReporterController` bound to `ctx`.
 *
 * @param {object} ctx - Explicit `RuntimeContext`.
 * @param {object} [options] - Controller options.
 * @returns {WormReporterController}
 */
export function createWormReporter(ctx, options = {}) {
  return new WormReporterController(ctx, options);
}
