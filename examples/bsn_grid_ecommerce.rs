//! Nordic Studio E-Commerce Store - CSS Grid Layout with BSN.
//!
//! Visual Theme: Warm minimalist design boutique (Charcoal, Warm Linen, Muted Sage & Brass).
//!
//! CSS Grid features demonstrated:
//! - Multi-track grid layout: `grid_template_columns` with flexible `Fr` sizing
//! - Multi-row templates: `grid_template_rows` with explicit `Px` track heights
//! - Spanning grid items: `USelf { column_span: 2, row_span: 2 }` creating a featured Bento hero card
//! - Cell gap distribution: `row_gap` and `column_gap`
//! - Interactive "Add to Bag" cart counter with BSN pointer observer
//! - Lucide vector icons and SDF text outlines for clean high-contrast product cards
//! - Responsive text wrapping with WrapToParent and line clamping (max_lines)

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource, Default)]
#[allow(dead_code)]
struct CartState {
    item_count: usize,
    cart_total: f32,
    last_added: String,
}

#[derive(Component, Clone, Copy, Default)]
struct CartBadgeText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Nordic Boutique Store (CSS Grid & BSN)".into(),
                resolution: (1260, 820).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .insert_resource(CartState {
            item_count: 2,
            cart_total: 420.0,
            last_added: "Initial items".to_string(),
        })
        .add_systems(Startup, setup_store)
        .add_systems(Update, update_cart_display)
        .run();
}

fn setup_store(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(store_root_scene(&theme));
}

/// Root scene for the E-Commerce Store.
fn store_root_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let body_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.06, 0.06, 0.08),
            padding: USides::all(24.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
        Children [
            (store_header(icon_font.clone(), body_font.clone())),
            (product_grid_bento(icon_font.clone(), body_font.clone())),
            (store_footer(icon_font, body_font)),
        ]
    }
}

/// Store navigation header.
fn store_header(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(60.0),
            background_color: Color::srgba(0.09, 0.09, 0.12, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(24.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.3, 0.38, 0.25),
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
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 14.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(34.0),
                            height: UVal::Px(34.0),
                            background_color: Color::srgba(0.85, 0.65, 0.35, 0.18),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        UBorder {
                            color: Color::srgba(0.85, 0.65, 0.35, 0.45),
                            width: 1.0,
                            radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::BOX.to_string() },
                                font_size: 18.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.95, 0.8, 0.5),
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
                                text: { "NORDIC OBJECTS".to_string() },
                                font_size: 16.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.0,
                                outline_color: Color::srgba(0.85, 0.65, 0.35, 0.35),
                            }),
                            (UText {
                                text: { "Furniture • Lighting • Ceramics • Textiles".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.55, 0.55, 0.65),
                            })
                        ]
                    )
                ]
            ),
            (
                // Shopping Bag Cart Pill
                UNode {
                    background_color: Color::srgba(0.18, 0.18, 0.24, 0.8),
                    border_radius: UCornerRadius::all(20.0),
                    padding: USides::axes(16.0, 8.0),
                }
                UBorder {
                    color: Color::srgba(0.85, 0.65, 0.35, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(20.0),
                }
                UInteraction::default()
                UInteractionColors {
                    normal: Color::srgba(0.18, 0.18, 0.24, 0.8),
                    hovered: Color::srgba(0.24, 0.24, 0.32, 0.9),
                    pressed: Color::srgba(0.14, 0.14, 0.2, 0.95),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::SHOPPING_BAG.to_string() },
                        font_size: 14.0,
                        font: { icon_font },
                        color: Color::srgb(0.95, 0.8, 0.5),
                    }),
                    (
                        CartBadgeText
                        UText {
                            text: { "BAG (2 ITEMS • $420)".to_string() },
                            font_size: 12.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.95, 0.8, 0.5),
                        }
                    )
                ]
            )
        ]
    }
}

