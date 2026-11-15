# JavaScript Style, Zero-Globals & Workstation Architecture

Architectural invariants, coding style standards, zero-globals isolation rules, HiDPI rendering requirements, and unit testing mandates for first-party and modifiable vendor JavaScript ES modules across the `firmware` and `hardware` repositories.

---

## 1. Canonical Single-Root Module Layout & Zero Inline Template JS

* **Single Canonical JS Directory**: All browser JavaScript ES modules live under a single root directory per repository:
  - `firmware`: `tools/js/` (`tools/js/core/`, `tools/js/diff/`, `tools/js/vcs/`, `tools/js/review/`, `tools/js/tracker/`, `tools/js/daemon/`, and `tools/js/static/`)
  - `hardware`: `js/` (`js/core/`, `js/diff/`, `js/vcs/`, `js/review/`, `js/tracker/`, `js/scope/`, and `js/static/`)
* **Unified Static Serving**: Both `tools/forge.py` (Xerxes VCS, Diff View, Code Review, Worm Matrix) and `tools/host.py` (Host Daemon RTT/CLI/Bringup Workstation) serve `/js/*` and `/static/*` from the single canonical `tools/js/` directory via `tools/dashboard/provider/http_static.py` with `Cache-Control: no-cache` and strict path-traversal containment.
* **Zero Inline JS & Zero HTML Event Attributes**: Jinja2 workstation templates (`tools/dashboard/templates/*.html.j2` and `tools/host_daemon/templates/*.html.j2`) MUST contain:
  - **Zero** inline executable `<script>` logic blocks.
  - **Zero** inline HTML event handler attributes (`onclick="..."`, `onchange="..."`, `oninput="..."`, `onsubmit="..."`, etc.).
* **Typed JSON Boot Payloads**: Server-rendered configuration must be serialized strictly via `<script id="app-boot-config" type="application/json">{{ boot_config | tojson }}</script>` and parsed via `readBootConfig(ctx)` inside `<script type="module" src="/js/<app>/<entry>.js"></script>`.

---

## 2. Strict Zero-Globals & `RuntimeContext` Accessor Mandate

* **No Ambient Browser Globals**: `.oxlintrc.json` explicitly omits `"browser"` from `env` and enforces `eslint(no-undef)` with `--deny-warnings`. Undeclared browser globals (`window`, `document`, `localStorage`, `sessionStorage`, `navigator`, `location`, `history`, `fetch`, `requestAnimationFrame`, `EventSource`, `WebSocket`, etc.) are compile/lint errors everywhere outside `runtime_context.js`.
* **Single Controlled Boundary (`tools/js/core/runtime_context.js`)**:
  - `tools/js/core/runtime_context.js` is the **sole** module permitted to touch `globalThis` (`/* oxlint-disable no-undef */` is restricted exclusively to that file).
  - Browser entrypoints (`daemon_app.js`, `diff_app.js`, `review_app.js`, `tracker_app.js`) obtain the frozen platform capability object once via `getRuntimeContext()` and pass `ctx` explicitly into module constructors and helper functions.
  - Unit tests construct deterministic, isolated test doubles via `createMockRuntimeContext(overrides)` without mutating Node's `globalThis`.
* **Zero `window.*` Exports**: Modules must NEVER attach functions, singletons, or state onto `window.*` or `globalThis.*`. Bind UI interactions exclusively via `addEventListener` or delegated `data-action` handlers (`onDelegate`).

---

## 3. Code Style & Cleanliness Parity with Rust & Python

JavaScript modules are held to the exact same engineering rigor as Rust firmware crates and host Python utilities:

