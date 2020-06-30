use super::bindings;
use russ_internal::{CSSWriter, WriteDeclaration, WriteResult};
use std::{
    borrow::Cow,
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

trait DeclarationInner: WriteDeclaration {
    fn box_clone(&self) -> Box<dyn DeclarationInner>;
    fn debug_fmt(&self, f: &mut Formatter) -> fmt::Result;
    fn generate_key(&self) -> CSSKey;
}
impl Clone for Box<dyn DeclarationInner> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
impl Debug for Box<dyn DeclarationInner> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.debug_fmt(f)
    }
}
impl Hash for Box<dyn DeclarationInner> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.generate_key().hash(state)
    }
}

impl<T> DeclarationInner for T
where
    T: 'static + WriteDeclaration + Clone + Debug + Hash,
{
    fn box_clone(&self) -> Box<dyn DeclarationInner> {
        Box::new(self.clone())
    }

    fn debug_fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt(f)
    }

    fn generate_key(&self) -> CSSKey {
        CSSKey::new_hash(self)
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Declaration(Box<dyn DeclarationInner>);
impl Declaration {
    pub fn write_declaration(&self, f: &mut CSSWriter) -> WriteResult {
        self.0.write_declaration(f)
    }
}
impl<T> From<T> for Declaration
where
    T: 'static + WriteDeclaration + Clone + Debug + Hash,
{
    fn from(v: T) -> Self {
        Self(Box::new(v))
    }
}

#[derive(Clone, Debug, Hash)]
pub struct DeclarationBlock(Vec<Declaration>);
impl DeclarationBlock {
    pub fn build<D: Into<Declaration>>(declarations: impl IntoIterator<Item = D>) -> Self {
        Self(declarations.into_iter().map(Into::into).collect())
    }

    pub fn write_block(&self, f: &mut CSSWriter) -> WriteResult {
        f.write_char('{')?;
        for decl in &self.0 {
            decl.write_declaration(f)?;
            f.write_char(';')?;
        }
        f.write_char('}')
    }

    pub fn write_block_with_selector(&self, f: &mut CSSWriter, selector: &str) -> WriteResult {
        f.write_str(selector)?;
        self.write_block(f)
    }
}

#[derive(Clone, Debug, Hash)]
pub struct RuleSet {
    pub block: DeclarationBlock,
}
impl RuleSet {
    pub fn build<D: Into<Declaration>>(declarations: impl IntoIterator<Item = D>) -> Self {
        Self {
            block: DeclarationBlock::build(declarations),
        }
    }

    pub fn write_rule_set(&self, f: &mut CSSWriter, class_id: impl Display) -> WriteResult {
        self.block
            .write_block_with_selector(f, &format!(".{}", class_id))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CSSKey(u64);
impl CSSKey {
    pub fn new_hash<T: Hash>(v: T) -> Self {
        let mut h = DefaultHasher::new();
        v.hash(&mut h);
        Self(h.finish())
    }

    pub fn unique_id(self) -> String {
        format!("{:x}", self.0)
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Styles {
    rule_sets: Vec<RuleSet>,
}
impl Styles {
    pub fn build(rule_sets: impl IntoIterator<Item = RuleSet>) -> Self {
        Self {
            rule_sets: rule_sets.into_iter().map(Into::into).collect(),
        }
    }

    pub fn generate_key(&self) -> CSSKey {
        CSSKey::new_hash(&self.rule_sets)
    }

    pub fn write_css(&self, f: &mut CSSWriter, prefix: impl Display) -> WriteResult {
        for (i, rule_set) in self.rule_sets.iter().enumerate() {
            rule_set.write_rule_set(f, format!("{}-{}", prefix, i))?;
        }
        Ok(())
    }
}

/// A reference to a style sheet.
/// When dropped, the style sheet is removed from the document.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StyleSheet(String);
impl StyleSheet {
    #[must_use = "style sheet is removed when this is dropped"]
    pub fn attach(id: Cow<str>, body: &str) -> Option<Self> {
        if bindings::add_style_sheet(&id, body) {
            Some(Self(id.into_owned()))
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}
impl Drop for StyleSheet {
    fn drop(&mut self) {
        let removed = bindings::remove_style_sheet(self.id());
        debug_assert!(
            removed,
            "style sheet was removed but it still has a reference pointing to it"
        );
    }
}

pub type StyleSheetRef = Rc<StyleSheet>;

#[derive(Clone, Debug, Default)]
pub struct StyleManager {
    sheets: HashMap<CSSKey, Weak<StyleSheet>>,
}
impl StyleManager {
    fn get(&self, key: CSSKey) -> Option<StyleSheetRef> {
        self.sheets.get(&key).and_then(Weak::upgrade)
    }

    fn track_sheet(&mut self, key: CSSKey, sheet_ref: StyleSheet) -> StyleSheetRef {
        let shared_ref = Rc::new(sheet_ref);
        self.sheets.insert(key, Rc::downgrade(&shared_ref));
        shared_ref
    }

    fn add_styles_with_key(&mut self, key: CSSKey, styles: &Styles) -> StyleSheetRef {
        let unique_id = key.unique_id();
        let mut body_buf = Vec::new();
        styles
            .write_css(&mut CSSWriter::new(&mut body_buf), &unique_id)
            .expect("failed to render CSS");
        // SAFETY: CSSWriter should never produce invalid UTF8.
        //  On the off chance that it does, it will be handled by `TextDecoder` in JavaScript and generate a panic.
        let body = unsafe { String::from_utf8_unchecked(body_buf) };
        let style_sheet =
            StyleSheet::attach(Cow::from(unique_id), &body).expect("failed to add style sheet");
        self.track_sheet(key, style_sheet)
    }

    pub fn track_styles_with_key(&mut self, key: CSSKey, styles: &Styles) -> StyleSheetRef {
        self.get(key)
            .unwrap_or_else(|| self.add_styles_with_key(key, styles))
    }

    pub fn track_styles(&mut self, styles: &Styles) -> StyleSheetRef {
        self.track_styles_with_key(styles.generate_key(), styles)
    }
}
