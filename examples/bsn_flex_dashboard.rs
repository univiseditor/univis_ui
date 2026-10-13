//! Modern SaaS Cloud & Analytics Dashboard - Flexbox Layout with BSN.
//!
//! Visual Theme: Sleek contemporary cloud infrastructure dashboard (Indigo & Emerald).
//!
//! Flexbox features demonstrated:
//! - Row & Column nested hierarchies (`UFlexDirection::Row` / `UFlexDirection::Column`)
//! - Space distribution: `UJustifyContent::SpaceBetween`, `UJustifyContent::Center`
//! - Cross-axis alignment: `UAlignItems::Center`, `UAlignItems::Stretch`
//! - Proportional distribution using `USelf { flex_grow: { Some(...) } }`
//! - Spacing with uniform `gap`
//! - Inline BSN event observers `on(...)` for real-time cluster refresh interaction
//! - Vector Lucide icons for cluster telemetry, servers, regions, and SLA status
//! - Real-time SDF text outlines on KPI metrics and platform branding
//! - WrapToParent responsive text layout without overflow

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource, Default)]
#[allow(dead_code)]
struct DashboardState {
    refresh_count: u32,
    active_region: &'static str,
}

#[derive(Component, Clone, Copy, Default)]
struct RefreshFeedbackText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Cloud Analytics Dashboard (Flexbox & BSN)".into(),
                resolution: (1260, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .insert_resource(DashboardState {
            refresh_count: 0,
            active_region: "US-EAST (N. VIRGINIA)",
        })
        .add_systems(Startup, setup_dashboard)
        .add_systems(Update, update_dashboard_state)
        .run();
}

fn setup_dashboard(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(dashboard_root_scene(&theme));
}

/// Root scene assembling the dashboard.
fn dashboard_root_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let body_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.05, 0.08),
            padding: USides::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
        Children [
            (top_navbar(icon_font.clone(), body_font.clone())),
            (metrics_kpi_row(icon_font.clone(), body_font.clone())),
            (middle_split_area(icon_font.clone(), body_font.clone())),
            (footer_status_bar(icon_font, body_font)),
        ]
    }
}

/// Top navigation bar with workspace branding, search chip, and user profile.
fn top_navbar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(58.0),
            background_color: Color::srgba(0.08, 0.1, 0.16, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(20.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                // Brand Logo & Org
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(34.0),
                            height: UVal::Px(34.0),
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::SERVER.to_string() },
                                font_size: 18.0,
                                font: { icon_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (UText {
                                text: { "UNIVIS CLOUD PLATFORM".to_string() },
                                font_size: 14.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.0,
                                outline_color: Color::srgba(0.38, 0.35, 0.95, 0.4),
                            }),
                            (UText {
                                text: { "Production Enterprise Cluster • US-East Fleet".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.7),
                            })
                        ]
                    )
                ]
            ),
            (
                // Right Nav Actions
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        // Live Status Pill
                        UNode {
                            background_color: Color::srgba(0.05, 0.3, 0.15, 0.5),
                            border_radius: UCornerRadius::all(20.0),
                            padding: USides::axes(12.0, 6.0),
                        }
                        UBorder {
                            color: Color::srgb(0.1, 0.8, 0.4),
                            width: 1.0,
                            radius: UCornerRadius::all(20.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::CHECK.to_string() },
                                font_size: 11.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.2, 0.9, 0.4),
                            }),
                            (UText {
                                text: { "ALL SYSTEMS OPERATIONAL".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.25, 0.95, 0.5),
                            })
                        ]
                    ),
                    (
                        // Refresh button
                        UNode {
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(16.0, 8.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.38, 0.35, 0.95),
                            hovered: Color::srgb(0.48, 0.45, 1.0),
                            pressed: Color::srgb(0.3, 0.25, 0.8),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::REFRESH_CW.to_string() },
                                font_size: 12.0,
                                font: { icon_font },
                                color: Color::WHITE,
                            }),
                            (
                                RefreshFeedbackText
                                UText {
                                    text: { "REFRESH METRICS".to_string() },
                                    font_size: 12.0,
                                    font: { body_font },
                                    color: Color::WHITE,
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<DashboardState>| {
                            state.refresh_count += 1;
                            info!("Dashboard Metrics Refreshed #{}", state.refresh_count);
                        })
                    )
                ]
            )
        ]
    }
}

