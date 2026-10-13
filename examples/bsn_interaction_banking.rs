//! Fintech Capital Management - Pointer Interactions & State Handling with BSN & Vector Icons.
//!
//! Visual Theme: Premium private wealth & business banking portal (Navy, Slate, Emerald, Gold).
//!
//! Interaction features demonstrated:
//! - Built-in vector icons: `Icon::LANDMARK`, `Icon::SHIELD_CHECK`, `Icon::WALLET`, `Icon::TRENDING_UP`,
//!   `Icon::ARROW_UP_RIGHT`, `Icon::ARROW_DOWN_LEFT`, `Icon::LOCK`, `Icon::CREDIT_CARD`, `Icon::CLOUD`,
//!   `Icon::GLOBE`, `Icon::PIGGY_BANK`, `Icon::CHECK_CIRCLE`.
//! - Reactive pointer states: `UInteraction` with `UInteractionColors { normal, hovered, pressed }`
//! - Inline BSN event observers `on(|_event: On<Pointer<Click>>, mut state: ResMut<BankingState>| { ... })`
//! - Real-time state mutators for quick fund transfers and card status toggle
//! - Dynamic text updates reflecting account balance, transaction count, and security freeze
//! - Real-time SDF text outline and WrapToParent dynamic layout

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct BankingState {
    balance: f64,
    transfers_sent: u32,
    is_card_frozen: bool,
    last_action: String,
}

impl Default for BankingState {
    fn default() -> Self {
        Self {
            balance: 142_850.00,
            transfers_sent: 0,
            is_card_frozen: false,
            last_action: "Account verified • All payment rails operational".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct BalanceDisplay;

#[derive(Component, Clone, Copy, Default)]
struct ActionStatusDisplay;

#[derive(Component, Clone, Copy, Default)]
struct FreezeButtonText;

#[derive(Component, Clone, Copy, Default)]
struct FreezeButtonIcon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Fintech Capital & Banking (Pointer Interactions & Vector Icons)".into(),
                resolution: (1260, 820).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<BankingState>()
        .add_systems(Startup, setup_banking)
        .add_systems(Update, sync_banking_ui)
        .run();
}

fn setup_banking(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(banking_root_scene(&theme));
}

/// Root scene constructing the banking portal.
fn banking_root_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let text_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.05, 0.08),
            padding: USides::all(24.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
        Children [
            (portal_navbar(icon_font.clone(), text_font.clone())),
            (portal_main_content(icon_font.clone(), text_font.clone())),
            (portal_audit_footer(icon_font, text_font)),
        ]
    }
}

/// Top navigation bar with institution branding and user profile.
fn portal_navbar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
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
                            width: UVal::Px(36.0),
                            height: UVal::Px(36.0),
                            background_color: Color::srgb(0.1, 0.65, 0.45),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::LIBRARY.to_string() },
                                font_size: 20.0,
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
                                text: { "AURA PRIVATE CAPITAL".to_string() },
                                font_size: 14.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { "Treasury & Institutional Custody".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.7),
                            })
                        ]
                    )
                ]
            ),
            (
                // Live Security Badge
                UNode {
                    background_color: Color::srgba(0.08, 0.2, 0.14, 0.6),
                    border_radius: UCornerRadius::all(20.0),
                    padding: USides::axes(14.0, 6.0),
                }
                UBorder {
                    color: Color::srgb(0.1, 0.75, 0.45),
                    width: 1.0,
                    radius: UCornerRadius::all(20.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::SHIELD.to_string() },
                        font_size: 14.0,
                        font: { icon_font },
                        color: Color::srgb(0.2, 0.9, 0.5),
                    }),
                    (UText {
                        text: { "256-BIT ENCRYPTED SESSION".to_string() },
                        font_size: 11.0,
                        font: { text_font },
                        color: Color::srgb(0.25, 0.9, 0.55),
                    })
                ]
            )
        ]
    }
}

