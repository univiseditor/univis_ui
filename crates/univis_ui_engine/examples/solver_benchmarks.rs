//! Solver-focused performance harness.
//!
//! Related docs:
//! - `docs/src/en/performance/benchmarks.md`
//! - `docs/src/ar/performance/benchmarks.md`

use std::env;
use std::time::Instant;

use bevy::prelude::*;
use univis_ui_engine::layout::core::solver::{
    SolverConfig, SolverItem, SolverResult, solve_flex_layout,
};
use univis_ui_engine::layout::geometry::BoxConstraints;
use univis_ui_engine::layout::solver_types::{SolverSizeMode, SolverSpec};
use univis_ui_engine::prelude::*;

const DEFAULT_WARMUP: usize = 48;
const DEFAULT_ITERATIONS: usize = 240;

#[derive(Clone)]
struct SolverWorkload {
    name: &'static str,
    item_count: usize,
    budget_ms: f64,
    config: SolverConfig,
    constraints: BoxConstraints,
    specs: Vec<SolverSpec>,
    margins: Vec<USides>,
}

#[derive(Clone, Copy)]
struct BenchmarkSummary {
    average_ms: f64,
    p95_ms: f64,
    max_ms: f64,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let enforce_budgets = args.iter().any(|arg| arg == "--check");
    let warmup = parse_env_usize("UNIVIS_PERF_WARMUP", DEFAULT_WARMUP);
    let iterations = parse_env_usize("UNIVIS_PERF_ITERATIONS", DEFAULT_ITERATIONS);

    let workloads = vec![
        dense_row_workload(),
        wrap_cards_workload(),
        grid_dashboard_workload(),
    ];

    println!("Univis solver benchmark");
    println!(
        "warmup={} iterations={} enforce_budgets={}",
        warmup, iterations, enforce_budgets
    );
    println!(
        "{:<24} {:>8} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "scenario", "items", "avg_ms", "p95_ms", "max_ms", "budget", "status"
    );

    let mut failed = Vec::new();

    for workload in &workloads {
        let summary = benchmark_workload(workload, warmup, iterations);
        let within_budget = summary.p95_ms <= workload.budget_ms;
        let status = if within_budget { "ok" } else { "over" };

        println!(
            "{:<24} {:>8} {:>10.3} {:>10.3} {:>10.3} {:>10.3} {:>8}",
            workload.name,
            workload.item_count,
            summary.average_ms,
            summary.p95_ms,
            summary.max_ms,
            workload.budget_ms,
            status
        );

        if enforce_budgets && !within_budget {
            failed.push((workload.name, summary, workload.budget_ms));
        }
    }

    if !failed.is_empty() {
        eprintln!();
        eprintln!("Budget failures:");
        for (name, summary, budget_ms) in failed {
            eprintln!(
                "- {}: p95 {:.3}ms > budget {:.3}ms (avg {:.3}ms, max {:.3}ms)",
                name, summary.p95_ms, budget_ms, summary.average_ms, summary.max_ms
            );
        }
        std::process::exit(1);
    }
}

fn parse_env_usize(key: &str, default_value: usize) -> usize {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn benchmark_workload(
    workload: &SolverWorkload,
    warmup: usize,
    iterations: usize,
) -> BenchmarkSummary {
    for _ in 0..warmup {
        let _ = run_solver_workload(workload);
    }

    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = run_solver_workload(workload);
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    samples.sort_by(|left, right| left.total_cmp(right));

    let average_ms = samples.iter().sum::<f64>() / samples.len() as f64;
    let p95_index = ((samples.len() - 1) as f64 * 0.95).round() as usize;
    let p95_ms = samples[p95_index];
    let max_ms = *samples.last().unwrap_or(&0.0);

    BenchmarkSummary {
        average_ms,
        p95_ms,
        max_ms,
    }
}

fn run_solver_workload(workload: &SolverWorkload) -> Vec2 {
    let mut results = vec![SolverResult::default(); workload.specs.len()];
    let mut items: Vec<SolverItem<'_>> = workload
        .specs
        .iter()
        .copied()
        .zip(results.iter_mut())
        .zip(workload.margins.iter().copied())
        .map(|((spec, result), margin)| SolverItem {
            spec,
            result,
            margin,
        })
        .collect();

    solve_flex_layout(&workload.config, workload.constraints, &mut items)
}

fn dense_row_workload() -> SolverWorkload {
    let item_count = 512;
    let config = SolverConfig {
        layout: ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Stretch,
            justify_content: UJustifyContent::Start,
            ..default()
        },
        gap: 6.0,
        row_gap: None,
        column_gap: None,
        padding: USides::axes(18.0, 12.0),
        grid_columns: 1,
        justify_items: None,
        align_content: None,
        flex_wrap: UFlexWrap::NoWrap,
        flex_align_content: None,
        grid_template_columns: Vec::new(),
        grid_template_rows: Vec::new(),
        grid_auto_flow: UGridAutoFlow::Row,
        grid_auto_rows: UTrackSize::Auto,
        grid_auto_columns: UTrackSize::Auto,
        width_mode: SolverSizeMode::Fixed,
        height_mode: SolverSizeMode::Fixed,
    };

    let specs = (0..item_count)
        .map(|index| SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 28.0 + (index % 5) as f32 * 8.0,
            width_flex: 0.0,
            min_width: 18.0,
            max_width: 96.0,
            height_mode: SolverSizeMode::Auto,
            height_val: 18.0 + (index % 3) as f32 * 4.0,
            height_flex: 0.0,
            min_height: 12.0,
            max_height: 42.0,
            position_type: UPositionType::Relative,
            left: UVal::Auto,
            right: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            align_self: None,
            align_self_ext: None,
            justify_self_ext: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
            flex_grow: Some(((index % 3) + 1) as f32 * 0.4),
            flex_shrink: Some(1.0),
            flex_basis: Some(UVal::Px(24.0 + (index % 4) as f32 * 6.0)),
            grid_column_start: None,
            grid_column_span: 1,
            grid_row_start: None,
            grid_row_span: 1,
            order: index as i32,
        })
        .collect();

    SolverWorkload {
        name: "dense_row_512",
        item_count,
        budget_ms: 1.000,
        config,
        constraints: BoxConstraints::tight(Vec2::new(4096.0, 96.0)),
        specs,
        margins: vec![USides::axes(2.0, 1.0); item_count],
    }
}

