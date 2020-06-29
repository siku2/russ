use super::helpers;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{Ident, Item, Type};

// TODO consider forcing the developer to add names to constructs without a name instead of generating them.

#[derive(Debug)]
pub struct GenerateTypeContext {
    ident_hint: String,
    is_namespace: bool,
    span: Span,
}
impl GenerateTypeContext {
    fn new(ident_hint: String, span: Span, is_namespace: bool) -> Self {
        Self {
            ident_hint,
            span,
            is_namespace,
        }
    }

    pub fn from_ident(ident: Ident, is_namespace: bool) -> Self {
        Self::new(ident.to_string(), ident.span(), is_namespace)
    }

    pub fn empty() -> Self {
        Self::new(String::new(), Span::call_site(), true)
    }

    pub fn get_ident(&self) -> Option<Ident> {
        if self.is_namespace {
            None
        } else {
            helpers::parse_ident_with_span(&self.ident_hint, self.span).ok()
        }
    }

    pub fn create_ident(&self, ident: &Ident) -> syn::Result<Ident> {
        helpers::parse_ident_with_span(&format!("{}{}", self.ident_hint, ident), ident.span())
    }

    pub fn fork(&self, ident: &Ident) -> syn::Result<Self> {
        Ok(Self::from_ident(self.create_ident(ident)?, false))
    }

    pub fn fork_index(&self, index: usize) -> syn::Result<Self> {
        self.fork(&format_ident!("V{:X}", index, span = self.span))
    }

    pub fn fork_namespace(&self, ident: &Ident) -> syn::Result<(Ident, Self)> {
        let ident = self.create_ident(ident)?;
        Ok((ident.clone(), Self::from_ident(ident, true)))
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
    pub name: Option<Ident>,
    pub definition: Option<TypeDefinition>,
    pub dependencies: Vec<TypeInfo>,
}
impl TypeInfo {
    pub fn new(value_type: Type) -> Self {
        Self {
            value_type,
            name: None,
            definition: None,
            dependencies: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: Ident) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_definition(mut self, definition: TypeDefinition) -> Self {
        self.definition = Some(definition);
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<TypeInfo>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn value_type_unwrap_tuple(&self) -> TokenStream {
        use quote::ToTokens;
        match &self.value_type {
            Type::Tuple(v) => v.elems.to_token_stream(),
            v => quote! { #v },
        }
    }

    pub fn get_name(&self) -> Option<Ident> {
        if let Some(ref name) = self.name {
            return Some(name.clone());
        }

        if let [first] = self.dependencies.as_slice() {
            if let Some(ident) = first.get_name() {
                return Some(ident);
            }
        }

        None
    }

    fn collect_infos(&self) -> Vec<&TypeInfo> {
        let mut infos = vec![self];
        for dep in &self.dependencies {
            infos.extend(dep.collect_infos());
        }
        infos
    }

    fn iter_definitions(&self) -> impl Iterator<Item = &TypeDefinition> {
        self.collect_infos()
            .into_iter()
            .filter_map(|info| info.definition.as_ref())
    }

    pub fn gen_definitions(&self) -> TokenStream {
        let mut defined = HashSet::new();
        let defs_it = self
            .iter_definitions()
            .filter(|dep| defined.insert(dep.ident.clone()))
            .map(|dep| &dep.definition);
        quote! {
            #(#defs_it)*
        }
    }
}

pub trait GenerateTypeInfo {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo>;
}
