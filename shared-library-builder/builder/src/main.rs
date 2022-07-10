use std::error::Error;

use shared_library_builder::build_standalone;

use libclipboard_library::latest_libclipboard;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libclipboard())))
}