/// Row of 4 KPI Metric summary cards (Active Users, Requests, Error Rate, Revenue).
fn metrics_kpi_row(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(110.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
        }
        Children [
            (kpi_card(icon_font.clone(), body_font.clone(), Icon::USERS, "ACTIVE CONCURRENT USERS", "48,290", "+14.2% vs yesterday", Color::srgb(0.2, 0.8, 0.5))),
            (kpi_card(icon_font.clone(), body_font.clone(), Icon::ZAP, "API INGESTION RATE", "18.4k req/s", "Avg Latency 14.2ms", Color::srgb(0.38, 0.65, 1.0))),
            (kpi_card(icon_font.clone(), body_font.clone(), Icon::CIRCLE_ALERT, "HTTP ERROR RATE", "0.018%", "Target < 0.05%", Color::srgb(0.9, 0.7, 0.2))),
            (kpi_card(icon_font, body_font, Icon::BANKNOTE, "CURRENT CLOUD RUN-RATE", "$1,420 / mo", "Within Budget (-8%)", Color::srgb(0.65, 0.45, 1.0))),
        ]
    }
}

/// Middle split area: Cluster rack (flex_grow: 2.0) and Region status (flex_grow: 1.0).
fn middle_split_area(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
        }
        Children [
            (cluster_nodes_card(icon_font.clone(), body_font.clone())),
            (regions_sidebar(icon_font, body_font)),
        ]
    }
}

/// Cluster nodes card displaying active compute containers.
fn cluster_nodes_card(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.95),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(20.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        USelf {
            flex_grow: { Some(2.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 14.0,
        }
        Children [
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::SERVER.to_string() },
                                font_size: 15.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.38, 0.65, 1.0),
                            }),
                            (UText {
                                text: { "COMPUTE CLUSTER NODES (4 ACTIVE)".to_string() },
                                font_size: 14.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (
                        UNode {
                            padding: USides::axes(10.0, 4.0),
                            background_color: Color::srgba(0.1, 0.3, 0.2, 0.5),
                            border_radius: UCornerRadius::all(6.0),
                        }
                        Children [
                            (UText {
                                text: { "AUTO-SCALING: 2-16 NODES".to_string() },
                                font_size: 10.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.3, 0.9, 0.55),
                            })
                        ]
                    )
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 10.0,
                }
                Children [
                    (node_row(icon_font.clone(), body_font.clone(), "node-prod-east-01", "8 vCPU / 32 GB RAM", 0.62, "62% LOAD", Color::srgb(0.35, 0.65, 1.0))),
                    (node_row(icon_font.clone(), body_font.clone(), "node-prod-east-02", "8 vCPU / 32 GB RAM", 0.74, "74% LOAD", Color::srgb(0.35, 0.65, 1.0))),
                    (node_row(icon_font.clone(), body_font.clone(), "node-prod-east-03", "16 vCPU / 64 GB RAM", 0.41, "41% LOAD", Color::srgb(0.2, 0.8, 0.5))),
                    (node_row(icon_font.clone(), body_font.clone(), "node-prod-east-04", "16 vCPU / 64 GB RAM", 0.88, "88% LOAD", Color::srgb(0.95, 0.55, 0.2))),
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.06, 0.1, 0.6),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(10.0),
                }
                Children [
                    (UText {
                        text: { "Flexbox Solver dynamically balances container space across nodes with zero allocations per frame.".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.7),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    })
                ]
            )
        ]
    }
}

/// Right sidebar displaying multi-region ping and status.
fn regions_sidebar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.95),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(20.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::GLOBE.to_string() },
                        font_size: 15.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.4, 0.7, 1.0),
                    }),
                    (UText {
                        text: { "REGIONAL EDGE POPS".to_string() },
                        font_size: 14.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                    })
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    (region_card(icon_font.clone(), body_font.clone(), "us-east-1 (N. Virginia)", "12ms", "HEALTHY", Color::srgb(0.2, 0.85, 0.5))),
                    (region_card(icon_font.clone(), body_font.clone(), "eu-west-1 (Frankfurt)", "34ms", "HEALTHY", Color::srgb(0.2, 0.85, 0.5))),
                    (region_card(icon_font.clone(), body_font.clone(), "ap-northeast-1 (Tokyo)", "112ms", "HEALTHY", Color::srgb(0.2, 0.85, 0.5))),
                    (region_card(icon_font.clone(), body_font.clone(), "sa-east-1 (São Paulo)", "148ms", "ELEVATED", Color::srgb(0.9, 0.7, 0.2))),
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.38, 0.35, 0.95, 0.15),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(10.0, 7.0),
                }
                UBorder {
                    color: Color::srgba(0.38, 0.35, 0.95, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::SHIELD.to_string() },
                        font_size: 11.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.65, 0.6, 1.0),
                    }),
                    (UText {
                        text: { "DNS GLOBAL ANYCAST ROUTING: OPTIMAL".to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.65, 0.6, 1.0),
                    })
                ]
            )
        ]
    }
}

