mod bindings;

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use russ_macro::static_css;

use std::collections::HashSet;
use std::fmt::{self, Formatter};

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RuleSet {
    name: &'static str,
    declaration_block: &'static str,
}

impl RuleSet {
    pub const fn _new_from_macro(name: &'static str, declaration_block: &'static str) -> Self {
        Self {
            name,
            declaration_block,
        }
    }

    fn render(&self, formatter: &mut Formatter, selector: &str) -> fmt::Result {
        write!(formatter, "{}{}", selector, self.declaration_block)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Styles {
    rules: HashSet<RuleSet>,
}

impl Styles {}
