use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use univis_ui::prelude::*;

// 1. Settings Enum & Menu State Resource
#[derive(Clone, Copy, Debug, PartialEq)]
enum SettingType {
    TextureFiltering,
    EnvironmentalDetail,
    DirectX,
    TextureQuality,
    AmbientOcclusion,
    DynamicReflections,
    MotionBlur,
    Supersampling,
}

#[derive(Resource)]
struct SettingsMenuState {
    selected_setting: SettingType,
    directx_version: u32, // 11 or 12
    env_detail: u32,      // 0 = Low, 1 = Med, 2 = High, 3 = Ultra
    reflections: bool,
    motion_blur: bool,
    supersampling: bool,
}

impl Default for SettingsMenuState {
    fn default() -> Self {
        Self {
            selected_setting: SettingType::DirectX, // DirectX selected by default
            directx_version: 12,
            env_detail: 2, // High
            reflections: true,
            motion_blur: true,
            supersampling: false,
        }
    }
}

// 2. Components for UI synchronization
#[derive(Component)]
struct RightPaneTitle;

#[derive(Component)]
struct RightPaneDesc;

#[derive(Component)]
struct VramLabel;

#[derive(Component)]
struct VramBar;

#[derive(Component)]
struct DirectxValueText;

#[derive(Component)]
struct EnvDetailValueText;

#[derive(Component)]
struct ReflectionCheckbox;

#[derive(Component)]
struct MotionBlurCheckbox;

#[derive(Component)]
struct SupersamplingCheckbox;

// Segmented bar markers
#[derive(Component)]
struct SegmentedBar {
    setting: SettingType,
}

// Custom Hover animation component matching the game style
#[derive(Component)]
struct CardHover {
    base_scale: Vec3,
    target_scale: Vec3,
    current_scale: Vec3,

    base_border_width: f32,
    target_border_width: f32,
    current_border_width: f32,

    base_border_color: Color,
    target_border_color: Color,
    current_border_color: Color,
}

