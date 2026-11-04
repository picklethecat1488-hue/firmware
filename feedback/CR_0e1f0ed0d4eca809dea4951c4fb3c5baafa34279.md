# Code Review Report: Code Review: firmware

> Automated code review log and findings generated via Quake Code Review Engine.

## Review Overview

| Metric | Details |
| :--- | :--- |
| **Review Date** | `2026-10-04 22:47:53 UTC` |
| **Revisions** | `0e1f0ed0d4eca809dea4951c4fb3c5baafa34279` |
| **Overall Verdict** | **`CHANGES_REQUESTED`** |
| **Review Progress** | `0/2 files reviewed (0%)` |
| **Total Comments** | `12 findings` |

## Findings by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **[MUST FIX]** | 12 | Must be resolved before merge; bugs, defects, or safety regressions. |
| **[PROPOSAL]** | 0 | Architecture ideas, design proposals, or optional enhancements. |
| **[NIT]** | 0 | Minor formatting, naming, or cosmetic cleanups. |

## File-by-File Review Findings

### [`app/carrier_board.md`](file:///Users/<username>/gh/firmware/app/carrier_board.md) — ⏳ `PENDING`

#### **[MUST FIX]** [app/carrier_board.md:L835](file:///Users/<username>/gh/firmware/app/carrier_board.md#L835)
<!-- comment-uuid: acea940c-10e4-4fc0-b758-6e3a281bcfd8 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
- **Temporal Gesture Classifier**: Core 1 evaluates sliding touch windows to classify discrete 1-finger gestures with confidence $\ge 0.85$.
```

> **Reviewer (Reviewer)**: I think we should shoot for >=95%

#### **[MUST FIX]** [app/carrier_board.md:L836](file:///Users/<username>/gh/firmware/app/carrier_board.md#L836)
<!-- comment-uuid: 2ee14223-c270-4375-aed5-d550368a84fb -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
- **Zero-Copy IPC Dispatch**: When a gesture is confirmed, Core 1 packages a typed `GestureEvent` struct into the lock-free shared SRAMX SPSC ring buffer (`0x2006_8000`), pulsing the hardware Messaging Unit (MU) doorbell to wake Core 0.
```

> **Reviewer (Reviewer)**: let's use the same type safe enum gesture type we use for rp2040

#### **[MUST FIX]** [app/carrier_board.md:L833](file:///Users/<username>/gh/firmware/app/carrier_board.md#L833)
<!-- comment-uuid: d678adc1-d8bb-4287-afa1-3bdff7e038d8 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
- **100 Hz Async eDMA Ingestion**: Core 1 runs an Embassy task polling the IQS7222A over $\text{I}^2\text{C}$ at 100 Hz upon `CAP_INT` assertion.
```

> **Reviewer (Reviewer)**: core 0's sensor controller should be doing this. we just need to specialize it for cap touch devices. I don't see a reason to create a separate polling task for this

#### **[MUST FIX]** [app/carrier_board.md:L833](file:///Users/<username>/gh/firmware/app/carrier_board.md#L833)
<!-- comment-uuid: f9d898b7-8ca9-4c68-9e2d-adb50e719a7e -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
- **100 Hz Async eDMA Ingestion**: Core 1 runs an Embassy task polling the IQS7222A over $\text{I}^2\text{C}$ at 100 Hz upon `CAP_INT` assertion.
```

> **Reviewer (Reviewer)**: we have a FIFO on this chip. can we assume a 10-30Hz poll rate instead?

