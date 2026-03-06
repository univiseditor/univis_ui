# مصفوفة التوافق

تاريخ خط الأساس للتحقق: March 6, 2026.

المعاني:
- `Yes`: مدعوم ومتحقق منه في خط الأساس الحالي.
- `Partial`: مدعوم جزئيًا مع قيود معروفة.
- `No`: غير مدعوم في خط الأساس الحالي.
- `Deferred`: تحقق مخطط له لكنه أُجِّل عمدًا في هذه الدورة.

| القدرة | Screen UI | World UI (2D) | World UI (3D) | مصدر التحقق |
|---|---|---|---|---|
| runtime smoke اليدوي (`cargo run --release --example ...`) | Deferred | Deferred | Deferred | تم التخطي في March 6, 2026 (قيود موارد)، انظر [خطة Smoke Tests](smoke-test-plan.md) |
| بناء أمثلة `release` | Yes | Yes | Yes | `./scripts/check_examples_serial_release.sh` (نجاح 28/28) |
| مسار الرندر الأساسي | Yes | Yes | Yes | الأمثلة + إعداد render plugin |
| تفاعل المؤشر | Yes | Yes | Partial | `univis_picking_backend` يستعلم `Camera2d` حاليًا |
| الالتقاط مع مراعاة القص | Yes | Yes | Partial | فحص قص الأسلاف داخل picking backend |
| تغيير حجم `UPanelWindow` | Yes | Yes | Partial | مسار تغيير الحجم يستعلم `Camera2d` حاليًا |
| سلوك/أحداث `UTextField` | Yes | Yes | Partial | يتطلب `UnivisTextFieldPlugin` |
| التحديثات الديناميكية لـ `UBadge` | Yes | Yes | Yes | تتطلب `UnivisBadgePlugin` |
| سلوك `UScrollContainer` | Yes | Yes | Partial | مسار التفاعل يتبع قيود picking الحالية |
| خصائص `UPbr` | No | No | Yes | مخصصة لمسار `UI3d` |

## مرتبط

- [مصفوفة دعم التفاعل](../interaction/support-matrix.md)
- [القيود الحالية](current-limitations.md)
- [تقرير التحقق من الأمثلة](example-validation.md)
