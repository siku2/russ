use super::helpers;
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashSet;
use syn::{Ident, Item, Type};

#[derive(Clone, Debug)]
pub struct GenerateTypeContext {
    ident_hint: String,
    span: Span,
}
impl GenerateTypeContext {
    fn new(ident_hint: String, span: Span) -> Self {
        Self { ident_hint, span }
    }

    pub fn from_ident(ident: Ident) -> Self {
        Self::new(ident.to_string(), ident.span())
    }

    pub fn empty() -> Self {
        Self::new(String::new(), Span::call_site())
    }

    pub fn namespace(&self, name: &str) -> syn::Result<(Ident, Self)> {
        let ident = helpers::parse_ident_with_span(
            &format!("{}{}", self.ident_hint, name.to_camel_case()),
            self.span,
        )?;
        Ok((ident.clone(), Self::from_ident(ident)))
    }

    pub fn namespace_ident(&self, ident: &Ident) -> syn::Result<(Ident, Self)> {
        let ident =
            helpers::parse_ident_with_span(&format!("{}{}", self.ident_hint, ident), ident.span())?;
        Ok((ident.clone(), Self::from_ident(ident)))
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

    pub fn get_type_ident(&self) -> Option<Ident> {
        if let Some(ref def) = self.definition {
            return Some(def.ident.clone());
        }

        match self.dependencies.as_slice() {
            [first] => {
                if let Some(ident) = first.get_type_ident() {
                    return Some(ident);
                }
            }
            _ => {}
        }

        match &self.value_type {
            Type::Path(p) => {
                if let Some(seg) = p.path.segments.last() {
                    return Some(seg.ident.clone());
                }
            }
            _ => {}
        }

        None
    }

    pub fn gen_definitions(&self) -> TokenStream {
        let mut defined = HashSet::new();

        let definition = self.definition.as_ref().map(|def| {
            defined.insert(def.ident.clone());
            &def.definition
        });
        let dep_defs_it = self
            .dependencies
            .iter()
            .filter(|dep| {
                if let Some(ref def) = dep.definition {
                    defined.insert(def.ident.clone())
                } else {
                    false
                }
            })
            .map(Self::gen_definitions);
        quote! {
            #definition
            #(#dep_defs_it)*
        }
    }
}

pub trait GenerateTypeInfo {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo>;
}
