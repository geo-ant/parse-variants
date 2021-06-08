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
    SumOfInts(LitInt, syn::token::Add, LitInt),
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
    let variant = syn::parse_str::<SimpleVariant>("1 + 1239874").unwrap();

    let_assert!(SimpleVariant::SumOfInts(int1,comma,int2) = variant);
    check!(int1 == syn::parse_str::<LitInt>("1").unwrap());
    check!(comma == syn::parse_str::<syn::token::Add>("+").unwrap());
    check!(int2 == syn::parse_str::<LitInt>("1239874").unwrap());
}

#[test]
fn test_testy_test1() {
    // this cannot be parsed as a sum of integer literals, so this must be parsed as an expr
    let variant = syn::parse_str::<SimpleVariant>("a+1239874").unwrap();
    let expression = syn::parse_str::<Expr>("a+1239874").unwrap();
    let_assert!(SimpleVariant::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}


#[test]
fn test_testy_test2() {
    let variant = syn::parse_str::<SimpleVariant>("this.looks(&like) == an + expression").unwrap();
    let expression = syn::parse_str::<Expr>("this.looks(&like) == an + expression").unwrap();
    let_assert!(SimpleVariant::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}



#[test]
fn test_testy_test3() {
    // this tests that expression will be parsed although the ident is just as suitable
    // this is because the order matters
    let variant = syn::parse_str::<SimpleVariant>("this_could_also_be_an_ident").unwrap();
    let expression = syn::parse_str::<Expr>("this_could_also_be_an_ident").unwrap();
    let_assert!(SimpleVariant::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}