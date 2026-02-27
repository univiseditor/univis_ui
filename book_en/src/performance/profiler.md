# Profiler Overlay

الملف: `src/layout/profiling.rs`

## Plugin

- `LayoutProfilingPlugin`

## بيانات يعرضها

- زمن `upward pass`
- زمن `downward pass`
- زمن تحديث المواد
- إحصائيات cache
- history + graph

## اختصارات لوحة المفاتيح

- `F10`: enable/disable profiler
- `F11`: toggle overlay
- `F9`: toggle graph
- `F12`: تغيير موضع overlay

## متى تستخدمه؟

- عند ملاحظة frame spikes.
- عند مقارنة تأثير تعديل layout كبير.
- عند تقييم فعالية cache.
