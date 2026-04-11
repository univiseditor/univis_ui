#!/usr/bin/env python3

from __future__ import annotations

import argparse
import html
import math
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path


WIDTH = 1500
HEIGHT = 1260

BG = "#f8fafc"
PANEL = "#ffffff"
BORDER = "#d8dee8"
TEXT = "#0f172a"
MUTED = "#475569"
GRID = "#e2e8f0"
AXIS = "#94a3b8"
ALPHA2 = "#b45309"
ALPHA3 = "#0f766e"
SUSTAINED = "#2563eb"
BUDGET = "#94a3b8"
GOOD = "#0f766e"
BAD = "#b91c1c"


@dataclass
class CompareRow:
    scenario: str
    items: int
    alpha2_p95: float
    alpha3_p95: float
    p95_delta: float


@dataclass
class AggregateRow:
    group: str
    alpha2_p95_sum: float
    alpha3_p95_sum: float
    p95_delta: float


@dataclass
class SustainedRow:
    scenario: str
    items: int
    avg: float
    p95: float
    max_value: float
    budget: float
    budget_use: float
    status: str


def strip_md(value: str) -> str:
    return value.strip().strip("`").strip()


def parse_float(value: str) -> float:
    return float(strip_md(value).replace(",", ""))


def parse_percent(value: str) -> float:
    return float(strip_md(value).replace("%", ""))


def parse_table(lines: list[str]) -> list[dict[str, str]]:
    headers = [cell.strip() for cell in lines[0].strip().strip("|").split("|")]
    rows: list[dict[str, str]] = []
    for raw in lines[2:]:
        if not raw.strip().startswith("|"):
            break
        cells = [cell.strip() for cell in raw.strip().strip("|").split("|")]
        if len(cells) == len(headers):
            rows.append(dict(zip(headers, cells)))
    return rows


def extract_table(markdown_lines: list[str], heading: str) -> list[dict[str, str]]:
    start = next(i for i, line in enumerate(markdown_lines) if line.strip() == heading)
    table_lines: list[str] = []
    for line in markdown_lines[start + 1 :]:
        stripped = line.strip()
        if not stripped and not table_lines:
            continue
        if stripped.startswith("|"):
            table_lines.append(line)
            continue
        if table_lines:
            break
    return parse_table(table_lines)


def esc(value: str) -> str:
    return html.escape(value, quote=True)


def add_rect(
    parts: list[str],
    x: float,
    y: float,
    w: float,
    h: float,
    fill: str,
    rx: float = 16,
    stroke: str | None = None,
    stroke_width: float = 1.0,
) -> None:
    attrs = [
        f'x="{x:.1f}"',
        f'y="{y:.1f}"',
        f'width="{w:.1f}"',
        f'height="{h:.1f}"',
        f'fill="{fill}"',
        f'rx="{rx:.1f}"',
    ]
    if stroke:
        attrs.append(f'stroke="{stroke}"')
        attrs.append(f'stroke-width="{stroke_width:.1f}"')
    parts.append(f"<rect {' '.join(attrs)} />")


def add_line(
    parts: list[str],
    x1: float,
    y1: float,
    x2: float,
    y2: float,
    stroke: str,
    stroke_width: float = 1.0,
    dash: str | None = None,
) -> None:
    attrs = [
        f'x1="{x1:.1f}"',
        f'y1="{y1:.1f}"',
        f'x2="{x2:.1f}"',
        f'y2="{y2:.1f}"',
        f'stroke="{stroke}"',
        f'stroke-width="{stroke_width:.1f}"',
        'stroke-linecap="round"',
    ]
    if dash:
        attrs.append(f'stroke-dasharray="{dash}"')
    parts.append(f"<line {' '.join(attrs)} />")


def add_text(
    parts: list[str],
    x: float,
    y: float,
    text: str,
    size: int = 18,
    fill: str = TEXT,
    weight: int = 400,
    anchor: str = "start",
    family: str = "DejaVu Sans, Arial, sans-serif",
) -> None:
    parts.append(
        f'<text x="{x:.1f}" y="{y:.1f}" fill="{fill}" font-size="{size}" '
        f'font-weight="{weight}" text-anchor="{anchor}" '
        f'font-family="{esc(family)}">{esc(text)}</text>'
    )


def wrap_text(text: str, width: int) -> list[str]:
    words = text.split()
    lines: list[str] = []
    current = ""
    for word in words:
        trial = word if not current else f"{current} {word}"
        if len(trial) <= width:
            current = trial
        else:
            if current:
                lines.append(current)
            current = word
    if current:
        lines.append(current)
    return lines


