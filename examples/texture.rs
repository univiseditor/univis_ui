use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,UnivisUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        UScreenRoot,
        ULayout {
            gap: 10.0,
            ..default()
        }
    )).with_children(|root| {
        root.spawn((
            UImage {
                texture: assets.load("background.png"),
                width: UVal::Px(600.),
                height: UVal::Px(400.),
                ..default()
            },
        ));
        root.spawn((
            UImage {
                texture: assets.load("background.png"),
                width: UVal::Px(600.),
                height: UVal::Px(400.),
                radius: Some(UCornerRadius::all(50.0)),
                ..default()
            },
        ));
    });
}