use bevy::prelude::*;
use univis_ui::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, masonry_gallery_test)
        .run();
}
fn masonry_gallery_test(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::BLACK,
            ..default()
        },
        ULayout {
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Start, 
            ..default()
        },
        UScreenRoot,
    ))
    .with_children(|root| {

        // =========================================================
        // حاوية Masonry (The Gallery)
        // =========================================================
        root.spawn((
            UNode {
                width: UVal::Px(600.0), 
                height: UVal::Px(450.),     
                background_color: Color::srgb(0.1, 0.1, 0.1),
                padding: USides::all(20.0),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                justify_content: UJustifyContent::Center,
                display: UDisplay::Masonry, 
                grid_columns: 3, 
                gap: 15.0,       
                ..default()
            }
        )).with_children(|gallery| {

            spawn_card(gallery, 200.0, Color::srgb(0.8, 0.2, 0.2)); 
            
            spawn_card(gallery, 100.0, Color::srgb(0.2, 0.8, 0.2)); 
            
            spawn_card(gallery, 150.0, Color::srgb(0.2, 0.2, 0.8)); 

            spawn_card(gallery, 120.0, Color::srgb(0.8, 0.8, 0.2)); 

            spawn_card(gallery, 80.0, Color::srgb(0.2, 0.8, 0.8));

            spawn_card(gallery, 180.0, Color::srgb(0.8, 0.2, 0.8));
            
            spawn_card(gallery, 100.0, Color::WHITE); 
        });
    });
}

fn spawn_card(parent: &mut ChildSpawnerCommands, height: f32, color: Color) {
    parent.spawn(UNode {
        width: UVal::Auto, 
        height: UVal::Px(height),
        background_color: color,
        border_radius: UCornerRadius::all(8.0),
        
        margin: USides::bottom(5.0), 
        ..default()
    });
}