#### **[MUST FIX]** [app/carrier_board.md:L837](file:///Users/<username>/gh/firmware/app/carrier_board.md#L837)
<!-- comment-uuid: 77e5adf8-0009-46b2-9dfe-e13639e5fdc9 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
- **Controller Action Routing**: Core 0's `SensorController` and `SystemController` map incoming gestures to system state changes, UI feedback (RGB LED patterns via `LedController`, acoustic chimes via `SpeakerController`), and wireless BLE notifications via `BleController`.
```

> **Reviewer (Reviewer)**: I'm confused about this statement bcuz I thought core 1 was the "lower power" CPU that would run the system controller, and after boot, core 0 would be running sensor controller and doing all the ML inference and DSP operations because it had the FPU?

#### **[MUST FIX]** [app/carrier_board.md:L849-L850](file:///Users/<username>/gh/firmware/app/carrier_board.md#L849-L850)
<!-- comment-uuid: 445a0d81-1f2a-4531-95b9-3d9ccc62337f -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
| **1F Long Press** | Stationary contact held for $1.5\text{ s} \le T < 3.5\text{ s}$ with displacement drift $\Delta X < 2.5\,\text{mm}$. | **BLE Pairing Mode Trigger**: Enters BLE discoverable advertising mode for 60 seconds; if already connected, initiates graceful disconnect and re-advertising. | In `Sleep`, awakens system and initiates immediate BLE pairing sequence. | Rising dual-tone chime ($523\text{ Hz} \to 784\text{ Hz}$, $120\text{ ms}$); transition from Solid Cyan to Fast Blue Blink (`BLE_PAIRING` @ 2 Hz). | Emits `GestureType::LongPress { hold_duration_ms }`; Core 0 signals `BleController` to start fast advertising. |
| **1F Extra Long Press** | Stationary contact held continuously for $T \ge 5.0\text{ s}$ with displacement drift $\Delta X < 2.5\,\text{mm}$. | **Graceful Sleep Transition (`PowerDown`)**: Initiates graceful shutdown: commits NVRAM dirty cache to `metadata`, flushes NAND telemetry, and enters `PowerDown` ($\le 185\,\mu\text{A}$). | If held $\ge 10.0\text{ s}$ during system panic/fault: triggers hardware watchdog hard reboot (`NVIC_SystemReset()`). | Triple descending warning chime ($880\text{ Hz} \to 659\text{ Hz} \to 440\text{ Hz}$, $250\text{ ms}$); Amber/Red pulse followed by LED dark (Off). | Emits `GestureType::ExtraLongPress { hold_duration_ms: 5000 }`; `SystemController` initiates `SystemStatus::PowerDown`. |
```

> **Reviewer (Reviewer)**: let's swap these- 1F long extra long press initiates BLE pairing, and 1F long press initiates a power down

