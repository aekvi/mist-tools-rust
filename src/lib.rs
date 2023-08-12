pub mod envelope;
pub mod mime_types;
pub mod mist;

pub use envelope::Envelope;
pub use mime_types::MimeType;
pub use mist::{
    mist_service, post_event_to_rapids, post_str_to_rapids, post_to_rapids, reply_file_to_origin,
    reply_file_to_origin_with_content_type, reply_str_to_origin, reply_to_origin,
};

#[macro_export]
macro_rules! mist_service {
    ( { actions: { $( $a:literal : $h:ident ) , * } $(, init: $i:ident )? } ) => {
        {
            let mut v: Vec<(&'static str, Box<dyn FnOnce(Vec<u8>, Envelope) -> Result<(), String>>)> = Vec::new();
            $(
                v.push(($a, Box::new($h)));
            )*
            match true {
            $(
                true => mist_service(v, $i).unwrap(),
            )?
                _ => mist_service(v, || Ok(())).unwrap()
            }
        }
    };
}
