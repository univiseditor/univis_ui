use super::*;

#[derive(Component)]
pub(super) struct ProfilerOverlayText;

pub(super) fn setup_profiler_overlay(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Text2d::new("UNIVIS PROFILER"),
        TextLayout {
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
        },
        TextFont {
            font: theme.text.font.inter_regular.clone().into(),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.94, 1.0)),
        Anchor::TOP_LEFT,
        Transform::from_xyz(0.0, 0.0, 50.0),
        Visibility::Hidden,
        ProfilerOverlayText,
    ));
}

pub(super) fn update_profiler_overlay_text(
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    mut query: Query<
        (&mut Text2d, &mut Transform, &mut Visibility, &mut TextColor),
        With<ProfilerOverlayText>,
    >,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let panel_size = overlay_panel_size(&settings);
    let center = overlay_center(window, settings.overlay_position, panel_size);
    let half = panel_size * 0.5;
    let panel_left = center.x - half.x;
    let panel_top = center.y + half.y;

    let latest = profiler.latest_frame().unwrap_or_default();
    let recent_avg = profiler.average_total_time_recent(30);
    let p95 = profiler.percentile_total_time(95.0);
    let frame_budget = frame_budget_ms(settings.target_fps);
    let budget_usage = if frame_budget > 0.0 {
        (profiler.total_time() / frame_budget) * 100.0
    } else {
        0.0
    };
    let (up_share, down_share, mat_share) = profiler.timing_share();
    let perf_state = performance_label(profiler.total_time(), frame_budget);

    let text_color_value = match perf_state {
        "GOOD" => Color::srgb(0.72, 0.96, 0.77),
        "WARN" => Color::srgb(1.0, 0.88, 0.62),
        _ => Color::srgb(1.0, 0.72, 0.72),
    };

    let overlay_text = format!(
        "UNIVIS PROFILER [{}]\n\
FPS now/avg: {:>5.1} / {:>5.1}   frame #{:>6}\n\
Frame ms now/avg/p95/max: {:>6.2} / {:>6.2} / {:>6.2} / {:>6.2}\n\
Budget @{:>3.0}fps: {:>6.2}ms  usage {:>6.1}%\n\
Breakdown up/down/mat: {:>4.1}% / {:>4.1}% / {:>4.1}%\n\
Nodes total/dirty/visible: {} / {} / {}\n\
Cache hit: {:>5.1}%   Material reuse: {:>5.1}%\n\
Scratch grow m/s/r: {} / {} / {}   peaks: {} / {} / {}\n\
Picking buckets/candidates: {} / {}\n\
History: {:>3} frames | Recent30 avg: {:>6.2}ms | Graph: {}\n\
Keys: F10 profiler  F11 overlay  F9 graph  F12 move",
        perf_state,
        latest.fps,
        profiler.average_fps(),
        latest.frame,
        profiler.total_time(),
        profiler.average_total_time(),
        p95,
        profiler.max_total_time(),
        settings.target_fps,
        frame_budget,
        budget_usage,
        up_share,
        down_share,
        mat_share,
        latest.node_count,
        latest.dirty_count,
        profiler.visible_nodes,
        profiler.cache_hit_rate(),
        profiler.material_reuse_rate(),
        profiler.measure_scratch_alloc_grows,
        profiler.solve_scratch_alloc_grows,
        profiler.solve_ref_alloc_grows,
        profiler.measure_scratch_peak,
        profiler.solve_scratch_peak,
        profiler.solve_ref_peak,
        profiler.picking_bucket_count,
        profiler.picking_candidate_count,
        profiler.frame_history.len(),
        recent_avg,
        if settings.show_graph { "ON" } else { "OFF" },
    );

    for (mut text, mut transform, mut visibility, mut text_color) in query.iter_mut() {
        if !settings.enabled || !settings.show_overlay {
            *visibility = Visibility::Hidden;
            continue;
        }

        *visibility = Visibility::Visible;
        transform.translation = Vec3::new(
            panel_left + DEFAULT_PANEL_PADDING,
            panel_top - DEFAULT_PANEL_PADDING,
            50.0,
        );
        **text = overlay_text.clone();
        text_color.0 = text_color_value;
    }
}