/// Main layout split: Left column (Balance & Quick Transfer), Right column (Transaction Feed).
fn portal_main_content(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
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
            gap: 16.0,
        }
        Children [
            (left_account_controls(icon_font.clone(), text_font.clone())),
            (right_transaction_feed(icon_font, text_font)),
        ]
    }
}

/// Left column with primary account metrics, quick transfer buttons, and card controls.
fn left_account_controls(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
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
            flex_grow: { Some(1.15) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 14.0,
        }
        Children [
            // Account balance summary card
            (balance_hero_card(icon_font.clone(), text_font.clone())),

            // Quick transfer actions
            (quick_transfer_section(icon_font.clone(), text_font.clone())),

            // Account management row (Card freeze & Security)
            (card_actions_row(icon_font, text_font)),
        ]
    }
}

/// Hero card displaying current capital balance.
fn balance_hero_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.8),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.3, 0.45, 0.35),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 8.0,
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
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::CREDIT_CARD.to_string() },
                                font_size: 13.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.5, 0.7, 0.9),
                            }),
                            (UText {
                                text: { "OPERATING CAPITAL ACCOUNT".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.7),
                            })
                        ]
                    ),
                    (UText {
                        text: { "CHECKING •• 8941".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.6, 0.65, 0.8),
                    })
                ]
            ),
            (
                BalanceDisplay
                UText {
                    text: { "$142,850.00 USD".to_string() },
                    font_size: 32.0,
                    font: { text_font.clone() },
                    color: Color::WHITE,
                    outline_width: 1.2,
                    outline_color: Color::srgba(0.1, 0.85, 0.5, 0.3),
                }
            ),
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
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::TRENDING_UP.to_string() },
                                font_size: 13.0,
                                font: { icon_font },
                                color: Color::srgb(0.2, 0.85, 0.5),
                            }),
                            (UText {
                                text: { "+$12,400.00 (+8.7%) this quarter".to_string() },
                                font_size: 12.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.2, 0.85, 0.5),
                            })
                        ]
                    ),
                    (UText {
                        text: { "APY: 4.85% Treasury Yield".to_string() },
                        font_size: 11.0,
                        font: { text_font },
                        color: Color::srgb(0.85, 0.65, 0.3),
                    })
                ]
            )
        ]
    }
}

/// Quick transfer section allowing interactive 1-click fund dispatch.
fn quick_transfer_section(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            (UText {
                text: { "DISPATCH INSTANT WIRE TRANSFER".to_string() },
                font_size: 12.0,
                font: { text_font.clone() },
                color: Color::srgb(0.7, 0.75, 0.9),
            }),
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    (transfer_button(250.0, "$250", icon_font.clone(), text_font.clone())),
                    (transfer_button(500.0, "$500", icon_font.clone(), text_font.clone())),
                    (transfer_button(1000.0, "$1,000", icon_font.clone(), text_font.clone())),
                    (transfer_button(5000.0, "$5,000", icon_font, text_font)),
                ]
            )
        ]
    }
}

