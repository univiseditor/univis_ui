use bevy::prelude::*;
use crate::prelude::*;

// =========================================================
// Plugin
// =========================================================

pub struct UnivisSeekBarPlugin;

impl Plugin for UnivisSeekBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<USeekBar>()
            .add_message::<SeekBarChangedEvent>()
            .add_systems(Update, (
                init_seekbar_visuals,
                // handle_seekbar_interaction,
                update_seekbar_visuals,
                animate_seekbar_thumb,
                emit_seekbar_events,
            ).chain());
    }
}

// =========================================================
// Components
// =========================================================

/// مكون SeekBar الرئيسي
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct USeekBar {
    /// القيمة الحالية (0.0 - 1.0)
    pub value: f32,
    
    /// القيمة السابقة (للكشف عن التغييرات)
    previous_value: f32,
    
    // --- الأبعاد ---
    pub width: f32,
    pub track_height: f32,
    pub thumb_size: f32,
    
    // --- الألوان ---
    pub track_color: Color,
    pub fill_color: Color,
    pub thumb_color: Color,
    pub thumb_hover_color: Color,
    
    // --- الحالة ---
    pub disabled: bool,
    pub is_dragging: bool,
    pub show_value: bool,
    
    // --- الحركة ---
    pub smooth_animation: bool,
    pub animation_speed: f32,
    pub target_value: f32,
    
    // --- النطاق (اختياري للقيم الفعلية) ---
    pub min_value: f32,
    pub max_value: f32,
    pub step: Option<f32>, // للقفز بين قيم محددة
}

impl Default for USeekBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            previous_value: 0.0,
            width: 200.0,
            track_height: 6.0,
            thumb_size: 18.0,
            track_color: Color::srgb(0.3, 0.3, 0.35),
            fill_color: Color::srgb(0.2, 0.6, 1.0),
            thumb_color: Color::WHITE,
            thumb_hover_color: Color::srgb(0.9, 0.95, 1.0),
            disabled: false,
            is_dragging: false,
            show_value: false,
            smooth_animation: true,
            animation_speed: 15.0,
            target_value: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            step: None,
        }
    }
}

impl USeekBar {
    /// إنشاء SeekBar جديد
    pub fn new() -> Self {
        Self::default()
    }
    
    /// تعيين القيمة الأولية
    pub fn with_value(mut self, value: f32) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        self.value = clamped;
        self.previous_value = clamped;
        self.target_value = clamped;
        self
    }
    
    /// تخصيص الحجم
    pub fn with_size(mut self, width: f32, track_height: f32, thumb_size: f32) -> Self {
        self.width = width;
        self.track_height = track_height;
        self.thumb_size = thumb_size;
        self
    }
    
    /// تخصيص الألوان
    pub fn with_colors(mut self, track: Color, fill: Color, thumb: Color) -> Self {
        self.track_color = track;
        self.fill_color = fill;
        self.thumb_color = thumb;
        self.thumb_hover_color = thumb;
        self
    }
    
    /// تخصيص النطاق
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }
    
    /// تفعيل القفز بخطوات
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }
    
    /// إظهار القيمة
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }
    
    /// تعطيل الحركة السلسة
    pub fn instant(mut self) -> Self {
        self.smooth_animation = false;
        self
    }
    
    /// الحصول على القيمة الفعلية (ضمن النطاق)
    pub fn real_value(&self) -> f32 {
        self.min_value + (self.value * (self.max_value - self.min_value))
    }
    
    /// تعيين القيمة الفعلية
    pub fn set_real_value(&mut self, real_value: f32) {
        let normalized = (real_value - self.min_value) / (self.max_value - self.min_value);
        self.value = normalized.clamp(0.0, 1.0);
        self.target_value = self.value;
    }
    
    // === أنماط جاهزة ===
    
    pub fn volume_style() -> Self {
        Self {
            width: 150.0,
            track_height: 4.0,
            thumb_size: 16.0,
            track_color: Color::srgb(0.25, 0.25, 0.3),
            fill_color: Color::srgb(0.0, 0.8, 0.4),
            thumb_color: Color::srgb(0.9, 0.95, 1.0),
            show_value: true,
            ..default()
        }
    }
    
    pub fn video_style() -> Self {
        Self {
            width: 400.0,
            track_height: 5.0,
            thumb_size: 14.0,
            track_color: Color::srgba(0.5, 0.5, 0.5, 0.5),
            fill_color: Color::srgb(0.9, 0.1, 0.2),
            thumb_color: Color::srgb(0.9, 0.1, 0.2),
            show_value: false,
            ..default()
        }
    }
    
    pub fn brightness_style() -> Self {
        Self {
            width: 200.0,
            track_height: 6.0,
            thumb_size: 20.0,
            track_color: Color::srgb(0.2, 0.2, 0.25),
            fill_color: Color::srgb(1.0, 0.9, 0.3),
            thumb_color: Color::srgb(1.0, 1.0, 0.5),
            show_value: true,
            min_value: 0.0,
            max_value: 100.0,
            ..default()
        }
    }
    
    pub fn sci_fi_style() -> Self {
        Self {
            width: 300.0,
            track_height: 8.0,
            thumb_size: 24.0,
            track_color: Color::srgb(0.05, 0.1, 0.2),
            fill_color: Color::srgb(0.0, 0.8, 1.0),
            thumb_color: Color::srgb(0.5, 1.0, 1.0),
            thumb_hover_color: Color::srgb(0.8, 1.0, 1.0),
            show_value: true,
            ..default()
        }
    }
}