fn wrap_cards_workload() -> SolverWorkload {
    let item_count = 400;
    let config = SolverConfig {
        layout: ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Start,
            justify_content: UJustifyContent::Start,
            ..default()
        },
        gap: 12.0,
        row_gap: Some(14.0),
        column_gap: Some(12.0),
        padding: USides::axes(20.0, 16.0),
        grid_columns: 1,
        justify_items: None,
        align_content: Some(UContentAlignExt::Start),
        flex_wrap: UFlexWrap::Wrap,
        flex_align_content: Some(UContentAlignExt::Start),
        grid_template_columns: Vec::new(),
        grid_template_rows: Vec::new(),
        grid_auto_flow: UGridAutoFlow::Row,
        grid_auto_rows: UTrackSize::Auto,
        grid_auto_columns: UTrackSize::Auto,
        width_mode: SolverSizeMode::Fixed,
        height_mode: SolverSizeMode::Fixed,
    };

    let specs = (0..item_count)
        .map(|index| SolverSpec {
            width_mode: SolverSizeMode::Fixed,
            width_val: 132.0 + (index % 4) as f32 * 10.0,
            width_flex: 0.0,
            min_width: 96.0,
            max_width: 188.0,
            height_mode: SolverSizeMode::Content,
            height_val: 48.0 + (index % 5) as f32 * 4.0,
            height_flex: 0.0,
            min_height: 40.0,
            max_height: 92.0,
            position_type: UPositionType::Relative,
            left: UVal::Auto,
            right: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            align_self: None,
            align_self_ext: None,
            justify_self_ext: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
            flex_grow: Some(1.0 + (index % 3) as f32 * 0.25),
            flex_shrink: Some(1.0),
            flex_basis: Some(UVal::Px(120.0)),
            grid_column_start: None,
            grid_column_span: 1,
            grid_row_start: None,
            grid_row_span: 1,
            order: index as i32,
        })
        .collect();

    SolverWorkload {
        name: "wrap_cards_400",
        item_count,
        budget_ms: 1.400,
        config,
        constraints: BoxConstraints::tight(Vec2::new(1440.0, 1080.0)),
        specs,
        margins: vec![USides::all(3.0); item_count],
    }
}

fn grid_dashboard_workload() -> SolverWorkload {
    let columns = 14usize;
    let rows = 14usize;
    let item_count = columns * rows;
    let config = SolverConfig {
        layout: ULayout {
            display: UDisplay::Grid,
            ..default()
        },
        gap: 10.0,
        row_gap: Some(10.0),
        column_gap: Some(10.0),
        padding: USides::axes(18.0, 18.0),
        grid_columns: columns as u32,
        justify_items: Some(UAlignItemsExt::Stretch),
        align_content: Some(UContentAlignExt::Start),
        flex_wrap: UFlexWrap::NoWrap,
        flex_align_content: None,
        grid_template_columns: vec![UTrackSize::Fr(1.0); columns],
        grid_template_rows: vec![UTrackSize::Px(54.0); rows],
        grid_auto_flow: UGridAutoFlow::Row,
        grid_auto_rows: UTrackSize::Px(54.0),
        grid_auto_columns: UTrackSize::Fr(1.0),
        width_mode: SolverSizeMode::Fixed,
        height_mode: SolverSizeMode::Fixed,
    };

    let specs = (0..item_count)
        .map(|index| SolverSpec {
            width_mode: SolverSizeMode::Auto,
            width_val: 96.0 + (index % 3) as f32 * 6.0,
            width_flex: 0.0,
            min_width: 64.0,
            max_width: 180.0,
            height_mode: SolverSizeMode::Auto,
            height_val: 42.0 + (index % 4) as f32 * 3.0,
            height_flex: 0.0,
            min_height: 36.0,
            max_height: 72.0,
            position_type: UPositionType::Relative,
            left: UVal::Auto,
            right: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            align_self: None,
            align_self_ext: None,
            justify_self_ext: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
            flex_grow: None,
            flex_shrink: Some(1.0),
            flex_basis: None,
            grid_column_start: None,
            grid_column_span: if index % 17 == 0 { 2 } else { 1 },
            grid_row_start: None,
            grid_row_span: if index % 29 == 0 { 2 } else { 1 },
            order: index as i32,
        })
        .collect();

    SolverWorkload {
        name: "grid_dashboard_196",
        item_count,
        budget_ms: 1.800,
        config,
        constraints: BoxConstraints::tight(Vec2::new(1680.0, 960.0)),
        specs,
        margins: vec![USides::all(2.0); item_count],
    }
}
