use assert2::check;
use assert2::let_assert;
use assert_matches::assert_matches;
use syn::{Expr, Ident};
use syn::{LitInt, LitStr};

use super::Parse;

/// TODO: ATTENTION THE ORDER MATTERS HERE! THE FIRST THING THAT WORKS IS PARSED
///DOCUMENT THIS. PARSE_STR WILL ACTUALLY THROW AN ERROR IF NOT ALL TOKENS ARE USED IN THE EXPRESSION
#[derive(Parse)]
enum SimpleVariant {
    IntCommaInt(LitInt, syn::token::Comma, LitInt),
    Expression(Expr),
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

#[test]
fn test_testy_test2() {
    let variant = syn::parse_str::<SimpleVariant>("this.looks(&like) == an + expression").unwrap();
    let expression = syn::parse_str::<Expr>("this.looks(&like) == an + expression").unwrap();
    let_assert!(SimpleVariant::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}
