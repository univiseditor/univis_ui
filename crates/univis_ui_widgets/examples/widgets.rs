//! Broad sampler for the built-in widget surface.
//!
//! Related docs:
//! - `docs/src/en/widgets/overview.md`
//! - `docs/src/ar/widgets/overview.md`
//! - `docs/src/en/examples/index.md#widgets`

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
#[allow(unused_imports)]
use univis_ui_interaction::prelude::*;
#[allow(unused_imports)]
use univis_ui_style::prelude::*;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UnivisUiStylePlugin,
            UnivisEnginePlugin,
            UnivisInteractionPlugin,
            UnivisWidgetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(URootUi::screen()).with_children(|root| {
        // root.spawn((
        //     UIconButton {
        //         width: UVal::Px(40.0),
        //         height: UVal::Px(40.0),
        //         icon: Icon::CODESANDBOX,
        //         icon_size: 30.0,
        //         ..Default::default()
        //     },
        // ));

        root.spawn((
            UCheckbox {
                // label: Some("Hello".to_string()),
                checked: true,
                ..default()
            },
            // إضافة تفاعل للألوان (اختياري)
            // UInteractionColors {
            //     normal: Color::BLACK,
            //     hovered: Color::srgb(0.1, 0.1, 0.1),
            //     // pressed: Color::srgb(0.2, 0.2, 0.2),

            // },
            // هام: لتفعيل التقاط النقر
            // Pickable::default(),
        ));
        root.spawn(UToggle::material_style());
        // إضافة مراقب للنقر (Observer Pattern - Bevy 0.15+)
        // .observe(|trigger: On<Pointer<Press>>, mut query: Query<&mut UCheckbox>| {
        //     if let Ok(mut checkbox) = query.get_mut(trigger.entity.entity()) {
        //         checkbox.is_checked = !checkbox.is_checked;
        //         info!("checked ? {}",checkbox.is_checked);
        //     }
        // });

        // 2. النص المجاور
        // root.spawn((
        //     UNode::default(),
        //     UTextLabel {
        //         text: "ACTIVATE SHIELDS".to_string(),
        //         font_size: 24.0,
        //         color: Color::srgb(0.8, 0.9, 1.0),
        //         ..default()
        //     },
        // ));
    });
}
