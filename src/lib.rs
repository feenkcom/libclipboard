#![allow(non_snake_case)]

extern crate clipboard;
extern crate libc;
extern crate boxer;

use boxer::CBox;

use std::os::raw::c_char;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[no_mangle]
pub fn clipboard_test() -> bool {
    return true
}

#[no_mangle]
pub fn clipboard_create_clipboard_context() -> *mut ClipboardContext {
    CBox::into_raw(ClipboardProvider::new().unwrap())
}

#[no_mangle]
pub fn clipboard_destroy_clipboard_context(_ptr: *mut ClipboardContext) {
    CBox::from_raw(_ptr);
}

#[no_mangle]
pub fn clipboard_get_contents(_ptr_context: *mut ClipboardContext) -> *mut c_char {
    CBox::with_raw(_ptr_context, |context| {
        let contents_string: String = match context.get_contents() {
            Ok(_string) => { _string },
            Err(e) => {
                eprintln!("[Clipboard] Error while getting a content {:?}", e);
                "".to_string() }
        };
        CBox::to_chars(contents_string)
    })
}

#[no_mangle]
pub fn clipboard_free_contents(_ptr_contents: *mut c_char) {
    CBox::free_chars(_ptr_contents)
}

#[no_mangle]
pub fn clipboard_set_contents(_ptr_context: *mut ClipboardContext, _ptr_contents: *const c_char) {
    CBox::with_raw(_ptr_context, |context| {
        let contents_string = CBox::to_string(_ptr_contents);
        match context.set_contents(contents_string) {
            Ok(_) => {},
            Err(e) => { eprintln!("[Clipboard] Error while setting a content {:?}", e) },
        }
    });
}