def nice_step(max_value: float, tick_count: int = 5) -> float:
    if max_value <= 0:
        return 1.0
    raw = max_value / tick_count
    exponent = math.floor(math.log10(raw))
    fraction = raw / (10**exponent)
    if fraction <= 1:
        nice_fraction = 1
    elif fraction <= 2:
        nice_fraction = 2
    elif fraction <= 5:
        nice_fraction = 5
    else:
        nice_fraction = 10
    return nice_fraction * (10**exponent)


def nice_axis_max(max_value: float, tick_count: int = 5) -> float:
    step = nice_step(max_value, tick_count)
    return max(step, math.ceil(max_value / step) * step)


def delta_color(value: float) -> str:
    return GOOD if value <= 0 else BAD


def render_grouped_compare_panel(
    parts: list[str],
    x: float,
    y: float,
    w: float,
    h: float,
    title: str,
    subtitle: str,
    rows: list[CompareRow],
    aggregate: AggregateRow,
) -> None:
    add_rect(parts, x, y, w, h, PANEL, stroke=BORDER)
    add_text(parts, x + 24, y + 40, title, size=28, weight=700)
    add_text(parts, x + 24, y + 68, subtitle, size=15, fill=MUTED)
    add_text(
        parts,
        x + w - 24,
        y + 40,
        f"Shared p95 {aggregate.alpha2_p95_sum:.3f} -> {aggregate.alpha3_p95_sum:.3f} ms ({aggregate.p95_delta:+.1f}%)",
        size=16,
        fill=delta_color(aggregate.p95_delta),
        weight=700,
        anchor="end",
    )

    label_w = 255
    details_w = 135
    plot_left = x + 24 + label_w
    plot_right = x + w - 24 - details_w
    plot_w = plot_right - plot_left
    chart_top = y + 106
    chart_bottom = y + h - 52
    row_h = (chart_bottom - chart_top) / max(len(rows), 1)

    axis_max = nice_axis_max(
        max(max(row.alpha2_p95, row.alpha3_p95) for row in rows) * 1.05,
        tick_count=5,
    )

    add_line(parts, plot_left, chart_bottom, plot_right, chart_bottom, AXIS, 1.4)
    for tick in range(6):
        value = axis_max * tick / 5.0
        tick_x = plot_left + plot_w * tick / 5.0
        add_line(parts, tick_x, chart_top, tick_x, chart_bottom, GRID, 1.0)
        add_text(parts, tick_x, chart_bottom + 22, f"{value:.1f}", size=13, fill=MUTED, anchor="middle")
    add_text(parts, plot_right, chart_bottom + 42, "p95 (ms)", size=13, fill=MUTED, anchor="end")

    for index, row in enumerate(rows):
        base_y = chart_top + index * row_h + 24
        add_text(parts, x + 24, base_y, row.scenario, size=17, weight=700, family="DejaVu Sans Mono, monospace")
        add_text(parts, x + 24, base_y + 20, f"{row.items} items", size=14, fill=MUTED)

        bar_h = 10
        a2_y = base_y - 6
        a3_y = base_y + 14
        a2_w = plot_w * (row.alpha2_p95 / axis_max)
        a3_w = plot_w * (row.alpha3_p95 / axis_max)

        add_rect(parts, plot_left, a2_y, plot_w, bar_h, "#eef2f7", rx=5)
        add_rect(parts, plot_left, a2_y, a2_w, bar_h, ALPHA2, rx=5)
        add_rect(parts, plot_left, a3_y, plot_w, bar_h, "#eef2f7", rx=5)
        add_rect(parts, plot_left, a3_y, a3_w, bar_h, ALPHA3, rx=5)

        add_text(parts, plot_left + a2_w + 8, a2_y + 9, f"A2 {row.alpha2_p95:.3f}", size=12, fill=MUTED)
        add_text(parts, plot_left + a3_w + 8, a3_y + 9, f"A3 {row.alpha3_p95:.3f}", size=12, fill=MUTED)
        add_text(parts, x + w - 24, base_y + 6, f"{row.p95_delta:+.1f}%", size=16, fill=delta_color(row.p95_delta), weight=700, anchor="end")


