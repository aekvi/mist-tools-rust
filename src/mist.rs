use crate::{mime_types, Envelope, MimeType, MistTools};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::env;
use std::fs::File;
use std::io::{self, Read};

pub struct Mist {
    action: String,
    envelope: Envelope,
    payload: Vec<u8>,
}

impl MistTools for Mist {
    fn handle(
        &self,
        action: &'static str,
        handler: impl FnOnce(Vec<u8>, Envelope) -> Result<(), &'static str>,
    ) -> &Self {
        if self.action == action {
            handler(self.payload.clone(), self.envelope.clone())
                .unwrap_or_else(|_| panic!("unable to execute action {}", action));
        }
        self
    }

    fn init(&self, handler: impl FnOnce() -> Result<(), &'static str>) -> Result<(), &'static str> {
        handler()
    }
}

impl Mist {
    fn get_payload() -> Result<Vec<u8>, &'static str> {
        let mut buffer = Vec::new();
        io::stdin()
            .read_to_end(&mut buffer)
            .unwrap_or_else(|_| panic!("unable to read from stdin"));
        Ok(buffer)
    }
    fn new(args: Vec<String>) -> Result<Self, &'static str> {
        let action = &args[args.len() - 2];
        let json = &args[args.len() - 1];
        let envelope = Envelope::new(json)
            .unwrap_or_else(|_| panic!("unable to parse envelope from {}", json));
        let payload = Self::get_payload()?;
        Ok(Mist {
            action: action.clone(),
            envelope,
            payload,
        })
    }
}

pub fn service(args: Vec<String>) -> Result<Mist, &'static str> {
    Mist::new(args)
}

fn internal_post_to_rapids(
    event: &'static str,
    request_completer: impl FnOnce(RequestBuilder) -> Result<Response, reqwest::Error>,
) -> Result<(), &'static str> {
    let rapids_url =
        env::var("RAPIDS").unwrap_or_else(|_| panic!("RAPIDS environment variable not set"));
    let event_url = format!("{}/{}", rapids_url, event);

    let client = Client::new();
    let init_request_builder = client.post(&event_url);

    request_completer(init_request_builder)
        .unwrap_or_else(|_| panic!("unable to post event '{}' to url '{}'", event, event_url));
    Ok(())
}

pub fn post_to_rapids(
    event: &'static str,
    body: Vec<u8>,
    content_type: MimeType,
) -> Result<(), &'static str> {
    internal_post_to_rapids(event, |r| {
        r.header("Content-Type", content_type.to_string())
            .body(body)
            .send()
    })
}

pub fn post_str_to_rapids(
    event: &'static str,
    body: &'static str,
    content_type: MimeType,
) -> Result<(), &'static str> {
    internal_post_to_rapids(event, |r| {
        r.header("Content-Type", content_type.to_string())
            .body(body)
            .send()
    })
}

pub fn post_event_to_rapids(event: &'static str) -> Result<(), &'static str> {
    internal_post_to_rapids(event, |r| r.send())
}

pub fn reply_to_origin(body: Vec<u8>, content_type: MimeType) -> Result<(), &'static str> {
    post_to_rapids("$reply", body, content_type)
}

pub fn reply_str_to_origin(body: &'static str, content_type: MimeType) -> Result<(), &'static str> {
    post_str_to_rapids("$reply", body, content_type)
}

pub fn reply_file_to_origin_with_content_type(
    path: &'static str,
    content_type: MimeType,
) -> Result<(), &'static str> {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("unable to open file '{}'", path));
    let mut body = Vec::new();
    file.read_to_end(&mut body)
        .unwrap_or_else(|_| panic!("unable to read file '{}'", path));
    post_to_rapids("$reply", body, content_type)
}

pub fn reply_file_to_origin(path: &'static str) -> Result<(), &'static str> {
    let file_ext = path
        .split('.')
        .last()
        .unwrap_or_else(|| panic!("unable to locate file extension from file path '{}'", path));
    let content_type = mime_types::ext2mime(file_ext)
        .unwrap_or_else(|| panic!("unknown file extension from file path '{}'", path));
    reply_file_to_origin_with_content_type(path, content_type)
}
