use crate::{ActionHandler, InitHandler};

pub trait MistTools {
    fn handle(&self, action: &str, handler: &impl ActionHandler) -> Self;
    fn init(&self, handler: &impl InitHandler) -> Result<(), &'static str>;
}
