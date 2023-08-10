use crate::Envelope;

pub trait ActionHandler {
    fn execute(&self, payload_bytes: Box<[u8]>, envelope: Envelope) -> Result<(), &'static str>;
}
