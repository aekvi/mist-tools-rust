use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, Deserialize, Serialize)]
pub struct Envelope {
    message_id: String,
    trace_id: String,
}

impl Envelope {
    pub fn new(json: &str) -> Result<Self> {
        let envelope: Envelope = serde_json::from_str(json.clone())?;
        Ok(envelope)
    }
}
