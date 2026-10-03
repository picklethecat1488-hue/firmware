# Peripheral Sharing & Concurrency Patterns

## Architectural Purpose
To guarantee race-free, deterministic peripheral access, memory safety, and thread/task isolation across asynchronous embedded tasks on bare-metal targets.

---

## Core Invariants

### 1. Peripheral Sharing Strategies
When multiple controllers or tasks require access to the same physical hardware peripheral or communication bus (e.g., I2C bus, SPI bus, UART port):
* **System Integration (Production)**: Use the **Actor / Message-Passing Pattern**. The peripheral driver runs inside its own isolated task and communicates with client controllers exclusively via asynchronous channels (e.g., `embassy_sync::channel::Channel`). Clients send requests and await typed response messages.
* **Bringup & Diagnostic Shell**: For simple test harnesses or diagnostic shell command dispatch, use **Interior Mutability & Shared References** (`Rc` + `RefCell` for single-threaded or `embassy_sync::blocking_mutex::Mutex` / `Arc` for multi-threaded/interrupt contexts).
* **Strictly Forbidden**: NEVER pass raw mutable references (`&mut T`) across tasks or threads.

### 2. Static Memory & Global State Governance
* **Restriction on Global Statics**: Static mutable variables (`static mut`) and unrestricted global statics MUST NOT be used outside of core hardware monitor tasks, interrupt callbacks, panic/fault handlers, and DMA communication buffers requiring static memory lifetimes.
* **OnceLock Validation Mandate**: When initializing global statics (such as `OnceLock` or `embassy_sync::once_lock::OnceLock` instances), always explicitly verify the result of `.set(...)` (e.g., via `.expect("...")` or `.unwrap()`) to catch double-initialization defects and race conditions immediately. NEVER discard the result with `let _ =`.

### 3. Resource Contention & Mutex Guardrails
* **No Long Critical Sections**: Async await points (`.await`) must NEVER be held while holding blocking interrupt-disabling spinlocks or critical-section tokens.
* **Bounded Queues**: All channels, ring buffers, and communication queues must be statically sized with bounded capacities (e.g., via `heapless::Deque` or `embassy_sync::channel::Channel<M, T, N>`) with explicit overflow error handling.
