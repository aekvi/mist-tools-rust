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
/// use mist_tools::{mist_service, Envelope};
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

pub fn get_payload() -> Result<Vec<u8>, &'static str> {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|_| "unable to read from stdin")?;
    Ok(buffer)
}

pub fn get_args() -> Result<(String, Envelope), &'static str> {
    let mut args: Vec<_> = env::args().collect();
    let envelope_str = args
        .pop()
        .ok_or("unable to read 'envelope' from program arguments")?;
    let envelope = Envelope::new(envelope_str.as_str())?;
    let action = args
        .pop()
        .ok_or("unable to read 'action' from program arguments")?;

    Ok((action.to_string(), envelope))
}

fn internal_post_to_rapids(
    event: &'static str,
    request_completer: impl FnOnce(RequestBuilder) -> Result<Response, reqwest::Error>,
) -> Result<(), String> {
    let rapids_url = env::var("RAPIDS").map_err(|_| "RAPIDS environment variable not set")?;
    let event_url = format!("{}/{}", rapids_url, event);

    let client = Client::new();
    let init_request_builder = client.post(&event_url);

    request_completer(init_request_builder)
        .map_err(|_| format!("unable to post event '{}' to url '{}'", event, event_url))?;

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
    let mut file = File::open(path).map_err(|_| format!("unable to open file '{}'", path))?;

    let mut body = Vec::new();
    file.read_to_end(&mut body)
        .map_err(|_| format!("unable to read file '{}'", path))?;

    post_to_rapids("$reply", body, content_type)
}

pub fn reply_file_to_origin(path: &'static str) -> Result<(), String> {
    let file_ext = path.split('.').last();
    match file_ext {
        Some(f) => {
            let content_type = mime_types::ext2mime(f);
            match content_type {
                Some(ct) => reply_file_to_origin_with_content_type(path, ct),
                None => Err(format!("unknown file extension from file path '{}'", path)),
            }
        }
        None => Err(format!(
            "unable to locate file extension from file path '{}'",
            path
        )),
    }
}
