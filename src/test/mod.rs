use syn::Ident;
use syn::LitInt;

use super::Parse;

use assert2::let_assert;
use assert2::check;

#[derive(Parse)]
enum SimpleVariant {
    IntCommaInt(LitInt, syn::token::Comma, LitInt),
    Identifier(Ident),
}


#[derive(Parse)]
enum ComplicatedVariants {
    TwoIntegersWithComma {
        first: syn::LitInt,
        comma: syn::token::Comma,
        second: syn::LitInt,
    },
    OneInt {
        first: syn::LitInt,
    },
    Identifier(Ident),
}

#[test]
fn test_testy_test() {
    let variant = syn::parse_str::<SimpleVariant>("1,2").unwrap();

    let_assert!(SimpleVariant::IntCommaInt(int1,comma,int2) = variant);



}