/// Render the profiling panel and graph using gizmos.
pub fn display_profiler_overlay(
    mut gizmos: Gizmos,
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    windows: Query<&Window>,
) {
    if !settings.enabled || !settings.show_overlay {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let panel_size = overlay_panel_size(&settings);
    let center = overlay_center(window, settings.overlay_position, panel_size);
    let half = panel_size * 0.5;
    let left = center.x - half.x;
    let right = center.x + half.x;
    let top = center.y + half.y;
    let bottom = center.y - half.y;
    let padding = DEFAULT_PANEL_PADDING;

    let background = Color::srgba(0.02, 0.03, 0.05, settings.panel_opacity);
    let border = Color::srgba(0.56, 0.64, 0.79, 0.95);
    let separator = Color::srgba(0.28, 0.35, 0.47, 0.9);

    gizmos.rect_2d(
        Isometry2d::from_xy(center.x, center.y),
        panel_size,
        background,
    );

    gizmos.line_2d(Vec2::new(left, top), Vec2::new(right, top), border);
    gizmos.line_2d(Vec2::new(right, top), Vec2::new(right, bottom), border);
    gizmos.line_2d(Vec2::new(right, bottom), Vec2::new(left, bottom), border);
    gizmos.line_2d(Vec2::new(left, bottom), Vec2::new(left, top), border);

    let text_top = top - padding;
    let text_bottom = text_top - DEFAULT_TEXT_SECTION_HEIGHT;
    let bars_top = text_bottom - DEFAULT_SECTION_GAP;
    let bars_bottom = bars_top - DEFAULT_BARS_SECTION_HEIGHT;

    gizmos.line_2d(
        Vec2::new(left + padding, text_bottom - (DEFAULT_SECTION_GAP * 0.5)),
        Vec2::new(right - padding, text_bottom - (DEFAULT_SECTION_GAP * 0.5)),
        separator,
    );
    if settings.show_graph {
        gizmos.line_2d(
            Vec2::new(left + padding, bars_bottom - (DEFAULT_SECTION_GAP * 0.5)),
            Vec2::new(right - padding, bars_bottom - (DEFAULT_SECTION_GAP * 0.5)),
            separator,
        );
    }

    let bar_left = left + padding;
    let bar_width = panel_size.x - (padding * 2.0);
    let bar_height = 10.0;
    let total_ms = profiler.total_time().max(0.0001);

    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 14.0,
        bar_width,
        bar_height,
        (profiler.upward_pass_time / total_ms) as f32,
        Color::srgb(0.26, 0.64, 1.0),
    );
    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 32.0,
        bar_width,
        bar_height,
        (profiler.downward_pass_time / total_ms) as f32,
        Color::srgb(0.2, 0.84, 0.45),
    );
    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 50.0,
        bar_width,
        bar_height,
        (profiler.material_update_time / total_ms) as f32,
        Color::srgb(1.0, 0.67, 0.21),
    );

    if settings.show_graph {
        let graph_width = panel_size.x - (padding * 2.0);
        let graph_left = left + padding;
        let graph_top = bars_bottom - DEFAULT_SECTION_GAP;
        let graph_bottom = bottom + padding;
        let graph_height = (graph_top - graph_bottom).max(10.0);

        gizmos.rect_2d(
            Isometry2d::from_xy(
                graph_left + graph_width * 0.5,
                graph_bottom + graph_height * 0.5,
            ),
            Vec2::new(graph_width, graph_height),
            Color::srgba(0.06, 0.08, 0.12, 0.92),
        );

        let grid_color = Color::srgba(0.2, 0.26, 0.34, 0.65);
        for row in 0..=4 {
            let t = row as f32 / 4.0;
            let y = graph_bottom + t * graph_height;
            gizmos.line_2d(
                Vec2::new(graph_left, y),
                Vec2::new(graph_left + graph_width, y),
                grid_color,
            );
        }

        let sample_count = settings
            .graph_samples
            .max(2)
            .min(profiler.frame_history.len());
        if sample_count >= 2 {
            let slice = &profiler.frame_history[profiler.frame_history.len() - sample_count..];
            let frame_budget = frame_budget_ms(settings.target_fps);
            let max_ms = slice
                .iter()
                .map(|s| s.total_layout_ms)
                .fold(frame_budget, f64::max)
                .max(1.0);

            let budget_y =
                graph_bottom + ((frame_budget / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
            gizmos.line_2d(
                Vec2::new(graph_left, budget_y),
                Vec2::new(graph_left + graph_width, budget_y),
                Color::srgba(0.96, 0.84, 0.28, 0.95),
            );

            for i in 1..slice.len() {
                let prev = slice[i - 1].total_layout_ms;
                let curr = slice[i].total_layout_ms;
                let x0 = graph_left + ((i - 1) as f32 / (slice.len() - 1) as f32) * graph_width;
                let x1 = graph_left + (i as f32 / (slice.len() - 1) as f32) * graph_width;
                let y0 = graph_bottom + ((prev / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
                let y1 = graph_bottom + ((curr / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
                gizmos.line_2d(
                    Vec2::new(x0, y0),
                    Vec2::new(x1, y1),
                    performance_color(curr, frame_budget),
                );
            }
        }
    }
}

fn frame_budget_ms(target_fps: f32) -> f64 {
    let fps = target_fps.max(1.0) as f64;
    1000.0 / fps
}

fn overlay_panel_size(settings: &ProfilerSettings) -> Vec2 {
    let base_height = (DEFAULT_PANEL_PADDING * 2.0)
        + DEFAULT_TEXT_SECTION_HEIGHT
        + DEFAULT_SECTION_GAP
        + DEFAULT_BARS_SECTION_HEIGHT;

    if settings.show_graph {
        Vec2::new(
            DEFAULT_PANEL_WIDTH,
            base_height + DEFAULT_SECTION_GAP + DEFAULT_GRAPH_HEIGHT,
        )
    } else {
        Vec2::new(DEFAULT_PANEL_WIDTH, base_height)
    }
}

fn overlay_center(window: &Window, position: OverlayPosition, panel_size: Vec2) -> Vec2 {
    let margin = 14.0;
    let half_window = Vec2::new(window.width() * 0.5, window.height() * 0.5);
    let half_panel = panel_size * 0.5;

    match position {
        OverlayPosition::TopLeft => Vec2::new(
            -half_window.x + margin + half_panel.x,
            half_window.y - margin - half_panel.y,
        ),
        OverlayPosition::TopRight => Vec2::new(
            half_window.x - margin - half_panel.x,
            half_window.y - margin - half_panel.y,
        ),
        OverlayPosition::BottomLeft => Vec2::new(
            -half_window.x + margin + half_panel.x,
            -half_window.y + margin + half_panel.y,
        ),
        OverlayPosition::BottomRight => Vec2::new(
            half_window.x - margin - half_panel.x,
            -half_window.y + margin + half_panel.y,
        ),
    }
}

fn draw_horizontal_bar(
    gizmos: &mut Gizmos,
    left: f32,
    center_y: f32,
    width: f32,
    height: f32,
    ratio: f32,
    fill_color: Color,
) {
    let bg = Color::srgba(0.15, 0.19, 0.26, 0.92);
    gizmos.rect_2d(
        Isometry2d::from_xy(left + width * 0.5, center_y),
        Vec2::new(width, height),
        bg,
    );

    let fill_ratio = ratio.clamp(0.0, 1.0);
    if fill_ratio <= 0.0 {
        return;
    }

    let fill_width = width * fill_ratio;
    gizmos.rect_2d(
        Isometry2d::from_xy(left + fill_width * 0.5, center_y),
        Vec2::new(fill_width, height),
        fill_color,
    );
}

fn performance_label(total_ms: f64, frame_budget: f64) -> &'static str {
    if frame_budget <= f64::EPSILON {
        return "N/A";
    }
    if total_ms <= frame_budget * 0.7 {
        "GOOD"
    } else if total_ms <= frame_budget {
        "WARN"
    } else {
        "HOT"
    }
}

fn performance_color(total_ms: f64, frame_budget: f64) -> Color {
    match performance_label(total_ms, frame_budget) {
        "GOOD" => Color::srgb(0.24, 0.86, 0.49),
        "WARN" => Color::srgb(1.0, 0.74, 0.24),
        _ => Color::srgb(0.98, 0.33, 0.33),
    }
}