#### **[MUST FIX]** [app/carrier_board.md:L845-L848](file:///Users/<username>/gh/firmware/app/carrier_board.md#L845-L848)
<!-- comment-uuid: b0cbe13a-5c8c-450e-82f9-667279d9924e -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
| **1F Tap (Single Tap)** | Single contact: $50\,\text{ms} \le T < 300\,\text{ms}$, displacement $\Delta X < 3.0\,\text{mm}$. | **Toggle / Primary Action**: Toggles active feature state (e.g. telemetry sampling enable/pause); confirms active system prompt. | **Instant System Wake**: In `Sleep`, restores 150 MHz clocks ($\approx 250\,\mu\text{s}$). In `PowerDown`, triggers hardware WUU wake ($\le 1.8\,\text{ms}$). | `Gesture Confirmed Tone` ($784\text{ Hz}$, $60\text{ ms}$, $60\text{ dBA}$); momentary Cyan LED pulse ($150\text{ ms}$). | Emits `GestureType::SingleTap`; sends BLE GATT notification; appends CBOR record to NAND `telemetry` queue. |
| **1F Double Tap** | Two sequential taps within $T_{interval} \le 350\,\text{ms}$; individual tap duration $< 250\,\text{ms}$; impact delta $\le 4.0\,\text{mm}$. | **Cycle Mode / Quick Action**: Cycles active sensor acquisition profiles; triggers immediate forced dirty-buffer flush over BLE. | Ignored in `PowerDown` (first tap awakens device to `Active`). | Ascending two-tone ping ($659\text{ Hz} \to 784\text{ Hz}$, $40\text{ ms}$ each); double Cyan LED flash ($100\text{ ms} \times 2$). | Emits `GestureType::DoubleTap`; Core 0 commands `TelemetryController` to initiate high-speed BLE flush. |
| **1F Swipe Forward** | Contact swept forward along strip axis: $\Delta X \ge +12.0\,\text{mm}$ within $80\,\text{ms} \le T \le 500\,\text{ms}$ ($v \ge +40\,\text{mm/s}$). | **Navigate Forward / Increment**: Advances to next operational preset/profile; increases Class-D speaker output volume (+3 dB). | In `Sleep`, awakens system directly into next operational preset. | Ascending chirp sweep ($587\text{ Hz} \to 880\text{ Hz}$, $80\text{ ms}$); forward trailing Cyan chase animation on LP5009 RGB LED. | Emits `GestureType::SwipeForward { displacement_mm, velocity_mm_s }`; Core 0 updates active preset index. |
| **1F Swipe Back** | Contact swept backward along strip axis: $\Delta X \le -12.0\,\text{mm}$ within $80\,\text{ms} \le T \le 500\,\text{ms}$ ($v \le -40\,\text{mm/s}$). | **Navigate Backward / Decrement**: Returns to previous operational preset/profile; decreases Class-D speaker output volume (-3 dB). | In `Sleep`, awakens system directly into previous operational preset. | Descending chirp sweep ($880\text{ Hz} \to 587\text{ Hz}$, $80\text{ ms}$); backward trailing Cyan chase animation on LP5009 RGB LED. | Emits `GestureType::SwipeBack { displacement_mm, velocity_mm_s }`; Core 0 updates active preset index. |
```

> **Reviewer (Reviewer)**: I think it's fine to have everything 1FLT and 1LLT do nothing but send BLE GATT notifications

#### **[MUST FIX]** [app/carrier_board.md:L845-L850](file:///Users/<username>/gh/firmware/app/carrier_board.md#L845-L850)
<!-- comment-uuid: db54a08e-e8fe-4789-9f50-e1cb6dc44785 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
| **1F Tap (Single Tap)** | Single contact: $50\,\text{ms} \le T < 300\,\text{ms}$, displacement $\Delta X < 3.0\,\text{mm}$. | **Toggle / Primary Action**: Toggles active feature state (e.g. telemetry sampling enable/pause); confirms active system prompt. | **Instant System Wake**: In `Sleep`, restores 150 MHz clocks ($\approx 250\,\mu\text{s}$). In `PowerDown`, triggers hardware WUU wake ($\le 1.8\,\text{ms}$). | `Gesture Confirmed Tone` ($784\text{ Hz}$, $60\text{ ms}$, $60\text{ dBA}$); momentary Cyan LED pulse ($150\text{ ms}$). | Emits `GestureType::SingleTap`; sends BLE GATT notification; appends CBOR record to NAND `telemetry` queue. |
| **1F Double Tap** | Two sequential taps within $T_{interval} \le 350\,\text{ms}$; individual tap duration $< 250\,\text{ms}$; impact delta $\le 4.0\,\text{mm}$. | **Cycle Mode / Quick Action**: Cycles active sensor acquisition profiles; triggers immediate forced dirty-buffer flush over BLE. | Ignored in `PowerDown` (first tap awakens device to `Active`). | Ascending two-tone ping ($659\text{ Hz} \to 784\text{ Hz}$, $40\text{ ms}$ each); double Cyan LED flash ($100\text{ ms} \times 2$). | Emits `GestureType::DoubleTap`; Core 0 commands `TelemetryController` to initiate high-speed BLE flush. |
| **1F Swipe Forward** | Contact swept forward along strip axis: $\Delta X \ge +12.0\,\text{mm}$ within $80\,\text{ms} \le T \le 500\,\text{ms}$ ($v \ge +40\,\text{mm/s}$). | **Navigate Forward / Increment**: Advances to next operational preset/profile; increases Class-D speaker output volume (+3 dB). | In `Sleep`, awakens system directly into next operational preset. | Ascending chirp sweep ($587\text{ Hz} \to 880\text{ Hz}$, $80\text{ ms}$); forward trailing Cyan chase animation on LP5009 RGB LED. | Emits `GestureType::SwipeForward { displacement_mm, velocity_mm_s }`; Core 0 updates active preset index. |
| **1F Swipe Back** | Contact swept backward along strip axis: $\Delta X \le -12.0\,\text{mm}$ within $80\,\text{ms} \le T \le 500\,\text{ms}$ ($v \le -40\,\text{mm/s}$). | **Navigate Backward / Decrement**: Returns to previous operational preset/profile; decreases Class-D speaker output volume (-3 dB). | In `Sleep`, awakens system directly into previous operational preset. | Descending chirp sweep ($880\text{ Hz} \to 587\text{ Hz}$, $80\text{ ms}$); backward trailing Cyan chase animation on LP5009 RGB LED. | Emits `GestureType::SwipeBack { displacement_mm, velocity_mm_s }`; Core 0 updates active preset index. |
| **1F Long Press** | Stationary contact held for $1.5\text{ s} \le T < 3.5\text{ s}$ with displacement drift $\Delta X < 2.5\,\text{mm}$. | **BLE Pairing Mode Trigger**: Enters BLE discoverable advertising mode for 60 seconds; if already connected, initiates graceful disconnect and re-advertising. | In `Sleep`, awakens system and initiates immediate BLE pairing sequence. | Rising dual-tone chime ($523\text{ Hz} \to 784\text{ Hz}$, $120\text{ ms}$); transition from Solid Cyan to Fast Blue Blink (`BLE_PAIRING` @ 2 Hz). | Emits `GestureType::LongPress { hold_duration_ms }`; Core 0 signals `BleController` to start fast advertising. |
| **1F Extra Long Press** | Stationary contact held continuously for $T \ge 5.0\text{ s}$ with displacement drift $\Delta X < 2.5\,\text{mm}$. | **Graceful Sleep Transition (`PowerDown`)**: Initiates graceful shutdown: commits NVRAM dirty cache to `metadata`, flushes NAND telemetry, and enters `PowerDown` ($\le 185\,\mu\text{A}$). | If held $\ge 10.0\text{ s}$ during system panic/fault: triggers hardware watchdog hard reboot (`NVIC_SystemReset()`). | Triple descending warning chime ($880\text{ Hz} \to 659\text{ Hz} \to 440\text{ Hz}$, $250\text{ ms}$); Amber/Red pulse followed by LED dark (Off). | Emits `GestureType::ExtraLongPress { hold_duration_ms: 5000 }`; `SystemController` initiates `SystemStatus::PowerDown`. |
```

