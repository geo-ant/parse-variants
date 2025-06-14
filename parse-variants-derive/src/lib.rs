use proc_macro::TokenStream;

use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

/*
 See the documentation in the parse-variants crate on how to use the macro.
 Here is a quick example for how the generated code looks.
 We apply the macro to this enum:

     #[derive(crate::Parse)]
     enum EnumWithMixedVariants {
        TwoExpressionsSeparatedByKeyword {
            first: syn::Expr,
            _the_dude: keywords::lebowski,
            second: syn::Expr,
        },
        IdentifierPlusPlus(Ident, syn::token::Add,syn::token::Add),
     }

 Then the generate syn::parse::Parse implementation looks like this:

    impl ::syn::parse::Parse for EnumWithMixedVariants {
        fn parse(input: &::syn::parse::ParseBuffer) -> ::std::result::Result<Self, ::syn::Error> {
            use ::syn::parse::discouraged::Speculative; //needed for input.advance_to(...)
            // 1) fork input for first variant and try if we can parse it
            let fork = input.fork();
            if let Ok(variant) = (|| {   // here we use a closure to return a result or error without returning the error directly from our parse function
                // this is how parsing named fields looks like
                Ok(EnumWithMixedVariants::TwoExpressionsSeparatedByKeyword {
                    first: fork.parse()?,
                    _the_dude: fork.parse()?,
                    second: fork.parse()?,
                }) as ::std::result::Result<EnumWithMixedVariants, ::syn::Error>
            })() {
                // if we can parse the variant, advance the parsebuffer and return immediately
                input.advance_to(&fork);
                return Ok(variant);
            }
            // 2) fork the second variant
            let fork = input.fork();
            if let Ok(variant) = (|| {   // same trick with the closure as above to catch the error returns
                // this is how parsing named fields looks like
                Ok(EnumWithMixedVariants::IdentifierPlusPlus(
                    fork.parse()?,
                    fork.parse()?,
                    fork.parse()?,
                )) as ::std::result::Result<EnumWithMixedVariants, ::syn::Error>
            })() {
                input.advance_to(&fork);
                return Ok(variant);
            }
            // if no variants can be parsed, return an error
            Err(syn::Error::new(
                input.span(),
                ::std::format! {
                    "parse error: tokens cannot be parsed as any variant of {}",
                    ::std::stringify! { EnumWithMixedVariants }
                },
            ))
        }
    }
*/

#[proc_macro_derive(Parse)]
pub fn derive_parse_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;
    let data_enum = match get_data_enum(&input) {
        Ok(data_enum) => data_enum,
        Err(error) => {
            return error;
        }
    };

    // here we generate the code that tries to parse the actual variants by repeatedly forking
    // the input parse buffer and then trying to parse the input as the contents of the respective
    // variant.
    let mut try_parse_variants = proc_macro2::TokenStream::new();

    for variant in data_enum.variants.iter() {
        let variant_name = &variant.ident;
        let try_parse_variant = match variant.fields {
            Fields::Named(ref fields_named) => {
                let fields: Vec<_> = fields_named
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap())
                    .collect();
                // generated code looks e.g. like this
                // Ok(MyEnum::StructLikeVariant{field1 : fork.parse()?, field2 : fork.parse()?})
                quote! {
                        Ok(#enum_ident::#variant_name {#(#fields : fork.parse()?),*})
                }
            }
            Fields::Unnamed(ref fields_unnamed) => {
                let fork_parse_questionmark = quote! {fork.parse()?};
                let repeated_input_parsing =
                    std::iter::repeat_n(fork_parse_questionmark, fields_unnamed.unnamed.len());
                // code looks e.g. like this
                // Ok(MyEnum::TupleLikeVariant(fork.parse()?,fork.parse()?))
                // where fork.parse()? is repeated for each field of the tuple like variant
                quote! {
                    Ok(#enum_ident::#variant_name (#(#repeated_input_parsing),*))
                }
            }
            Fields::Unit => {
                // unit like variants (i.e. variants with no fields)
                // cannot be parsed and will return a compile error
                return syn::Error::new(
                    variant.ident.span(),
                    "illegal unit variant: enumeration may not have variants without fields",
                )
                .to_compile_error()
                .into();
            }
        };

        try_parse_variants.extend(quote! {
            let fork = input.fork();
            if let Ok(variant) = (||{#try_parse_variant as ::std::result::Result<#enum_ident,::syn::Error>})() { //TODO: document this: less verbose variant than before. So that the error type can be caught at the boundary of this closure and does not propagate outside of the parse function
                input.advance_to(&fork);
                return Ok(variant);
            }
        })
    }

    // the implementation of the derive trait
    let parse_impl_tokens = quote! {
        impl ::syn::parse::Parse for #enum_ident {
            fn parse(input : & ::syn::parse::ParseBuffer) -> ::std::result::Result<Self, ::syn::Error> {
                // we have to use this for the advante_to method in the parsing body
                use ::syn::parse::discouraged::Speculative;
                // parsing the variants
                #try_parse_variants
                // if none of the variants can be parsed, return an error
                Err(syn::Error::new(input.span(),::std::format!{"parse error: tokens cannot be parsed as any variant of {}", ::std::stringify!{#enum_ident}}))
            }
        }
    };

    //println!("IMPLEMENTATION = \n{}", parse_impl_tokens.to_string());
    //panic!();

    parse_impl_tokens.into()
}

// helper function to return the DataEnum of the derive input.
// # Returns
// the DataEnum field, if the derive input is an enum and if the enum has at least one variant.
// If the enum has no variants or the derive input is not an enum, a descriptive error is returned. The
// error is returned as TokenStream and can be passed on directly.
fn get_data_enum(input: &DeriveInput) -> Result<&DataEnum, TokenStream> {
    match input.data {
        Data::Enum(ref data_enum) => {
            if !data_enum.variants.is_empty() {
                Ok(data_enum)
            } else {
                Err(syn::Error::new(
                    input
                        .span()
                        .join(input.ident.span())
                        .unwrap_or_else(|| input.ident.span()),
                    "no variants: enumeration must have at least one variant",
                )
                .to_compile_error()
                .into())
            }
        }
        Data::Union(_) | Data::Struct(_) => Err(syn::Error::new(
            input.span(),
            "expected enum: parsing variants only works with enumerations",
        )
        .to_compile_error()
        .into()),
    }
}
