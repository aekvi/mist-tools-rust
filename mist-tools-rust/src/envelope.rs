// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
pub struct Envelope<'a> {
    message_id: &'a str,
    trace_id: &'a str,
}

impl<'a> Envelope<'a> {
    pub fn new(json: &'a str) -> Result<Self, &'static str> {
        // TODO: Use Serde
        Err("Unable to parse value")
    }
}
