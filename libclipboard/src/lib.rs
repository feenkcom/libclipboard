#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_use]
extern crate phlow;
extern crate phlow_extensions;
extern crate phlow_ffi;
#[macro_use]
extern crate value_box;
extern crate value_box_ffi;

use phlow_extensions::CoreExtensions;
// Re-export everything from the `value_box_ffi` and `phlow_ffi` in order to tell Rust to include
// the corresponding `no_mangle` functions.
pub use phlow_ffi::*;
pub use value_box_ffi::*;

mod clipboard_context;
mod clipboard_extensions;

define_extensions!(ClipboardExtensions);
import_extensions!(CoreExtensions, ClipboardExtensions);

#[no_mangle]
pub fn clipboard_test() -> bool {
    return true;
}
