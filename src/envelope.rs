use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Envelope {
    #[serde(alias = "messageId")]
    message_id: String,
    #[serde(alias = "traceId")]
    trace_id: String,
}

impl Envelope {
    pub fn new(json: &str) -> Result<Self, String> {
        let envelope: Envelope =
            serde_json::from_str(json).map_err(|_| "Unable to parse envelope from json")?;
        Ok(envelope)
    }
}
