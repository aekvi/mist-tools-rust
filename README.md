# mist-tools-rust

## Usage

In your main function, simply use the `mist_service!` macro. Provide it with a JSON-like
object, that defines the actions and corresponding handlers.
Optionally, you can provide an `init` function.
See `examples/use.rs` for a more detailed example.

```rust
use mist_tools::{mist_service, Envelope};

pub fn main() -> Result<(), String> {
    mist_service!(
        {
            actions: {
                "hello": handle_english_action,
                "hola": handle_spanish_action
            },
            init: init
        }
    )
}
```

Add following dependecy to your Mist service
```
mist-tools = { git = "https://github.com/anbclausen/mist-tools-rust.git" }
```
