# إعداد الإضافات وأولى الأمثلة

هذه الصفحة هي أقصر مسار موجّه بحسب المهمة بعد [البدء السريع](quick-start.md).

استخدمها عندما تريد صفحة واحدة تجيب عن سؤالين:

- ما هي الإضافات التي أحصل عليها فعليًا بشكل افتراضي؟
- ما هو أول مثال يجب أن أشغّله بحسب المهمة التي تهمني؟

## إعداد الواجهة المجمعة

إذا أضفت `UnivisUiPlugin` فأنت تحصل مسبقًا على:

- `UnivisUiStylePlugin`
- `UnivisEnginePlugin`
- `UnivisInteractionPlugin`
- `UnivisWidgetPlugin`

وهذا هو المسار الموصى به لمعظم التطبيقات.

## تغطية Runtime للوحدات الجاهزة

يتضمن `UnivisUiPlugin` إضافة `UnivisWidgetPlugin`، وهذا السطح الافتراضي يضم الآن أيضًا:

- `UnivisTextFieldPlugin` لسلوك وأحداث `UTextField`
- `UnivisBadgePlugin` للتحديثات الديناميكية لـ `UBadge` / `UTag`

إذا ركّبت الإضافات يدويًا حول `UnivisWidgetPlugin`، فلست بحاجة إلى إضافات runtime إضافية:

```rust,no_run
use bevy::prelude::*;
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

واستخدم الإضافات المخصصة مباشرة فقط عندما تريد سطح widgets أضيق من `UnivisWidgetPlugin`.

## إعداد الكاميرا

- التفاعل وتغيير حجم اللوحات يحسمان الكاميرا من كل `URootUi`
- تكفي `Camera2d` بسيطة لأصغر أمثلة الواجهة الثابتة على الشاشة
- في المشاهد متعددة الكاميرات يُفضّل `UiCameraRef::Entity(camera_entity)`

## مسارات البدء الأفضل

### واجهة شاشة أساسية

شغّل هذه الأمثلة بالترتيب:

1. `cargo run -p univis_ui --example hello_world`
2. `cargo run -p univis_ui --example root_screen_hud`

ما الذي يجب التركيز عليه:

- أصغر مسار تشغيل على مستوى الواجهة المجمعة
- ثبات واجهة HUD على الشاشة أثناء حركة الكاميرا

### لوحة داخل العالم

شغّل هذه الأمثلة بالترتيب:

1. `cargo run -p univis_ui --example root_world_scale`
2. `cargo run -p univis_ui --example root_fit_content`

ما الذي يجب التركيز عليه:

- الفرق بين مساحة الرسم المنطقية والحجم الفيزيائي في العالم
- اللوحات العالمية التي يتحدد حجمها من المحتوى والجذور الشبيهة بلوحات الأدوات

### إدخال نصي

ابدأ بهذا المثال:

1. `cargo run -p univis_ui_widgets --example text_field`

ما الذي يجب التركيز عليه:

- الإدخال القابل للتحرير
- سلوك التغيير والإرسال
- تغطية `UTextField` الافتراضية عبر `UnivisWidgetPlugin`

### عناصر الاختيار

شغّل هذه الأمثلة بالترتيب:

1. `cargo run -p univis_ui_widgets --example select`
2. `cargo run -p univis_ui_widgets --example widgets`

ما الذي يجب التركيز عليه:

- التنقل بين الخيارات وتجاوز الخيارات المعطلة
- كيف ينسجم `USelect` مع بقية سطح الوحدات الجاهزة

## صفحات مرتبطة

- [البدء السريع](quick-start.md)
- [جدول حقيقة الإضافات](architecture/plugin-truth-table.md)
- [معرض الأمثلة](examples/gallery.md)
- [القيود الحالية](development/current-limitations.md)