> **Reviewer (Reviewer)**: in terms of gesture on/off tones, let's just have a finger down tone that plays for single, double tap, and long press notifications, and another tone for swiping forward or backward

#### **[MUST FIX]** [app/carrier_board.md:L868-L877](file:///Users/<username>/gh/firmware/app/carrier_board.md#L868-L877)
<!-- comment-uuid: 8ce91422-5ee7-4e84-b82b-65466e18233a -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
#[repr(C)]
#[derive(Copy, Clone, Debug, minicbor::Encode, minicbor::Decode)]
pub struct GestureEvent {
    #[n(0)] pub gesture: GestureType,
    #[n(1)] pub timestamp_us: u64,
    #[n(2)] pub duration_ms: u16,
    #[n(3)] pub displacement_mm: i16,
    #[n(4)] pub velocity_mm_s: i16,
    #[n(5)] pub confidence_pct: u8,
}
```

> **Reviewer (Reviewer)**: let's change this into a telemetry event that is logged by the sensor controller when a gesture is detected and add a section on gesture telemetry and FP detection

#### **[MUST FIX]** [app/carrier_board.md:L857-L866](file:///Users/<username>/gh/firmware/app/carrier_board.md#L857-L866)
<!-- comment-uuid: 9ff99b58-fa14-46ff-8334-8b45463359bf -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, minicbor::Encode, minicbor::Decode)]
pub enum GestureType {
    #[n(0)] SingleTap = 0,
    #[n(1)] DoubleTap = 1,
    #[n(2)] SwipeForward = 2,
    #[n(3)] SwipeBack = 3,
    #[n(4)] LongPress = 4,
    #[n(5)] ExtraLongPress = 5,
}
```

