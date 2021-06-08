use syn::Ident;
use syn::{LitInt,LitStr};

use super::Parse;

use assert2::let_assert;
use assert2::check;
use assert_matches::assert_matches;

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
    let variant = syn::parse_str::<SimpleVariant>("1,1239874").unwrap();

    let_assert!(SimpleVariant::IntCommaInt(int1,comma,int2) = variant);
    check!(int1 == syn::parse_str::<LitInt>("1").unwrap());
    check!(comma == syn::parse_str::<syn::token::Comma>(",").unwrap());
    check!(int2 == syn::parse_str::<LitInt>("1239874").unwrap());
}
