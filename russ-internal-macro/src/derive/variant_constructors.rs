use super::args::{self, Args, FromArgs};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, Data, DeriveInput, Fields, Ident, LitStr, Variant};

struct VariantConstructorAttr {
    attr: Attribute,
    skip: bool,
    name: Option<LitStr>,
}
impl FromArgs for VariantConstructorAttr {
    fn attr_path() -> &'static str {
        "constructor"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            skip: args.has_flag("skip"),
            name: args.get_kwarg_str("name")?,
        })
    }
}
impl ToTokens for VariantConstructorAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

fn generate_variant_constructor_fn(variant: &Variant) -> syn::Result<TokenStream> {
    let arg: Option<VariantConstructorAttr> =
        args::parse_single_from_attrs(&variant.attrs).transpose()?;

    if arg.as_ref().map(|arg| arg.skip).unwrap_or_default() {
        return Ok(quote! {});
    }

    let variant_ident_str = arg
        .and_then(|arg| arg.name)
        .map_or_else(|| variant.ident.to_string(), |name| name.value());
    let variant_ident: Ident = syn::parse_str(&variant_ident_str)?;
    let fn_ident = Ident::new(
        &variant_ident.to_string().to_snake_case(),
        variant_ident.span(),
    );

    let is_named = matches!(variant.fields, Fields::Named(_));

    let mut params = Vec::new();
    let mut values = Vec::new();
    for (i, field) in variant.fields.iter().enumerate() {
        let param_ident_str = field
            .ident
            .as_ref()
            .map_or_else(|| format!("v{}", i), ToString::to_string);
        syn::parse_str::<Ident>(&param_ident_str)?;

        let param_ident = Ident::new(&param_ident_str, field.span());

        let ty = &field.ty;
        params.push(quote! { #param_ident: impl Into<#ty> });

        values.push(if is_named {
            quote! { #param_ident: #param_ident.into() }
        } else {
            quote! { #param_ident.into() }
        });
    }

    let variant_body = if is_named {
        quote! { {#(#values),*} }
    } else {
        quote! { (#(#values),*) }
    };

    Ok(quote! {
        pub fn #fn_ident(#(#params),*) -> Self {
            Self::#variant_ident#variant_body
        }
    })
}

pub fn generate_variant_constructors(input: DeriveInput) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    Ok(match input.data {
        Data::Enum(data) => {
            let functions = data
                .variants
                .iter()
                .filter_map(|variant| {
                    if matches!(variant.fields, Fields::Unit) {
                        None
                    } else {
                        Some(generate_variant_constructor_fn(variant))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

            quote! {
                impl #type_ident {
                    #(#functions)*
                }
            }
        }
        _ => return Err(syn::Error::new_spanned(input, "only enums are supported")),
    })
}
