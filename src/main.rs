use mist_tools_rust::{mist_service, Envelope};

// Some dummy action
pub fn handle_english_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    Ok(())
}
// Some other dummy action
pub fn handle_spanish_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    println!("reached spanish handler!");
    Ok(())
}

pub fn init() -> Result<(), &'static str> {
    Ok(())
}

pub fn main() {
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
