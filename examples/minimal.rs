use mist_tools::{mist_service, Envelope};

pub fn main() -> Result<(), String> {
    mist_service!(
        {
            actions: {
                "hello": handle_english_action
            }
        }
    )
}

// Some action
pub fn handle_english_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    // Do stuff...
    Ok(())
}
