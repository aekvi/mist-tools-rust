# Mist Tools Rust

Wrapper library for [Mist Cloud](https://mist-cloud.eu/).


## Getting Started

Add following dependecy to your Mist service:

```
mist-tools = { git = "https://github.com/aekvi/mist-tools-rust.git", tag = "v0.1.1" }
```

Also, Mist requires an entry point '`app`'. You can specify your `src/main.rs` file as this entry point by adding the following to your `Cargo.toml`:
```
[[bin]]
name = "app"
path = "src/main.rs"
```

## Usage

In your main function, simply use the `mist_service!` macro. Provide it with a JSON-like
object, that defines the actions and corresponding handlers.
Optionally, you can provide an `init` function.
See `examples/use.rs` for a more detailed example.

```rust
use mist_tools::{mist_service};

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

A handler can respond to the outside world with the following functions:
- `post_event_to_rapids`
- `post_str_to_rapids`
- `post_to_rapids`
- `reply_file_to_origin`
- `reply_file_to_origin_with_content_type`
- `reply_str_to_origin`
- `reply_to_origin`

Posting to *rapids* publishes a new event on the event stream that other services
can potentially listen for.
These functions end with `post_X_to_rapids`
(also includes `post_to_rapids`).

To respond to the client that initiated the session, you can use
the functions with names containing `reply_X_to_origin`
(also inlcudes `reply_file_to_origin_with_content_type` and `reply_to_origin`).
