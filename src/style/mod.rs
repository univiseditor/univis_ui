use bevy::{asset::embedded_asset, prelude::*};

pub mod icons;

pub mod prelude {
    pub use crate::style::icons::*;
    pub use crate::style::Theme;
}

pub struct UnivisUiStylePlugin;

impl Plugin for UnivisUiStylePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/fonts/Inter-Regular.ttf");
        embedded_asset!(app, "assets/fonts/AdwaitaSans-Regular.ttf");
        embedded_asset!(app, "assets/fonts/FiraSans-Regular.ttf");
        embedded_asset!(app, "assets/icons/Lucide.ttf");
        app.init_resource::<Theme>();
    }
}

#[derive(Resource)]
pub struct Theme {
    pub text: TextStyles,
    pub icon: IconStyles,
}

pub struct TextStyles {
    pub font: Fonts
}

pub struct IconStyles {
    pub font: Handle<Font>
}

pub struct Fonts {
    pub inter_regular: Handle<Font>,
    pub adwaita_sans_regular: Handle<Font>,
    pub fira_sans_regular: Handle<Font>
}

impl FromWorld for Theme {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Theme {
            text: TextStyles {
                font: Fonts {
                    inter_regular: asset_server.load(
                "embedded://univis_ui/style/assets/fonts/Inter-Regular.ttf"),
                    adwaita_sans_regular: asset_server.load(
                        "embedded://univis_ui/style/assets/fonts/AdwaitaSans-Regular.ttf"
                    ),
                    fira_sans_regular: asset_server.load(
                        "embedded://univis_ui/style/assets/fonts/FiraSans-Regular.ttf"
                    )
                }
            },
            icon: IconStyles {
                font: asset_server.load(
                "embedded://univis_ui/style/assets/icons/Lucide.ttf")
            },           
        }
    }
}