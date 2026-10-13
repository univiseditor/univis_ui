//! Editorial Magazine & Typography - Rich Text Hierarchy & Interactive Type Scaling with BSN.
//!
//! Visual Theme: Prestigious Architectural & Literary Journal ("The Athenaeum Review").
//! Palette: Warm Slate, Deep Ink, Ivory, and Antique Gold.
//!
//! Typography & Layout features demonstrated:
//! - Built-in embedded font integration: using `Theme` from `univis_ui_style` (`Inter`, `FreeSerif`, and `Lucide` icons).
//! - Vector iconography: `Icon::BOOKMARK`, `Icon::BOOK_OPEN`, `Icon::FEATHER`, `Icon::LIBRARY` from Lucide.ttf.
//! - Rich editorial hierarchy: Masthead, issue colophon, category badges, headlines, drop caps, subheadings, and body prose.
//! - Typographical styling: Font sizes, contrasting weights, line wrapping, and text alignments.
//! - Multi-column editorial article layout with lead narrative and secondary sidebar/colophon.
//! - Styled decorative Pull-Quote card with an accent border.
//! - Interactive Type Scale Controller: Clickable `[ A- ]`, `[ A ]`, and `[ A+ ]` controls that dynamically update the `font_size` of all body text in real-time.
//! - Interactive bookmark toggle with reactive state transitions.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum TypeScaleMode {
    Compact,
    #[default]
    Standard,
    Comfortable,
}

impl TypeScaleMode {
    fn font_size(&self) -> f32 {
        match self {
            Self::Compact => 11.5,
            Self::Standard => 13.0,
            Self::Comfortable => 14.5,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Compact => "Scale: Compact (11.5px)",
            Self::Standard => "Scale: Standard (13px)",
            Self::Comfortable => "Scale: Comfortable (14.5px)",
        }
    }
}

#[derive(Resource)]
struct EditorialState {
    scale_mode: TypeScaleMode,
    bookmarked: bool,
}

impl Default for EditorialState {
    fn default() -> Self {
        Self {
            scale_mode: TypeScaleMode::Standard,
            bookmarked: false,
        }
    }
}

#[derive(Component, Clone, Default)]
struct ArticleBodyText;

#[derive(Component, Clone, Default)]
struct BookmarkBtnLabel;

#[derive(Component, Clone, Default)]
struct BookmarkBtnIcon;

#[derive(Component, Clone, Default)]
struct TypeScaleLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Editorial Magazine & Typography (BSN)".into(),
                resolution: (1280, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<EditorialState>()
        .add_systems(Startup, setup_editorial)
        .add_systems(Update, sync_editorial_ui)
        .run();
}

fn setup_editorial(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(editorial_root_scene(&theme));
}

fn editorial_root_scene(theme: &Theme) -> impl Scene {
    let body_font = theme.text.font.inter_regular.clone();
    let serif_font = theme.text.font.free_serif.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Px(1280.0),
            height: UVal::Px(840.0),
            background_color: Color::srgb(0.06, 0.07, 0.09),
            padding: USides::axes(32.0, 18.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Start,
            gap: 10.0,
        }
        Children [
            (editorial_masthead(body_font.clone(), serif_font.clone())),
            (editorial_divider()),
            (editorial_toolbar(icon_font.clone(), body_font.clone())),
            (editorial_article_header(body_font.clone())),
            (editorial_body_columns(body_font.clone(), serif_font.clone(), icon_font)),
            (editorial_footer(body_font)),
        ]
    }
}

/// 1. Publication Masthead
fn editorial_masthead(body_font: Handle<Font>, serif_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            padding: USides::axes(0.0, 2.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 2.0,
        }
        Children [
            (
                UText {
                    text: { "OCTOBER 2026  *  VOLUME XXIV  *  ISSUE 04  *  MONOGRAPH SERIES".to_string() },
                    font_size: 10.5,
                    font: { body_font.clone() },
                    color: Color::srgb(0.78, 0.68, 0.48),
                }
            ),
            (
                UText {
                    text: { "THE ATHENAEUM REVIEW".to_string() },
                    font_size: 26.0,
                    font: { serif_font },
                    color: Color::srgb(0.97, 0.95, 0.92),
                }
            ),
            (
                UText {
                    text: { "LITTERAE  *  PHILOSOPHIA  *  MONUMENTA  *  ARS VIVENDI".to_string() },
                    font_size: 10.0,
                    font: { body_font },
                    color: Color::srgb(0.55, 0.60, 0.68),
                }
            ),
        ]
    }
}

