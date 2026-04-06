# Univis UI
[![Crates.io](https://img.shields.io/crates/v/univis_ui)](https://crates.io/crates/univis_ui)
[![Bevy](https://img.shields.io/badge/Bevy-0.18.1-blue)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](LICENSE)

ابنِ واجهات حادة وقابلة للتوسع في Bevy للشاشة، ولوحات العالم، والواجهات ثلاثية الأبعاد المضاءة، عبر طبقة ECS واحدة.

ملخص الترحيل: [MIGRATION_AR.md](MIGRATION_AR.md)
رابط الوثائق العامة: `https://univiseditor.github.io/univis_ui/`

> مهم:
> ما تزال `univis_ui` في مرحلة **alpha**، لذلك يمكن أن تتغير الـ API والسلوك بين الإصدارات.

من المثال `cargo run --release -p univis_ui --features example_bloom --example card_profile`:

![profile](profile.png)

## لماذا Univis UI؟

تم بناء Univis UI للفرق التي تحتاج أكثر من مجرد طبقة HUD بسيطة.

- طبقة UI واحدة لمسارات `screen` و`world_2d` و`world_3d`
- تركيب طبيعي مع ECS: الواجهة عبارة عن كيانات ومكوّنات، لا شجرة منفصلة محتفظ بها
- محرك تخطيط مخصص يدعم `Flex` و`Grid` و`Masonry` و`Stack` و`Radial`
- رندر مبني على SDF لإخراج حواف ناعمة وأشكال واضحة تحت التكبير
- التقاط وتفاعل ووحدات جاهزة مدمجة
- جذور عالمية تعتمد على المحتوى لعناصر مثل node panels وinspectors والبطاقات الداخلية
- سطح Plugins مرن عندما تريد المحرك من دون الواجهة المجمعة الكاملة

## ماذا تساعدك على بناء؟

- واجهات HUD وعناصر overlay داخل اللعبة
- واجهات داخل العالم
- لوحات تحكم ثلاثية الأبعاد وشاشات خيال علمي
- أدوات داخل اللعبة ومحررات وinspectors
- واجهات node graph مع جذور متداخلة لكن معزولة

## التثبيت

```toml
[dependencies]
univis_ui = "0.2.0-alpha.2"
```

إذا أردت تحكمًا مباشرًا في الطبقات الداخلية:

```toml
[dependencies]
univis_ui_engine = "0.2.0-alpha.2"
univis_ui_style = "0.2.0-alpha.2"
univis_ui_interaction = "0.2.0-alpha.2"
univis_ui_widgets = "0.2.0-alpha.2"
```

## بداية سريعة

```rust
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
                background_color: Color::srgb(0.08, 0.10, 0.14),
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
            root.spawn(UTextLabel {
                text: "Hello Univis UI".into(),
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            });
        });
}
```

## السطح الحالي في مرحلة alpha

### ما يكفي للبناء عليه

- `URootUi` هو واجهة الجذور العامة الرسمية
- `URootUi::screen()` و`URootUi::world_2d(...)` و`URootUi::world_3d(...)` هي نقاط الدخول المقصودة
- `UiSpace` و`UiCameraRef` و`UiCanvasSize` بما فيه `UiCanvasSize::FitContent { min, max }` جزء من قصة الجذور العامة الحالية
- يفترض أن تستخدم الأمثلة مسارات تتبع الـ crate المالكة وأوامر مرتبطة بالحزمة

### ما هو مهجور لكنه ما يزال مدعومًا

- `UScreenRoot` و`UWorldRoot` متاحان فقط كطبقات توافق مهجورة
- تبقى قيمة `meters_per_unit: 1.0` مخرج توافق قديم عندما تحتاج الحجم الفيزيائي التاريخي نفسه لبعض أمثلة العالم الأقدم

### ما يزال في طور الاستقرار

- جذور `fit-content` العالمية عند وجود اعتماد قوي على `%` أو على flex مرتبط بالجذر نفسه
- بعض الضبط البصري المصقول لأمثلة `World3d`
- ما يزال التحقق البصري للتغييرات الرسومية الثقيلة يعتمد جزئيًا على المراجعة اليدوية

## نقاط القوة الأساسية

### جذور UI تطابق الاستخدام الحقيقي

- `URootUi::screen()` لواجهة HUD ثابتة على الـ viewport
- `URootUi::world_2d(size)` لواجهة عالمية مسطحة
- `URootUi::world_3d(size)` لواجهة ثلاثية الأبعاد مضاءة
- `world_2d_fit_content()` و`world_3d_fit_content()` لجذور عالمية يتحدد حجمها من المحتوى

### تخطيط يتجاوز الصفوف التقليدية

- flex وgrid للواجهات المعتادة
- masonry وradial للتوزيعات الأكثر تعبيرًا
- امتدادات متقدمة على مستوى الحاوية والعنصر للمحاذاة والمسارات
- root capsules تمنع شجرة UI من التسرب فوق شجرة أخرى

### رندر يحافظ على الجودة تحت التكبير

- مواد SDF للحواف الناعمة والحدود والزوايا
- مسارا 2D و3D من نفس نموذج الواجهة
- دعم للقص وخصائص PBR عند الحاجة
- تحكم بالحجم الفيزيائي داخل العالم عبر `meters_per_unit`

### التفاعل والوحدات الجاهزة موجودان أصلًا

- pointer picking يُحل من كل root
- `UInteraction` و`UInteractionColors`
- وحدات جاهزة للنص والصور والأزرار وcheckbox وtoggle وradio وseekbar وselect وpanel وscroll وغيرها

## الوثائق والأمثلة

الوثائق الكاملة موجودة في `docs/` على شكل mdBook موحد يحتوي الشجرتين:

- العربية: `docs/src/ar/`
- الإنجليزية: `docs/src/en/`
- صفحة الدخول: `docs/src/index.md`
- رابط الوثائق العامة: `https://univiseditor.github.io/univis_ui/`

ابنِ الوثائق:

```bash
mdbook build docs
```

شغّلها محليًا:

```bash
mdbook serve docs -n 127.0.0.1 -p 3000
```

نقاط بداية مفيدة:

- [صفحة الوثائق الرئيسية](docs/src/index.md)
- [ملخص الترحيل](MIGRATION_AR.md)
- [جرد الدين التقني الحالي](TECH_DEBT_INVENTORY.md)
- [البدء السريع (AR)](docs/src/ar/quick-start.md)
- [Quick Start (EN)](docs/src/en/quick-start.md)
- [معرض الأمثلة (AR)](docs/src/ar/examples/gallery.md)
- [Example Gallery (EN)](docs/src/en/examples/gallery.md)
- [الجذور والمساحات (AR)](docs/src/ar/layout/roots.md)
- [Roots and Spaces (EN)](docs/src/en/layout/roots.md)
- [فهرس الأمثلة (AR)](docs/src/ar/examples/index.md)
- [Examples Index (EN)](docs/src/en/examples/index.md)
- [الترحيل والقيود (AR)](docs/src/ar/migration/index.md)
- [Migration and Limitations (EN)](docs/src/en/migration/index.md)

## مسار الاكتشاف من GitHub

1. اقرأ `README.md` أو `README_AR.md` لفهم قصة المشروع وخريطة الحزم.
2. راجع [RELEASE_NOTES.md](RELEASE_NOTES.md) لفهم قصة الإصدار الحالية و[changelog.md](changelog.md) لمراجعة السجل الزمني للتنفيذ.
3. افتح [صفحة الوثائق الرئيسية](docs/src/index.md) أو رابط الوثائق العامة ثم انتقل إلى فصل الشرح المناسب.
4. استخدم [معرض الأمثلة (AR)](docs/src/ar/examples/gallery.md) أو [Example Gallery (EN)](docs/src/en/examples/gallery.md) قبل الرجوع إلى الفهرس الكامل.
5. ولّد `cargo doc --no-deps -p univis_ui` عندما تحتاج المسارات الدقيقة والتواقيع.

## دليل ملفات الجذر

- `README.md`: قصة المشروع وأسرع نقطة دخول.
- `README_AR.md`: صفحة الدخول العربية على المستوى العالي نفسه.
- `MIGRATION.md`: مسار الترحيل من الوثائق الأقدم ومسارات الأمثلة القديمة وافتراضات الجذور القديمة.
- `MIGRATION_AR.md`: مسار الترحيل العربي.
- `RELEASE_NOTES.md`: الملخص الحالي على مستوى الإصدار alpha.
- `TECH_DEBT_INVENTORY.md`: قائمة الصيانة الحالية الخاصة بالملفات الكبيرة وإعادة تقسيمها.
- `changelog.md`: السجل الزمني المرتب حسب التواريخ للتغييرات الملحوظة.

## `API Docs`

استخدم صفحات الشرح للمفاهيم، وخطوات الترحيل، والتعلم عبر الأمثلة. واستخدم الوثائق المولدة آليًا عندما تحتاج المسارات الدقيقة، والحقول، والتواقيع.

ولّدهـا بالأمر:

```bash
cargo doc --no-deps -p univis_ui
```

نقاط الدخول الأهم:

- `univis_ui::UnivisUiPlugin`
- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::univis_node::UNode`
- `univis_ui_interaction::interaction::feedback::UInteraction`
- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_style::style::Theme`

أمثلة جيدة للبداية:

- `hello_world`
- `root_screen_hud`
- `root_world_scale`
- `root_fit_content`
- `root_capsule_overlap`
- `border_light_3d`
- `card_profile`
- `sci_fi`

شغّل مثالًا:

```bash
cargo run -p univis_ui_engine --example root_fit_content
```

## الحزم

- `univis_ui`: نقطة الدخول المجمعة
- `univis_ui_engine`: الجذور، التخطيط، الرندر، ونموذج العقدة الأساسي
- `univis_ui_interaction`: الالتقاط والتفاعل
- `univis_ui_style`: الخطوط، الأيقونات، والموارد البصرية المشتركة
- `univis_ui_widgets`: الوحدات الجاهزة والـ widget plugins

## الحالة الحالية

تتركز المرحلة alpha الحالية حول `URootUi`، والقياس داخل العالم، وroot capsules، والوثائق الموحدة، وتنظيف الـ API، وخارطة الطريق النشطة بعد المراجعة.

إذا كنت تريد طبقة UI واحدة في Bevy تستطيع التعامل مع HUD، ولوحات العالم، والواجهات ثلاثية الأبعاد المضاءة من دون تقسيم النموذج الذهني بين أنظمة متفرقة، فهذا هو الاتجاه الذي تحاول Univis UI أن تقدمه.
