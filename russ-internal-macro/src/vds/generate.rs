use super::helpers;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};
use syn::{Ident, Item, Type};

fn get_letter(n: u8) -> char {
    ('A' as u8 + (n % 26)) as char
}
pub fn get_letter_code(mut num: u16) -> String {
    let mut s = String::new();
    // TODO test this because it doesn't behave correctly for num >= 26
    loop {
        s.push(get_letter((num % 26) as u8));
        if num >= 26 {
            num -= 26;
        } else {
            break;
        }
    }
    s
}

#[derive(Debug)]
pub struct GenerateTypeContext {
    ident_hint: String,
    span: Span,
    used_idents: Rc<RefCell<HashSet<String>>>,
}
impl GenerateTypeContext {
    fn new(ident_hint: String, span: Span) -> Self {
        Self {
            ident_hint,
            span,
            used_idents: Rc::default(),
        }
    }

    pub fn from_ident(ident: Ident) -> Self {
        Self::new(ident.to_string(), ident.span())
    }

    pub fn empty() -> Self {
        Self::new(String::new(), Span::call_site())
    }

    pub fn clone_namespace(&self) -> Self {}

    fn track_ident(&self, ident: &Ident) -> bool {
        let mut used_ident = self.used_idents.borrow_mut();
        used_ident.insert(ident.to_string())
    }

    pub fn propose_ident(&self, name: &str) -> syn::Result<Ident> {
        // TODO this entire thing needs to be improved
        assert!(!name.is_empty());

        let ident =
            helpers::parse_ident_with_span(&format!("{}{}", self.ident_hint, name), self.span)?;
        Ok(if self.track_ident(&ident) {
            ident
        } else {
            self.generate_ident()
        })
    }
}

pub struct TypeDefinition {
    pub ident: Ident,
    pub definition: Item,
}
impl TypeDefinition {
    pub fn new(ident: Ident, definition: Item) -> Self {
        Self { ident, definition }
    }
}

pub struct TypeInfo {
    pub value_type: Type,
    pub definition: Option<TypeDefinition>,
    pub dependencies: Vec<TypeInfo>,
}
impl TypeInfo {
    pub fn new(value_type: Type) -> Self {
        Self {
            value_type,
            definition: None,
            dependencies: Vec::new(),
        }
    }
    pub fn with_definition(mut self, definition: TypeDefinition) -> Self {
        self.definition = Some(definition);
        self
    }
    pub fn with_dependencies(mut self, dependencies: Vec<TypeInfo>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn gen_definitions(&self) -> TokenStream {
        let definition = self.definition.as_ref().map(|def| &def.definition);
        let dep_defs_it = self.dependencies.iter().map(Self::gen_definitions);
        quote! {
            #definition
            #(#dep_defs_it)*
        }
    }
}

pub trait GenerateTypeInfo {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo>;
}
