# Layout Cache

الملف: `src/layout/core/layout_cache.rs`

## Resource: LayoutCache

يحفظ:

- intrinsic sizes
- dirty nodes
- entities by depth
- frame counters

## أفضل ممارسات

- لا تعدّل `UNode/ULayout/USelf` كل frame بدون سبب.
- فضّل التحديث عند تغيّر حالة فعلية.
- قلّل churn في hierarchy (إضافة/حذف عشوائي مستمر).

## متى تفكر بالتوسعة؟

- إذا زاد عدد العقد كبيرًا جدًا.
- إذا أصبح dirty_ratio مرتفعًا أغلب الوقت.
