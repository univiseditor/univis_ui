# Cache و Invalidation

`LayoutCache` يقلل إعادة الحساب المتكرر.

## ماذا يخزن؟

- intrinsic sizes لكل entity.
- dirty nodes.
- entities grouped by depth.

## متى تصبح العقدة dirty؟

عند تغيّر واحد من:

- `UNode`
- `ULayout`
- `USelf`
- `Children`
- `IntrinsicSize`

مع استثناء مهم:

- إذا كان التغيير الوحيد `IntrinsicSize` على عقدة لديها أبناء، يمكن تجاهله لتقليل الضوضاء.

## Lifecycle

- `track_layout_changes` يحدد dirty.
- `update_depth_cache` يعيد بناء depth map عند تغييرات هيكلية.
- `upward_measure_pass_cached` يقرأ cache ويمسح dirty flags في النهاية.

## تشخيص الأداء

- راقب `dirty_count/dirty_ratio`.
- فعّل profiler overlay عند الحاجة.