/// علامات داخلية
#[derive(Component)]
struct SeekBarTrack;

#[derive(Component)]
struct SeekBarFill;

#[derive(Component)]
struct SeekBarThumb;

#[derive(Component)]
struct SeekBarValueLabel;

// =========================================================
// Systems
// =========================================================

/// إنشاء الهيكل البصري
fn init_seekbar_visuals(
    mut commands: Commands,
    query: Query<(Entity, &USeekBar), Added<USeekBar>>,
) {
    for (entity, seekbar) in query.iter() {
        
        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(seekbar.width),
                height: UVal::Px(seekbar.thumb_size + 10.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                gap: 5.0,
                ..default()
            },
        )).with_children(|parent| {
            
            // Container للـ Track و Thumb
            parent.spawn((
                UNode {
                    width: UVal::Px(seekbar.width),
                    height: UVal::Px(seekbar.thumb_size),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            )).with_children(|track_container| {
                
                // Track (الخلفية)
                track_container.spawn((
                    UNode {
                        width: UVal::Px(seekbar.width),
                        height: UVal::Px(seekbar.track_height),
                        background_color: seekbar.track_color,
                        border_radius: UCornerRadius::all(seekbar.track_height / 2.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        ..default()
                    },
                    SeekBarTrack,
                    Pickable::default(),
                )).with_children(|track_parent| {
                    
                    // Fill (الجزء المملوء)
                    let fill_width = seekbar.width * seekbar.value;
                    track_parent.spawn((
                        UNode {
                            width: UVal::Px(fill_width),
                            height: UVal::Px(seekbar.track_height),
                            background_color: seekbar.fill_color,
                            border_radius: UCornerRadius::all(seekbar.track_height / 2.0),
                            ..default()
                        },
                        SeekBarFill,
                    ));
                    
                    // Thumb (الزر المتحرك)
                    let thumb_x = (seekbar.width - seekbar.thumb_size) * seekbar.value;
                    track_parent.spawn((
                        UNode {
                            width: UVal::Px(seekbar.thumb_size),
                            height: UVal::Px(seekbar.thumb_size),
                            background_color: seekbar.thumb_color,
                            border_radius: UCornerRadius::all(seekbar.thumb_size / 2.0),
                            ..default()
                        },
                        USelf {
                            position_type: UPositionType::Absolute,
                            left: UVal::Px(thumb_x),
                            top: UVal::Px((seekbar.track_height - seekbar.thumb_size) / 2.0),
                            ..default()
                        },
                        SeekBarThumb,
                        Pickable::default(),
                    ));
                });
            });
            
            // Value Label (اختياري)
            if seekbar.show_value {
                parent.spawn((
                    UTextLabel {
                        text: format!("{:.0}", seekbar.real_value()),
                        font_size: 12.0,
                        color: Color::srgb(0.7, 0.7, 0.8),
                        ..default()
                    },
                    SeekBarValueLabel,
                ));
            }
        });
    }
}

// /// معالجة التفاعل (السحب)
// fn handle_seekbar_interaction(
//     mut seekbar_query: Query<(&mut USeekBar, &Children, &GlobalTransform)>,
//     track_query: Query<(Entity, &GlobalTransform, &ComputedSize), With<SeekBarTrack>>,
//     mouse_button: Res<ButtonInput<MouseButton>>,
//     windows: Query<&Window>,
// ) {
//     let Ok(window) = windows.single() else { return };
//     let Some(cursor_pos) = window.cursor_position() else { return };

    
    
