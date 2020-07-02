use crate::ToRussRepr;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, LitStr,
};

// https://www.w3.org/TR/css-values-4

mod color;
mod dimensions;
mod functional;
mod image;
mod numeric;
mod position;
mod textual;

pub use numeric::{Integer, Number, Percentage};
pub use textual::{CssString, CustomIdent, Url, UrlModifier};

pub type Color = String;
pub type LengthPercentage = String;
pub type Image = String;

pub enum ValueDiscriminant {
    CustomIdent,
    String,
    Url,

    Integer,
    Number,
    Percentage,
}
impl ValueDiscriminant {
    fn peek_textual(input: ParseStream) -> Option<Self> {
        if Url::peek(input) {
            Some(Self::Url)
        } else if input.peek(Ident) {
            Some(Self::CustomIdent)
        } else if input.peek(LitStr) {
            Some(Self::String)
        } else {
            None
        }
    }

    fn peek_numeric(input: ParseStream) -> Option<Self> {
        // parsing these is cheap and dealing with the optional + would be a pain otherwise.
        if Integer::parse(&input.fork()).is_ok() {
            Some(Self::Integer)
        } else if Percentage::parse(&input.fork()).is_ok() {
            Some(Self::Percentage)
        } else if Number::parse(&input.fork()).is_ok() {
            Some(Self::Number)
        } else {
            None
        }
    }

    pub fn peek(input: ParseStream) -> syn::Result<Self> {
        if let Some(v) = Self::peek_textual(input) {
            Ok(v)
        } else if let Some(v) = Self::peek_numeric(input) {
            Ok(v)
        } else {
            Err(input.error("expected a value"))
        }
    }
}

pub struct Value(Box<dyn ToRussRepr>);
impl Value {
    fn new_box<T: 'static + ToRussRepr>(v: T) -> Self {
        Self(Box::new(v))
    }
}
impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match ValueDiscriminant::peek(input)? {
            ValueDiscriminant::CustomIdent => input.parse::<CustomIdent>().map(Self::new_box),
            ValueDiscriminant::String => input.parse::<CssString>().map(Self::new_box),
            _ => todo!(),
        }
    }
}
impl ToRussRepr for Value {
    fn to_russ_repr(&self) -> Expr {
        self.0.to_russ_repr()
    }
}