/// Horizontal divider line
fn editorial_divider() -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            height: UVal::Px(1.0),
            background_color: Color::srgb(0.18, 0.21, 0.28),
        }
    }
}

/// 2. Interactive Reading Toolbar
fn editorial_toolbar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            height: UVal::Px(38.0),
            background_color: Color::srgb(0.10, 0.12, 0.16),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(14.0, 0.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.29),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Left metadata tags
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
                            background_color: Color::srgb(0.16, 0.14, 0.10),
                            border_radius: UCornerRadius::all(4.0),
                            padding: USides::axes(8.0, 3.0),
                        }
                        UBorder {
                            color: Color::srgb(0.72, 0.60, 0.38),
                            width: 1.0,
                            radius: UCornerRadius::all(4.0),
                        }
                        Children [
                            (
                                UText {
                                    text: { "ESSAY & CRITICISM".to_string() },
                                    font_size: 10.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.88, 0.76, 0.52),
                                }
                            )
                        ]
                    ),
                    (
                        UText {
                            text: { "Curated by Julian Vance  *  8 Min Read  *  Kyoto & Zurich".to_string() },
                            font_size: 11.5,
                            font: { body_font.clone() },
                            color: Color::srgb(0.60, 0.66, 0.74),
                        }
                    ),
                ]
            ),

            // Right interactive reading controls
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    // Type scale indicator
                    (
                        TypeScaleLabel
                        UText {
                            text: { "Scale: Standard (13px)".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.70, 0.75, 0.82),
                        }
                    ),

                    // Button A- (Compact)
                    (
                        UNode {
                            width: UVal::Px(30.0),
                            height: UVal::Px(24.0),
                            background_color: Color::srgb(0.14, 0.17, 0.22),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        UBorder {
                            color: Color::srgb(0.24, 0.28, 0.38),
                            width: 1.0,
                            radius: UCornerRadius::all(4.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        UInteraction
                        UInteractionColors {
                            normal: Color::srgb(0.14, 0.17, 0.22),
                            hovered: Color::srgb(0.22, 0.26, 0.34),
                            pressed: Color::srgb(0.30, 0.24, 0.16),
                        }
                        Children [
                            (
                                UText {
                                    text: { "A-".to_string() },
                                    font_size: 10.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.85, 0.88, 0.94),
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<EditorialState>| {
                            state.scale_mode = TypeScaleMode::Compact;
                        })
                    ),

                    // Button A (Standard)
                    (
                        UNode {
                            width: UVal::Px(30.0),
                            height: UVal::Px(24.0),
                            background_color: Color::srgb(0.14, 0.17, 0.22),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        UBorder {
                            color: Color::srgb(0.24, 0.28, 0.38),
                            width: 1.0,
                            radius: UCornerRadius::all(4.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        UInteraction
                        UInteractionColors {
                            normal: Color::srgb(0.14, 0.17, 0.22),
                            hovered: Color::srgb(0.22, 0.26, 0.34),
                            pressed: Color::srgb(0.30, 0.24, 0.16),
                        }
                        Children [
                            (
                                UText {
                                    text: { "A".to_string() },
                                    font_size: 11.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.85, 0.88, 0.94),
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<EditorialState>| {
                            state.scale_mode = TypeScaleMode::Standard;
                        })
                    ),

                    // Button A+ (Comfortable)
                    (
                        UNode {
                            width: UVal::Px(30.0),
                            height: UVal::Px(24.0),
                            background_color: Color::srgb(0.14, 0.17, 0.22),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        UBorder {
                            color: Color::srgb(0.24, 0.28, 0.38),
                            width: 1.0,
                            radius: UCornerRadius::all(4.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        UInteraction
                        UInteractionColors {
                            normal: Color::srgb(0.14, 0.17, 0.22),
                            hovered: Color::srgb(0.22, 0.26, 0.34),
                            pressed: Color::srgb(0.30, 0.24, 0.16),
                        }
                        Children [
                            (
                                UText {
                                    text: { "A+".to_string() },
                                    font_size: 13.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.85, 0.88, 0.94),
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<EditorialState>| {
                            state.scale_mode = TypeScaleMode::Comfortable;
                        })
                    ),

                    // Bookmark Button with Lucide Icon!
                    (
                        UNode {
                            height: UVal::Px(24.0),
                            background_color: Color::srgb(0.16, 0.18, 0.24),
                            border_radius: UCornerRadius::all(4.0),
                            padding: USides::axes(10.0, 0.0),
                        }
                        UBorder {
                            color: Color::srgb(0.28, 0.32, 0.44),
                            width: 1.0,
                            radius: UCornerRadius::all(4.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        UInteraction
                        UInteractionColors {
                            normal: Color::srgb(0.16, 0.18, 0.24),
                            hovered: Color::srgb(0.24, 0.28, 0.38),
                            pressed: Color::srgb(0.32, 0.38, 0.52),
                        }
                        Children [
                            (
                                BookmarkBtnIcon
                                UText {
                                    text: { Icon::BOOKMARK.to_string() },
                                    font_size: 12.0,
                                    font: { icon_font },
                                    color: Color::srgb(0.88, 0.90, 0.96),
                                }
                            ),
                            (
                                BookmarkBtnLabel
                                UText {
                                    text: { "Save to Library".to_string() },
                                    font_size: 10.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.88, 0.90, 0.96),
                                }
                            )
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<EditorialState>| {
                            state.bookmarked = !state.bookmarked;
                        })
                    ),
                ]
            ),
        ]
    }
}

/// 3. Article Title & Deck
fn editorial_article_header(body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            padding: USides::axes(4.0, 2.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 4.0,
        }
        Children [
            (
                UText {
                    text: { "The Architecture of Quietude: Form, Shadow, and the Monolith".to_string() },
                    font_size: 22.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.96, 0.94, 0.90),
                }
            ),
            (
                UText {
                    text: { "How contemporary builders abandon synthetic noise to rediscover acoustic stillness, tactile honesty,\n\
                             and the sacred balance between raw masonry and indirect illumination.".to_string() },
                    font_size: 12.5,
                    font: { body_font },
                    color: Color::srgb(0.65, 0.70, 0.78),
                }
            ),
        ]
    }
}

/// 4. Main Two-Column Editorial Content
fn editorial_body_columns(
    body_font: Handle<Font>,
    serif_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            height: UVal::Px(510.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Stretch,
            gap: 16.0,
        }
        Children [
            (editorial_left_column(body_font.clone())),
            (editorial_right_column(body_font, serif_font, icon_font)),
        ]
    }
}

/// Left Column: Primary Narrative (Width: 780px)
fn editorial_left_column(body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(780.0),
            height: UVal::Px(510.0),
            background_color: Color::srgb(0.09, 0.11, 0.14),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgb(0.16, 0.19, 0.25),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Stretch,
            gap: 12.0,
        }
        Children [
            // Lead Paragraph with Drop Cap
            (
                UNode {
                    width: UVal::Px(744.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Start,
                    gap: 12.0,
                }
                Children [
                    // Drop Cap Box
                    (
                        UNode {
                            width: UVal::Px(46.0),
                            height: UVal::Px(46.0),
                            background_color: Color::srgb(0.18, 0.15, 0.10),
                            border_radius: UCornerRadius::all(6.0),
                        }
                        UBorder {
                            color: Color::srgb(0.78, 0.66, 0.42),
                            width: 1.5,
                            radius: UCornerRadius::all(6.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (
                                UText {
                                    text: { "I".to_string() },
                                    font_size: 32.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.92, 0.80, 0.54),
                                }
                            )
                        ]
                    ),
                    // Opening text beside Drop Cap
                    (
                        UNode {
                            width: UVal::Px(686.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 3.0,
                        }
                        Children [
                            (
                                ArticleBodyText
                                UText {
                                    text: { "N AN AGE DOMINATED BY PERVASIVE DIGITAL CHATTER and glass curtain facades, a deliberate spatial counter-movement has taken root across modern ateliers.".to_string() },
                                    font_size: 13.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.90, 0.88, 0.84),
                                    wrap_mode: UTextWrapMode::WrapToParent,
                                }
                            ),
                            (
                                ArticleBodyText
                                UText {
                                    text: { "Architects are abandoning fragile lightweight assemblies in pursuit of tectonic mass, acoustic silence, and unyielding material sincerity.".to_string() },
                                    font_size: 13.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.82, 0.80, 0.77),
                                    wrap_mode: UTextWrapMode::WrapToParent,
                                }
                            ),
                        ]
                    ),
                ]
            ),

            // Subheading I
            (
                UText {
                    text: { "1. The Resonance of Heavy Mass".to_string() },
                    font_size: 15.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.82, 0.70, 0.48),
                }
            ),

            // Paragraph 1
            (
                ArticleBodyText
                UText {
                    text: { "When structural mass exceeds functional necessity, an instinctive psychological sanctuary is formed. Board-marked concrete, cyclopean limestone, and unvarnished timber do not demand visual admiration; rather, they gently absorb ambient noise and ground human consciousness in physical permanence.".to_string() },
                    font_size: 13.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.80, 0.79, 0.76),
                    wrap_mode: UTextWrapMode::WrapToParent,
                }
            ),

            (
                ArticleBodyText
                UText {
                    text: { "In the Alpine sanctuaries of Peter Zumthor and the ancient tea chambers of Kyoto, silence is not an empty void. It is a dense, sculpted medium held securely within monolithic walls, shielding the occupant from the frantic rhythm of urban life.".to_string() },
                    font_size: 13.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.80, 0.79, 0.76),
                    wrap_mode: UTextWrapMode::WrapToParent,
                }
            ),

            // Subheading II
            (
                UText {
                    text: { "2. The Choreography of Daylight and Shadow".to_string() },
                    font_size: 15.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.82, 0.70, 0.48),
                }
            ),

            // Paragraph 2
            (
                ArticleBodyText
                UText {
                    text: { "Light without deep shadow is blinding; architecture that bathes every plane in uniform illumination robs space of mystery. By constricting exterior openings into deep-set reveals and vertical skylight clefts, natural light enters as an exquisite living sundial that breathes quiet life across textured plaster and raw stone.".to_string() },
                    font_size: 13.0,
                    font: { body_font },
                    color: Color::srgb(0.80, 0.79, 0.76),
                    wrap_mode: UTextWrapMode::WrapToParent,
                }
            ),
        ]
    }
}

