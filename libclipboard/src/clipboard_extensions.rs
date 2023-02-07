use copypasta::{ClipboardContext, ClipboardProvider};
use phlow::PhlowView;

#[phlow::extensions(ClipboardExtensions, ClipboardContext)]
impl ClipboardContextExtensions {
    #[phlow::view]
    fn contents_for(_this: &ClipboardContext, view: impl PhlowView) -> impl PhlowView {
        view.text()
            .title("Contents")
            .text_mut::<ClipboardContext>(|mut context| {
                context.get_contents().unwrap_or("".to_string())
            })
    }
}
