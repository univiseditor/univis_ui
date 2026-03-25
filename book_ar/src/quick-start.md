# البدء السريع

## 1) إضافة الحزمة

```toml
[dependencies]
univis_ui = "0.2.0-alpha.1"
```

## 2) تطبيق بسيط

```rust,no_run
use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.1, 0.14),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel::new("Hello Univis UI"));
        });
}
```

## 3) اختيارات الجذر

- `URootUi::screen()` لمسار HUD الحقيقي والعناصر الثابتة على الشاشة.
- `URootUi::world_2d(size)` لواجهة world-space المسطحة.
- `URootUi::world_3d(size)` لواجهة world-space التي تستخدم مسار المواد ثلاثي الأبعاد.
- `UVal::Px` يعني وحدات UI منطقية، لا pixels فعلية للشاشة.
- `UiCanvasSize::Viewport` يتبع viewport الكاميرا المحلولة.
- الحجم الفيزيائي في world-space يُشتق بالعلاقة:
  `world_size = canvas_size * meters_per_unit`

## 4) ماذا يضيف `UnivisUiPlugin`؟

- Interaction: `UnivisInteractionPlugin`
- المحرك: `UnivisEnginePlugin`
- Style/fonts/icons: `UnivisUiStylePlugin`
- Widgets: `UnivisWidgetPlugin`

## 5) ملاحظات مهمة مباشرة

- `UnivisWidgetPlugin` لا يضيف `UnivisTextFieldPlugin` تلقائيًا؛ أضفه يدويًا عند استخدام `UTextField`.
- `UnivisWidgetPlugin` لا يضيف `UnivisBadgePlugin` تلقائيًا.
- `UnivisScrollViewPlugin` مضاف تلقائيًا داخل `UnivisWidgetPlugin`.
- التفاعل يحسم الكاميرا من كل `URootUi`.
- في المشاهد متعددة الكاميرات يُفضّل ربط الجذر صراحة عبر `UiCameraRef::Entity`.
- `UScreenRoot` و`UWorldRoot` موجودان فقط كطبقات توافق deprecated خلال `alpha2`.
- إذا احتجت نفس الحجم الفيزيائي التاريخي لأمثلة world-space القديمة، فاضبط `meters_per_unit: 1.0` صراحة.

إذا أردت تشغيل ميزات إضافية اختيارية عند تركيبك الجزئي للـ plugins، أضفها يدويًا.

## 6) وضع Direct Crates (متقدم)

```rust,no_run
use bevy::prelude::*;
use univis_ui_engine::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiStylePlugin)
        .add_plugins(UnivisEnginePlugin)
        .add_plugins(UnivisInteractionPlugin)
        .add_plugins(UnivisWidgetPlugin)
        .run();
}
```
