use super::args::{self, Args, FromArgs};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, spanned::Spanned, Attribute, Data, DeriveInput, Fields, Generics, Ident, Variant,
};

struct FromVariantAttr {
    attr: Attribute,
    into: bool,
}
impl FromArgs for FromVariantAttr {
    fn attr_path() -> &'static str {
        "from_variant"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            into: args.has_flag("into"),
        })
    }
}
impl ToTokens for FromVariantAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

fn generate_from_variant(
    input: &DeriveInput,
    type_ident: &Ident,
    variant: &Variant,
) -> syn::Result<TokenStream> {
    let mut generics: Generics = input.generics.clone();

    let variant_ident = &variant.ident;

    let arg: Option<FromVariantAttr> = args::parse_single_from_attrs(&variant.attrs).transpose()?;
    let gen_into = arg.map(|arg| arg.into).unwrap_or_default();

    let mut types = Vec::new();
    let mut bind_idents = Vec::new();
    let mut use_idents = Vec::new();
    for (i, field) in variant.fields.iter().enumerate() {
        let ty = &field.ty;
        if gen_into {
            let ty_n = Ident::new(&format!("T{}", i), ty.span());
            types.push(ty_n.to_token_stream());

            let clause = generics.make_where_clause();
            clause
                .predicates
                .push_value(parse_quote! { #ty_n: ::std::convert::Into<#ty> });
        } else {
            types.push(ty.to_token_stream());
        }

        let bind_ident = field
            .ident
            .clone()
            .unwrap_or_else(|| Ident::new(&format!("v{}", i), field.span()));
        use_idents.push(if gen_into {
            quote! { #bind_ident.into() }
        } else {
            bind_ident.to_token_stream()
        });
        bind_idents.push(bind_ident);
    }

    let from_types = quote! { (#(#types),*) };
    let mut impl_types: Vec<_> = generics
        .params
        .iter()
        .map(|param| param.to_token_stream())
        .collect();
    if gen_into {
        impl_types.extend(types);
    }

    Ok(quote! {
        impl<#(#impl_types),*> ::std::convert::From<#from_types> for #type_ident#generics {
            fn from((#(#bind_idents),*): #from_types) -> Self {
                Self::#variant_ident(#(#use_idents),*)
            }
        }
    })
}

pub fn generate_from_variants(input: DeriveInput) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    Ok(match &input.data {
        Data::Enum(data) => data
            .variants
            .iter()
            .filter_map(|variant| {
                if matches!(variant.fields, Fields::Unnamed(_)) {
                    Some(generate_from_variant(&input, type_ident, variant))
                } else {
                    None
                }
            })
            .collect::<Result<TokenStream, _>>()?,
        _ => return Err(syn::Error::new_spanned(input, "only enums are supported")),
    })
}