* **Comprehensive JSDoc Documentation**: Write clear JSDoc (`/** ... */`) docstrings with `@param` and `@returns` annotations for all classes, methods, exported functions, and public properties.
* **Module-Level `ALL_CAPS` Constants & Frozen Enums**: Assign constant values (timeouts, reconnect backoffs, buffer limits, keycodes, CSS class tokens, storage keys) to module-level `ALL_CAPS` `const` bindings rather than embedding inline magic numbers or strings. Represent structured mode/type sets with `Object.freeze({ ... })`.
* **Explicit Named Parameter Objects**: Instantiate controllers and UI components with explicit named options objects (`new RttStreamController({ ctx, logContainer, onStatusChange })`) rather than long positional argument lists or opaque object spreading.
* **No Dead Code or Backward-Compatibility Shims**: Unused variables, unreferenced exports, dead branches, unused function parameters, and legacy alias wrappers are strictly prohibited and rejected by `oxlint --deny-warnings` and `tools/validation/validate_js.py`.
* **Error Handling Guardrails**: Validate inputs, nullability, and array bounds explicitly rather than wrapping logic in generic `try/catch` blocks. Reserve `try/catch` strictly for external I/O boundaries (`ctx.fetch`, `JSON.parse` on untrusted payloads, `EventSource`, storage quota errors) and never silently ignore errors with empty `catch {}` blocks.
* **Modifiable Vendor Code & Standalone `.glsl` Shader Files**: Any modifiable vendor ES modules checked into the repository MUST adhere to the exact same zero-globals rules, JSDoc style, `ALL_CAPS` constants, and `oxlint --deny-warnings` checks as first-party code. Custom WebGL vertex and fragment shaders must live in dedicated `.glsl` files imported directly via ES module `import` (served via `http_static.py`) rather than inline GLSL template strings inside `.js` files.

---

## 4. Full Diff View Visibility, HiDPI Scaling & Cross-Repo Synchronization

* **Unminified Source & Zero `.js` VCS Exclusions**: Ship readable, unminified ES modules across `tools/js/`. `GitEngine` (`tools/vcs/git_engine.py`) must maintain **zero `.js` or `.mjs` path exclusions** so every JavaScript file is inspectable and reviewable line-by-line in Forge Diff View and Code Review.
* **UI Scaling without CSS `zoom`**: Never set `document.documentElement.style.zoom` (which blurs `<canvas>` backing stores via browser raster upscaling). Drive HUD font and control scaling strictly via `--ui-scale` CSS custom properties (`tools/js/core/ui_scale.js`), and scale any 2D/WebGL `<canvas>` backing stores by physical `ctx.devicePixelRatio`.
* **Byte-for-Byte Cross-Repo Parity**: Shared workstation modules (`tools/js/core/`, `tools/js/diff/`, `tools/js/vcs/`, `tools/js/review/`, `tools/js/tracker/`) and their corresponding unit test suites (`shared_*.test.mjs`, `runtime_context.test.mjs`) MUST remain byte-for-byte identical between `firmware/tools/js/` and `hardware/js/`. Any bug fix or enhancement in one repository must be mirrored immediately to the other.

---

## 5. Isolated Parallel Unit Testing & Pre-Commit Validation

* **Test Isolation**: All JavaScript unit tests live in `tools/tests/js/*.test.mjs`, completely separated from production modules under `tools/js/`.
* **Parallel Execution**: Run the Node built-in test runner concurrently across CPU cores (invoked automatically by `./tools/verify.sh`):
  ```bash
  node --test --test-concurrency=$(nproc 2>/dev/null || sysctl -n hw.ncpu || echo 4) tools/tests/js/*.test.mjs
  ```
* **Functional Behavioral Testing Mandate**: JavaScript unit tests MUST exercise real module execution, DOM state transitions (via `createMockRuntimeContext()`), SSE/CLI command flows, diff rendering, and DAG selection behavior. Never write tests that merely read `.js` or `.html.j2` files as strings to assert regex/token presence.
* **Unified Verification (`./tools/verify.sh`)**: `./tools/verify.sh` runs `tools/validation/validate_all.py` (enforcing `oxlint --deny-warnings`, zero-globals compliance outside `runtime_context.js`, zero inline template `on*="..."` attributes, and valid JSON boot blocks) alongside parallel `node --test` suites.