fn transfer_button(amount: f64, label: &'static str, icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let label = label.to_string();
    bsn! {
        UNode {
            background_color: Color::srgb(0.12, 0.16, 0.24),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(12.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.35, 0.55, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgb(0.12, 0.16, 0.24),
            hovered: Color::srgb(0.18, 0.24, 0.36),
            pressed: Color::srgb(0.08, 0.12, 0.18),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            (UText {
                text: { Icon::ARROW_UP_RIGHT.to_string() },
                font_size: 13.0,
                font: { icon_font },
                color: Color::srgb(0.3, 0.75, 1.0),
            }),
            (UText {
                text: label,
                font_size: 13.0,
                font: { text_font },
                color: Color::srgb(0.3, 0.75, 1.0),
            })
        ]
        on(move |_event: On<Pointer<Click>>, mut state: ResMut<BankingState>| {
            if state.is_card_frozen {
                state.last_action = "Action Blocked: Account card is currently frozen!".to_string();
            } else if state.balance >= amount {
                state.balance -= amount;
                state.transfers_sent += 1;
                state.last_action = format!("Success: Sent wire transfer of ${:.2} (Transfer #{})", amount, state.transfers_sent);
                info!("{}", state.last_action);
            } else {
                state.last_action = "Error: Insufficient capital funds for transfer!".to_string();
            }
        })
    }
}

/// Security card management row with freeze toggle.
fn card_actions_row(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.5),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
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
                    flex_direction: UFlexDirection::Column,
                    gap: 2.0,
                }
                Children [
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
                                text: { Icon::CREDIT_CARD.to_string() },
                                font_size: 13.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.7, 0.8, 0.95),
                            }),
                            (UText {
                                text: { "PHYSICAL & VIRTUAL CARDS".to_string() },
                                font_size: 12.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (UText {
                        text: { "Instant lock toggles all card transactions immediately.".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            ),
            (
                // Toggle Freeze Button
                UNode {
                    background_color: Color::srgb(0.85, 0.25, 0.25),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 10.0),
                }
                UInteraction::default()
                UInteractionColors {
                    normal: Color::srgb(0.85, 0.25, 0.25),
                    hovered: Color::srgb(0.95, 0.35, 0.35),
                    pressed: Color::srgb(0.7, 0.18, 0.18),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (
                        FreezeButtonIcon
                        UText {
                            text: { Icon::LOCK.to_string() },
                            font_size: 13.0,
                            font: { icon_font },
                            color: Color::WHITE,
                        }
                    ),
                    (
                        FreezeButtonText
                        UText {
                            text: { "FREEZE CARD".to_string() },
                            font_size: 12.0,
                            font: { text_font },
                            color: Color::WHITE,
                        }
                    )
                ]
                on(|_event: On<Pointer<Click>>, mut state: ResMut<BankingState>| {
                    state.is_card_frozen = !state.is_card_frozen;
                    state.last_action = if state.is_card_frozen {
                        "Security Status: Cards frozen. Outgoing transfers suspended.".to_string()
                    } else {
                        "Security Status: Cards unfrozen. Normal operations restored.".to_string()
                    };
                    info!("{}", state.last_action);
                })
            )
        ]
    }
}

/// Right column displaying real-time ledger feed of recent transactions.
fn right_transaction_feed(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
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
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::FILE_TEXT.to_string() },
                                font_size: 14.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.6, 0.7, 0.9),
                            }),
                            (UText {
                                text: { "SETTLED TRANSACTIONS".to_string() },
                                font_size: 13.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (UText {
                        text: { "FEDWIRE • ACH CLEARING".to_string() },
                        font_size: 10.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
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
                    (ledger_row(Icon::ARROW_DOWN_LEFT, "Stripe Platform Payout", "Direct Deposit • Revenue", "+$8,420.00", Color::srgb(0.2, 0.85, 0.5), icon_font.clone(), text_font.clone())),
                    (ledger_row(Icon::CLOUD, "Amazon Web Services", "Cloud Infrastructure Services", "-$1,245.30", Color::srgb(0.9, 0.45, 0.4), icon_font.clone(), text_font.clone())),
                    (ledger_row(Icon::GLOBE, "Tokyo Partner Wire", "Cross-Border Liquidity Transfer", "+$15,000.00", Color::srgb(0.2, 0.85, 0.5), icon_font.clone(), text_font.clone())),
                    (ledger_row(Icon::CREDIT_CARD, "Figma Enterprise License", "Annual Design Workspace Seat", "-$540.00", Color::srgb(0.9, 0.45, 0.4), icon_font.clone(), text_font.clone())),
                    (ledger_row(Icon::PIGGY_BANK, "Treasury Yield Dividend", "Automatic Sweep Distribution", "+$382.40", Color::srgb(0.2, 0.85, 0.5), icon_font.clone(), text_font.clone())),
                ]
            ),
            (
                // Live Status Box
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.06, 0.1, 0.6),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::CIRCLE_CHECK_BIG.to_string() },
                        font_size: 13.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.4, 0.8, 0.6),
                    }),
                    (
                        ActionStatusDisplay
                        UText {
                            text: { "Account verified • All payment rails operational".to_string() },
                            font_size: 11.0,
                            font: { text_font.clone() },
                            color: Color::srgb(0.4, 0.8, 0.6),
                            wrap_mode: UTextWrapMode::WrapToParent,
                        }
                    )
                ]
            )
        ]
    }
}