/// Right Column: Sidebar & Pull-Quote (Width: 420px)
fn editorial_right_column(
    body_font: Handle<Font>,
    serif_font: Handle<Font>,
    _icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(420.0),
            height: UVal::Px(510.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            // 1. Pull-Quote Card
            (
                UNode {
                    width: UVal::Px(420.0),
                    background_color: Color::srgb(0.11, 0.13, 0.17),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(16.0, 14.0),
                }
                UBorder {
                    color: Color::srgb(0.78, 0.66, 0.42),
                    width: 2.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    (
                        UText {
                            text: { "\"Architecture begins where utility ends, where\n\
                                     masonry embraces silence, and where morning light\n\
                                     carves dignity out of cold stone.\"".to_string() },
                            font_size: 13.5,
                            font: { serif_font.clone() },
                            color: Color::srgb(0.94, 0.91, 0.86),
                        }
                    ),
                    (
                        UText {
                            text: { "-- Julian Vance, 'The Monolith in Modern Space' (1998)".to_string() },
                            font_size: 10.5,
                            font: { body_font.clone() },
                            color: Color::srgb(0.75, 0.64, 0.44),
                        }
                    ),
                ]
            ),

            // 2. Architectural Lexicon / Annotations Card
            (
                UNode {
                    width: UVal::Px(420.0),
                    background_color: Color::srgb(0.09, 0.11, 0.14),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 12.0),
                }
                UBorder {
                    color: Color::srgb(0.16, 0.20, 0.26),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 7.0,
                }
                Children [
                    (
                        UText {
                            text: { "EDITORIAL ANNOTATIONS".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.78, 0.68, 0.48),
                        }
                    ),
                    (
                        UText {
                            text: { "* Cyclopean Masonry: Interlocking dry-stone construction\n  granting immense acoustic and thermal inertia.".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.70, 0.74, 0.80),
                        }
                    ),
                    (
                        UText {
                            text: { "* Chiaroscuro Volume: Deep gradient transitions between\n  sunlit surfaces and contemplative shadows.".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.70, 0.74, 0.80),
                        }
                    ),
                    (
                        UText {
                            text: { "* Tactile Honesty: Exposing rough aggregate and saw kerfs\n  without masking coats or artificial veneers.".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.70, 0.74, 0.80),
                        }
                    ),
                ]
            ),

            // 3. Issue Colophon Card
            (
                UNode {
                    width: UVal::Px(420.0),
                    background_color: Color::srgb(0.08, 0.10, 0.13),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 10.0),
                }
                UBorder {
                    color: Color::srgb(0.15, 0.18, 0.24),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 5.0,
                }
                Children [
                    (
                        UText {
                            text: { "PUBLICATION COLOPHON".to_string() },
                            font_size: 10.5,
                            font: { serif_font },
                            color: Color::srgb(0.78, 0.68, 0.48),
                        }
                    ),
                    (
                        UText {
                            text: { "Typeset in Univis SDF font engine with embedded Inter Regular,\nFreeSerif, and Lucide vector iconography.".to_string() },
                            font_size: 10.5,
                            font: { body_font.clone() },
                            color: Color::srgb(0.60, 0.65, 0.72),
                        }
                    ),
                    (
                        UText {
                            text: { "The Athenaeum Press * Oxford & Zurich * All Rights Reserved".to_string() },
                            font_size: 10.0,
                            font: { body_font },
                            color: Color::srgb(0.48, 0.52, 0.60),
                        }
                    ),
                ]
            ),
        ]
    }
}

