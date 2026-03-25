# خريطة الطريق - إعادة هيكلة URootUi

## الهدف

استبدال نموذج الجذور الحالي (`UScreenRoot` / `UWorldRoot`) بواجهة عامة واحدة،
وجعل واجهة الشاشة HUD حقيقيًا، وفصل وحدات تخطيط الواجهة عن أبعاد العالم.

## لماذا نحتاج هذا التغيير

التصميم الحالي يحتوي على مشكلات هيكلية واضحة:

- `UScreenRoot` يوصف كأنه HUD ثابت على الشاشة، لكنه حاليًا يتصرف كلوحة داخل العالم بحجم النافذة.
- `UWorldRoot` يخلط بين مسؤولية الجذر ومسؤولية نمط الرندر عبر `is_3d`.
- `UVal::Px` و`ComputedSize` موثقان بلغة pixels، بينما world/3D يستهلك نفس القيم كأبعاد هندسية داخل العالم.
- الـ picking وبعض مسارات التفاعل ما زالت مربوطة مباشرة بـ `Camera2d`.
- `UI3d` يُنشر كحالة mutable بدل أن يُشتق من root context محسوم داخليًا.

## الواجهة العامة المستهدفة

```rust
#[derive(Component, Clone, Reflect)]
#[require(UNode)]
pub struct URootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}

#[derive(Clone, Copy, Reflect, PartialEq, Eq)]
pub enum UiSpace {
    Screen,
    World2d,
    World3d,
}

#[derive(Clone, Copy, Reflect)]
pub enum UiCanvasSize {
    Viewport,
    Fixed(Vec2),
}

#[derive(Clone, Copy, Reflect)]
pub enum UiCameraRef {
    Auto,
    Entity(Entity),
}
```

## القواعد الدلالية

- `UVal` يبقى نوعًا خاصًا بوحدات الـ layout داخل شجرة الواجهة فقط.
- `URootUi.canvas` يحدد مساحة الـ canvas المنطقية التي يعمل عليها التخطيط.
- `UiCanvasSize::Viewport` تعني اتباع viewport الكاميرا المستهدفة.
- `UiCanvasSize::Fixed(Vec2)` تعني canvas منطقيًا ثابت الحجم.
- `meters_per_unit` يحدد تحويل الحجم إلى العالم فقط في أوضاع world.
- `resolution_scale` يؤثر على الجودة البصرية، لا على الحجم الفيزيائي في العالم.
- `Screen` يعني HUD حقيقيًا.
- `World2d` يعني canvas داخل العالم بمسار 2D.
- `World3d` يعني canvas داخل العالم بمسار 3D.

## ما ليس ضمن هذا العمل

- عدم إعادة تصميم solver كاملًا ضمن هذه المهمة.
- عدم تغيير واجهات الـ widgets إلا إذا كانت مرتبطة مباشرة بدلالة الجذر.
- عدم تحميل `UVal` معنى الأمتار.
- عدم الإبقاء على سلوك مخفي يجعل "screen" يعني في الحقيقة "world canvas بحجم النافذة".

## المرحلة 0 - تثبيت التصميم

- [x] حسم `URootUi` و`UiSpace` و`UiCanvasSize` و`UiCameraRef`.
- [x] تحديد ما إذا كانت الجذور القديمة ستبقى مؤقتًا كطبقة توافق deprecated أو ستزال مباشرة في `alpha2`.
- [x] تحديد القيمة الافتراضية لـ `meters_per_unit` في جذور العالم.
- [x] تحديد قاعدة عمل `UiCameraRef::Auto` عند وجود أكثر من كاميرا.

### قرارات المرحلة 0

- [x] تثبيت شكل الـ public root كما هو موثق في قسم `الواجهة العامة المستهدفة`.
- [x] تثبيت الـ constructors الأساسية كما يلي:
  - `URootUi::screen()` -> `space: Screen`, `canvas: Viewport`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
  - `URootUi::world_2d(size)` -> `space: World2d`, `canvas: Fixed(size)`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
  - `URootUi::world_3d(size)` -> `space: World3d`, `canvas: Fixed(size)`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