> **Reviewer (Reviewer)**: let's reuse https://gitkraken.dev/link/dnNjb2RlOi8vZWFtb2Rpby5naXRsZW5zL2xpbmsvci81MDQyMWEwYWMxOGU5YjkwZTZhMjFlZmMxODVkMGEwOWZlNTY2MDJjL2YvbW9kZWwvc3JjL3R5cGVzLnJzP3VybD1odHRwcyUzQSUyRiUyRmdpdGh1Yi5jb20lMkZwaWNrbGV0aGVjYXQxNDg4LWh1ZSUyRmZpcm13YXJlJmxpbmVzPTI1Ny0yNjU%3D?origin=gitlens pls

#### **[MUST FIX]** [app/carrier_board.md:L883-L899](file:///Users/<username>/gh/firmware/app/carrier_board.md#L883-L899)
<!-- comment-uuid: f6606db9-fb26-4162-a2de-6c21fc4e1277 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

```markdown
flowchart TD
    IDLE["Touch Surface IDLE (No Contact)"] -->|"Touch Down (Delta > Threshold)"| TOUCH_DOWN["Touch Contact Initiated (Timer Started)"]
    TOUCH_DOWN -->|"Released in 50-300 ms, Delta X < 3mm"| TAP_PENDING{"Inter-Tap Window (350 ms)"}
    TAP_PENDING -->|"Second Tap within 350 ms"| DOUBLE_TAP["Emit GestureType::DoubleTap"]
    TAP_PENDING -->|"Timeout without Second Tap"| SINGLE_TAP["Emit GestureType::SingleTap"]
    TOUCH_DOWN -->|"Displacement Delta X >= +12mm"| SWIPE_FWD["Emit GestureType::SwipeForward"]
    TOUCH_DOWN -->|"Displacement Delta X <= -12mm"| SWIPE_BACK["Emit GestureType::SwipeBack"]
    TOUCH_DOWN -->|"Stationary Hold >= 1.5 s"| LONG_PRESS_EVAL{"Hold Duration"}
    LONG_PRESS_EVAL -->|"Released before 3.5 s"| LONG_PRESS["Emit GestureType::LongPress (BLE Pairing)"]
    LONG_PRESS_EVAL -->|"Sustained Hold >= 5.0 s"| EXTRA_LONG_PRESS["Emit GestureType::ExtraLongPress (Enter PowerDown)"]
    LONG_PRESS_EVAL -->|"Sustained Hold >= 10.0 s"| HARD_RESET["Trigger Emergency Reboot (NVIC_SystemReset)"]
    DOUBLE_TAP --> IDLE
    SINGLE_TAP --> IDLE
    SWIPE_FWD --> IDLE
    SWIPE_BACK --> IDLE
    LONG_PRESS --> IDLE
    EXTRA_LONG_PRESS --> IDLE
```

> **Reviewer (Reviewer)**: instead of a single FSM, can we define FSMs for each gesture state we implement Gesture enum, and introduce state cancellation to prevent FP detections. also, we need to adapt gesture detector to implement each gesture state and support differing gesture sets for rp2040/n947 add support for cap touch to the gesture detector, and have a plan for lowering the gesture detector onto the NPU for the N947 processors. we can define these intermediate states for each gesture primitive: FINGER_DOWN, FINGER_MOVE, FINGER_UP

#### **[MUST FIX]** [app/carrier_board.md:L901](file:///Users/<username>/gh/firmware/app/carrier_board.md#L901)
<!-- comment-uuid: 8c479a32-b0c2-4083-9c8d-a64114ffb782 -->
<!-- comment-commit: 0e1f0ed0d4eca809dea4951c4fb3c5baafa34279 -->

