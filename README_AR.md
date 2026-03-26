# Univis UI
[![Crates.io](https://img.shields.io/crates/v/univis_ui)](https://crates.io/crates/univis_ui)
[![Bevy](https://img.shields.io/badge/Bevy-0.18.1-blue)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](LICENSE)

ابنِ واجهات حادة وقابلة للتوسع في Bevy للشاشة، ولوحات العالم، والواجهات ثلاثية الأبعاد المضاءة، عبر طبقة ECS واحدة.

ملخص الترحيل: [MIGRATION_AR.md](MIGRATION_AR.md)

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
univis_ui = "0.2.0-alpha.1"
```

إذا أردت تحكمًا مباشرًا في الطبقات الداخلية:

```toml
[dependencies]
univis_ui_engine = "0.2.0-alpha.1"
univis_ui_style = "0.2.0-alpha.1"
univis_ui_interaction = "0.2.0-alpha.1"
univis_ui_widgets = "0.2.0-alpha.1"
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
- [البدء السريع (AR)](docs/src/ar/quick-start.md)
- [Quick Start (EN)](docs/src/en/quick-start.md)
- [الجذور والمساحات (AR)](docs/src/ar/layout/roots.md)
- [Roots and Spaces (EN)](docs/src/en/layout/roots.md)
- [فهرس الأمثلة (AR)](docs/src/ar/examples/index.md)
- [Examples Index (EN)](docs/src/en/examples/index.md)
- [الترحيل والقيود (AR)](docs/src/ar/migration/index.md)
- [Migration and Limitations (EN)](docs/src/en/migration/index.md)

## مسار الاكتشاف من GitHub

1. اقرأ `README.md` أو `README_AR.md` لفهم قصة المشروع وخريطة الحزم.
2. افتح [صفحة الوثائق الرئيسية](docs/src/index.md) ثم انتقل إلى فصل الشرح المناسب.
3. استخدم [فهرس الأمثلة (AR)](docs/src/ar/examples/index.md) أو [Examples Index (EN)](docs/src/en/examples/index.md).
4. ولّد `cargo doc --no-deps -p univis_ui` عندما تحتاج المسارات الدقيقة والتواقيع.

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

سطر `alpha2` يتطور حاليًا حول `URootUi`، والقياس داخل العالم، وroot capsules، والوثائق الموحدة، وتنظيف الـ API.

إذا كنت تريد طبقة UI واحدة في Bevy تستطيع التعامل مع HUD، ولوحات العالم، والواجهات ثلاثية الأبعاد المضاءة من دون تقسيم النموذج الذهني بين أنظمة متفرقة، فهذا هو الاتجاه الذي تحاول Univis UI أن تقدمه.