- [x] الإبقاء على `UScreenRoot` و`UWorldRoot` طوال دورة `alpha2` كطبقات توافق deprecated لتقليل الكسور أثناء الترحيل.
- [x] عدم إزالة الجذور القديمة قبل تحديث الأمثلة والتوثيق وملاحظات الترحيل إلى `URootUi`.
- [x] تثبيت القيمة الافتراضية لـ `meters_per_unit` على `0.001` في جذور العالم.
- [x] تثبيت معنى `meters_per_unit` على أنه world scaling فقط. لا يغيّر حجم الـ layout المنطقي، ويُتجاهل في سلوك الشاشة الخالص.
- [x] تثبيت قاعدة `UiCameraRef::Auto` بصيغة fail-closed:
  - يحسم فقط عند وجود كاميرا نشطة ومتوافقة واحدة بالضبط مع الجذر
  - إذا لم توجد كاميرا متوافقة، يبقى الجذر unresolved مع تحذير
  - إذا وُجدت أكثر من كاميرا متوافقة، يبقى الجذر unresolved ويجب استخدام `UiCameraRef::Entity`
- [x] تثبيت سياسة تعدد الكاميرات:
  - التطبيقات البسيطة يمكنها الاعتماد على `Auto`
  - التطبيقات متعددة الكاميرات يجب أن تربط الكاميرا صراحة عبر `UiCameraRef::Entity`

## المرحلة 1 - إدخال طبقة حل داخلي للجذر

أضف نموذجًا داخليًا محلولًا حتى يستهلك layout والرندر والتفاعل نفس الحقيقة.

الشكل المقترح داخليًا:

```rust
pub struct ResolvedRootUi {
    pub root_entity: Entity,
    pub space: UiSpace,
    pub canvas_size: Vec2,
    pub camera_entity: Option<Entity>,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}
```

- [x] إضافة `URootUi` إلى الـ prelude العام في المحرك.
- [x] إضافة نظام resolver داخلي يحسب `ResolvedRootUi`.
- [x] حساب `canvas_size` من viewport أو من canvas ثابت حسب الإعداد.
- [x] حسم الكاميرا من `UiCameraRef`.
- [x] جعل النموذج المحلول هو المصدر المشترك لبقية الأنظمة.

الملفات الأساسية:

- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_engine/src/layout/mod.rs`

## المرحلة 2 - جعل Screen HUD حقيقيًا

جذور `Screen` يجب ألا تبقى جذورًا world-space عادية بحجم النافذة.

- [x] استبدال fallback المباشر على `Window.width()/height()` بحساب يعتمد على viewport الحقيقي.
- [x] ربط جذور الشاشة بالكاميرا المحسومة.
- [x] التأكد أن حركة الكاميرا لا تحرك واجهة الشاشة بصريًا.
- [x] التأكد أن zoom الكاميرا لا يغير حجم واجهة الشاشة بصريًا.
- [x] التأكد أن دوران الكاميرا لا يدير واجهة الشاشة بصريًا.
- [x] تثبيت سياسة transform الخاصة بجذور الشاشة بشكل واضح ومستقر.

الملفات الأساسية:

- `crates/univis_ui_engine/src/layout/core/pass_down.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`

## المرحلة 3 - إزالة الاعتماد الصلب على Camera2d

التفاعل يجب أن يعتمد على كاميرا الجذر المحلولة، لا على افتراض عام بوجود `Camera2d`.

- [x] إعادة بناء الـ picking backend بحيث يحسم الكاميرا من الجذر المستهدف.
- [x] إيقاف الاعتماد على `With<Camera2d>` كمصدر وحيد للـ picking.
- [x] تحديث مسار resize في `UPanelWindow` ليستخدم كاميرا الجذر المحلولة.
- [x] مراجعة مسارات `text_field` و`scroll` لنفس المشكلة.
- [x] دعم تعدد الكاميرات بدون ترتيب اختيار غير محدد.

الملفات الأساسية:

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- أي أنظمة widgets أو input تعتمد حاليًا على `Camera2d`

## المرحلة 4 - فصل الفضاء عن نمط الرندر

نوع الجذر يجب أن يصف مكان الواجهة، بينما الرندر يجب أن يتبع الفضاء المحلول لا `UWorldRoot.is_3d`.

- [x] إزالة `is_3d` من القصة العامة للجذر.
- [x] ربط `UiSpace::World3d` بمسار المواد والرندر ثلاثي الأبعاد.
- [x] ربط `UiSpace::Screen` و`UiSpace::World2d` بمسار 2D ما لم نوسع ذلك لاحقًا.
- [x] تحديث أنظمة الرندر لتقرأ من root context المحلول بدل الاستعلامات الحالية المتفرقة.

الملفات الأساسية:

- `crates/univis_ui_engine/src/layout/render/system.rs`
- `crates/univis_ui_engine/src/layout/render/mod.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`

## المرحلة 5 - استبدال نشر UI3d بحالة مشتقة

المحرك لا يجب أن يعتمد على نشر أحادي الاتجاه لـ `UI3d` من الآباء إلى الأبناء كمصدر الحقيقة.

- [ ] مراجعة كل الاستخدامات الحالية لـ `UI3d`.
- [ ] تحديد ما إذا كان `UI3d` سيصبح marker داخليًا مخزنًا مؤقتًا أو حالة transient مشتقة بالكامل.
- [ ] ضمان إزالة أي حالة 3D قديمة عند التبديل بين `World2d` و`World3d`.
- [ ] ضمان حصول الأبناء الجدد على الوضع الصحيح بدون الاعتماد على quirks ناتجة عن التأخير بين frames.

الملفات الأساسية:

- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
- `crates/univis_ui_engine/src/layout/components.rs`

## المرحلة 6 - توحيد نموذج الوحدات

يجب توضيح أن التخطيط يستخدم وحدات UI منطقية، بينما التحويل إلى العالم يتم بشكل منفصل.

- [ ] تحديث توثيق `UVal` بحيث تعني `Px` وحدات UI منطقية، لا pixels فعلية للشاشة.
- [ ] تحديث توثيق `ComputedSize` بنفس المعنى.
- [ ] توثيق العلاقة `world_size = canvas_size * meters_per_unit`.
- [ ] ضمان أن أحجام الـ mesh والتحويلات تستخدم world scale المحلول في أوضاع world.
- [ ] ضمان أن `resolution_scale` يبقى مستقلًا عن الحجم الفيزيائي في العالم.

الملفات الأساسية:

- `crates/univis_ui_engine/src/layout/geometry.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
- أي أنظمة نصوص أو صور ما زالت تفترض semantics مرتبطة بالـ pixels

