use bevy::{post_process::bloom::Bloom, prelude::*};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UnivisUiPlugin,LayoutProfilingPlugin))
        .add_systems(Startup, setup_sci_fi_hud)
        .add_systems(Update, (animate_holo_pulse, animate_sim_data))
        .run();
}

pub fn setup_sci_fi_hud(mut commands: Commands) {
    // الألوان الرئيسية (Neon Palette)
    let cyan = Color::srgb(0.0, 0.5, 2.5);
    let orange = Color::srgb(5.0, 0.6, 0.0);
    let deep_bg = Color::srgb(0.02, 0.05, 0.1).with_alpha(0.85);
    
    // الخط الافتراضي
    let font = Handle::<Font>::default();

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Bloom::NATURAL,
    ));

    // 1. Root Container (Full Screen Overlay)
    commands.spawn((
    
        // UWorldRoot { size: Vec2::new(1280.0, 720.0), resolution_scale: 1.0 },
        UScreenRoot,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            // خلفية شفافة لتبدو كشاشة عرض رأسية
            background_color: Color::BLACK.with_alpha(0.2), 
            padding: USides::all(40.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
             // هوامش الشاشة
            ..default()
        }
    )).with_children(|screen| {

        // =================================================
        // LEFT PANEL: SYSTEM STATUS (Vertical Stack)
        // =================================================
        screen.spawn((
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Percent(1.0),
                background_color: deep_bg,
                shape_mode: UShapeMode::Cut,
                border_radius: UCornerRadius::bottom(20.0),
                padding: USides::all(20.0),
                ..default()
            },
            UBorder { width: 2.0, color: cyan.with_alpha(0.5), ..default() },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
                ..default()
            }
        )).with_children(|left| {
            // Header
            left.spawn(UTextLabel {
                text: "SYSTEM INTEGRITY".to_string(),
                font_size: 20.0,
                color: cyan,
                font: font.clone(),
                justify: Justify::Center,
                ..default()
            });

            // Bars with simulated data
            spawn_hud_bar(left, "SHIELD CORE", cyan, 1.0, 0.0, font.clone());
            spawn_hud_bar(left, "HULL INTEGRITY", orange, 0.5, 10.0, font.clone());
            spawn_hud_bar(left, "AUX POWER", cyan, 2.0, 5.0, font.clone());
            
            // Spacer
            left.spawn(UNode { height: UVal::Flex(1.0), ..default() });

            // Log Window (Simulated)
            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(150.0),
                    background_color: Color::BLACK.with_alpha(0.5),
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(10.0),
                    ..default()
                },
                ULayout { display: UDisplay::Flex, flex_direction: UFlexDirection::Column, gap: 5.0, ..default() }
            )).with_children(|log| {
                 for i in 1..5 {
                     log.spawn(UTextLabel {
                         text: format!("> Sys.check_{} ... OK", i),
                         font_size: 12.0,
                         color: cyan.with_alpha(0.7),
                         font: font.clone(),
                         ..default()
                     });
                 }
            });
        });

        // =================================================
        // CENTER: ARC REACTOR (Radial Layout) - THE BOSS
        // =================================================
        screen.spawn((
            UNode {

                width: UVal::Flex(2.0), 
                height: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            }
        )).with_children(|center| {
            
            // Container for Radial Layout
            center.spawn((
                UNode {
                    // حجم الحاوية الدائرية (سيتمدد RadialPlacer بناء على المحتوى)
                    width: UVal::Px(300.0),
                    height: UVal::Px(300.0),
                    padding: USides::all(50.0), 
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Radial, 
                    ..default()
                }
            )).with_children(|reactor| {
                
         
                reactor.spawn((
                    UNode {
                        width: UVal::Px(120.0),
                        height: UVal::Px(120.0),
                        border_radius: UCornerRadius::all(60.0), // دائرة كاملة
                        background_color: cyan.with_alpha(0.2),
                        ..default()
                    },
                    UBorder { width: 4.0, color: cyan, ..default() },
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Percent(0.5),
                        top: UVal::Percent(0.5),
                        ..default() 
                    },
                    // إضافة نبض للنواة
                    HoloPulse { speed: 2.0, base_color: Srgba::new(0.0, 0.9, 1.0, 0.2) }
                )).with_children(|core| {
                    core.spawn((
                        UTextLabel {
                            text: "100%".to_string(),
                            font_size: 30.0,
                            color: Color::WHITE,
                            font: font.clone(),
                            ..default()
                        },
                        // USelf {
                        //     align_self: UAlignSelf::Center,
                        //     ..default()
                        // },
                        UNode { 
                            margin: USides { top: 45.0, left: 30.0, ..default() },
                            ..default() 
                        }
                    ));
                });

                // 2. Satellites (Radial Items)
                for i in 0..8 {
                    reactor.spawn((
                        UNode {
                            width: UVal::Px(60.0),
                            height: UVal::Px(60.0),
                            border_radius: UCornerRadius::all(20.0),
                            background_color: deep_bg,
                            shape_mode: UShapeMode::Cut,
                            ..default()
                        },
                        ULayout {
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            ..default()
                        },
                        UBorder { width: 2.0, color: orange.with_alpha(0.8), ..default() },
                        HoloPulse { speed: 1.0 + (i as f32 * 0.5), base_color: Srgba::new(0.02, 0.05, 0.1, 0.8) }
                    )).with_children(|sat| {
                        sat.spawn(UTextLabel {
                            text: format!("M-{}", i),
                            font_size: 14.0,
                            color: Color::srgb(2.0, 2.0, 2.0),
                            font: font.clone(),
                            autosize: false, 
                            justify: Justify::Center,
                            ..default()
                        });
                    });
                }
            });
        });

        // =================================================
        // RIGHT PANEL: WEAPONS GRID (Masonry/Grid)
        // =================================================
        screen.spawn((
            UNode {
                width: UVal::Px(350.0),
                height: UVal::Percent(1.0),
                background_color: deep_bg,
                shape_mode: UShapeMode::Cut,
                border_radius: UCornerRadius {top_left: 0.0,top_right: 20.0, bottom_left: 20.0, bottom_right: 0.0},
                padding: USides::all(20.0),
                ..default()
            },
            UBorder { width: 2.0, color: cyan.with_alpha(0.5), ..default() },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 15.0,
                ..default()
            }
        )).with_children(|right| {
             right.spawn(UTextLabel {
                text: "ACTIVE MODULES".to_string(),
                font_size: 20.0,
                color: cyan,
                font: font.clone(),
                justify: Justify::Center,
                ..default()
            });

            // Grid Container
            right.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Grid,
                    grid_columns: 2, 
                    gap: 10.0,
                    ..default()
                }
            )).with_children(|grid| {

                for i in 0..6 {
                    let is_active = i % 2 == 0;
                    let color = if is_active { cyan } else { orange };
                    let status = if is_active { "ONLINE" } else { "OFFLINE" };

                    grid.spawn((
                        UNode {
                            height: UVal::Px(80.0), 
                            background_color: color.with_alpha(0.1),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::all(10.0),
                            ..default()
                        },
                        UBorder { width: 1.0, color: color.with_alpha(0.6), ..default() },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            justify_content: UJustifyContent::SpaceBetween,
                            ..default()
                        }
                    )).with_children(|card| {
                        card.spawn(UTextLabel {
                            text: format!("MOD-0{}", i+1),
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: font.clone(),
                            ..default()
                        });
                        
                        card.spawn(UTextLabel {
                            text: status.to_string(),
                            font_size: 12.0,
                            color: color,
                            font: font.clone(),
                            ..default()
                        });
                    });
                }
            });

            right.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(60.0),
                    background_color: orange.with_alpha(0.2),
                    border_radius: UCornerRadius::all(10.0),
                    margin: USides::top(20.0),
                    ..default()
                },
                UBorder { width: 2.0, color: orange, ..default() },
                ULayout { justify_content: UJustifyContent::Center, align_items: UAlignItems::Center, ..default() },
                Pickable::default(),
                UInteractionColors {
                    normal: orange.with_alpha(0.2),
                    hovered: orange.with_alpha(0.4),
                    pressed: Color::srgb(0.0, 10.0, 0.0),
                }
            )).with_children(|btn| {
                btn.spawn(UTextLabel {
                    text: "EMERGENCY VENT".to_string(),
                    font_size: 18.0,
                    color: orange,
                    font: font.clone(),
                    ..default()
                });
            });
        });

    });
}

