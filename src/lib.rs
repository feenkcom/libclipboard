#![allow(non_snake_case)]

extern crate clipboard;
extern crate libc;

pub mod cstruct;

use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use cstruct::*;

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
        let contents_string = context.get_contents().unwrap();
        let c_to_print = CString::new(contents_string).expect("CString::new failed");
        c_to_print.into_raw()
    })
}

#[no_mangle]
pub fn clipboard_free_contents(_ptr_contents: *mut c_char) {
    unsafe { CString::from_raw(_ptr_contents) };
}

#[no_mangle]
pub fn clipboard_set_contents(_ptr_context: *mut ClipboardContext, _ptr_contents: *const c_char) {
    CBox::with_raw(_ptr_context, |context| {
        let contents_string = unsafe {
            CStr::from_ptr(_ptr_contents).to_string_lossy().into_owned()
        };
        match context.set_contents(contents_string) {
            Ok(_) => {},
            Err(e) => { eprintln!("[Clipboard] Error while setting a content {:?}", e) },
        }
    });
}