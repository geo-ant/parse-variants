use assert2::check;
use assert2::let_assert;
use syn::{Expr};
use syn::{LitInt};

use crate::Parse;

mod keywords {
    syn::custom_keyword!(or);
}

#[derive(Parse)]
enum EnumWithNamedVariants {
    TwoIntegersWithComma {
        first: syn::LitInt,
        _comma: syn::token::Comma,
        second: syn::LitInt,
    },
    TwoExpressionsSeparatedByKeyword {
        first: syn::Expr,
        _or : keywords::or,
        second : syn::Expr,
    },
}

#[test]
fn the_first_variant_that_parses_is_returned2() {
    // this tests that expression will be parsed although the ident is just as suitable
    // this is because the order matters
    let variant = syn::parse_str::<EnumWithNamedVariants>("1+1 or x+y").unwrap();
    let expected_first = syn::parse_str::<Expr>("1+1").unwrap();
    let expected_second = syn::parse_str::<Expr>("x+y").unwrap();
    let_assert!(EnumWithNamedVariants::TwoExpressionsSeparatedByKeyword{first, _or, second} = variant);
    check!(first == expected_first);
    check!(second == expected_second);
}

#[test]
fn the_first_variant_that_parses_is_returned1() {
    // this tests that expression will be parsed although the ident is just as suitable
    // this is because the order matters
    let variant = syn::parse_str::<EnumWithNamedVariants>("123,456").unwrap();
    let expected_first = syn::parse_str::<LitInt>("123").unwrap();
    let expected_second = syn::parse_str::<LitInt>("456").unwrap();
    let_assert!(EnumWithNamedVariants::TwoIntegersWithComma{first, _comma, second} = variant);
    check!(first == expected_first);
    check!(second == expected_second);
}

#[test]
fn parsing_gives_error_if_none_of_the_variants_can_be_parsed() {
    let variant = syn::parse_str::<EnumWithNamedVariants>("---this cannot be parsed---");
    check!(variant.is_err());
}
