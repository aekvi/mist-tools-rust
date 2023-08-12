use crate::{mime_types, Envelope, MimeType};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::env;
use std::fs::File;
use std::io::{self, Read};

pub struct ActionHandler<T>(&'static str, T)
where
    T: FnOnce(Vec<u8>, Envelope) -> Result<(), String>;

pub fn mist_service<A, B>(handlers: Vec<ActionHandler<A>>, init: Option<B>) -> Result<(), String>
where
    A: FnOnce(Vec<u8>, Envelope) -> Result<(), String>,
    B: FnOnce() -> Result<(), &'static str>,
{
    let (arg_action, envelope) = get_args()?;
    for ActionHandler(action, handler) in handlers {
        if action == arg_action {
            handler(get_payload()?, envelope)?;
            break;
        }
    }

    if let Some(f) = init {
        f()?
    }

    Ok(())
}

fn get_payload() -> Result<Vec<u8>, &'static str> {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|_| "unable to read from stdin")?;
    Ok(buffer)
}

fn get_args() -> Result<(String, Envelope), String> {
    let args: Vec<_> = env::args().collect();
    let action = args[args.len() - 2].clone();
    let envelope = Envelope::new(args[args.len() - 1].as_str())?;
    Ok((action, envelope))
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
    let file_ext = path.split('.').last();
    match file_ext {
        Some(f) => {
            let content_type = mime_types::ext2mime(f);
            match content_type {
                Some(ct) => reply_file_to_origin_with_content_type(path, ct),
                None => {
                    let mut s = "unknown file extension from file path '".to_owned();
                    s.push_str(path);
                    s.push('\'');
                    Err(s)
                }
            }
        }
        None => {
            let mut s = "unable to locate file extension from file path '".to_owned();
            s.push_str(path);
            s.push('\'');
            Err(s)
        }
    }
}
