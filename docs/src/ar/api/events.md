# الأحداث Messages

الأحداث في Univis تستخدم Bevy Messages (`#[derive(Message)]`).

## Toggle

- `ToggleChangedEvent`

## Radio

- `RadioButtonChangedEvent`

## SeekBar

- `SeekBarChangedEvent`

## DragValue

- `DragValueChangedEvent`
- `DragValueCommitEvent`

## Select

- `SelectChangedEvent`
- `SelectOpenStateChangedEvent`

## TextField

- `TextFieldChangedEvent`
- `TextFieldSubmitEvent`

## نمط استهلاك الأحداث

```rust,no_run
use bevy::prelude::*;
use univis_ui::prelude::*;

fn consume(mut events: MessageReader<ToggleChangedEvent>) {
    for ev in events.read() {
        // handle
    }
}
```
