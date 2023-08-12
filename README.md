# mist-tools-rust

## Usage

In your main function, simply use the `mist_service!` macro. You provide it with a JSON-like
object, that defines the actions and corresponding handlers.
Optionally, you can provide an `init` function.

```rust
use mist_tools_rust::{mist_service, Envelope};

pub fn main() {
    mist_service!(
        {
            actions: {
                // "action": handler
                "hello": handle_english_action,
                "hola": handle_spanish_action
            },
            // Optional init field
            init: init
        }
    )
}

// Some action
pub fn handle_english_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    // Do stuff...
    Ok(())
}
// Some other action
pub fn handle_spanish_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    // Do stuff...
    Ok(())
}

pub fn init() -> Result<(), &'static str> {
    Ok(())
}
```