fn ledger_row(icon_code: &str, entity: &str, category: &str, amount: &str, amount_color: Color, icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let icon_str = icon_code.to_string();
    let entity = entity.to_string();
    let category = category.to_string();
    let amount = amount.to_string();
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
                    (
                        UNode {
                            width: UVal::Px(30.0),
                            height: UVal::Px(30.0),
                            background_color: Color::srgba(0.12, 0.16, 0.24, 0.8),
                            border_radius: UCornerRadius::all(15.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: icon_str,
                                font_size: 13.0,
                                font: { icon_font },
                                color: amount_color,
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
                                text: entity,
                                font_size: 13.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: category,
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                            })
                        ]
                    )
                ]
            ),
            (UText {
                text: amount,
                font_size: 13.0,
                font: { text_font },
                color: amount_color,
            })
        ]
    }
}

/// Audit footer bar.
fn portal_audit_footer(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(36.0),
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.85),
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
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::SHIELD.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    }),
                    (UText {
                        text: { "Univis Interaction Engine • Real-time Pointer Click Observers & Color States".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            ),
            (UText {
                text: { "Member FDIC Insured • Sovereign Custody v4.1".to_string() },
                font_size: 11.0,
                font: { text_font },
                color: Color::srgb(0.2, 0.85, 0.5),
            })
        ]
    }
}

/// Synchronizes reactive text components with `BankingState`.
fn sync_banking_ui(
    state: Res<BankingState>,
    mut balance_q: Query<
        &mut UText,
        (
            With<BalanceDisplay>,
            Without<ActionStatusDisplay>,
            Without<FreezeButtonText>,
            Without<FreezeButtonIcon>,
        ),
    >,
    mut status_q: Query<
        &mut UText,
        (
            With<ActionStatusDisplay>,
            Without<BalanceDisplay>,
            Without<FreezeButtonText>,
            Without<FreezeButtonIcon>,
        ),
    >,
    mut freeze_text_q: Query<
        &mut UText,
        (
            With<FreezeButtonText>,
            Without<BalanceDisplay>,
            Without<ActionStatusDisplay>,
            Without<FreezeButtonIcon>,
        ),
    >,
    mut freeze_icon_q: Query<
        &mut UText,
        (
            With<FreezeButtonIcon>,
            Without<BalanceDisplay>,
            Without<ActionStatusDisplay>,
            Without<FreezeButtonText>,
        ),
    >,
) {
    if state.is_changed() {
        for mut text in &mut balance_q {
            text.text = format!("${:.2} USD", state.balance);
        }
        for mut text in &mut status_q {
            text.text = state.last_action.clone();
            text.color = if state.is_card_frozen {
                Color::srgb(0.95, 0.5, 0.4)
            } else {
                Color::srgb(0.4, 0.8, 0.6)
            };
        }
        for mut text in &mut freeze_text_q {
            text.text = if state.is_card_frozen {
                "UNFREEZE CARD".to_string()
            } else {
                "FREEZE CARD".to_string()
            };
        }
        for mut text in &mut freeze_icon_q {
            text.text = if state.is_card_frozen {
                Icon::LOCK_OPEN.to_string()
            } else {
                Icon::LOCK.to_string()
            };
        }
    }
}
