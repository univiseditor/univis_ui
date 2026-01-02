use bevy::prelude::*;
use crate::prelude::*;

/// Component defining interaction colors.
///
/// Attaching this component to a `UNode` enables automatic hover/press color changes.
#[derive(Component, Clone, Reflect)]
pub struct UInteractionColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Event Handler: Pointer Over (Hover Enter)
pub fn on_pointer_over(
    trigger: On<Pointer<Over>>,
    mut query: Query<(&UInteractionColors, &mut UNode) , With<UInteractionColors>>,
) {
    let entity = trigger.entity.entity(); 
    
    if let Ok((colors_opt, mut node)) = query.get_mut(entity) {
        node.background_color = colors_opt.hovered;
    }
}

/// Event Handler: Pointer Out (Hover Exit)
pub fn on_pointer_out(
    trigger: On<Pointer<Out>>,
    mut query: Query<(&UInteractionColors, &mut UNode), With<UInteractionColors>>,
) {
    let entity = trigger.entity.entity(); 
    
    if let Ok(( colors_opt, mut node)) = query.get_mut(entity) {
        node.background_color = colors_opt.normal;
        
    }
}

/// Event Handler: Pointer Press (Click Down)
pub fn on_pointer_press(
    trigger: On<Pointer<Press>>,
    mut query: Query<(&UInteractionColors, &mut UNode), With<UInteractionColors>>,
) {
    let entity = trigger.entity.entity(); 
    
    if let Ok(( colors_opt, mut node)) = query.get_mut(entity) {
        node.background_color = colors_opt.pressed;
        
    }
}

/// Event Handler: Pointer Release (Click Up)
pub fn on_pointer_release(
    trigger: On<Pointer<Release>>,
    mut query: Query<(&UInteractionColors, &mut UNode), With<UInteractionColors>>,
) {
    let entity = trigger.entity.entity(); 
    
    if let Ok(( colors_opt, mut node)) = query.get_mut(entity) {
        node.background_color = colors_opt.hovered;
        
    }
}