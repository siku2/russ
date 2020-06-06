pub mod bindings;
pub mod css;
mod styles;

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use russ_macro::static_css;

/// A reference to a style sheet.
/// When dropped, the style sheet is removed.
#[derive(Debug, PartialEq, Hash, Eq)]
struct StyleSheetRef(String);

impl StyleSheetRef {
    fn create(id: &str, body: &str) -> Option<Self> {
        if bindings::add_style_sheet(id, body) {
            Some(Self(String::from(id)))
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Drop for StyleSheetRef {
    fn drop(&mut self) {
        let removed = bindings::remove_style_sheet(self.id());
        assert!(removed, "style sheet was removed outside of russ");
    }
}
