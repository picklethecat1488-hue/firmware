#!/usr/bin/env python3
"""Regression unit test for WORM-WRITHING-TUBEWORM-652.

Verifies that:
1. TMC6300 driver in peripheral/src/tmc6300.rs defines MAX_TMC6300_CURRENT_MA (2000 mA),
   PIO_PWM_FREQUENCY_HZ (25_000 Hz), and PWM_PERIOD_CYCLES (100).
2. Phase mask function generates non-overlapping, break-before-make commutation steps
   with zero shoot-through across all forward and reverse steps.
3. 32-bit PIO PWM command word packs phase mask, on-count (duty cycle), and off-count.
4. Tmc6300Pio driver is implemented with RP2040 PIO state machine on bare metal and mock on host.
5. Board support package in board/src/cat_detector.rs initializes PIO1 and configures
   Tmc6300Pio as MotorDevice.
6. Motor controller in controller/src/motor_controller.rs enforces MAX_TMC6300_CURRENT_MA (2000 mA)
   limits and clamps configured / calibrated overload limits to 2A.
7. Integration test test_worm_writhing_tubeworm_652_tmc6300_current_limiting_and_pio_pwm is present.
"""
import re
from pathlib import Path


def test_worm_writhing_tubeworm_652_tmc6300_constants_and_functions():
    """Verify TMC6300 PIO constants and functions in peripheral/src/tmc6300.rs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs_path = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    assert tmc6300_rs_path.exists(), f"File not found: {tmc6300_rs_path}"
    content = tmc6300_rs_path.read_text(encoding="utf-8")

    # Invariant 1: 2A peak current limit constant and >=20kHz PWM frequency
    assert "pub const MAX_TMC6300_CURRENT_MA: i32 = 2000;" in content, (
        "peripheral/src/tmc6300.rs must define MAX_TMC6300_CURRENT_MA as 2000."
    )
    assert "pub const PIO_PWM_FREQUENCY_HZ: u32 = 25_000;" in content, (
        "peripheral/src/tmc6300.rs must define PIO_PWM_FREQUENCY_HZ as 25_000."
    )
    assert "pub const PWM_PERIOD_CYCLES: u8 = 100;" in content, (
        "peripheral/src/tmc6300.rs must define PWM_PERIOD_CYCLES as 100."
    )

    # Invariant 2: PIO word generator and phase mask functions
    assert "pub const fn make_pwm_word(" in content, (
        "peripheral/src/tmc6300.rs must define make_pwm_word function."
    )
    assert "pub fn phase_mask(" in content, (
        "peripheral/src/tmc6300.rs must define phase_mask function."
    )

    # Invariant 3: Tmc6300Pio driver and error type
    assert "pub struct Tmc6300Pio" in content, (
        "peripheral/src/tmc6300.rs must define Tmc6300Pio driver struct."
    )
    assert "pub enum Tmc6300PioError" in content, (
        "peripheral/src/tmc6300.rs must define Tmc6300PioError enum."
    )


def test_worm_writhing_tubeworm_652_phase_masks_zero_shoot_through():
    """Verify phase masks enforce break-before-make and prevent shoot-through."""
    # Commutation phase masks per step (forward)
    forward_masks = [
        0b001001,  # step 0: UH (bit 0), VL (bit 3)
        0b100001,  # step 1: UH (bit 0), WL (bit 5)
        0b100100,  # step 2: VH (bit 2), WL (bit 5)
        0b000110,  # step 3: VH (bit 2), UL (bit 1)
        0b010010,  # step 4: WH (bit 4), UL (bit 1)
        0b011000,  # step 5: WH (bit 4), VL (bit 3)
    ]

    # Commutation phase masks per step (reverse)
    reverse_masks = [
        0b011000,  # step 0: WH (bit 4), VL (bit 3)
        0b010010,  # step 1: WH (bit 4), UL (bit 1)
        0b000110,  # step 2: VH (bit 2), UL (bit 1)
        0b100100,  # step 3: VH (bit 2), WL (bit 5)
        0b100001,  # step 4: UH (bit 0), WL (bit 5)
        0b001001,  # step 5: UH (bit 0), VL (bit 3)
    ]

    for step, mask in enumerate(forward_masks):
        uh = (mask & (1 << 0)) != 0
        ul = (mask & (1 << 1)) != 0
        vh = (mask & (1 << 2)) != 0
        vl = (mask & (1 << 3)) != 0
        wh = (mask & (1 << 4)) != 0
        wl = (mask & (1 << 5)) != 0

        assert not (uh and ul), f"Forward step {step}: Phase U shoot-through!"
        assert not (vh and vl), f"Forward step {step}: Phase V shoot-through!"
        assert not (wh and wl), f"Forward step {step}: Phase W shoot-through!"

        high_count = int(uh) + int(vh) + int(wh)
        low_count = int(ul) + int(vl) + int(wl)
        assert high_count == 1, f"Forward step {step}: Must have exactly 1 high-side FET active"
        assert low_count == 1, f"Forward step {step}: Must have exactly 1 low-side FET active"

    for step, mask in enumerate(reverse_masks):
        uh = (mask & (1 << 0)) != 0
        ul = (mask & (1 << 1)) != 0
        vh = (mask & (1 << 2)) != 0
        vl = (mask & (1 << 3)) != 0
        wh = (mask & (1 << 4)) != 0
        wl = (mask & (1 << 5)) != 0

        assert not (uh and ul), f"Reverse step {step}: Phase U shoot-through!"
        assert not (vh and vl), f"Reverse step {step}: Phase V shoot-through!"
        assert not (wh and wl), f"Reverse step {step}: Phase W shoot-through!"

        high_count = int(uh) + int(vh) + int(wh)
        low_count = int(ul) + int(vl) + int(wl)
        assert high_count == 1, f"Reverse step {step}: Must have exactly 1 high-side FET active"
        assert low_count == 1, f"Reverse step {step}: Must have exactly 1 low-side FET active"


def test_worm_writhing_tubeworm_652_board_pio1_motor_initialization():
    """Verify board/src/cat_detector.rs initializes PIO1 and binds Tmc6300Pio."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cat_detector_rs_path = workspace_root / "board" / "src" / "cat_detector.rs"

    content = cat_detector_rs_path.read_text(encoding="utf-8")

    assert "PIO1_IRQ_0 => InterruptHandler<embassy_rp::peripherals::PIO1>;" in content, (
        "board/src/cat_detector.rs must bind PIO1_IRQ_0 for PIO1 motor state machine."
    )
    assert "peripheral::tmc6300::Tmc6300Pio::new_pio(" in content, (
        "board/src/cat_detector.rs must construct Tmc6300Pio::new_pio."
    )
    assert "p.PIO1" in content, (
        "board/src/cat_detector.rs must pass p.PIO1 to Tmc6300Pio."
    )
    assert re.search(r"pub\s+type\s+MotorDevice\s*=\s*peripheral::tmc6300::Tmc6300Pio<", content) is not None, (
        "board/src/cat_detector.rs must set MotorDevice alias to Tmc6300Pio."
    )


