# معرض الأمثلة

هذه الصفحة هي العرض المنسق لسطح الأمثلة. استخدم [فهرس الأمثلة](index.md) إذا كنت تريد الفهرس الكامل وقائمة الحزم.

## أفضل أمثلة للبدء

1. `hello_world`
2. `root_screen_hud`
3. `root_world_scale`
4. `text_field`
5. `interaction`

## واجهات الشاشة الثابتة

| المثال | الحزمة | لماذا قد تشغّله | الأمر | المرجع البصري |
| --- | --- | --- | --- | --- |
| `hello_world` | `univis_ui` | ابدأ به إذا أردت أصغر مسار تشغيل على مستوى الواجهة المجمعة. | `cargo run -p univis_ui --example hello_world` | شغّل المثال |
| `root_screen_hud` | `univis_ui_engine` | شغّله إذا أردت التأكد من سلوك الواجهة الثابتة على الشاشة مع حركة الكاميرا. | `cargo run -p univis_ui_engine --example root_screen_hud` | شغّل المثال |
| `layout_cache` | `univis_ui_engine` | شغّله إذا أردت فحص الذاكرة المؤقتة للتخطيط وسلوك الإبطال وإعادة الحل. | `cargo run -p univis_ui_engine --example layout_cache` | شغّل المثال |

## الواجهات داخل العالم

| المثال | الحزمة | لماذا قد تشغّله | الأمر | المرجع البصري |
| --- | --- | --- | --- | --- |
| `root_world_scale` | `univis_ui_engine` | شغّله إذا أردت معايرة حجم مساحة الرسم المنطقية مقابل الحجم الفيزيائي في العالم. | `cargo run -p univis_ui_engine --example root_world_scale` | شغّل المثال |
| `root_fit_content` | `univis_ui_engine` | شغّله إذا أردت لوحات عالمية يتحدد حجمها من المحتوى. | `cargo run -p univis_ui_engine --example root_fit_content` | شغّل المثال |
| `root_capsule_overlap` | `univis_ui_engine` | شغّله إذا أردت التحقق من قواعد كبسولات الجذور المغلقة عند التداخل. | `cargo run -p univis_ui_engine --example root_capsule_overlap` | شغّل المثال |
| `border_light_3d` | `univis_ui_engine` | شغّله إذا أردت فحص مسار `World3d` المضيء و`UPbr`. | `cargo run -p univis_ui_engine --features example_bloom --example border_light_3d` | شغّل المثال |

## الوحدات الجاهزة والنماذج

| المثال | الحزمة | لماذا قد تشغّله | الأمر | المرجع البصري |
| --- | --- | --- | --- | --- |
| `widgets` | `univis_ui_widgets` | شغّله إذا أردت نظرة أولى واسعة على الوحدات الجاهزة المضمنة. | `cargo run -p univis_ui_widgets --example widgets` | شغّل المثال |
| `text_label` | `univis_ui_widgets` | شغّله إذا أردت دراسة قص النص والفيض و`autosize`. | `cargo run -p univis_ui_widgets --example text_label` | شغّل المثال |
| `text_field` | `univis_ui_widgets` | شغّله إذا أردت إدخالًا نصيًا قابلًا للتحرير مع التصفية ورسائل الإرسال والتغيير. | `cargo run -p univis_ui_widgets --example text_field` | شغّل المثال |
| `panel_window` | `univis_ui_widgets` | شغّله إذا أردت سلوك نوافذ الأدوات العائمة والقابلة لتغيير الحجم. | `cargo run -p univis_ui_widgets --example panel_window` | شغّل المثال |
| `scroll_view` | `univis_ui_widgets` | شغّله إذا أردت سلوك الفيض والتمرير الصريح. | `cargo run -p univis_ui_widgets --example scroll_view` | شغّل المثال |

## التفاعل

| المثال | الحزمة | لماذا قد تشغّله | الأمر | المرجع البصري |
| --- | --- | --- | --- | --- |
| `interaction` | `univis_ui_interaction` | شغّله إذا أردت فهم الحجب والتجاهل والتمرير في الضربات. | `cargo run -p univis_ui_interaction --example interaction` | شغّل المثال |

## أمثلة العرض الكبيرة

| المثال | الحزمة | لماذا قد تشغّله | الأمر | المرجع البصري |
| --- | --- | --- | --- | --- |
| `card_profile` | `univis_ui` | شغّله إذا أردت أكثر بطاقة عرض صغيرة مصقولة في المستودع. | `cargo run -p univis_ui --features example_bloom --example card_profile` | [لقطة شاشة](../../assets/profile.png) |
| `sci_fi` | `univis_ui` | شغّله إذا أردت مشهدًا أكبر يخلط عدة أنظمة معًا. | `cargo run -p univis_ui --features example_bloom --example sci_fi` | شغّل المثال |
| `complex_dashboard` | `univis_ui` | شغّله إذا أردت تركيبًا أكثف يضم عدة مناطق واجهة. | `cargo run -p univis_ui --example complex_dashboard` | [مرجع ثابت](../../assets/visual-references/complex_dashboard.html) |

## مراجع بصرية ثابتة

هذه المراجع مفيدة عندما تريد نظرة بصرية سريعة من دون تشغيل Bevy:

- [مرجع ثابت للوحة التحكم المعقدة](../../assets/visual-references/complex_dashboard.html)
- [مرجع ثابت لمحلل التخطيط دون الوحدات الجاهزة](../../assets/visual-references/layout_solver_no_widgets.html)
- [مرجع ثابت لمشهد التخطيط شديد التعقيد](../../assets/visual-references/layout_solver_ultra_complex.html)
- [لقطة شاشة بطاقة الملف الشخصي](../../assets/profile.png)

## صفحات مرتبطة

- [فهرس الأمثلة](index.md)
- [الترحيل والقيود](../migration/index.md)
- [تقرير التحقق من الأمثلة](../development/example-validation.md)
