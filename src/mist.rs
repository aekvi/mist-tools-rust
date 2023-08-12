use crate::{mime_types, Envelope, MimeType, MistTools};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::env;
use std::fs::File;
use std::io::{self, Read};

pub struct Mist {
    action: &'static str,
    envelope: Envelope,
    payload: Vec<u8>,
}

impl MistTools for Mist {
    fn handle(
        &self,
        action: &'static str,
        handler: impl FnOnce(Vec<u8>, Envelope) -> Result<(), String>,
    ) -> Result<&Self, String> {
        if self.action == action {
            handler(self.payload.clone(), self.envelope.clone()).map_err(|_| {
                let mut s = "unable to execute action ".to_owned();
                s.push_str(action);
                s
            })?;
        }
        Ok(self)
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
            .map_err(|_| "unable to read from stdin")?;
        Ok(buffer)
    }

    fn new(args: Vec<&'static str>) -> Result<Self, String> {
        let action = args[args.len() - 2];
        let json = args[args.len() - 1];
        let envelope = Envelope::new(json).map_err(|_| {
            let mut s = "unable to parse envelope from ".to_owned();
            s.push_str(json);
            s
        })?;
        let payload = Self::get_payload()?;
        Ok(Mist {
            action,
            envelope,
            payload,
        })
    }
}

pub fn service(args: Vec<&'static str>) -> Result<Mist, String> {
    Mist::new(args)
}

fn internal_post_to_rapids(
    event: &'static str,
    request_completer: impl FnOnce(RequestBuilder) -> Result<Response, reqwest::Error>,
) -> Result<(), String> {
    let rapids_url = env::var("RAPIDS").map_err(|_| "RAPIDS environment variable not set")?;
    let event_url = format!("{}/{}", rapids_url, event);

    let client = Client::new();
    let init_request_builder = client.post(&event_url);

    request_completer(init_request_builder).map_err(|_| {
        let mut s = "unable to post event '".to_owned();
        s.push_str(event);
        s.push_str("' to url '");
        s.push_str(event_url.as_str());
        s.push('\'');
        s
    })?;

    Ok(())
}

pub fn post_to_rapids(
    event: &'static str,
    body: Vec<u8>,
    content_type: MimeType,
) -> Result<(), String> {
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
) -> Result<(), String> {
    internal_post_to_rapids(event, |r| {
        r.header("Content-Type", content_type.to_string())
            .body(body)
            .send()
    })
}

pub fn post_event_to_rapids(event: &'static str) -> Result<(), String> {
    internal_post_to_rapids(event, |r| r.send())
}

pub fn reply_to_origin(body: Vec<u8>, content_type: MimeType) -> Result<(), String> {
    post_to_rapids("$reply", body, content_type)
}

pub fn reply_str_to_origin(body: &'static str, content_type: MimeType) -> Result<(), String> {
    post_str_to_rapids("$reply", body, content_type)
}

pub fn reply_file_to_origin_with_content_type(
    path: &'static str,
    content_type: MimeType,
) -> Result<(), String> {
    let mut file = File::open(path).map_err(|_| {
        let mut s = "unable to open file '".to_owned();
        s.push_str(path);
        s.push('\'');
        s
    })?;

    let mut body = Vec::new();
    file.read_to_end(&mut body).map_err(|_| {
        let mut s = "unable to read file '".to_owned();
        s.push_str(path);
        s.push('\'');
        s
    })?;

    post_to_rapids("$reply", body, content_type)
}

pub fn reply_file_to_origin(path: &'static str) -> Result<(), String> {
    let file_ext = path
        .split('.')
        .last()
        .unwrap_or_else(|| panic!("unable to locate file extension from file path '{}'", path));
    let content_type = mime_types::ext2mime(file_ext)
        .unwrap_or_else(|| panic!("unknown file extension from file path '{}'", path));
    reply_file_to_origin_with_content_type(path, content_type)
}
