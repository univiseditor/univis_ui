use bevy::prelude::*;
use crate::prelude::*;

// =========================================================
// Plugin
// =========================================================

pub struct UnivisRadioPlugin;

impl Plugin for UnivisRadioPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<URadioButton>()
            .register_type::<URadioGroup>()
            .add_message::<RadioButtonChangedEvent>()
            .add_systems(Update, (
                init_radio_visuals,
                init_radio_group,
                update_radio_visuals,
                animate_radio_check,
                emit_radio_events,
            ).chain());
    }
}

// =========================================================
// Components
// =========================================================

/// مكون RadioButton الفردي
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Pickable)]
pub struct URadioButton {
    /// القيمة الفريدة لهذا الزر (في المجموعة)
    pub value: String,
    
    /// هل هو مختار؟
    pub checked: bool,
    
    /// الحالة السابقة
    previous_checked: bool,
    
    // --- الأبعاد ---
    pub size: f32,
    
    // --- الألوان ---
    pub ring_color: Color,
    pub ring_checked_color: Color,
    pub dot_color: Color,
    pub ring_width: f32,
    
    // --- الحالة ---
    pub disabled: bool,
    
    // --- الحركة ---
    pub animation_speed: f32,
    pub current_scale: f32, // حجم النقطة الداخلية (0.0 - 1.0)
    
    // --- المجموعة ---
    pub group: Option<Entity>, // Entity الـ RadioGroup
}

impl Default for URadioButton {
    fn default() -> Self {
        Self {
            value: String::new(),
            checked: false,
            previous_checked: false,
            size: 20.0,
            ring_color: Color::srgb(0.4, 0.4, 0.45),
            ring_checked_color: Color::srgb(0.2, 0.6, 1.0),
            dot_color: Color::srgb(0.2, 0.6, 1.0),
            ring_width: 2.0,
            disabled: false,
            animation_speed: 12.0,
            current_scale: 0.0,
            group: None,
        }
    }
}

impl URadioButton {
    /// إنشاء RadioButton جديد
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            ..default()
        }
    }
    
    /// تعيين كمختار
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self.previous_checked = true;
        self.current_scale = 1.0;
        self
    }
    
    /// تخصيص الحجم
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    /// تخصيص الألوان
    pub fn with_colors(mut self, ring: Color, checked: Color, dot: Color) -> Self {
        self.ring_color = ring;
        self.ring_checked_color = checked;
        self.dot_color = dot;
        self
    }
    
    /// تعطيل
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    
    // === أنماط جاهزة ===
    
    pub fn primary_style(value: impl Into<String>) -> Self {
        Self::new(value)
            .with_colors(
                Color::srgb(0.4, 0.4, 0.45),
                Color::srgb(0.2, 0.6, 1.0),
                Color::srgb(0.2, 0.6, 1.0),
            )
    }
    
    pub fn success_style(value: impl Into<String>) -> Self {
        Self::new(value)
            .with_colors(
                Color::srgb(0.3, 0.4, 0.35),
                Color::srgb(0.2, 0.8, 0.3),
                Color::srgb(0.2, 0.8, 0.3),
            )
    }
    
    pub fn danger_style(value: impl Into<String>) -> Self {
        Self::new(value)
            .with_colors(
                Color::srgb(0.4, 0.3, 0.3),
                Color::srgb(0.9, 0.2, 0.2),
                Color::srgb(0.9, 0.2, 0.2),
            )
    }
}

/// مكون RadioGroup - للتحكم في مجموعة أزرار
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct URadioGroup {
    /// القيمة المختارة حالياً
    pub selected_value: Option<String>,
    
    /// القيمة السابقة
    previous_value: Option<String>,
    
    /// الأزرار في المجموعة (يتم تعبئتها تلقائياً)
    pub buttons: Vec<Entity>,
    
    /// هل يجب أن يكون هناك اختيار دائماً؟
    pub require_selection: bool,
    
    // === إضافة خيارات التخطيط ===
    pub gap: f32,
    pub direction: UFlexDirection,
}

impl Default for URadioGroup {
    fn default() -> Self {
        Self {
            selected_value: None,
            previous_value: None,
            buttons: Vec::new(),
            require_selection: true,
            gap: 15.0,
            direction: UFlexDirection::Column,
        }
    }
}