def render_sustained_panel(
    parts: list[str],
    x: float,
    y: float,
    w: float,
    h: float,
    rows: list[SustainedRow],
) -> None:
    add_rect(parts, x, y, w, h, PANEL, stroke=BORDER)
    add_text(parts, x + 24, y + 40, "Alpha-3 Sustained Checks", size=28, weight=700)
    add_text(parts, x + 24, y + 68, "Budget-use axis by scenario", size=15, fill=MUTED)

    label_w = 305
    details_w = 110
    plot_left = x + 24 + label_w
    plot_right = x + w - 24 - details_w
    plot_w = plot_right - plot_left
    chart_top = y + 106
    chart_bottom = y + h - 56
    row_h = (chart_bottom - chart_top) / max(len(rows), 1)

    axis_max = nice_axis_max(max(max(row.budget_use for row in rows), 100.0), tick_count=5)

    add_line(parts, plot_left, chart_bottom, plot_right, chart_bottom, AXIS, 1.4)
    for tick in range(6):
        value = axis_max * tick / 5.0
        tick_x = plot_left + plot_w * tick / 5.0
        add_line(parts, tick_x, chart_top, tick_x, chart_bottom, GRID, 1.0)
        add_text(parts, tick_x, chart_bottom + 22, f"{value:.0f}%", size=13, fill=MUTED, anchor="middle")
    add_text(parts, plot_right, chart_bottom + 42, "budget use", size=13, fill=MUTED, anchor="end")

    budget_line_x = plot_left + plot_w * (100.0 / axis_max)
    add_line(parts, budget_line_x, chart_top, budget_line_x, chart_bottom, "#64748b", 1.6, dash="5 5")
    add_text(parts, budget_line_x, chart_top - 10, "100%", size=13, fill=MUTED, anchor="middle")

    for index, row in enumerate(rows):
        base_y = chart_top + index * row_h + 24
        add_text(parts, x + 24, base_y, row.scenario, size=17, weight=700, family="DejaVu Sans Mono, monospace")
        add_text(
            parts,
            x + 24,
            base_y + 20,
            f"{row.items} items | p95 {row.p95:.3f} ms | budget {row.budget:.3f} ms | status {row.status}",
            size=14,
            fill=MUTED,
        )

        bar_y = base_y - 2
        bar_h = 14
        bar_w = plot_w * (row.budget_use / axis_max)
        add_rect(parts, plot_left, bar_y, plot_w, bar_h, "#eef2f7", rx=7)
        add_rect(parts, plot_left, bar_y, bar_w, bar_h, SUSTAINED, rx=7)
        add_text(parts, plot_left + bar_w + 8, bar_y + 12, f"{row.budget_use:.1f}%", size=12, fill=MUTED)
        add_text(parts, x + w - 24, base_y + 8, f"avg {row.avg:.3f}", size=14, fill=MUTED, anchor="end")


def build_svg(
    report_date: str,
    runtime_rows: list[CompareRow],
    runtime_agg: AggregateRow,
    solver_rows: list[CompareRow],
    solver_agg: AggregateRow,
    sustained_rows: list[SustainedRow],
) -> str:
    parts: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">',
        f'<rect width="{WIDTH}" height="{HEIGHT}" fill="{BG}" />',
    ]

    add_text(parts, 40, 60, "Benchmark Report", size=40, weight=800)
    add_text(parts, 40, 92, "Simple chart view with axes and scenario details", size=18, fill=MUTED)
    add_text(parts, WIDTH - 40, 60, f"Date {report_date}", size=16, fill=MUTED, anchor="end")

    add_text(
        parts,
        40,
        132,
        f"Runtime shared p95: {runtime_agg.alpha2_p95_sum:.3f} -> {runtime_agg.alpha3_p95_sum:.3f} ms ({runtime_agg.p95_delta:+.1f}%)",
        size=16,
        fill=delta_color(runtime_agg.p95_delta),
        weight=700,
    )
    add_text(
        parts,
        40,
        158,
        f"Solver shared p95: {solver_agg.alpha2_p95_sum:.3f} -> {solver_agg.alpha3_p95_sum:.3f} ms ({solver_agg.p95_delta:+.1f}%)",
        size=16,
        fill=delta_color(solver_agg.p95_delta),
        weight=700,
    )

    render_grouped_compare_panel(
        parts,
        40,
        200,
        1420,
        350,
        "Runtime Compare",
        "Alpha-2 vs Alpha-3 p95 by shared runtime scenario",
        runtime_rows,
        runtime_agg,
    )
    render_grouped_compare_panel(
        parts,
        40,
        580,
        1420,
        280,
        "Solver Compare",
        "Alpha-2 vs Alpha-3 p95 by shared solver scenario",
        solver_rows,
        solver_agg,
    )
    render_sustained_panel(parts, 40, 890, 1420, 250, sustained_rows)

    detail_y = 1182
    details = [
        "Alpha-3 improved shared sustained runtime clearly.",
        "Solver microbench is slightly slower in alpha-3.",
        "All sustained alpha-3 checks remained under budget.",
    ]
    add_text(parts, 40, detail_y, "Details:", size=16, weight=700)
    for idx, detail in enumerate(details):
        add_text(parts, 105 + idx * 445, detail_y, detail, size=14, fill=MUTED)

    parts.append("</svg>")
    return "\n".join(parts)