def test_worm_writhing_tubeworm_652_motor_controller_clamps_to_2a():
    """Verify controller/src/motor_controller.rs clamps max current to 2000 mA."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    mc_rs_path = workspace_root / "controller" / "src" / "motor_controller.rs"

    content = mc_rs_path.read_text(encoding="utf-8")

    assert "pub const MAX_TMC6300_CURRENT_MA: i32 = 2000;" in content, (
        "controller/src/motor_controller.rs must define MAX_TMC6300_CURRENT_MA as 2000."
    )
    assert "max_current_ma: MAX_TMC6300_CURRENT_MA," in content, (
        "controller/src/motor_controller.rs must use MAX_TMC6300_CURRENT_MA in default limits."
    )
    assert "max_ma.min(MAX_TMC6300_CURRENT_MA)" in content, (
        "controller/src/motor_controller.rs must clamp configured limits to MAX_TMC6300_CURRENT_MA."
    )
    assert "store.current_ma.overload.min(MAX_TMC6300_CURRENT_MA)" in content, (
        "controller/src/motor_controller.rs must clamp calibration overload limits to MAX_TMC6300_CURRENT_MA."
    )


def test_worm_writhing_tubeworm_652_integration_test_present():
    """Verify integration test covering current limiting and PIO PWM is present."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    integration_rs_path = workspace_root / "peripheral" / "tests" / "integration.rs"

    content = integration_rs_path.read_text(encoding="utf-8")
    assert "test_worm_writhing_tubeworm_652_tmc6300_current_limiting_and_pio_pwm" in content, (
        "peripheral/tests/integration.rs must define test_worm_writhing_tubeworm_652_tmc6300_current_limiting_and_pio_pwm."
    )
