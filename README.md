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
                // (Required) "action": handler
                // Type: FnOnce(Vec<u8>, Envelope) -> Result<(), String>
                "hello": handle_english_action,
                "hola": handle_spanish_action
            },
            // (Optional) init: init_handler
            // Type FnOnce() -> Result<(), &'static str>
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
