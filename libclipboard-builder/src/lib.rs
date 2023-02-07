use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libclipboard(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Clipboard",
        LibraryLocation::Git(
            GitLocation::github("feenkcom", "libclipboard").tag_or_latest(version),
        ),
    )
    .package("libclipboard")
}

pub fn latest_libclipboard() -> RustLibrary {
    let version: Option<String> = None;
    libclipboard(version)
}