//     for (mut seekbar, children, _) in seekbar_query.iter_mut() {
//         if seekbar.disabled {
//             continue;
//         }
        
//         // البحث عن Track
//         for (_track_entity,track_transform,track_size) in track_query.iter() {
            
//             seekbar.is_dragging = true;

//             // أثناء السحب
//             if seekbar.is_dragging && mouse_button.pressed(MouseButton::Left) {
                
//                 // حساب الموضع النسبي
//                 let track_world_pos = track_transform.translation().truncate();
//                 let track_left = track_world_pos.x - (track_size.width / 2.0);
                
//                 let relative_x = cursor_pos.x - track_left;
//                 let mut new_value = (relative_x / seekbar.width).clamp(0.0, 1.0);
                
//                 info!("seekbar {}",new_value);
//                 // تطبيق Step إذا وُجد
//                 if let Some(step) = seekbar.step {
//                     let range = seekbar.max_value - seekbar.min_value;
//                     let real = seekbar.min_value + (new_value * range);
//                     let stepped = (real / step).round() * step;
//                     new_value = (stepped - seekbar.min_value) / range;
//                 }
                
//                 seekbar.target_value = new_value;
                
//                 if !seekbar.smooth_animation {
//                     seekbar.value = new_value;
//                 }
//             }
            
//             // نهاية السحب
//             if !mouse_button.pressed(MouseButton::Left) {
//                 seekbar.is_dragging = false;
//             }
//         }
//     }
// }

/// تحديث المظهر
fn update_seekbar_visuals(
    seekbar_query: Query<(&USeekBar, &Children), Changed<USeekBar>>,
    track_query: Query<&Children, With<SeekBarTrack>>,
    mut fill_query: Query<&mut UNode, With<SeekBarFill>>,
    mut thumb_query: Query<&mut USelf, With<SeekBarThumb>>,
    mut label_query: Query<&mut UTextLabel, With<SeekBarValueLabel>>,
) {
    for (seekbar, children) in seekbar_query.iter() {
        
        // تحديث Fill و Thumb
        for child in children.iter() {
            if let Ok(track_children) = track_query.get(child) {
                for track_child in track_children.iter() {
                    
                    // تحديث Fill
                    if let Ok(mut fill_node) = fill_query.get_mut(track_child) {
                        let fill_width = seekbar.width * seekbar.value;
                        fill_node.width = UVal::Px(fill_width);
                    }
                    
                    // تحديث Thumb
                    if let Ok(mut thumb_uself) = thumb_query.get_mut(track_child) {
                        let thumb_x = (seekbar.width - seekbar.thumb_size) * seekbar.value;
                        thumb_uself.left = UVal::Px(thumb_x);
                    }
                }
            }
            
            // تحديث Label
            if let Ok(mut label) = label_query.get_mut(child) {
                label.text = format!("{:.0}", seekbar.real_value());
            }
        }
    }
}

/// حركة سلسة
fn animate_seekbar_thumb(
    time: Res<Time>,
    mut query: Query<&mut USeekBar>,
) {
    for mut seekbar in query.iter_mut() {
        if !seekbar.smooth_animation {
            continue;
        }
        
        let diff = seekbar.target_value - seekbar.value;
        
        if diff.abs() < 0.001 {
            seekbar.value = seekbar.target_value;
            continue;
        }
        
        let delta = time.delta_secs() * seekbar.animation_speed;
        seekbar.value += diff * delta;
    }
}

/// إطلاق الأحداث
fn emit_seekbar_events(
    mut events:MessageWriter<SeekBarChangedEvent>,
    mut query: Query<(Entity, &mut USeekBar)>,
) {
    for (entity, mut seekbar) in query.iter_mut() {
        if (seekbar.value - seekbar.previous_value).abs() > 0.001 {
            events.write(SeekBarChangedEvent {
                entity,
                value: seekbar.value,
                real_value: seekbar.real_value(),
            });
            
            seekbar.previous_value = seekbar.value;
        }
    }
}

// =========================================================
// Event
// =========================================================

#[derive(Message)]
pub struct SeekBarChangedEvent {
    pub entity: Entity,
    pub value: f32,      // 0.0 - 1.0
    pub real_value: f32, // القيمة الفعلية ضمن النطاق
}