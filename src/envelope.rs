use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, Deserialize, Serialize)]
pub struct Envelope<'a> {
    message_id: &'a str,
    trace_id: &'a str,
}

impl<'a> Envelope<'a> {
    pub fn new(json: &'a str) -> Result<Self> {
        let envelope: Envelope = serde_json::from_str(json)?;
        Ok(envelope)
    }
}
