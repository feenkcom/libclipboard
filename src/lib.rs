#![allow(non_snake_case)]

extern crate clipboard;
extern crate boxer;

use boxer::CBox;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use boxer::string::BoxerString;

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
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn clipboard_get_contents(_ptr_context: *mut ClipboardContext, _ptr_contents: *mut BoxerString) {
    CBox::with_raw(_ptr_context, |context| {
        let contents_string: String = match context.get_contents() {
            Ok(_string) => { _string },
            Err(e) => {
                eprintln!("[Clipboard] Error while getting a content {:?}", e);
                "".to_string() }
        };
        CBox::with_raw(_ptr_contents, |contents| contents.set_string(contents_string))
    })
}

#[no_mangle]
pub fn clipboard_set_contents(_ptr_context: *mut ClipboardContext, _ptr_contents: *mut BoxerString) {
    CBox::with_two_raw(_ptr_context, _ptr_contents,|context, contents| {
        match context.set_contents(contents.to_string()) {
            Ok(_) => {},
            Err(e) => { eprintln!("[Clipboard] Error while setting a content {:?}", e) },
        }
    });
}