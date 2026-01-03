use bevy::prelude::*;
use crate::prelude::*;

pub mod button;
pub mod text_label;
pub mod badge;
pub mod progress;
pub mod image;
pub mod text_3d_label;

pub mod prelude {
    pub use crate::widget::{
        text_label::*,
        badge::*,
        progress::*,
        button::*,
        image::*,
        // text_3d_label::*,
    };
}

pub struct UnivisWidgetPlugin;

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
         .register_type::<UImage>()
         .add_systems(Update, 
            sync_image_geometry
               .before(update_shader_visuals) // قبل الرسم
               .before(upward_measure_pass)) // قبل التخطيط
         .add_plugins(UnivisTextPlugin)
         .add_plugins(UnivisProgressPlugin)
         .add_plugins(UnivisButtonPlugin)
        //  .add_plugins(UnivisText3dPlugin)
         .add_plugins(UnivisBadgePlugin);
    }
}