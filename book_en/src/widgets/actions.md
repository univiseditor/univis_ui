# Action Widgets

## UButton

- الملف: `src/widget/button.rs`
- يطبق نمط زر عبر `UNode` + `UInteractionColors`.
- presets: `primary`, `secondary`, `danger`, `success`.

## UIconButton

- الملف: `src/widget/icon_btn.rs`
- نفس فكرة button مع دعم icon font.

## UToggle

- الملف: `src/widget/toggle.rs`
- toggle switch مع animation للknob.
- event:
  - `ToggleChangedEvent`

## UCheckbox

- الملف: `src/widget/checkbox.rs`
- منطق بسيط يعتمد pointer click.

## URadioButton / URadioGroup

- الملف: `src/widget/radio.rs`
- إدارة مجموعة اختيار واحد.
- event:
  - `RadioButtonChangedEvent`