/// Bento-style CSS Grid containing featured hero product and grid items.
fn product_grid_bento(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    let col_tracks = vec![
        UTrackSize::Fr(1.15),
        UTrackSize::Fr(1.15),
        UTrackSize::Fr(1.0),
        UTrackSize::Fr(1.0),
    ];
    let row_tracks = vec![UTrackSize::Fr(1.0), UTrackSize::Fr(1.0)];

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Grid,
            grid_template_columns: col_tracks,
            grid_template_rows: row_tracks,
            row_gap: { Some(16.0) },
            column_gap: { Some(16.0) },
            align_items: UAlignItems::Stretch,
        }
        Children [
            (featured_hero_product(icon_font.clone(), body_font.clone())),
            (product_card(icon_font.clone(), body_font.clone(), Icon::DROPLET, "MATTE CERAMIC VASE", "Hand-thrown sand stoneware with subtle ribbed organic finish.", "$110", Color::srgb(0.85, 0.6, 0.4))),
            (product_card(icon_font.clone(), body_font.clone(), Icon::SUN, "BRASS ARC DESK LAMP", "Spun brass shade with woven warm fabric power cable.", "$310", Color::srgb(0.9, 0.75, 0.4))),
            (product_card(icon_font.clone(), body_font.clone(), Icon::FEATHER, "MERINO WOOL BLANKET", "100% fine spun merino wool woven in charcoal herringbone.", "$185", Color::srgb(0.6, 0.7, 0.8))),
            (product_card(icon_font, body_font, Icon::COFFEE, "SMOKED GLASS CARAFE", "Mouth-blown heat-resistant borosilicate glass with walnut stopper.", "$95", Color::srgb(0.5, 0.8, 0.75))),
        ]
    }
}

/// Featured hero card spanning 2 columns and 2 rows.
fn featured_hero_product(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.09, 0.09, 0.13, 0.95),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: Color::srgba(0.85, 0.65, 0.35, 0.35),
            width: 1.5,
            radius: UCornerRadius::all(16.0),
        }
        USelf {
            column_span: 2,
            row_span: 2,
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            (
                // Header tags
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
                        UNode {
                            background_color: Color::srgba(0.85, 0.65, 0.35, 0.2),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(10.0, 5.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::STAR.to_string() },
                                font_size: 11.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.95, 0.8, 0.5),
                            }),
                            (UText {
                                text: { "FEATURED PIECE • CSS GRID 2x2 SPAN".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.95, 0.8, 0.5),
                            })
                        ]
                    ),
                    (UText {
                        text: { "LIMITED PRODUCTION (BATCH 04)".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            ),
            (
                // Product Visual Mockup Area
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(240.0),
                    background_color: Color::srgb(0.05, 0.05, 0.07),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(16.0),
                }
                UBorder {
                    color: Color::srgba(0.25, 0.25, 0.35, 0.25),
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        // Decorative chair silhouette shape
                        UNode {
                            width: UVal::Px(180.0),
                            height: UVal::Px(120.0),
                            background_color: Color::srgba(0.85, 0.65, 0.35, 0.12),
                            border_radius: UCornerRadius::all(16.0),
                        }
                        UBorder {
                            color: Color::srgb(0.85, 0.65, 0.35),
                            width: 1.5,
                            radius: UCornerRadius::all(16.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::BOX.to_string() },
                                font_size: 24.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.95, 0.8, 0.5),
                            }),
                            (UText {
                                text: { "SOLID OAK FRAME".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.9, 0.8, 0.6),
                            })
                        ]
                    ),
                    (UText {
                        text: { "The Kvadrat Natural Linen Upholstery Lounge Chair".to_string() },
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.8, 0.85, 0.95),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    })
                ]
            ),
            (
                // Price & Add to Bag Row
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
                        USelf {
                            flex_shrink: { Some(0.0) },
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 4.0,
                        }
                        Children [
                            (UText {
                                text: { "$890 USD".to_string() },
                                font_size: 22.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.2,
                                outline_color: Color::srgba(0.85, 0.65, 0.35, 0.5),
                            }),
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
                                        text: { Icon::TRUCK.to_string() },
                                        font_size: 12.0,
                                        font: { icon_font.clone() },
                                        color: Color::srgb(0.5, 0.55, 0.65),
                                    }),
                                    (UText {
                                        text: { "Free white-glove courier shipping worldwide".to_string() },
                                        font_size: 11.0,
                                        font: { body_font.clone() },
                                        color: Color::srgb(0.5, 0.55, 0.65),
                                        wrap_mode: UTextWrapMode::WrapToParent,
                                    })
                                ]
                            )
                        ]
                    ),
                    (
                        // Add to Bag Button
                        UNode {
                            background_color: Color::srgb(0.85, 0.65, 0.35),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(22.0, 12.0),
                        }
                        USelf {
                            flex_shrink: { Some(0.0) },
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.85, 0.65, 0.35),
                            hovered: Color::srgb(0.95, 0.75, 0.45),
                            pressed: Color::srgb(0.7, 0.5, 0.25),
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
                                text: { Icon::PLUS.to_string() },
                                font_size: 13.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.08, 0.08, 0.1),
                            }),
                            (UText {
                                text: { "ADD TO BAG".to_string() },
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.08, 0.08, 0.1),
                            })
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<CartState>| {
                            state.item_count += 1;
                            state.cart_total += 890.0;
                            state.last_added = "Oak Lounge Chair".to_string();
                            info!("Added Oak Lounge Chair to bag. Total items: {}", state.item_count);
                        })
                    )
                ]
            )
        ]
    }
}

