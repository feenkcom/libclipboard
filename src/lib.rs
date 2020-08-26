#![allow(non_snake_case)]
extern crate boxer;
extern crate clipboard;

use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::BoxerString;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

#[no_mangle]
pub fn clipboard_test() -> bool {
    return true;
}

#[no_mangle]
pub fn clipboard_create_clipboard_context() -> *mut ValueBox<ClipboardContext> {
    match ClipboardProvider::new() {
        Ok(clipboard_provider) => ValueBox::new(clipboard_provider).into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub fn clipboard_destroy_clipboard_context(ptr: *mut ValueBox<ClipboardContext>) {
    ptr.drop()
}

#[no_mangle]
pub fn clipboard_get_contents(
    context_ptr: *mut ValueBox<ClipboardContext>,
    contents_ptr: *mut ValueBox<BoxerString>,
) {
    context_ptr.with_not_null(|context| {
        contents_ptr.with_not_null(|contents| {
            let contents_string: String = match context.get_contents() {
                Ok(string) => string,
                Err(e) => {
                    eprintln!("[Clipboard] Error while getting a content {:?}", e);
                    "".to_string()
                }
            };
            contents.set_string(contents_string)
        })
    });
}

#[no_mangle]
pub fn clipboard_set_contents(
    context_ptr: *mut ValueBox<ClipboardContext>,
    contents_ptr: *mut ValueBox<BoxerString>,
) {
    context_ptr.with_not_null(|context| {
        contents_ptr.with_not_null(
            |contents| match context.set_contents(contents.to_string()) {
                Ok(_) => {}
                Err(e) => eprintln!("[Clipboard] Error while setting a content {:?}", e),
            },
        )
    });
}
