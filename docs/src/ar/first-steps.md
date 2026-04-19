# إعداد الإضافات وأولى المسارات

هذه الصفحة هي أقصر مسار موجّه بحسب المهمة بعد [البدء السريع](quick-start.md).

استخدمها عندما تريد صفحة واحدة تجيب عن سؤالين:

- ما هي الإضافات التي أحصل عليها فعليًا بشكل افتراضي؟
- ما الذي يجب أن أفتحه أو أشغّله أولًا بحسب المهمة التي تهمني؟

## ملاحظة تخص هذا الفرع

أغلب ملفات الأمثلة القديمة في مساحة العمل أصبحت مؤرشفة داخل هذا الفرع. العرض الحي الحالي هو
الحزمة المستقلة ذات الطابع Android داخل `android/android_phone_app`، بينما أصبح فهرس الأمثلة
خريطة للملكية والحالة التاريخية أيضًا.

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
- تكفي `Camera2d` بسيطة لأصغر مشاهد الواجهة الثابتة على الشاشة
- في المشاهد متعددة الكاميرات يُفضّل `UiCameraRef::Entity(camera_entity)`

## مسارات البدء الأفضل

### واجهة شاشة أساسية

ابدأ من:

- [البدء السريع](quick-start.md)
- [`hello_world`](examples/index.md#hello_world)
- [`root_screen_hud`](examples/index.md#root_screen_hud)

ما الذي يجب التركيز عليه:

- أصغر مسار تشغيل على مستوى الواجهة المجمعة
- ثبات واجهة HUD على الشاشة أثناء حركة الكاميرا

### شاشة تطبيق بطابع Android

شغّل هذا العرض:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

ما الذي يجب التركيز عليه:

- تركيب سطح تطبيق نظيف بطابع Android داخل `URootUi::screen()`
- دمج `UTextField` و`UToggle` و`USeekBar` و`UButton` داخل شاشة واحدة ضيقة
- التأكد من أن كثافة العناصر ما تزال مقروءة من دون رسم جسم الهاتف نفسه

### لوحة داخل العالم

ابدأ من:

- [الجذور والمساحات](layout/roots.md)
- [`root_world_scale`](examples/index.md#root_world_scale)
- [`root_fit_content`](examples/index.md#root_fit_content)

ما الذي يجب التركيز عليه:

- الفرق بين مساحة الرسم المنطقية والحجم الفيزيائي في العالم
- اللوحات العالمية التي يتحدد حجمها من المحتوى والجذور الشبيهة بلوحات الأدوات

### إدخال نصي

ابدأ من:

- [مدخلات الوحدات الجاهزة](widgets/inputs.md)
- [`text_field`](examples/index.md#text_field)

ما الذي يجب التركيز عليه:

- الإدخال القابل للتحرير
- سلوك التغيير والإرسال
- تغطية `UTextField` الافتراضية عبر `UnivisWidgetPlugin`

### عناصر الاختيار

ابدأ من:

- [نظرة عامة على الوحدات الجاهزة](widgets/overview.md)
- [`widgets`](examples/index.md#widgets)

ما الذي يجب التركيز عليه:

- كيف تنسجم عناصر الاختيار مع بقية سطح الوحدات الجاهزة
- كيف تعود أسماء الأمثلة المؤرشفة إلى واجهات الـ API الحالية

## صفحات مرتبطة

- [البدء السريع](quick-start.md)
- [جدول حقيقة الإضافات](architecture/plugin-truth-table.md)
- [معرض الأمثلة](examples/gallery.md)
- [شاشة تطبيق Android](examples/android-phone.md)
- [القيود الحالية](development/current-limitations.md)
