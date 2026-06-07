# البدء السريع

## 1) إضافة الحزمة

```toml
[dependencies]
univis_ui = "0.3.0"
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

- `URootUi::screen()` لمسار الواجهة الثابتة على الشاشة والعناصر المرتبطة بها.
- `URootUi::world_2d(size)` لواجهة في فضاء العالم ثنائية الأبعاد.
- `URootUi::world_3d(size)` لواجهة في فضاء العالم تستخدم مسار المواد ثلاثي الأبعاد.
- `URootUi::world_2d_fit_content()` و`URootUi::world_3d_fit_content()` لقياس حجم الجذر العالمي من المحتوى الداخلي.
- `UVal::Px` يعني وحدات UI منطقية، لا pixels فعلية للشاشة.
- `UiCanvasSize::Viewport` يتبع مساحة رؤية الكاميرا المحلولة.
- `UiCanvasSize::FitContent { min, max }` يقيس مساحة الرسم المنطقية من المحتوى ثم يطبّق الحدود عند الحاجة.
- الحجم الفيزيائي في فضاء العالم يُشتق بالعلاقة:
  `world_size = canvas_size * meters_per_unit`

## 4) ماذا يضيف `UnivisUiPlugin`؟

- التفاعل: `UnivisInteractionPlugin`
- المحرك: `UnivisEnginePlugin`
- النمط والخطوط والأيقونات: `UnivisUiStylePlugin`
- الوحدات الجاهزة: `UnivisWidgetPlugin`

## 5) ملاحظات مهمة مباشرة

- `UnivisWidgetPlugin` يضيف الآن أنظمة Runtime المدمجة الخاصة بـ `UTextField` و`UBadge` تلقائيًا.
- `UnivisScrollViewPlugin` مضاف تلقائيًا داخل `UnivisWidgetPlugin`.
- التفاعل يحسم الكاميرا من كل `URootUi`.
- في المشاهد متعددة الكاميرات يُفضّل ربط الجذر صراحة عبر `UiCameraRef::Entity`.
- `UScreenRoot` و`UWorldRoot` موجودان فقط كطبقات توافق مهجورة على مسارات صريحة مثل `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}`.
- اضبط `meters_per_unit` صراحة عندما تحتاج حجمًا فيزيائيًا محددًا لجذور العالم.
- إذا أردت قائمة إعداد موجزة ومسار أمثلة بحسب المهمة، فانتقل إلى [إعداد الإضافات وأولى الأمثلة](first-steps.md).

إذا كنت تركّب سطح widgets ضيقًا من دون `UnivisWidgetPlugin`، فما يزال بإمكانك إضافة `UnivisTextFieldPlugin` أو `UnivisBadgePlugin` يدويًا.

## 6) وضع الحزم المباشرة (متقدم)

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

## أمثلة مرتبطة

- [`responsive_layout_test`](examples/index.md#أمثلة-مساحة-العمل)
- [`widgets_controls`](examples/index.md#الوحدات-الجاهزة)
- [`widgets_inputs`](examples/index.md#الوحدات-الجاهزة)

## صفحات ترحيل مرتبطة

- [ترحيل الجذور إلى `URootUi`](migration/root-api.md)
- [ترحيل مسارات الأمثلة](migration/example-paths.md)

## نقاط الدخول الرسمية في `API`

- `univis_ui::UnivisUiPlugin`
- `univis_ui::prelude`
- `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}` لمسار التوافق القديم فقط
- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::univis_node::{UNode, ULayout}`

## إلى أين بعد ذلك؟

- المثال المرتبط: [`responsive_layout_test`](examples/index.md#أمثلة-مساحة-العمل)
- صفحة الإعداد: [إعداد الإضافات وأولى الأمثلة](first-steps.md)
- فهرس `API`: [مرجع الواجهة العامة](api/index.md)
- صفحة الترحيل: [ترحيل الجذور إلى `URootUi`](migration/root-api.md)
