use mist_tools::{mist_service, Envelope};

// Requires main to return Result<A, B>
pub fn main() -> Result<(), String> {
    mist_service!(
        {
            actions: {
                // (Required) "action": handler
                // Type: FnOnce(Vec<u8>, Envelope) -> Result<A, B>
                "hello": handle_english_action,
                "hola": handle_spanish_action
            },
            // (Optional) init: init_handler
            // Type FnOnce() -> Result<A, B>
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
