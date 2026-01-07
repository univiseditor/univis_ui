use bevy::prelude::*;
use crate::prelude::*;

// =========================================================
// Plugin
// =========================================================

pub struct UnivisTogglePlugin;

impl Plugin for UnivisTogglePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UToggle>()
            .add_message::<ToggleChangedEvent>()
            .add_systems(Update, (
                init_toggle_visuals,
                animate_toggle_knob,
                sync_toggle_colors,
            ).chain())
            .add_observer(update_toggle_state);
    }
}

// =========================================================
// Components
// =========================================================

/// مكون Toggle الرئيسي
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct UToggle {
    /// الحالة الحالية
    pub checked: bool,
    
    /// الحالة السابقة (للكشف عن التغييرات)
    previous_checked: bool,
    
    // --- الأبعاد ---
    pub width: f32,
    pub height: f32,
    
    // --- الألوان ---
    pub track_color_off: Color,
    pub track_color_on: Color,
    pub knob_color: Color,
    
    // --- الحركة ---
    pub animation_speed: f32,
    pub current_offset: f32, // موضع الزر الحالي (0.0 = يسار، 1.0 = يمين)
    
    // --- خيارات إضافية ---
    pub disabled: bool,
}

impl Default for UToggle {
    fn default() -> Self {
        Self {
            checked: false,
            previous_checked: false,
            width: 60.0,
            height: 30.0,
            track_color_off: Color::srgb(0.3, 0.3, 0.3),
            track_color_on: Color::srgb(0.2, 0.6, 1.0),
            knob_color: Color::WHITE,
            animation_speed: 10.0,
            current_offset: 0.0,
            disabled: false,
        }
    }
}

impl UToggle {
    /// إنشاء Toggle جديد
    pub fn new() -> Self {
        Self::default()
    }
    
    /// تعيين الحالة الأولية
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.previous_checked = checked;
        self.current_offset = if checked { 1.0 } else { 0.0 };
        self
    }
    
    /// تخصيص الألوان
    pub fn with_colors(mut self, off: Color, on: Color, knob: Color) -> Self {
        self.track_color_off = off;
        self.track_color_on = on;
        self.knob_color = knob;
        self
    }
    
    /// تخصيص الحجم
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    /// تعطيل Toggle
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    
    /// أنماط جاهزة
    pub fn ios_style() -> Self {
        Self {
            width: 51.0,
            height: 31.0,
            track_color_off: Color::srgb(0.78, 0.78, 0.78),
            track_color_on: Color::srgb(0.2, 0.78, 0.35),
            knob_color: Color::WHITE,
            animation_speed: 12.0,
            ..default()
        }
    }
    
    pub fn material_style() -> Self {
        Self {
            width: 52.0,
            height: 32.0,
            track_color_off: Color::srgb(0.6, 0.6, 0.6),
            track_color_on: Color::srgb(0.38, 0.65, 0.87),
            knob_color: Color::WHITE,
            animation_speed: 15.0,
            ..default()
        }
    }
    
    pub fn sci_fi_style() -> Self {
        Self {
            width: 70.0,
            height: 35.0,
            track_color_off: Color::srgb(0.1, 0.1, 0.2),
            track_color_on: Color::srgb(0.0, 0.8, 1.0),
            knob_color: Color::srgb(0.9, 1.0, 1.0),
            animation_speed: 8.0,
            ..default()
        }
    }
}

/// علامات داخلية للأجزاء
#[derive(Component)]
struct ToggleTrack;

#[derive(Component)]
struct ToggleKnob;

// =========================================================
// Systems
// =========================================================

