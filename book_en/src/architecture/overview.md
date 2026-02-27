# المعمارية

بنية `univis_ui` مقسمة إلى أربع وحدات رئيسية:

- `layout/`: محرك التخطيط والحساب المكاني.
- `interaction/`: الالتقاط pointer picking وحالات التفاعل.
- `widget/`: widgets الجاهزة (أزرار، إدخال، قوائم، إلخ).
- `style/`: خطوط وأيقونات وثيم أساسي.

## الفكرة الأساسية

كل شيء في Univis هو ECS Entities + Components:

- لا يوجد retained UI tree خارجي.
- كل عنصر واجهة هو كيان يحمل `UNode` ومعه مكونات إضافية.
- التخطيط والحساب يتم عبر Systems ضمن جداول Bevy.

## المسار القياسي للإطار

1. `PreUpdate`:
- تشغيل backend الالتقاط لاكتشاف hit entities.

2. `Update`:
- أنظمة widgets والتفاعل وتحديث المرئيات المنطقية.

3. `PostUpdate`:
- pipeline التخطيط:
  - `update_layout_hierarchy`
  - `upward_measure_pass_cached`
  - `downward_solve_pass_safe`
- أنظمة الرندر/المواد (حسب التغييرات).

## أهداف التصميم

- دقة بصرية عالية عبر SDF.
- مرونة عالية في التخطيط (Flex/Grid/Masonry/Stack/Radial).
- قابلية توسيع عبر ECS Components وPlugins.
- قابلية تتبع الأداء عبر profiler overlay وcache.