impl URadioGroup {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self.previous_value = self.selected_value.clone();
        self
    }
    
    pub fn allow_deselect(mut self) -> Self {
        self.require_selection = false;
        self
    }
    
    /// تخطيط أفقي
    pub fn horizontal(mut self) -> Self {
        self.direction = UFlexDirection::Row;
        self
    }
    
    /// تخطيط عمودي
    pub fn vertical(mut self) -> Self {
        self.direction = UFlexDirection::Column;
        self
    }
    
    /// تخصيص المسافة بين الأزرار
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

/// علامات داخلية
#[derive(Component)]
struct RadioRing;

#[derive(Component)]
struct RadioDot;

// =========================================================
// Systems
// =========================================================

/// إنشاء الهيكل البصري للـ RadioButton
fn init_radio_visuals(
    mut commands: Commands,
    query: Query<(Entity, &URadioButton), Added<URadioButton>>,
) {
    for (entity, radio) in query.iter() {
        
        let ring_color = if radio.checked {
            radio.ring_checked_color
        } else {
            radio.ring_color
        };
        
        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(radio.size),
                height: UVal::Px(radio.size),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ));
        
        // إضافة Observer
        commands.entity(entity).observe(on_radio_press);
        
        commands.entity(entity).with_children(|parent| {
            
            // Ring (الحلقة الخارجية)
            parent.spawn((
                UNode {
                    width: UVal::Px(radio.size),
                    height: UVal::Px(radio.size),
                    background_color: Color::NONE,
                    border_radius: UCornerRadius::all(radio.size / 2.0),
                    ..default()
                },
                UBorder {
                    color: ring_color,
                    width: radio.ring_width,
                    radius: UCornerRadius::all(radio.size / 2.0),
                    offset: 0.0,
                },
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
                RadioRing,
            )).with_children(|ring_parent| {
                
                // Dot (النقطة الداخلية)
                let dot_size = if radio.checked {
                    radio.size * 0.5
                } else {
                    0.0
                };
                
                ring_parent.spawn((
                    UNode {
                        width: UVal::Px(dot_size),
                        height: UVal::Px(dot_size),
                        background_color: radio.dot_color,
                        border_radius: UCornerRadius::all(dot_size / 2.0),
                        ..default()
                    },
                    RadioDot,
                ));
            });
        });
    }
}

/// ربط الأزرار مع المجموعة وإعداد التخطيط
fn init_radio_group(
    mut commands: Commands,
    mut group_query: Query<(Entity, &mut URadioGroup, &Children), Added<URadioGroup>>,
    mut radio_query: Query<&mut URadioButton>,
) {
    for (group_entity, mut group, children) in group_query.iter_mut() {
        
        // إضافة UNode و ULayout للـ RadioGroup
        commands.entity(group_entity).insert((
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: group.direction,
                gap: group.gap,
                ..default()
            },
        ));
        
        // مسح القائمة القديمة
        group.buttons.clear();
        
        // جمع كل الأزرار في المجموعة
        for child in children.iter() {
            if let Ok(mut radio) = radio_query.get_mut(child) {
                // ربط الزر مع المجموعة
                radio.group = Some(group_entity);
                
                // إضافة الزر لقائمة المجموعة
                group.buttons.push(child);
                
                // تعيين الحالة الأولية
                if let Some(ref selected) = group.selected_value {
                    radio.checked = radio.value == *selected;
                    radio.current_scale = if radio.checked { 1.0 } else { 0.0 };
                }
            }
        }
    }
}