## المرحلة 7 - ترحيل الواجهة العامة

أدخل `URootUi` بشكل واضح ونظيف، وأزل الغموض من السطح العام للـ API.

- [ ] تصدير `URootUi` من الـ prelude العام.
- [ ] إضافة `URootUi::screen()`.
- [ ] إضافة `URootUi::world_2d(size)`.
- [ ] إضافة `URootUi::world_3d(size)`.
- [ ] الإبقاء على `UScreenRoot` كطبقة توافق deprecated إلى `URootUi::screen()` خلال `alpha2`.
- [ ] الإبقاء على `UWorldRoot { size, is_3d: false }` كطبقة توافق deprecated إلى `URootUi::world_2d(size)` خلال `alpha2`.
- [ ] الإبقاء على `UWorldRoot { size, is_3d: true }` كطبقة توافق deprecated إلى `URootUi::world_3d(size)` خلال `alpha2`.
- [ ] وسم الجذور القديمة بـ deprecated مع ملاحظات ترحيل إذا أبقيناها مؤقتًا.
- [ ] إزالة أي صياغة توثيقية لم تعد تعكس السلوك الحقيقي.

الملفات الأساسية:

- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `README.md`

## المرحلة 8 - تحديث الأمثلة

كل مثال يجب أن ينتقل إلى الـ API الجديد ويبرهن الدلالات الصحيحة.

- [ ] استبدال `UScreenRoot` بـ `URootUi::screen()`.
- [ ] استبدال `UWorldRoot` بـ `URootUi::world_2d(...)` أو `URootUi::world_3d(...)`.
- [ ] إضافة مثال واحد على الأقل يثبت أن HUD الشاشة ثابت بصريًا أثناء حركة الكاميرا.
- [ ] إضافة مثال واحد على الأقل يثبت canvas منطقيًا ثابتًا مع world scale صريح.
- [ ] مراجعة الأمثلة التي تعتمد ضمنيًا على `Camera2d`.

المسارات الأساسية:

- `examples/`

## المرحلة 9 - تحديث التوثيق

التوثيق يجب أن يتوقف عن شرح النموذج المكسور.

- [ ] إعادة كتابة توثيق الجذور في الكتابين.
- [ ] تحديث مقتطفات البداية السريعة.
- [ ] تحديث support matrix لتمييز `Screen` و`World2d` و`World3d`.
- [ ] إضافة ملاحظة ترحيل لـ `alpha2`.
- [ ] توثيق وحدات UI المنطقية.
- [ ] توثيق معنى viewport canvas.
- [ ] توثيق world scaling عبر `meters_per_unit`.

