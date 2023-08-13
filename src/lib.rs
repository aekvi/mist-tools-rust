pub mod envelope;
pub mod mime_types;
pub mod mist;

pub use envelope::Envelope;
pub use mime_types::MimeType;
pub use mist::{
    post_event_to_rapids, post_str_to_rapids, post_to_rapids, reply_file_to_origin,
    reply_file_to_origin_with_content_type, reply_str_to_origin, reply_to_origin,
};

#[macro_export]
macro_rules! mist_service {
    ( { actions: { $( $action:literal : $handler:ident ) , * } $(, init: $init:ident )? } ) => {
        {
            use mist_tools::mist::{get_args, get_payload};
            let (arg_action, envelope) = get_args()?;
            match arg_action.as_str() {
            $(
                $action => $handler(get_payload()?, envelope)?,
            )*
                _ => ()
            };
            $(
                let init_result = $init();
                init_result?;
                Ok(())
            )?
        }
    };
}