/// 5. Footer Status Bar
fn editorial_footer(body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1216.0),
            height: UVal::Px(24.0),
            padding: USides::axes(4.0, 0.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UText {
                    text: { "[<- Prev: The Monasteries of Mount Athos]          [Next: The Courtyard Houses of Fez ->]".to_string() },
                    font_size: 10.5,
                    font: { body_font.clone() },
                    color: Color::srgb(0.52, 0.58, 0.68),
                }
            ),
            (
                UText {
                    text: { "THE ATHENAEUM REVIEW  *  FOUNDED MDCCCXLVII".to_string() },
                    font_size: 10.0,
                    font: { body_font },
                    color: Color::srgb(0.44, 0.48, 0.56),
                }
            ),
        ]
    }
}

fn sync_editorial_ui(
    state: Res<EditorialState>,
    mut body_query: Query<&mut UText, With<ArticleBodyText>>,
    mut scale_label_query: Query<
        &mut UText,
        (
            With<TypeScaleLabel>,
            Without<ArticleBodyText>,
            Without<BookmarkBtnLabel>,
            Without<BookmarkBtnIcon>,
        ),
    >,
    mut bookmark_label_query: Query<
        &mut UText,
        (
            With<BookmarkBtnLabel>,
            Without<ArticleBodyText>,
            Without<TypeScaleLabel>,
            Without<BookmarkBtnIcon>,
        ),
    >,
    mut bookmark_icon_query: Query<
        &mut UText,
        (
            With<BookmarkBtnIcon>,
            Without<ArticleBodyText>,
            Without<TypeScaleLabel>,
            Without<BookmarkBtnLabel>,
        ),
    >,
) {
    if !state.is_changed() {
        return;
    }

    let target_size = state.scale_mode.font_size();
    for mut text in body_query.iter_mut() {
        text.font_size = target_size;
    }

    for mut text in scale_label_query.iter_mut() {
        text.text = state.scale_mode.label().to_string();
    }

    for mut text in bookmark_label_query.iter_mut() {
        if state.bookmarked {
            text.text = "Saved in Library".to_string();
            text.color = Color::srgb(0.92, 0.82, 0.55);
        } else {
            text.text = "Save to Library".to_string();
            text.color = Color::srgb(0.88, 0.90, 0.96);
        }
    }

    for mut text in bookmark_icon_query.iter_mut() {
        if state.bookmarked {
            text.color = Color::srgb(0.92, 0.82, 0.55);
        } else {
            text.color = Color::srgb(0.88, 0.90, 0.96);
        }
    }
}
