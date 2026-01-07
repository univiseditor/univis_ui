use bevy::prelude::*;
use crate::prelude::*;

pub mod button;
pub mod text_label;
pub mod badge;
pub mod progress;
pub mod image;
pub mod seekbar;
pub mod checkbox;
pub mod menu;
pub mod scrolling;
pub mod icon_btn;
pub mod toggle;
pub mod radio;
// pub mod radio2;

pub mod prelude {
    pub use crate::widget::{
        text_label::*,
        badge::*,
        progress::*,
        button::*,
        image::*,
        // radio2::*,
        checkbox::*,
        radio::*,
        // menu::*,
        // scrolling::*,
        seekbar::*,
        icon_btn::*,
        toggle::*,
    };
}

pub struct UnivisWidgetPlugin;

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
         .register_type::<UImage>()
         .add_systems(Update, 
            sync_image_geometry
               .before(update_materials_optimized) 
               .before(upward_measure_pass_cached))
         .add_plugins(UnivisTextPlugin)
         .add_plugins(UnivisProgressPlugin)
         .add_plugins(UnivisButtonPlugin)
         .add_plugins(UnivisRadioPlugin)
         .add_plugins(UnivisIconButtonPlugin)
         .add_plugins(UnivisTogglePlugin)
         .add_plugins(UnivisCheckboxPlugin)
         .add_plugins(UnivisSeekBarPlugin)
         .add_plugins(UnivisBadgePlugin);
    }
}