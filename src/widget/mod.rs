use bevy::app::Plugin;
use crate::prelude::*;

pub mod button;
pub mod text_label;
pub mod badge;
pub mod progress;


pub mod prelude {
    pub use crate::widget::{
        text_label::*,
        badge::*,
        progress::*,
        button::*,
    };
}

pub struct UnivisWidgetPlugin;

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app 
         .add_plugins(UnivisTextPlugin)
         .add_plugins(UnivisProgressPlugin)
         .add_plugins(UnivisButtonPlugin)
         .add_plugins(UnivisBadgePlugin);
    }
}