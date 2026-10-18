#!/usr/bin/env python3
"""Generate SVG waveform comparison for WORM-GLIDING-LARVA-522.

Compares:
1. Observed Buggy Waveform: Inverted pin mapping resulting in driving (UL, WH) then (VL, WH)
   before stalling and tripping TMC6300 DIAG overcurrent fault at t = 20 ms.
2. Canonical Expected Waveform: Correct 6-step trapezoidal commutation progression
   (Step 0 -> Step 5) without stall or DIAG fault assertion.
"""

from pathlib import Path


def generate_svg() -> str:
    width = 1100
    height = 780

    svg = []
    svg.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}" style="background-color: #0e1117; font-family: -apple-system, BlinkMacSystemFont, \'Segoe UI\', Roboto, monospace;">'
    )

    # Definitions (gradients, glow filters)
    svg.append("<defs>")
    svg.append('  <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">')
    svg.append('    <feGaussianBlur stdDeviation="2" result="blur" />')
    svg.append("    <feMerge>")
    svg.append('      <feMergeNode in="blur" />')
    svg.append('      <feMergeNode in="SourceGraphic" />')
    svg.append("    </feMerge>")
    svg.append("  </filter>")
    svg.append("</defs>")

    # Title header
    svg.append('<rect x="0" y="0" width="1100" height="55" fill="#161b22" stroke="#30363d" stroke-width="1"/>')
    svg.append(
        '<text x="25" y="34" fill="#58a6ff" font-size="18" font-weight="bold">WORM-GLIDING-LARVA-522: TMC6300 Commutation Waveform Comparison</text>'
    )
    svg.append('<text x="820" y="34" fill="#8b949e" font-size="13">Saleae Scope vs Canonical 6-Step BLDC</text>')

    # Signal drawing helper
    def draw_signal(y_base, high_val, color, segments, label, pin_info):
        # Label
        svg.append(f'<text x="25" y="{y_base + 12}" fill="{color}" font-size="12" font-weight="bold">{label}</text>')
        svg.append(f'<text x="85" y="{y_base + 12}" fill="#6e7681" font-size="11">{pin_info}</text>')
        # Baseline rail
        svg.append(
            f'<line x1="160" y1="{y_base + 16}" x2="1050" y2="{y_base + 16}" stroke="#21262d" stroke-width="1" stroke-dasharray="2,2"/>'
        )
        # Waveform path
        path_d = []
        cur_x = 160
        cur_y = y_base + 16 if not segments[0][1] else y_base
        path_d.append(f"M {cur_x} {cur_y}")
        for dur_x, is_high in segments:
            target_y = y_base if is_high else y_base + 16
            if target_y != cur_y:
                path_d.append(f"L {cur_x} {target_y}")
                cur_y = target_y
            cur_x += dur_x
            path_d.append(f"L {cur_x} {cur_y}")
        stroke_w = "2"
        svg.append(f'<path d="{" ".join(path_d)}" fill="none" stroke="{color}" stroke-width="{stroke_w}"/>')

    # PANEL 1: OBSERVED BUGGY WAVEFORM (attachments/WORM-GLIDING-LARVA-522/digital.csv)
    p1_top = 75
    svg.append(
        f'<rect x="20" y="{p1_top}" width="1060" height="315" rx="6" fill="#161b22" stroke="#f85149" stroke-width="1.5"/>'
    )
    svg.append(
        f'<text x="35" y="{p1_top + 24}" fill="#f85149" font-size="14" font-weight="bold">🔴 OBSERVED WAVEFORM: Inverted Pin Mapping (Stalls at Phase 1, Trips DIAG)</text>'
    )
    svg.append(
        f'<text x="35" y="{p1_top + 42}" fill="#8b949e" font-size="11">Firmware energized UL &amp; WH (Step 0) then VL &amp; WH (Step 1) -> Reverse magnetic field locked rotor -> Overcurrent fault @ 20ms</text>'
    )

    # Step boundary lines
    # Scale: 0-10ms = 140px, 10-20ms = 140px, 20-60ms = 560px
    # x=160 (0ms), x=300 (10ms), x=440 (20ms), x=1000 (60ms)
    svg.append(
        f'<line x1="300" y1="{p1_top + 50}" x2="300" y2="{p1_top + 300}" stroke="#30363d" stroke-width="1" stroke-dasharray="4,4"/>'
    )
    svg.append(
        f'<line x1="440" y1="{p1_top + 50}" x2="440" y2="{p1_top + 300}" stroke="#f85149" stroke-width="1.5" stroke-dasharray="4,4"/>'
    )

    svg.append(
        f'<text x="210" y="{p1_top + 62}" fill="#8b949e" font-size="11" text-anchor="middle">Step 0 (0-10ms)</text>'
    )
    svg.append(
        f'<text x="370" y="{p1_top + 62}" fill="#8b949e" font-size="11" text-anchor="middle">Step 1 (10-20ms)</text>'
    )
    svg.append(
        f'<text x="720" y="{p1_top + 62}" fill="#f85149" font-size="11" font-weight="bold" text-anchor="middle">MOTOR HALTED / ALL GATES SHUT OFF (20ms - 60ms)</text>'
    )

    # Fault banner
    svg.append(f'<rect x="440" y="{p1_top + 70}" width="560" height="225" fill="#f85149" fill-opacity="0.08" />')
    svg.append(
        f'<text x="720" y="{p1_top + 180}" fill="#f85149" font-size="14" font-weight="bold" text-anchor="middle">⚠️ TMC6300 DIAG DRIVER FAULT ASSERTED (Stall Overcurrent)</text>'
    )

    # Signals in Panel 1
    # 0 to 10ms: 140px; 10 to 20ms: 140px; 20 to 60ms: 560px
    y = p1_top + 80
    draw_signal(y, True, "#58a6ff", [(140, False), (140, False), (560, False)], "UH", "GP16")
    y += 28
    draw_signal(y, True, "#79c0ff", [(140, True), (140, False), (560, False)], "UL", "GP17")
    y += 28
    draw_signal(y, True, "#d29922", [(140, False), (140, False), (560, False)], "VH", "GP18")
    y += 28
    draw_signal(y, True, "#e3b341", [(140, False), (140, True), (560, False)], "VL", "GP19")
    y += 28
    draw_signal(y, True, "#bc8cff", [(140, True), (140, True), (560, False)], "WH", "GP20")
    y += 28
    draw_signal(y, True, "#d2a8ff", [(140, False), (140, False), (560, False)], "WL", "GP21")
    y += 32
    draw_signal(y, True, "#f85149", [(140, False), (140, False), (560, True)], "DIAG", "GP22 [Fault]")

    # PANEL 2: CANONICAL EXPECTED WAVEFORM (Corrected 6-Step Trapezoidal Commutation)
    p2_top = 410
    svg.append(
        f'<rect x="20" y="{p2_top}" width="1060" height="340" rx="6" fill="#161b22" stroke="#3fb950" stroke-width="1.5"/>'
    )
    svg.append(
        f'<text x="35" y="{p2_top + 24}" fill="#3fb950" font-size="14" font-weight="bold">🟢 EXPECTED CANONICAL WAVEFORM: Correct Forward Commutation Progression</text>'
    )
    svg.append(
        f'<text x="35" y="{p2_top + 42}" fill="#8b949e" font-size="11">Standard 6-step trapezoidal sequence with break-before-make. Continuous smooth rotation, zero DIAG faults.</text>'
    )

    # Step boundary lines (each step 140px = 10ms, total 6 steps = 840px)
    steps = [
        ("Step 0 (UH+VL)", 140),
        ("Step 1 (UH+WL)", 140),
        ("Step 2 (VH+WL)", 140),
        ("Step 3 (VH+UL)", 140),
        ("Step 4 (WH+UL)", 140),
        ("Step 5 (WH+VL)", 140),
    ]
    cur_bx = 160
    for idx, (label, width_step) in enumerate(steps):
        if idx > 0:
            svg.append(
                f'<line x1="{cur_bx}" y1="{p2_top + 50}" x2="{cur_bx}" y2="{p2_top + 325}" stroke="#30363d" stroke-width="1" stroke-dasharray="4,4"/>'
            )
        svg.append(
            f'<text x="{cur_bx + width_step // 2}" y="{p2_top + 62}" fill="#58a6ff" font-size="10" font-weight="bold" text-anchor="middle">{label}</text>'
        )
        cur_bx += width_step

    # Signals in Panel 2
    # Step 0: UH(1), VL(1)
    # Step 1: UH(1), WL(1)
    # Step 2: VH(1), WL(1)
    # Step 3: VH(1), UL(1)
    # Step 4: WH(1), UL(1)
    # Step 5: WH(1), VL(1)
    y2 = p2_top + 80
    draw_signal(
        y2,
        True,
        "#58a6ff",
        [(140, True), (140, True), (140, False), (140, False), (140, False), (140, False)],
        "UH",
        "GP16",
    )
    y2 += 28
    draw_signal(
        y2,
        True,
        "#79c0ff",
        [(140, False), (140, False), (140, False), (140, True), (140, True), (140, False)],
        "UL",
        "GP17",
    )
    y2 += 28
    draw_signal(
        y2,
        True,
        "#d29922",
        [(140, False), (140, False), (140, True), (140, True), (140, False), (140, False)],
        "VH",
        "GP18",
    )
    y2 += 28
    draw_signal(
        y2,
        True,
        "#e3b341",
        [(140, True), (140, False), (140, False), (140, False), (140, False), (140, True)],
        "VL",
        "GP19",
    )
    y2 += 28
    draw_signal(
        y2,
        True,
        "#bc8cff",
        [(140, False), (140, False), (140, False), (140, False), (140, True), (140, True)],
        "WH",
        "GP20",
    )
    y2 += 28
    draw_signal(
        y2,
        True,
        "#d2a8ff",
        [(140, False), (140, True), (140, True), (140, False), (140, False), (140, False)],
        "WL",
        "GP21",
    )
    y2 += 32
    draw_signal(y2, True, "#3fb950", [(840, False)], "DIAG", "GP22 [Clear]")

    svg.append("</svg>")
    return "\n".join(svg)


def main():
    workspace = Path(__file__).resolve().parent.parent
    out_dir = workspace / "recordings" / "previews"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_file = out_dir / "waveform_gliding_larva_522.svg"
    svg_content = generate_svg()
    out_file.write_text(svg_content, encoding="utf-8")
    print(f"Generated {out_file} successfully ({len(svg_content)} bytes)")


if __name__ == "__main__":
    main()
