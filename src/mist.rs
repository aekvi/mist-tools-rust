use crate::{ActionHandler, Envelope, InitHandler, MistTools};
use std::io;

pub struct Mist<'a> {
    action: &'static str,
    envelope: Envelope<'a>,
    payload: String,
}

impl MistTools for Mist<'_> {
    fn handle(&self, action: &str, handler: &impl ActionHandler) -> Self {
        todo!()
    }

    fn init(&self, handler: &impl InitHandler) -> Result<(), &'static str> {
        handler.execute()
    }
}

impl Mist<'_> {
    pub fn service(args: Vec<&'static str>) -> Result<Self, &'static str> {
        Self::new(args)
    }

    fn get_payload() -> Result<String, &'static str> {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(|_| "Unable to read from stdin")?;
        Ok(buffer)
    }

    fn new(args: Vec<&'static str>) -> Result<Self, &'static str> {
        let action = args[args.len() - 2];
        let envelope = Envelope::new(args[args.len() - 1])?;
        let payload = Self::get_payload()?;
        Ok(Mist {
            action,
            envelope,
            payload,
        })
    }
}
