use proc_macro::TokenStream;
use syn::{Data, Fields,parse_macro_input, DeriveInput};
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_derive(Parse)]
pub fn derive_parse_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data_enum  = match input.data {
        Data::Enum(ref data_enum) => {
            data_enum
        }
        Data::Union(_) | Data::Struct(_)=> {
            return syn::Error::new(input.span(),"expected enum: parsing variants only works with enumerations").to_compile_error().into();
        }
    };

    let enum_ident = &input.ident;


    let mut parse_body = quote!{};

    if data_enum.variants.is_empty() {
        return syn::Error::new(input.span().join(input.ident.span()).unwrap_or(input.ident.span()),"no variants: enumeration must have at least one variant").to_compile_error().into();
    }

    for variant in data_enum.variants.iter() {

        let variant_name = &variant.ident;
        let try_parse_variant_helper_func_ident = quote::format_ident!("try_parse_{}",variant_name);
        let try_parse_variant_helper_func_impl = match variant.fields {
            Fields::Named(ref fields_named) => {
                let fields: Vec<_> = fields_named.named.iter().map(|field|field.ident.as_ref().unwrap()).collect();
                quote! {
                    fn #try_parse_variant_helper_func_ident(input : & ::syn::parse::ParseBuffer) -> Result<#enum_ident,::syn::Error> {
                        Ok(#enum_ident::#variant_name {#(#fields : input.parse()?),*})
                    }
                }
            }
            Fields::Unnamed(ref fields_unnamed) => {
                let input_parse_questionmark = (quote! {input.parse()?});
                let repeated_input_parsing  = std::iter::repeat(input_parse_questionmark).take(fields_unnamed.unnamed.len());
                quote! {
                    fn #try_parse_variant_helper_func_ident(input : & ::syn::parse::ParseBuffer) -> Result<#enum_ident,::syn::Error> {
                        Ok(#enum_ident::#variant_name (#(#repeated_input_parsing),*))
                    }
                }
            }
            Fields::Unit => {
                return syn::Error::new(variant.ident.span(),"illegal variant: enumeration may not have unit variants").to_compile_error().into();
            }
        };

        parse_body.extend(quote! {
            let fork = input.fork();
            #[allow(non_snake_case)]
            #try_parse_variant_helper_func_impl
            if let Ok(variant) = #try_parse_variant_helper_func_ident(&fork) {
                input.advance_to(&fork);
                return Ok(variant);
            }
        })
    }

    let parse_impl_tokens = quote! {
        impl ::syn::parse::Parse for #enum_ident {
            fn parse(input : & ::syn::parse::ParseBuffer)->Result<Self, ::syn::Error> {
                use ::syn::parse::discouraged::Speculative;
                #parse_body
                Err(syn::Error::new(input.span(),"expected integer literal or identifier"))
            }
        }
    };
    parse_impl_tokens.into()
}

