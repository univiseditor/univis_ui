use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::prelude::*;

/// Marker for the Screen Root node (HUD).
///
/// Use this for UI that stays fixed to the camera/screen.
/// It typically takes its size automatically from the window dimensions.
#[derive(Component, Default)]
#[require(UNode)]
pub struct UScreenRoot; 

/// Marker for World Space UI Root.
///
/// Use this for UI elements that exist in the 3D world (e.g., floating over a character).
/// You must manually define the `size` (Canvas Size).
#[derive(Component)]
#[require(UNode)]
pub struct UWorldRoot {
    pub size: Vec2, 
    pub is_3d: bool,
    /// Scaling factor for text resolution/quality relative to the size.
    pub resolution_scale: f32, 
}

impl Default for UWorldRoot {
    fn default() -> Self {
        Self {
            size: Vec2::new(800.0, 600.0),
            is_3d: false,
            resolution_scale: 1.0,
        }
    }
}

pub fn auto_propagate_ui3d(
    mut commands: Commands,
    
    // 1. مراقبة الجذور (Roots) التي تغيرت إعداداتها
    root_query: Query<(Entity, &UWorldRoot), (Changed<UWorldRoot>, Without<UI3d>)>,
    
    // 2. مراقبة الأبناء (Children) الذين ليس لديهم UI3d بعد
    // نبحث عن أي UNode له أب، ولكن ينقصه مكون UI3d
    child_query: Query<(Entity, &ChildOf), (With<UNode>, Without<UI3d>)>,
    
    // 3. استعلام للتحقق مما إذا كان الأب يمتلك UI3d
    parent_check: Query<&UI3d>,
) {
    // أ) معالجة الجذر: هل طلب المستخدم وضع 3D؟
    for (entity, root) in root_query.iter() {
        if root.is_3d {
            commands.entity(entity).insert(UI3d);
        }
    }

    // ب) التوريث المتسلسل:
    // نمر على كل الأبناء الذين ليس لديهم UI3d
    for (child_entity, parent) in child_query.iter() {
        // إذا كان "الأب" يمتلك العلامة UI3d
        if parent_check.get(parent.get()).is_ok() {
            // إذن "الابن" يجب أن يصبح 3D أيضاً
            commands.entity(child_entity).insert(UI3d);
        }
    }
}