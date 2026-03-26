# Events and Messages

Univis events use Bevy messages via `#[derive(Message)]`.

Common examples include:

- `ButtonClicked`
- `ToggleChanged`
- `SelectChanged`
- `SeekBarChanged`
- `TextFieldSubmitted`

## Consumption Pattern

Typical message handling looks like this:

```rust,no_run
fn read_messages(mut events: MessageReader<ButtonClicked>) {
    for event in events.read() {
        // react to the click
    }
}
```

## Practical Guidance

- use messages for app-level reactions
- keep local visual state inside widget systems
- avoid duplicating the same state in both widget components and external resources unless needed
