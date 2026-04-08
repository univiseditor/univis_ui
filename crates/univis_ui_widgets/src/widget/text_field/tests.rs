use super::*;

use self::runtime::{TextFieldPressedThisFrame, handle_global_unfocus};

#[test]
fn handle_global_unfocus_blurs_only_non_pressed_textfields() {
    let mut app = App::new();
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.add_systems(Update, handle_global_unfocus);

    let keep_focused = app
        .world_mut()
        .spawn((
            UTextField {
                focused: true,
                ..default()
            },
            TextFieldPressedThisFrame,
        ))
        .id();
    let should_blur = app
        .world_mut()
        .spawn(UTextField {
            focused: true,
            ..default()
        })
        .id();

    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .press(MouseButton::Left);

    app.update();

    assert!(app.world().get::<UTextField>(keep_focused).unwrap().focused);
    assert!(!app.world().get::<UTextField>(should_blur).unwrap().focused);
    assert!(
        app.world()
            .get::<TextFieldPressedThisFrame>(keep_focused)
            .is_none()
    );
}

#[test]
fn handle_global_unfocus_does_nothing_without_new_left_click() {
    let mut app = App::new();
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.add_systems(Update, handle_global_unfocus);

    let field = app
        .world_mut()
        .spawn(UTextField {
            focused: true,
            ..default()
        })
        .id();

    app.update();

    assert!(app.world().get::<UTextField>(field).unwrap().focused);
}