// دالة مساعدة لشريط HUD
fn spawn_hud_bar(parent: &mut ChildSpawnerCommands, label: &str, color: Color, speed: f32, offset: f32, font: Handle<Font>) {
    parent.spawn((
        UNode { width: UVal::Percent(1.0), ..default() },
        ULayout { display: UDisplay::Flex, flex_direction: UFlexDirection::Column, gap: 5.0, ..default() }
    )).with_children(|container| {
        container.spawn(UTextLabel {
            text: label.to_string(),
            font_size: 12.0,
            color: color,
            font: font,
            ..default()
        });

        container.spawn((
            UProgressBar { value: 0.5, bar_color: color },
            SimData { speed, offset }, 
            UNode {
                height: UVal::Px(8.0),
                background_color: Color::BLACK.with_alpha(0.5),
                ..default()
            },
            UBorder { width: 1.0, color: color.with_alpha(0.3), ..default() }
        ));
    });
}


#[derive(Component)]
pub struct HoloPulse {
    pub speed: f32,
    pub base_color: Srgba,
}

#[derive(Component)]
pub struct SimData {
    pub speed: f32,
    pub offset: f32,
}

fn animate_holo_pulse(
    time: Res<Time>,
    mut query: Query<(&mut UNode, &HoloPulse)>,
) {
    for (mut node, pulse) in query.iter_mut() {
        let t = time.elapsed_secs() * pulse.speed;
        let alpha_mod = (t.sin() + 1.0) * 0.5; // 0.0 to 1.0
        
        // تلاعب بالشفافية والسطوع
        let mut color = pulse.base_color;
        color.alpha = 0.3 + (alpha_mod * 0.4); // يتأرجح بين 0.3 و 0.7
        node.background_color = Color::Srgba(color);
    }
}

fn animate_sim_data(
    time: Res<Time>,
    mut query: Query<(&mut UProgressBar, &SimData)>,
) {
    for (mut bar, sim) in query.iter_mut() {
        let t = time.elapsed_secs() * sim.speed + sim.offset;
        // حركة عشوائية ناعمة باستخدام Noise-like math
        let val = (t.sin() * 0.3) + (t.cos() * 0.2) + 0.5;
        bar.value = val.clamp(0.05, 1.0);
    }
}