/// Standard single-cell product card with thumbnail preview.
fn product_card(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    title: &str,
    desc: &str,
    price: &str,
    accent: Color,
) -> impl Scene {
    let title = title.to_string();
    let desc = desc.to_string();
    let price = price.to_string();
    bsn! {
        UNode {
            background_color: Color::srgba(0.08, 0.08, 0.11, 0.95),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.25, 0.35, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 8.0,
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
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                        wrap_mode: UTextWrapMode::WrapToParent,
                    }),
                    (
                        UNode {
                            width: UVal::Px(8.0),
                            height: UVal::Px(8.0),
                            background_color: accent,
                            border_radius: UCornerRadius::all(4.0),
                        }
                        USelf {
                            flex_shrink: { Some(0.0) },
                        }
                    )
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(90.0),
                    background_color: Color::srgba(0.04, 0.04, 0.06, 0.7),
                    border_radius: UCornerRadius::all(8.0),
                }
                UBorder {
                    color: Color::srgba(0.2, 0.2, 0.28, 0.25),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { icon.to_string() },
                        font_size: 26.0,
                        font: { icon_font.clone() },
                        color: accent,
                    })
                ]
            ),
            (UText {
                text: desc,
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.55, 0.6, 0.7),
                wrap_mode: UTextWrapMode::WrapToParent,
                max_lines: { Some(2) },
                overflow: UTextOverflow::Ellipsis,
            }),
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
                        text: price,
                        font_size: 15.0,
                        font: { body_font.clone() },
                        color: accent,
                    }),
                    (
                        UNode {
                            background_color: Color::srgba(0.15, 0.15, 0.22, 0.8),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(12.0, 6.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgba(0.15, 0.15, 0.22, 0.8),
                            hovered: Color::srgba(0.22, 0.22, 0.32, 0.95),
                            pressed: Color::srgba(0.1, 0.1, 0.16, 0.95),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { "VIEW PIECE".to_string() },
                                font_size: 10.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.75, 0.8, 0.9),
                            }),
                            (UText {
                                text: { Icon::ARROW_RIGHT.to_string() },
                                font_size: 10.0,
                                font: { icon_font },
                                color: Color::srgb(0.75, 0.8, 0.9),
                            })
                        ]
                    )
                ]
            )
        ]
    }
}

/// Store footer notice.
fn store_footer(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(36.0),
            background_color: Color::srgba(0.06, 0.06, 0.08, 0.85),
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
                        color: Color::srgb(0.5, 0.55, 0.65),
                    }),
                    (UText {
                        text: { "CSS Grid solver natively handles explicit Fr & Px track templates with Bento span placement.".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                        wrap_mode: UTextWrapMode::WrapToParent,
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
                        color: Color::srgb(0.85, 0.65, 0.35),
                    }),
                    (UText {
                        text: { "Univis Grid Engine • BSN Architecture".to_string() },
                        font_size: 11.0,
                        font: { body_font },
                        color: Color::srgb(0.85, 0.65, 0.35),
                    })
                ]
            )
        ]
    }
}

/// Updates the cart badge text in real-time.
fn update_cart_display(cart: Res<CartState>, mut text_q: Query<&mut UText, With<CartBadgeText>>) {
    if cart.is_changed() {
        for mut text in &mut text_q {
            text.text = format!("BAG ({} ITEMS • ${:.0})", cart.item_count, cart.cart_total);
        }
    }
}
