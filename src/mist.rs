use crate::{mime_types, Envelope, MimeType};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::env;
use std::fs::File;
use std::io::{self, Read};

pub type ActionHandler = (
    &'static str,
    Box<dyn FnOnce(Vec<u8>, Envelope) -> Result<(), String>>,
);

/// Entry point for registering services on certain actions.
///
/// # Examples
///
/// ```
/// use mist_tools_rust::{mist_service, Envelope};
///
/// // Some dummy action
/// pub fn handle_english_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
///     Ok(())
/// }
///
/// // Some other dummy action
/// pub fn handle_spanish_action(_buffer: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
///     println!("reached spanish handler!");
///     Ok(())
/// }
///
/// mist_service(vec![("hello", Box::new(handle_english_action)), ("hola", Box::new(handle_spanish_action))], || Ok(()));
/// ```
///
pub fn mist_service<A>(handlers: Vec<ActionHandler>, init: A) -> Result<(), String>
where
    A: FnOnce() -> Result<(), &'static str>,
{
    let (arg_action, envelope) = get_args()?;
    for (action, handler) in handlers {
        if action == arg_action {
            handler(get_payload()?, envelope)?;
            break;
        }
    }

    init()?;

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
    let args = env::args().rev().take(2);
    if args.len() < 3 {
        Err("Insufficient program arguments".to_string())
    } else {
        let mut action = None;
        let mut envelope = None;
        for (i, arg) in args.enumerate() {
            match i {
                0 => {
                    action = Some(arg);
                }
                1 => {
                    envelope = Some(Envelope::new(arg.as_str())?);
                }
                _ => unreachable!(),
            }
        }
        match (action, envelope) {
            (Some(a), Some(e)) => Ok((a, e)),
            (None, _) => Err("Unable to get action".to_string()),
            _ => unreachable!(),
        }
    }
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