impl Default for CardHover {
    fn default() -> Self {
        Self {
            base_scale: Vec3::ONE,
            target_scale: Vec3::splat(1.02), // Gentle scale up to fit in list
            current_scale: Vec3::ONE,

            base_border_width: 1.0,
            target_border_width: 2.0, // White border highlight
            current_border_width: 1.0,

            base_border_color: Color::srgba(0.2, 0.25, 0.35, 0.15),
            target_border_color: Color::srgb(1.0, 1.0, 1.0), // Crisp white active border
            current_border_color: Color::srgba(0.2, 0.25, 0.35, 0.15),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "System Graphics Settings".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<SettingsMenuState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_hover_elements,
                update_right_pane,
                update_vram_and_progress,
                update_value_selectors,
                camera_control_2d,
                handle_keyboard_actions,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // --- 1. 2D CAMERA ---
    commands.spawn(Camera2d);

    // --- 2. SETTINGS INTERFACE (2D SCREEN SPACE HUD) ---
    let root = commands
        .spawn((
            URootUi {
                camera: UiCameraRef::Auto,
                ..URootUi::screen()
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgba(0.01, 0.015, 0.025, 0.95), // Dark overlay screen
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    // --- A. TOP TABS BAR ---
    let top_bar = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(48.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 36.0,
                ..default()
            },
        ))
        .id();

    let tabs = [
        "Display", "Graphics", "Gameplay", "Controls", "Audio", "Language",
    ];
    for name in tabs {
        let is_active = name == "Graphics";
        let tab_wrap = commands
            .spawn((
                ChildOf(top_bar),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(tab_wrap),
            UTextLabel {
                text: name.to_string(),
                font_size: 15.0,
                color: if is_active {
                    Color::WHITE
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.4)
                },
                ..default()
            },
            UNode::default(),
        ));

        // Active tab underline indicator
        if is_active {
            commands.spawn((
                ChildOf(tab_wrap),
                UNode {
                    width: UVal::Px(48.0),
                    height: UVal::Px(2.0),
                    background_color: Color::WHITE,
                    ..default()
                },
            ));
        }
    }

    // --- B. MAIN PANEL WORKSPACE (SPLIT LAYOUT) ---
    let workspace = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(520.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    // 1. LEFT PANE: Options list (width 480px)
    let left_pane = commands
        .spawn((
            ChildOf(workspace),
            UNode {
                width: UVal::Px(480.0),
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    // Spawn settings items
    // (Type, Label, Selector/Control description)
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::TextureFiltering,
        "Texture Filtering",
        ControlType::SelectorSegmented("Anisotropic 16x"),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::EnvironmentalDetail,
        "Environmental Detail",
        ControlType::SelectorArrows("High"),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::DirectX,
        "DirectX",
        ControlType::SelectorArrows("12"),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::TextureQuality,
        "Texture Quality",
        ControlType::Disabled("High"),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::AmbientOcclusion,
        "Ambient Occlusion",
        ControlType::SelectorSegmented("MHBAO"),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::DynamicReflections,
        "Dynamic Reflections",
        ControlType::Checkbox(true),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::MotionBlur,
        "Motion Blur",
        ControlType::Checkbox(true),
    );
    spawn_setting_item(
        commands.reborrow(),
        left_pane,
        SettingType::Supersampling,
        "Supersampling Anti-Aliasing",
        ControlType::Checkbox(false),
    );

    // 2. MIDDLE SCROLL BAR VISUAL
    let scrollbar_track = commands
        .spawn((
            ChildOf(workspace),
            UNode {
                width: UVal::Px(4.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgba(0.2, 0.25, 0.3, 0.15),
                border_radius: UCornerRadius::all(2.0),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(scrollbar_track),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(240.0), // Scroll handle size
            background_color: Color::srgba(1.0, 1.0, 1.0, 0.8),
            border_radius: UCornerRadius::all(2.0),
            ..default()
        },
    ));

    // 3. RIGHT PANE: Details and VRAM diagnostics (flex remaining width)
    let right_pane = commands
        .spawn((
            ChildOf(workspace),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Start,
                ..default()
            },
        ))
        .id();

    // Option Description Area
    let desc_area = commands
        .spawn((
            ChildOf(right_pane),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(desc_area),
        RightPaneTitle,
        UTextLabel {
            text: "DirectX".to_string(),
            font_size: 26.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    // Description Divider
    commands.spawn((
        ChildOf(desc_area),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(1.0),
            background_color: Color::srgba(1.0, 1.0, 1.0, 0.1),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(desc_area),
        RightPaneDesc,
        UTextLabel {
            text: "Select DirectX render mode. Appropriate version of Windows and supported GPU is required for DirectX 12. Using the DirectX 12 API can offer better performance depending on your hardware.".to_string(),
            font_size: 14.0,
            color: Color::srgba(0.8, 0.85, 0.9, 0.8),
            ..default()
        },
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
    ));

    // VRAM Usage Display Area (Bottom Right)
    let vram_area = commands
        .spawn((
            ChildOf(right_pane),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    let vram_header = commands
        .spawn((
            ChildOf(vram_area),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(vram_header),
        UTextLabel {
            text: "VRAM Usage".to_string(),
            font_size: 14.0,
            color: Color::WHITE,
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(vram_header),
        VramLabel,
        UTextLabel {
            text: "8412 MB / 11201 MB".to_string(),
            font_size: 14.0,
            color: Color::srgba(1.0, 1.0, 1.0, 0.7),
            ..default()
        },
        UNode::default(),
    ));

    commands.spawn((
        ChildOf(vram_area),
        VramBar,
        UProgressBar {
            value: 0.75,
            bar_color: Color::srgb(0.0, 0.75, 1.0), // Cyan-blue progress bar
        },
    ));

    // --- C. FOOTER ACTION BAR ---
    let footer = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(48.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Start,
                align_items: UAlignItems::Center,
                gap: 32.0,
                ..default()
            },
        ))
        .id();

    spawn_footer_shortcut(commands.reborrow(), footer, "ESC", "BACK");
    spawn_footer_shortcut(commands.reborrow(), footer, "ENTER", "APPLY CHANGES");
    spawn_footer_shortcut(commands.reborrow(), footer, "R", "RESET TO DEFAULT");
}

// ── HELPER SPAWNING FUNCTIONS ───────────────────────────────────────────────

enum ControlType {
    SelectorSegmented(&'static str),
    SelectorArrows(&'static str),
    Checkbox(bool),
    Disabled(&'static str),
}

fn spawn_setting_item(
    mut commands: Commands,
    parent: Entity,
    setting_type: SettingType,
    label: &str,
    control: ControlType,
) {
    let is_disabled = matches!(control, ControlType::Disabled(_));

    let mut item_entity = commands.spawn((
        ChildOf(parent),
        UInteraction::default(),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(50.0),
            padding: USides::axes(16.0, 8.0),
            background_color: if is_disabled {
                Color::srgba(0.04, 0.05, 0.08, 0.15)
            } else {
                Color::srgba(0.05, 0.06, 0.1, 0.35)
            },
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.15),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
            offset: 0.0,
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
            ..default()
        },
    ));

    // Enable interaction and hover effects if not disabled
    if !is_disabled {
        item_entity.insert((Pickable::default(), CardHover::default()));
        // Add observer to update selected option in state on hover
        item_entity.observe(
            move |_: On<Pointer<Over>>, mut state: ResMut<SettingsMenuState>| {
                state.selected_setting = setting_type;
            },
        );
    }

    let item_id = item_entity.id();

    // Left child: Label text
    commands.spawn((
        ChildOf(item_id),
        UTextLabel {
            text: label.to_string(),
            font_size: 14.0,
            color: if is_disabled {
                Color::srgba(1.0, 1.0, 1.0, 0.25)
            } else {
                Color::WHITE
            },
            ..default()
        },
        UNode::default(),
    ));

    // Right child: Control value & widget wrapper
    let ctrl_wrapper = commands
        .spawn((
            ChildOf(item_id),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::End,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    match control {
        ControlType::SelectorSegmented(initial_val) => {
            // Value text
            commands.spawn((
                ChildOf(ctrl_wrapper),
                UTextLabel {
                    text: initial_val.to_string(),
                    font_size: 13.0,
                    color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                    ..default()
                },
                UNode::default(),
            ));
            // Segmented Strength Indicator
            spawn_segmented_bar(commands.reborrow(), ctrl_wrapper, setting_type);
        }
        ControlType::SelectorArrows(val) => {
            let row = commands
                .spawn((
                    ChildOf(ctrl_wrapper),
                    UNode::default(),
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        gap: 12.0,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .id();

            // Left arrow button using ASCII '<' instead of unicode
            let left_btn = commands
                .spawn((
                    ChildOf(row),
                    UButton::secondary(),
                    Pickable::default(),
                    UNode {
                        width: UVal::Px(16.0),
                        height: UVal::Px(16.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|b| {
                    b.spawn((
                        UNode::default(),
                        UTextLabel {
                            text: "<".to_string(),
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                })
                .id();

            // Click observer for left arrow
            if setting_type == SettingType::DirectX {
                commands.entity(left_btn).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        if state.directx_version == 12 {
                            state.directx_version = 11;
                        }
                    },
                );
            } else if setting_type == SettingType::EnvironmentalDetail {
                commands.entity(left_btn).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        if state.env_detail > 0 {
                            state.env_detail -= 1;
                        }
                    },
                );
            }

            // Value label text
            let val_label = commands
                .spawn((
                    ChildOf(row),
                    UNode::default(),
                    UTextLabel {
                        text: val.to_string(),
                        font_size: 13.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ))
                .id();

            if setting_type == SettingType::DirectX {
                commands.entity(val_label).insert(DirectxValueText);
            } else if setting_type == SettingType::EnvironmentalDetail {
                commands.entity(val_label).insert(EnvDetailValueText);
            }

            // Right arrow button using ASCII '>' instead of unicode
            let right_btn = commands
                .spawn((
                    ChildOf(row),
                    UButton::secondary(),
                    Pickable::default(),
                    UNode {
                        width: UVal::Px(16.0),
                        height: UVal::Px(16.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|b| {
                    b.spawn((
                        UNode::default(),
                        UTextLabel {
                            text: ">".to_string(),
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                })
                .id();

            // Click observer for right arrow
            if setting_type == SettingType::DirectX {
                commands.entity(right_btn).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        if state.directx_version == 11 {
                            state.directx_version = 12;
                        }
                    },
                );
            } else if setting_type == SettingType::EnvironmentalDetail {
                commands.entity(right_btn).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        if state.env_detail < 3 {
                            state.env_detail += 1;
                        }
                    },
                );
            }

            // Segmented Strength Indicator
            spawn_segmented_bar(commands.reborrow(), ctrl_wrapper, setting_type);
        }
        ControlType::Checkbox(checked) => {
            // Visual outer box for checkbox (100% vector-based shape, no unicode squares)
            let checkbox_box = commands
                .spawn((
                    ChildOf(ctrl_wrapper),
                    UButton::secondary(),
                    Pickable::default(),
                    UNode {
                        width: UVal::Px(18.0),
                        height: UVal::Px(18.0),
                        border_radius: UCornerRadius::all(4.0),
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.3),
                        ..default()
                    },
                    UBorder {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.4),
                        width: 1.0,
                        radius: UCornerRadius::all(4.0),
                        offset: 0.0,
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .id();

            // Inner visual fill node representing checked state
            let check_fill = commands
                .spawn((
                    ChildOf(checkbox_box),
                    UNode {
                        width: UVal::Px(10.0),
                        height: UVal::Px(10.0),
                        border_radius: UCornerRadius::all(2.0),
                        background_color: if checked {
                            Color::srgb(0.0, 0.9, 1.0)
                        } else {
                            Color::NONE
                        },
                        ..default()
                    },
                ))
                .id();

            // Assign tags to the visual check_fill node and add toggle click actions
            if setting_type == SettingType::DynamicReflections {
                commands.entity(check_fill).insert(ReflectionCheckbox);
                commands.entity(checkbox_box).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        state.reflections = !state.reflections;
                    },
                );
            } else if setting_type == SettingType::MotionBlur {
                commands.entity(check_fill).insert(MotionBlurCheckbox);
                commands.entity(checkbox_box).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        state.motion_blur = !state.motion_blur;
                    },
                );
            } else if setting_type == SettingType::Supersampling {
                commands.entity(check_fill).insert(SupersamplingCheckbox);
                commands.entity(checkbox_box).observe(
                    |_: On<Pointer<Click>>, mut state: ResMut<SettingsMenuState>| {
                        state.supersampling = !state.supersampling;
                    },
                );
            }
        }
        ControlType::Disabled(val) => {
            commands.spawn((
                ChildOf(ctrl_wrapper),
                UTextLabel {
                    text: val.to_string(),
                    font_size: 13.0,
                    color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                    ..default()
                },
                UNode::default(),
            ));
            // Faded Segmented Strength Indicator
            spawn_segmented_bar_disabled(commands.reborrow(), ctrl_wrapper);
        }
    }
}

// Spawns a segmented bar representing the setting's strength/value (4 segments)
fn spawn_segmented_bar(mut commands: Commands, parent: Entity, setting: SettingType) {
    let row = commands
        .spawn((
            ChildOf(parent),
            SegmentedBar { setting },
            UNode {
                width: UVal::Px(80.0),
                height: UVal::Px(3.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 3.0,
                ..default()
            },
        ))
        .id();

    // Spawn 4 segment blocks
    for i in 0..4 {
        commands.spawn((
            ChildOf(row),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                background_color: if i < 3 {
                    Color::srgba(1.0, 1.0, 1.0, 0.7) // Default 3 segments filled
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.15) // 1 segment empty
                },
                ..default()
            },
        ));
    }
}

fn spawn_segmented_bar_disabled(mut commands: Commands, parent: Entity) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(80.0),
                height: UVal::Px(3.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 3.0,
                ..default()
            },
        ))
        .id();

    for i in 0..4 {
        commands.spawn((
            ChildOf(row),
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                background_color: if i < 3 {
                    Color::srgba(1.0, 1.0, 1.0, 0.15)
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.05)
                },
                ..default()
            },
        ));
    }
}

// Spawns a keyboard icon next to a label in the footer
fn spawn_footer_shortcut(mut commands: Commands, parent: Entity, key: &str, label: &str) {
    let row = commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Keycap block
    let keycap = commands
        .spawn((
            ChildOf(row),
            UNode {
                padding: USides::axes(8.0, 4.0),
                background_color: Color::WHITE,
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(keycap),
        UTextLabel {
            text: key.to_string(),
            font_size: 11.0,
            color: Color::BLACK, // Dark text inside keycap
            ..default()
        },
        UNode::default(),
    ));

    // Label description
    commands.spawn((
        ChildOf(row),
        UTextLabel {
            text: label.to_string(),
            font_size: 12.0,
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            ..default()
        },
        UNode::default(),
    ));
}

// ── CORE UPDATE SYSTEMS ─────────────────────────────────────────────────────

// System to smoothly lerp card scale and border highlights on hover
fn animate_hover_elements(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut UBorder, &UInteraction, &mut CardHover)>,
) {
    let dt = time.delta_secs();
    let speed = 15.0;

    for (mut transform, mut border, interaction, mut anim) in &mut query {
        let (dest_scale, dest_border_w, dest_border_c) = match interaction {
            UInteraction::Hovered | UInteraction::Pressed => (
                anim.target_scale,
                anim.target_border_width,
                anim.target_border_color,
            ),
            _ => (
                anim.base_scale,
                anim.base_border_width,
                anim.base_border_color,
            ),
        };

        // Lerp Transform scale
        anim.current_scale = anim.current_scale.lerp(dest_scale, speed * dt);
        transform.scale = anim.current_scale;

        // Lerp UBorder width
        anim.current_border_width =
            anim.current_border_width + (dest_border_w - anim.current_border_width) * speed * dt;
        border.width = anim.current_border_width;

        // Lerp UBorder color channel by channel
        let current_linear = anim.current_border_color.to_linear();
        let dest_linear = dest_border_c.to_linear();
        let r = current_linear.red + (dest_linear.red - current_linear.red) * speed * dt;
        let g = current_linear.green + (dest_linear.green - current_linear.green) * speed * dt;
        let b = current_linear.blue + (dest_linear.blue - current_linear.blue) * speed * dt;
        let a = current_linear.alpha + (dest_linear.alpha - current_linear.alpha) * speed * dt;
        anim.current_border_color = Color::srgba(r, g, b, a);
        border.color = anim.current_border_color;
    }
}

// System to update settings description depending on hovered category
fn update_right_pane(
    state: Res<SettingsMenuState>,
    mut pane_query: Query<(
        Entity,
        &mut UTextLabel,
        Option<&RightPaneTitle>,
        Option<&RightPaneDesc>,
    )>,
) {
    if state.is_changed() {
        let (title, desc) = match state.selected_setting {
            SettingType::TextureFiltering => (
                "Texture Filtering",
                "Adjust the sharpness of textures viewed at an angle. Higher values (like Anisotropic 16x) improve visual clarity of distant surfaces but may reduce performance slightly.",
            ),
            SettingType::EnvironmentalDetail => (
                "Environmental Detail",
                "Controls the complexity and density of objects in the game world, including vegetation, debris, and distant terrain features. Higher settings details geometry but requires more processor power.",
            ),
            SettingType::DirectX => (
                "DirectX",
                "Select DirectX render mode. Appropriate version of Windows and supported GPU is required for DirectX 12. Using the DirectX 12 API can offer better performance depending on your hardware.",
            ),
            SettingType::TextureQuality => (
                "Texture Quality",
                "Adjusts the resolution of textures used on characters, environments, and objects. Higher values require significantly more VRAM.",
            ),
            SettingType::AmbientOcclusion => (
                "Ambient Occlusion",
                "Simulates realistic shadows in crevices and corners where ambient light is blocked. MHBAO provides high-fidelity depth shading.",
            ),
            SettingType::DynamicReflections => (
                "Dynamic Reflections",
                "Enables real-time reflections on wet surfaces, metallic materials, and water bodies. Turning this off improves FPS.",
            ),
            SettingType::MotionBlur => (
                "Motion Blur",
                "Simulates camera blur during high-speed movements, adding cinematic realism to rapid turns.",
            ),
            SettingType::Supersampling => (
                "Supersampling Anti-Aliasing",
                "Renders the game at a higher resolution and scales it down, providing the cleanest possible edges at a high performance cost.",
            ),
        };

        for (_, mut label, is_title, is_desc) in &mut pane_query {
            if is_title.is_some() {
                label.text = title.to_string();
            } else if is_desc.is_some() {
                label.text = desc.to_string();
            }
        }
    }
}

// System to simulate VRAM usage dynamics based on active options
fn update_vram_and_progress(
    state: Res<SettingsMenuState>,
    mut bar_query: Query<&mut UProgressBar, With<VramBar>>,
    mut label_query: Query<&mut UTextLabel, With<VramLabel>>,
) {
    if state.is_changed() {
        // Base VRAM usage
        let mut vram = 4500.0;

        // Add based on env detail
        vram += match state.env_detail {
            3 => 1500.0, // Ultra
            2 => 900.0,  // High
            1 => 450.0,  // Medium
            _ => 150.0,  // Low
        };

        if state.supersampling {
            vram += 2200.0;
        }
        if state.reflections {
            vram += 900.0;
        }
        if state.motion_blur {
            vram += 300.0;
        }
        if state.directx_version == 12 {
            vram += 800.0;
        } // DX12 overhead

        let max_vram = 11201.0;
        let pct = vram / max_vram;

        for mut bar in &mut bar_query {
            bar.value = pct;
        }
        for mut label in &mut label_query {
            label.text = format!("{:.0} MB / {:.0} MB", vram, max_vram);
        }
    }
}

// System to update label texts of value switchers and visual checkbox fills
fn update_value_selectors(
    state: Res<SettingsMenuState>,
    mut label_query: Query<(
        Entity,
        &mut UTextLabel,
        Option<&DirectxValueText>,
        Option<&EnvDetailValueText>,
    )>,
    checkbox_entities: Query<(
        Entity,
        Option<&ReflectionCheckbox>,
        Option<&MotionBlurCheckbox>,
        Option<&SupersamplingCheckbox>,
    )>,
    mut node_query: Query<&mut UNode>,
    mut segments: Query<(&SegmentedBar, &Children)>,
) {
    if state.is_changed() {
        // 1. DirectX and Detail value texts (using standard ASCII '<' and '>')
        for (_, mut label, dx, env) in &mut label_query {
            if dx.is_some() {
                label.text = format!("<  {}  >", state.directx_version);
            } else if env.is_some() {
                let env_label = match state.env_detail {
                    3 => "<  Ultra  >",
                    2 => "<  High  >",
                    1 => "<  Medium  >",
                    _ => "<  Low  >",
                };
                label.text = env_label.to_string();
            }
        }

        // 2. Vector-based checkbox inner fills (avoiding unicode checkbox squares)
        for (entity, refl, blur, sup) in &checkbox_entities {
            let is_checked = if refl.is_some() {
                state.reflections
            } else if blur.is_some() {
                state.motion_blur
            } else if sup.is_some() {
                state.supersampling
            } else {
                continue;
            };

            if let Ok(mut node) = node_query.get_mut(entity) {
                node.background_color = if is_checked {
                    Color::srgb(0.0, 0.9, 1.0) // Glowing cyan check fill
                } else {
                    Color::NONE
                };
            }
        }

        // 3. Update Segmented bar visuals
        for (bar, children) in &mut segments {
            let filled_count = match bar.setting {
                SettingType::TextureFiltering => 4, // 16x anisotropic = full
                SettingType::AmbientOcclusion => 3, // MHBAO = 3/4
                SettingType::DirectX => {
                    if state.directx_version == 12 {
                        4
                    } else {
                        3
                    }
                }
                SettingType::EnvironmentalDetail => (state.env_detail + 1) as usize,
                _ => 3,
            };

            for (idx, child) in children.iter().enumerate() {
                if let Ok(mut node) = node_query.get_mut(child) {
                    node.background_color = if idx < filled_count {
                        Color::srgba(1.0, 1.0, 1.0, 0.7)
                    } else {
                        Color::srgba(1.0, 1.0, 1.0, 0.15)
                    };
                }
            }
        }
    }
}

// 2D Camera controller for panning and zooming the flat UI canvas
fn camera_control_2d(
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    let Some((mut transform, mut projection)) = camera_query.iter_mut().next() else {
        return;
    };

    if let Projection::Orthographic(ref mut ortho) = *projection {
        // Zoom flat UI via scroll wheel
        let zoom_delta = scroll.delta.y;
        if zoom_delta.abs() > 0.0 {
            ortho.scale -= zoom_delta * 0.05;
            ortho.scale = ortho.scale.clamp(0.5, 2.0); // 0.5x to 2x zoom
        }

        // Pan flat UI via right-click and drag
        if mouse_button.pressed(MouseButton::Right) {
            let delta = motion.delta;
            if delta.length_squared() > 0.0 {
                let sensitivity = ortho.scale; // Scale pan speed with zoom level
                transform.translation.x -= delta.x * sensitivity;
                transform.translation.y += delta.y * sensitivity;
            }
        }
    }
}

// Keyboard shortcuts: ESC to exit, R to reset, ENTER to apply
fn handle_keyboard_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<SettingsMenuState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("ESC pressed - closing settings menu.");
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        info!("R pressed - resetting settings to default.");
        *state = SettingsMenuState::default();
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        info!("ENTER pressed - applying changes.");
    }
}
