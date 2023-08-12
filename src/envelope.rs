use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Envelope {
    messageId: String,
    traceId: String,
}

impl Envelope {
    pub fn new(json: &str) -> Result<Self, String> {
        let envelope: Envelope =
            serde_json::from_str(json).map_err(|_| "Unable to parse envelope from json")?;
        Ok(envelope)
    }
}
