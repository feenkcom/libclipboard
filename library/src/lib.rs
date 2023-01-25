#![allow(non_snake_case)]

#[macro_use]
extern crate phlow;
extern crate phlow_extensions;
extern crate phlow_ffi;
extern crate value_box_ffi;

use phlow_extensions::CoreExtensions;
// Re-export everything from the `value_box_ffi` and `phlow_ffi` in order to tell Rust to include
// the corresponding `no_mangle` functions.
pub use phlow_ffi::*;
pub use value_box_ffi::*;

mod clipboard_context;

import_extensions!(CoreExtensions);

#[no_mangle]
pub fn clipboard_test() -> bool {
    return true;
}