/// Bottom footer status bar.
fn footer_status_bar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(36.0),
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.9),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(16.0, 8.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::INFO.to_string() },
                        font_size: 12.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.65),
                    }),
                    (UText {
                        text: { "Univis UI Flexbox Layout Engine • Declarative BSN Architecture".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.65),
                    })
                ]
            ),
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::CHECK.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.3, 0.8, 0.5),
                    }),
                    (UText {
                        text: { "Cluster v3.2.1 • 99.99% SLA".to_string() },
                        font_size: 11.0,
                        font: { body_font },
                        color: Color::srgb(0.3, 0.8, 0.5),
                    })
                ]
            )
        ]
    }
}

// ── Reusable Component Helpers ─────────────────────────────────────────────

fn kpi_card(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    title: &str,
    val: &str,
    note: &str,
    note_color: Color,
) -> impl Scene {
    let title = title.to_string();
    let val = val.to_string();
    let note = note.to_string();
    bsn! {
        UNode {
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 6.0,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: title,
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.7),
                    }),
                    (UText {
                        text: { icon.to_string() },
                        font_size: 13.0,
                        font: { icon_font },
                        color: Color::srgb(0.5, 0.55, 0.7),
                    })
                ]
            ),
            (UText {
                text: val,
                font_size: 22.0,
                font: { body_font.clone() },
                color: Color::WHITE,
                outline_width: 1.0,
                outline_color: Color::srgba(0.38, 0.35, 0.95, 0.35),
            }),
            (UText {
                text: note,
                font_size: 11.0,
                font: { body_font },
                color: note_color,
            })
        ]
    }
}

fn node_row(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    name: &str,
    specs: &str,
    load: f32,
    load_text: &str,
    bar_color: Color,
) -> impl Scene {
    let name = name.to_string();
    let specs = specs.to_string();
    let load_text = load_text.to_string();
    let bar_pct = load.clamp(0.05, 1.0);

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.09, 0.7),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(14.0, 10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    (UText {
                        text: { Icon::CPU.to_string() },
                        font_size: 14.0,
                        font: { icon_font },
                        color: bar_color,
                    }),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (UText {
                                text: name,
                                font_size: 13.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: specs,
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.45, 0.5, 0.65),
                            })
                        ]
                    )
                ]
            ),
            (
                UNode::default()
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(120.0),
                            height: UVal::Px(8.0),
                            background_color: Color::srgb(0.08, 0.1, 0.16),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                        }
                        Children [
                            (
                                UNode {
                                    width: UVal::Percent(bar_pct),
                                    height: UVal::Percent(1.0),
                                    background_color: bar_color,
                                    border_radius: UCornerRadius::all(4.0),
                                }
                            )
                        ]
                    ),
                    (UText {
                        text: load_text,
                        font_size: 12.0,
                        font: { body_font },
                        color: Color::srgb(0.8, 0.85, 0.95),
                    })
                ]
            )
        ]
    }
}

fn region_card(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    name: &str,
    latency: &str,
    status: &str,
    status_color: Color,
) -> impl Scene {
    let name = name.to_string();
    let latency = latency.to_string();
    let status = status.to_string();
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.09, 0.7),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(12.0, 8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::WIFI.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: status_color,
                    }),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (UText {
                                text: name,
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { format!("Latency: {}", latency) },
                                font_size: 10.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.45, 0.5, 0.65),
                            })
                        ]
                    )
                ]
            ),
            (
                UNode {
                    background_color: Color::srgba(0.05, 0.25, 0.15, 0.4),
                    border_radius: UCornerRadius::all(4.0),
                    padding: USides::axes(8.0, 3.0),
                }
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                Children [
                    (UText {
                        text: status,
                        font_size: 10.0,
                        font: { body_font },
                        color: status_color,
                    })
                ]
            )
        ]
    }
}

/// Updates the refresh counter feedback.
fn update_dashboard_state(state: Res<DashboardState>, mut text_q: Query<&mut UText, With<RefreshFeedbackText>>) {
    if state.is_changed() && state.refresh_count > 0 {
        for mut text in &mut text_q {
            text.text = format!("REFRESHED #{}", state.refresh_count);
        }
    }
}
