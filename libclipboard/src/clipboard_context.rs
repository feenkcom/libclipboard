use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn clipboard_create_clipboard_context() -> *mut ValueBox<ClipboardContext> {
    match ClipboardContext::new() {
        Ok(clipboard_provider) => value_box!(clipboard_provider).into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub fn clipboard_destroy_clipboard_context(context: *mut ValueBox<ClipboardContext>) {
    context.release();
}

#[no_mangle]
pub fn clipboard_get_contents(
    context: *mut ValueBox<ClipboardContext>,
    contents: *mut ValueBox<StringBox>,
) {
    contents
        .with_mut(|contents| {
            context
                .with_mut(|context| {
                    context
                        .get_contents()
                        .map_err(|error| BoxerError::AnyError(error))
                })
                .map(|string| contents.set_string(string))
        })
        .log();
}

#[no_mangle]
pub fn clipboard_set_contents(
    context: *mut ValueBox<ClipboardContext>,
    contents: *mut ValueBox<StringBox>,
) {
    contents
        .with_ref(|contents| {
            context.with_mut(|context| {
                context
                    .set_contents(contents.to_string())
                    .map_err(|error| BoxerError::AnyError(error))
            })
        })
        .log();
}
