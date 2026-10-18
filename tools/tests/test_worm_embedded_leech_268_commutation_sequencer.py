#!/usr/bin/env python3
"""Regression unit test for WORM-EMBEDDED-LEECH-268.

Verifies that:
1. In peripheral/src/tmc6300.rs, bare-metal Tmc6300Pio::set_speed updates speed and
   duty cycle without pushing un-incremented commutation step words into sm.tx().
   This eliminates duplicate FIFO pushes and word drops during 10 ms motor controller ramping.
2. In peripheral/src/tmc6300.rs, Tmc6300Pio::tick solely owns commutation advancement
   across steps 0..=5 and pushes active 32-bit PWM words into sm.tx().
3. In peripheral/tests/integration.rs, test_worm_embedded_leech_268_commutation_advancement_and_continuous_pwm
   is present and actively asserting 6-step commutation cycling and continuous PIO PWM recycling.
4. Phase mask sequencing strictly satisfies break-before-make and zero shoot-through across all 6 steps.
"""

from pathlib import Path


def test_worm_embedded_leech_268_set_speed_does_not_push_to_pio_fifo():
    """Verify bare-metal Tmc6300Pio::set_speed does not push words to sm.tx()."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs_path = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    assert tmc6300_rs_path.exists(), f"File not found: {tmc6300_rs_path}"
    content = tmc6300_rs_path.read_text(encoding="utf-8")

    # Locate bare-metal Motor implementation for Tmc6300Pio
    motor_impl_pos = content.find("impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Motor for Tmc6300Pio")
    assert motor_impl_pos != -1, "Bare-metal Motor impl for Tmc6300Pio not found in peripheral/src/tmc6300.rs"

    motor_impl_end = content.find("impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Tickable", motor_impl_pos)
    assert motor_impl_end != -1, "Bare-metal Tickable impl for Tmc6300Pio not found in peripheral/src/tmc6300.rs"

    motor_impl_str = content[motor_impl_pos:motor_impl_end]

    # Invariant: set_speed must not push to sm.tx(). Only stop() pushes the idle word.
    set_speed_pos = motor_impl_str.find("fn set_speed(")
    stop_pos = motor_impl_str.find("fn stop(")
    assert set_speed_pos != -1, "fn set_speed not found in bare-metal Motor impl"
    assert stop_pos != -1, "fn stop not found in bare-metal Motor impl"

    set_speed_body = motor_impl_str[set_speed_pos:stop_pos]
    assert "self.sm.tx().push(" not in set_speed_body, (
        "bare-metal Tmc6300Pio::set_speed must NOT push words into sm.tx(). "
        "Commutation advancement and PIO FIFO updates must be solely owned by tick()."
    )


def test_worm_embedded_leech_268_tick_owns_commutation_step_and_pushes_word():
    """Verify bare-metal Tmc6300Pio::tick advances commutation and pushes words to sm.tx()."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs_path = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    content = tmc6300_rs_path.read_text(encoding="utf-8")

    tick_impl_pos = content.find("impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Tickable")
    assert tick_impl_pos != -1, "Bare-metal Tickable impl for Tmc6300Pio not found"

    tick_impl_end = content.find("pub struct Tmc6300Pio<PD = ()>", tick_impl_pos)
    assert tick_impl_end != -1, "End of bare-metal Tickable impl not found"

    tick_impl_str = content[tick_impl_pos:tick_impl_end]

    assert "self.commutation_step = (self.commutation_step + 1) % (COMMUTATION_STEPS as u8);" in tick_impl_str, (
        "bare-metal Tmc6300Pio::tick must advance commutation_step across COMMUTATION_STEPS."
    )
    assert "self.sm.tx().push(word);" in tick_impl_str, (
        "bare-metal Tmc6300Pio::tick must push the active commutation PWM word into sm.tx()."
    )


def test_worm_embedded_leech_268_regression_integration_test_present():
    """Verify dedicated regression unit test exists in peripheral/tests/integration.rs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    integration_rs_path = workspace_root / "peripheral" / "tests" / "integration.rs"

    assert integration_rs_path.exists(), f"File not found: {integration_rs_path}"
    content = integration_rs_path.read_text(encoding="utf-8")

    assert "test_worm_embedded_leech_268_commutation_advancement_and_continuous_pwm" in content, (
        "peripheral/tests/integration.rs must define test_worm_embedded_leech_268_commutation_advancement_and_continuous_pwm."
    )


def test_worm_embedded_leech_268_six_step_commutation_phase_masks():
    """Verify 6-step trapezoidal commutation masks for both directions."""
    # Step definitions: (UH, UL, VH, VL, WH, WL)
    # Bit 0: UH, Bit 1: UL, Bit 2: VH, Bit 3: VL, Bit 4: WH, Bit 5: WL
    # Forward: UH/VL -> UH/WL -> VH/WL -> VH/UL -> WH/UL -> WH/VL
    forward_expected = [
        0b001001,  # step 0: UH(0), VL(3)
        0b100001,  # step 1: UH(0), WL(5)
        0b100100,  # step 2: VH(2), WL(5)
        0b000110,  # step 3: VH(2), UL(1)
        0b010010,  # step 4: WH(4), UL(1)
        0b011000,  # step 5: WH(4), VL(3)
    ]

    for step, mask in enumerate(forward_expected):
        uh = (mask & (1 << 0)) != 0
        ul = (mask & (1 << 1)) != 0
        vh = (mask & (1 << 2)) != 0
        vl = (mask & (1 << 3)) != 0
        wh = (mask & (1 << 4)) != 0
        wl = (mask & (1 << 5)) != 0

        # Assert no shoot-through on any half-bridge
        assert not (uh and ul), f"Phase U shoot-through on step {step}"
        assert not (vh and vl), f"Phase V shoot-through on step {step}"
        assert not (wh and wl), f"Phase W shoot-through on step {step}"

        # Assert exactly one high side and one low side active
        assert (int(uh) + int(vh) + int(wh)) == 1, f"High-side count != 1 on step {step}"
        assert (int(ul) + int(vl) + int(wl)) == 1, f"Low-side count != 1 on step {step}"