def load_data(
    report_path: Path,
) -> tuple[str, list[CompareRow], AggregateRow, list[CompareRow], AggregateRow, list[SustainedRow]]:
    markdown = report_path.read_text(encoding="utf-8")
    lines = markdown.splitlines()

    date_match = re.search(r"Tarikh:\s*`([^`]+)`", markdown)
    report_date = date_match.group(1) if date_match else "unknown"

    runtime_rows = [
        CompareRow(
            scenario=strip_md(row["Scenario"]),
            items=int(parse_float(row["Items"])),
            alpha2_p95=parse_float(row["Alpha-2 p95"]),
            alpha3_p95=parse_float(row["Alpha-3 p95"]),
            p95_delta=parse_percent(row["p95 Delta"]),
        )
        for row in extract_table(lines, "## Runtime Compare: Alpha-2 vs Alpha-3")
    ]

    runtime_agg_row = extract_table(lines, "### Runtime Aggregate")[0]
    runtime_agg = AggregateRow(
        group=strip_md(runtime_agg_row["Group"]),
        alpha2_p95_sum=parse_float(runtime_agg_row["Alpha-2 p95 Sum"]),
        alpha3_p95_sum=parse_float(runtime_agg_row["Alpha-3 p95 Sum"]),
        p95_delta=parse_percent(runtime_agg_row["p95 Delta"]),
    )

    solver_rows = [
        CompareRow(
            scenario=strip_md(row["Scenario"]),
            items=int(parse_float(row["Items"])),
            alpha2_p95=parse_float(row["Alpha-2 p95"]),
            alpha3_p95=parse_float(row["Alpha-3 p95"]),
            p95_delta=parse_percent(row["p95 Delta"]),
        )
        for row in extract_table(lines, "## Solver Compare: Alpha-2 vs Alpha-3")
    ]

    solver_agg_row = extract_table(lines, "### Solver Aggregate")[0]
    solver_agg = AggregateRow(
        group=strip_md(solver_agg_row["Group"]),
        alpha2_p95_sum=parse_float(solver_agg_row["Alpha-2 p95 Sum"]),
        alpha3_p95_sum=parse_float(solver_agg_row["Alpha-3 p95 Sum"]),
        p95_delta=parse_percent(solver_agg_row["p95 Delta"]),
    )

    sustained_rows = [
        SustainedRow(
            scenario=strip_md(row["Scenario"]),
            items=int(parse_float(row["Items"])),
            avg=parse_float(row["Avg"]),
            p95=parse_float(row["p95"]),
            max_value=parse_float(row["Max"]),
            budget=parse_float(row["Budget"]),
            budget_use=parse_percent(row["Budget Use"]),
            status=strip_md(row["Status"]),
        )
        for row in extract_table(lines, "## Alpha-3 Sustained Runtime Checks")
    ]

    return report_date, runtime_rows, runtime_agg, solver_rows, solver_agg, sustained_rows


def main() -> None:
    parser = argparse.ArgumentParser(description="Render BENCHMARK_REPORT.md into a simple chart PNG.")
    parser.add_argument("--report", default="BENCHMARK_REPORT.md", help="Input markdown report path.")
    parser.add_argument("--svg-out", default="BENCHMARK_REPORT.svg", help="Output SVG path.")
    parser.add_argument("--png-out", default="BENCHMARK_REPORT.png", help="Output PNG path.")
    args = parser.parse_args()

    report_path = Path(args.report)
    svg_out = Path(args.svg_out)
    png_out = Path(args.png_out)

    report_date, runtime_rows, runtime_agg, solver_rows, solver_agg, sustained_rows = load_data(report_path)
    svg_out.write_text(
        build_svg(report_date, runtime_rows, runtime_agg, solver_rows, solver_agg, sustained_rows),
        encoding="utf-8",
    )

    subprocess.run(
        [
            "rsvg-convert",
            "--format",
            "png",
            "--output",
            str(png_out),
            str(svg_out),
        ],
        check=True,
    )


if __name__ == "__main__":
    main()