/// Observer للنقر على RadioButton
fn on_radio_press(
    trigger: On<Pointer<Press>>,
    mut radio_query: Query<&mut URadioButton>,
    mut group_query: Query<&mut URadioGroup>,
) {
    let radio_entity = trigger.entity.entity();
    
    let Ok(mut radio) = radio_query.get_mut(radio_entity) else { return };
    
    if radio.disabled {
        return;
    }
    
    // إذا كان في مجموعة
    if let Some(group_entity) = radio.group {
        if let Ok(mut group) = group_query.get_mut(group_entity) {
            
            // إذا كان مختاراً مسبقاً وتسمح بإلغاء الاختيار
            if radio.checked && !group.require_selection {
                radio.checked = false;
                radio.current_scale = 0.0;
                group.selected_value = None;
            } else {
                // اختيار هذا الزر
                radio.checked = true;
                group.selected_value = Some(radio.value.clone());
                
                // إلغاء اختيار الباقي باستخدام قائمة الأزرار المسجلة
                let buttons_to_deselect: Vec<Entity> = group.buttons.iter()
                    .filter(|&&e| e != radio_entity)
                    .copied()
                    .collect();
                
                for button_entity in buttons_to_deselect {
                    if let Ok(mut other_radio) = radio_query.get_mut(button_entity) {
                        other_radio.checked = false;
                        other_radio.current_scale = 0.0;
                    }
                }
            }
        }
    } else {
        // زر منفرد (بدون مجموعة)
        radio.checked = !radio.checked;
    }
}

/// تحديث المظهر
fn update_radio_visuals(
    radio_query: Query<(&URadioButton, &Children), Changed<URadioButton>>,
    ring_query: Query<&Children, With<RadioRing>>,
    mut border_query: Query<&mut UBorder, With<RadioRing>>,
    mut dot_query: Query<&mut UNode, With<RadioDot>>,
) {
    for (radio, children) in radio_query.iter() {
        
        // تحديث لون Ring
        for child in children.iter() {
            if let Ok(mut border) = border_query.get_mut(child) {
                border.color = if radio.checked {
                    radio.ring_checked_color
                } else {
                    radio.ring_color
                };
            }
            
            // تحديث Dot
            if let Ok(ring_children) = ring_query.get(child) {
                for dot_entity in ring_children.iter() {
                    if let Ok(mut dot_node) = dot_query.get_mut(dot_entity) {
                        let target_size = radio.size * 0.5 * radio.current_scale;
                        
                        dot_node.width = UVal::Px(target_size);
                        dot_node.height = UVal::Px(target_size);
                        dot_node.border_radius = UCornerRadius::all(target_size / 2.0);
                    }
                }
            }
        }
    }
}

/// حركة سلسة للنقطة
fn animate_radio_check(
    time: Res<Time>,
    mut query: Query<&mut URadioButton>,
) {
    for mut radio in query.iter_mut() {
        let target_scale = if radio.checked { 1.0 } else { 0.0 };
        let diff = target_scale - radio.current_scale;
        
        if diff.abs() < 0.01 {
            radio.current_scale = target_scale;
            continue;
        }
        
        let delta = time.delta_secs() * radio.animation_speed;
        radio.current_scale += diff * delta;
    }
}

/// إطلاق الأحداث
fn emit_radio_events(
    mut events: MessageWriter<RadioButtonChangedEvent>,
    mut radio_query: Query<(Entity, &mut URadioButton)>,
    group_query: Query<&URadioGroup>,
) {
    for (entity, mut radio) in radio_query.iter_mut() {
        if radio.checked != radio.previous_checked {
            
            let group_value = radio.group
                .and_then(|g| group_query.get(g).ok())
                .and_then(|g| g.selected_value.clone());
            
            events.write(RadioButtonChangedEvent {
                entity,
                value: radio.value.clone(),
                checked: radio.checked,
                group_entity: radio.group,
                group_value,
            });
            
            radio.previous_checked = radio.checked;
        }
    }
}

// =========================================================
// Event
// =========================================================

#[derive(Message)]
pub struct RadioButtonChangedEvent {
    pub entity: Entity,
    pub value: String,
    pub checked: bool,
    pub group_entity: Option<Entity>,
    pub group_value: Option<String>,
}

// =========================================================
// Helper Functions
// =========================================================

/// دالة مساعدة لإنشاء RadioButton مع Label
pub fn create_radio_with_label(
    parent: &mut ChildSpawnerCommands,
    radio: URadioButton,
    label: &str,
) -> Entity {
    parent.spawn((
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
            ..default()
        },
    )).with_children(|row| {
        row.spawn(radio);
        row.spawn(UTextLabel {
            text: label.to_string(),
            font_size: 16.0,
            color: Color::srgb(0.9, 0.9, 0.95),
            ..default()
        });
    }).id()
}