> **Reviewer (Reviewer)**: we also need to support tap and long press for the action button. can you add the action button to this section?

## Action Items Checklist

- [x] **[MUST FIX]** [`app/carrier_board.md:L835`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L835): I think we should shoot for >=95% <!-- uuid:acea940c-10e4-4fc0-b758-6e3a281bcfd8 -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L836`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L836): let's use the same type safe enum gesture type we use for rp2040 <!-- uuid:2ee14223-c270-4375-aed5-d550368a84fb -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L833`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L833): core 0's sensor controller should be doing this. we just need to specialize it for cap touch devices. I don't see a reason to create a separate polling task for this <!-- uuid:d678adc1-d8bb-4287-afa1-3bdff7e038d8 -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L833`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L833): we have a FIFO on this chip. can we assume a 10-30Hz poll rate instead? <!-- uuid:f9d898b7-8ca9-4c68-9e2d-adb50e719a7e -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L837`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L837): I'm confused about this statement bcuz I thought core 1 was the "lower power" CPU that would run the system controller, and after boot, core 0 would be running sensor controller and doing all the ML inference and DSP operations because it had the FPU? <!-- uuid:77e5adf8-0009-46b2-9dfe-e13639e5fdc9 -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L849-L850`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L849-L850): let's swap these- 1F long extra long press initiates BLE pairing, and 1F long press initiates a power down <!-- uuid:445a0d81-1f2a-4531-95b9-3d9ccc62337f -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L845-L848`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L845-L848): I think it's fine to have everything 1FLT and 1LLT do nothing but send BLE GATT notifications <!-- uuid:b0cbe13a-5c8c-450e-82f9-667279d9924e -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L845-L850`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L845-L850): in terms of gesture on/off tones, let's just have a finger down tone that plays for single, double tap, and long press notifications, and another tone for swiping forward or backward <!-- uuid:db54a08e-e8fe-4789-9f50-e1cb6dc44785 -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L868-L877`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L868-L877): let's change this into a telemetry event that is logged by the sensor controller when a gesture is detected and add a section on gesture telemetry and FP detection <!-- uuid:8ce91422-5ee7-4e84-b82b-65466e18233a -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L857-L866`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L857-L866): let's reuse https://gitkraken.dev/link/dnNjb2RlOi8vZWFtb2Rpby5naXRsZW5zL2xpbmsvci81MDQyMWEwYWMxOGU5YjkwZTZhMjFlZmMxODVkMGEwOWZlNTY2MDJjL2YvbW9kZWwvc3JjL3R5cGVzLnJzP3VybD1odHRwcyUzQSUyRiUyRmdpdGh1Yi5jb20lMkZwaWNrbGV0aGVjYXQxNDg4LWh1ZSUyRmZpcm13YXJlJmxpbmVzPTI1Ny0yNjU%3D?origin=gitlens pls <!-- uuid:9ff99b58-fa14-46ff-8334-8b45463359bf -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L883-L899`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L883-L899): instead of a single FSM, can we define FSMs for each gesture state we implement Gesture enum, and introduce state cancellation to prevent FP detections. also, we need to adapt gesture detector to implement each gesture state and support differing gesture sets for rp2040/n947 add support for cap touch to the gesture detector, and have a plan for lowering the gesture detector onto the NPU for the N947 processors. we can define these intermediate states for each gesture primitive: FINGER_DOWN, FINGER_MOVE, FINGER_UP <!-- uuid:f6606db9-fb26-4162-a2de-6c21fc4e1277 -->
- [x] **[MUST FIX]** [`app/carrier_board.md:L901`](file:///Users/<username>/gh/firmware/app/carrier_board.md#L901): we also need to support tap and long press for the action button. can you add the action button to this section? <!-- uuid:8c479a32-b0c2-4083-9c8d-a64114ffb782 -->
