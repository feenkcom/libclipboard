#![allow(non_snake_case)]

extern crate clipboard;
extern crate boxer;

use boxer::CBox;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use boxer::string::BoxerString;
use boxer::boxes::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn clipboard_test() -> bool {
    return true
}

#[no_mangle]
pub fn clipboard_create_clipboard_context() -> *mut ValueBox<ClipboardContext> {
    match ClipboardProvider::new() {
        Ok(clipboard_provider) => { ValueBox::new(clipboard_provider).into_raw() },
        Err(_) => { std::ptr::null_mut() },
    }
}

#[no_mangle]
pub fn clipboard_destroy_clipboard_context(_ptr: *mut ValueBox<ClipboardContext>) {
    _ptr.drop()
}

#[no_mangle]
pub fn clipboard_get_contents(_ptr_context: *mut ValueBox<ClipboardContext>, _ptr_contents: *mut BoxerString) {
    _ptr_context.with_not_null(|context| {
        CBox::with_raw(_ptr_contents, |contents| {
            let contents_string: String = match context.get_contents() {
                Ok(_string) => { _string },
                Err(e) => {
                    eprintln!("[Clipboard] Error while getting a content {:?}", e);
                    "".to_string() }
            };
            contents.set_string(contents_string)
        })
    });
}

#[no_mangle]
pub fn clipboard_set_contents(_ptr_context: *mut ValueBox<ClipboardContext>, _ptr_contents: *mut BoxerString) {
    _ptr_context.with_not_null(|context| {
        CBox::with_raw(_ptr_contents, |contents| {
            match context.set_contents(contents.to_string()) {
            Ok(_) => {},
            Err(e) => { eprintln!("[Clipboard] Error while setting a content {:?}", e) },
        }
        })
    });
}