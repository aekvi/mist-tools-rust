use crate::envelope::Envelope;

pub trait MistTools {
    fn handle(
        &self,
        action: &'static str,
        handler: impl FnOnce(Vec<u8>, Envelope) -> Result<(), String>,
    ) -> Result<&Self, String>;
    fn init(&self, handler: impl FnOnce() -> Result<(), &'static str>) -> Result<(), &'static str>;
}