الملفات الأساسية:

- `README.md`
- `book_en/src/layout/roots.md`
- `book_ar/src/layout/roots.md`
- `book_en/src/quick-start.md`
- `book_ar/src/quick-start.md`
- `book_en/src/interaction/support-matrix.md`
- `book_ar/src/interaction/support-matrix.md`
- `changelog.md`

## المرحلة 10 - التحقق

هذا التغيير يحتاج تحققًا سلوكيًا، وليس مجرد نجاح compilation.

### اختبارات المحرك

- [ ] التحقق من حساب حجم الجذر في حالات viewport canvas وfixed canvas وfallback.
- [ ] التحقق من صحة الـ layout تحت مسار root resolution الجديد.
- [ ] التحقق من التبديل بين `World2d` و`World3d`.
- [ ] التحقق من إدخال أبناء جدد تحت كل root mode.

### اختبارات التفاعل

- [ ] التحقق من الـ picking تحت `Screen`.
- [ ] التحقق من الـ picking تحت `World2d`.
- [ ] التحقق من الـ picking تحت `World3d` وفق مسار الكاميرا المدعوم.
- [ ] التحقق من resize في `UPanelWindow` بعد إزالة hardcoding `Camera2d`.
- [ ] التحقق من clipping-aware hit testing بعد إعادة الهيكلة.

### التحقق السلوكي

- [ ] تحريك الكاميرا والتأكد أن واجهة الشاشة تبقى ثابتة بصريًا.
- [ ] عمل zoom للكاميرا والتأكد أن واجهة الشاشة تبقى ثابتة بصريًا.
- [ ] تدوير الكاميرا والتأكد أن واجهة الشاشة تبقى ثابتة بصريًا.
- [ ] التأكد أن world UI يبقى مرتبطًا بتحويلات العالم.
- [ ] التأكد أن تغيير `meters_per_unit` يغيّر الحجم الفيزيائي للواجهة دون تغيير الـ layout المنطقي.

### تحقق الإصدار

- [ ] تشغيل `cargo check --workspace`.
- [ ] تشغيل اختبارات مستهدفة لحزم المحرك والتفاعل.
- [ ] تشغيل smoke checks على أمثلة screen وworld و3D.
- [ ] إجراء تحقق بصري يدوي لصحة HUD.

## ترتيب التنفيذ المقترح

لتقليل الكسور أثناء `alpha2`، يكون التنفيذ بهذا الترتيب:

- [ ] إضافة `URootUi` و`ResolvedRootUi` الداخلي.
- [ ] إصلاح سلوك HUD الحقيقي للشاشة.
- [ ] إزالة الاعتماد الصلب على الكاميرا في picking وpanel resize.
- [ ] فصل نمط الرندر عن هوية الجذر.
- [ ] توحيد نموذج الوحدات وworld scaling.
- [ ] ترحيل الأمثلة والتوثيق.
- [ ] إزالة أو deprecate الجذور القديمة.

## معايير الإنجاز

لا تعتبر المهمة مكتملة إلا إذا تحققت جميع النقاط التالية:

- [ ] لم يعد `UScreenRoot` مصدرًا لدلالة HUD مضللة.
- [ ] أصبح `URootUi` هو نقطة الدخول العامة الوحيدة للجذور.
- [ ] أصبحت واجهة الشاشة ثابتة فعليًا على الشاشة.
- [ ] أصبحت جذور العالم تستخدم logical canvas sizing صريحًا.
- [ ] أصبح world scaling صريحًا بدل أن يكون ضمنيًا عبر `UVal::Px`.
- [ ] لم يعد الـ picking يعتمد على افتراض عام بوجود `Camera2d`.
- [ ] أصبح التوثيق مطابقًا للكود.
- [ ] أصبحت الأمثلة تشرح السلوك المستهدف بوضوح.

## متابعات اختيارية

بعد استقرار إعادة الهيكلة الأساسية:

- [ ] دراسة إزالة `UI3d` من السطح العام بالكامل إذا بقي داخليًا فقط.
- [ ] دراسة إضافة debug overlays تعرض root kind وcamera binding وcanvas size وworld size.
- [ ] دراسة إضافة صفحة ترحيل مخصصة لـ `0.2.0-alpha.2`.
