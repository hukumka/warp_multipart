extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Error, Data, DataStruct, Ident, Type, Attribute, spanned::Spanned};
use quote::{quote};

#[proc_macro_derive(FromMultipart, attributes(default))]
pub fn derive_from_part(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn impl_(input: &DeriveInput) -> Result<TokenStream2, Error> {
    if let Data::Struct(data) = &input.data {
        let ident = &input.ident;
        let (names, types, is_default) = fields(data)?;
        Ok(generate_code(ident, &names, &types, &is_default))
    } else {
        Err(Error::new(input.span(), "Expected struct."))
    }
}

fn generate_code(ident: &Ident, names: &[Ident], types: &[Type], is_default: &[bool]) -> TokenStream2 {
    let names_str: Vec<String> = names.iter().map(|n| n.to_string()).collect();
    let reassignments: Vec<TokenStream2> = names.iter()
        .zip(is_default)
        .map(|(ident, is_default)| {
            let s = ident.to_string();
            let offpath = if *is_default {
                quote! {Default::default()}
            } else {
                quote! {return Err(Error::MissingField(#s.to_string()));}
            };
            quote!{
                let #ident = if let Some(x) = #ident {
                    x
                } else {
                    #offpath
                };
            }
        })
        .collect();
    quote!{
        use warp_multipart::derive_imports::async_trait;
        #[async_trait]
        impl warp_multipart::FromMultipart for #ident {
            async fn from_multipart(mut body: warp_multipart::derive_imports::FormData) -> Result<Self, warp_multipart::Error> {
                use warp_multipart::derive_imports::*;
                #(
                let mut #names: Option<#types> = None;
                )*
                while let Some(part) = body.next().await {
                    let part = part?;
                    match part.name() {
                        #(#names_str => {
                            #names = Some(<#types as FromPart>::from_part(part).await?);
                        })*,
                        _ => {}
                    }
                }
                #(#reassignments)*
                Ok(Self{
                    #(#names),*
                })
            }
        }
    }
}

fn fields(data: &DataStruct) -> Result<(Vec<Ident>, Vec<Type>, Vec<bool>), Error> {
    let iter = data.fields.iter();
    let hint = iter.size_hint().0; 
    let mut names = Vec::with_capacity(hint);
    let mut types = Vec::with_capacity(hint);
    let mut default = Vec::with_capacity(hint);
    for field in iter {
        if let Some(ident) = &field.ident {
            names.push(ident.clone());
            types.push(field.ty.clone());
            default.push(is_default(&field.attrs));
        } else {
            return Err(Error::new(field.span(), "Expected struct with named fields."));
        }
    }
    Ok((names, types, default))
}

fn is_default(attrs: &[Attribute]) -> bool {
    attrs.iter()
        .any(|a| {
            let path = &a.path;
            quote!(#path).to_string() == quote!(default).to_string()
        })
}