/// إنشاء الهيكل البصري للـ Toggle
fn init_toggle_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UToggle), Added<UToggle>>,
) {
    for (entity, toggle) in query.iter() {
        
        let track_color = if toggle.checked {
            toggle.track_color_on
        } else {
            toggle.track_color_off
        };
        
        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(toggle.width),
                height: UVal::Px(toggle.height),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                ..default()
            },
        )).with_children(|parent| {
            // Track (المسار/الخلفية)
            parent.spawn((
                UNode {
                    width: UVal::Px(toggle.width),
                    height: UVal::Px(toggle.height),
                    background_color: track_color,
                    border_radius: UCornerRadius::all(toggle.height / 2.0),
                    padding: USides::all(2.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    ..default()
                },
                ToggleTrack,
            )).with_children(|track_parent| {
                // Knob (الزر المتحرك)
                let knob_size = toggle.height - 4.0;
                let initial_x = if toggle.checked {
                    toggle.width - knob_size - 4.0
                } else {
                    2.0
                };
                
                track_parent.spawn((
                    UNode {
                        width: UVal::Px(knob_size),
                        height: UVal::Px(knob_size),
                        background_color: toggle.knob_color,
                        border_radius: UCornerRadius::all(knob_size / 2.0),
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Px(initial_x),
                        top: UVal::Px(2.0),
                        ..default()
                    },
                    ToggleKnob,
                ));
            });
        });
    }
}

/// تحديث حالة Toggle عند النقر
fn update_toggle_state(
    events: On<Pointer<Click>>,
    mut toggle_query: Query<&mut UToggle>,
    
) {
    if let Ok(mut toggle) = toggle_query.get_mut(events.entity.entity()) {
        if toggle.disabled {
            return;
        }
            // عند النقر
            toggle.previous_checked = toggle.checked;
            toggle.checked = !toggle.checked;
        
    }
}

/// تحريك الزر (Animation)
fn animate_toggle_knob(
    time: Res<Time>,
    mut toggle_query: Query<(&mut UToggle, &Children)>,
    track_query: Query<&Children, With<ToggleTrack>>,
    mut knob_query: Query<&mut USelf, With<ToggleKnob>>,
) {
    for (mut toggle, children) in toggle_query.iter_mut() {
        
        // الهدف النهائي
        let target_offset = if toggle.checked { 1.0 } else { 0.0 };
        
        // الفرق
        let diff = target_offset - toggle.current_offset;
        
        // إذا كان الفرق صغير جداً، اعتبره وصل
        if diff.abs() < 0.01 {
            toggle.current_offset = target_offset;
            continue;
        }
        
        // الحركة السلسة (Lerp)
        let delta = time.delta_secs() * toggle.animation_speed;
        toggle.current_offset += diff * delta;
        
        // تطبيق على الـ Knob
        let track_entity = children.iter()
            .find(|&child| track_query.get(child).is_ok());
        
        if let Some(track) = track_entity {
            if let Ok(track_children) = track_query.get(track) {
                for knob_entity in track_children.iter() {
                    if let Ok(mut uself) = knob_query.get_mut(knob_entity) {
                        let knob_size = toggle.height - 4.0;
                        let max_offset = toggle.width - knob_size - 4.0;
                        let new_x = 2.0 + (toggle.current_offset * (max_offset - 2.0));
                        
                        uself.left = UVal::Px(new_x);
                    }
                }
            }
        }
    }
}

/// مزامنة لون Track مع الحالة
fn sync_toggle_colors(
    toggle_query: Query<(&UToggle, &Children), Changed<UToggle>>,
    mut track_query: Query<&mut UNode, With<ToggleTrack>>,
) {
    for (toggle, children) in toggle_query.iter() {
        
        let target_color = if toggle.checked {
            toggle.track_color_on
        } else {
            toggle.track_color_off
        };
        
        // تحديث لون Track
        for child in children.iter() {
            if let Ok(mut node) = track_query.get_mut(child) {
                // تحريك اللون تدريجياً (يمكن استخدام Lerp هنا أيضاً)
                node.background_color = target_color;
            }
        }
    }
}

// =========================================================
// Event (اختياري للتفاعل الخارجي)
// =========================================================

/// حدث يُطلق عند تغيير حالة Toggle
#[derive(Message)]
pub struct ToggleChangedEvent {
    pub entity: Entity,
    pub checked: bool,
}

/// نظام لإطلاق الأحداث
pub fn emit_toggle_events(
    mut events: MessageWriter<ToggleChangedEvent>,
    query: Query<(Entity, &UToggle), Changed<UToggle>>,
) {
    for (entity, toggle) in query.iter() {
        if toggle.checked != toggle.previous_checked {
            events.write(ToggleChangedEvent {
                entity,
                checked: toggle.checked,
            });
        }
    }
}

