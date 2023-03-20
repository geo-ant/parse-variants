use assert2::check;
use assert2::let_assert;
use syn::LitInt;
use syn::{Expr, Ident};

use crate::Parse;

/// TODO: ATTENTION THE ORDER MATTERS HERE! THE FIRST THING THAT WORKS IS PARSED
///DOCUMENT THIS. PARSE_STR WILL ACTUALLY THROW AN ERROR IF NOT ALL TOKENS ARE USED IN THE EXPRESSION
#[derive(Parse)]
enum EnumWithUnnamedVariants {
    SumOfInts(LitInt, syn::token::Plus, LitInt),
    Expression(Expr),
    Identifier(Ident), // due to the order of the enum, this can never be parsed because Expr is a superset of Ident
}

#[test]
fn first_variant_is_parsed_when_it_matches() {
    let variant = syn::parse_str::<EnumWithUnnamedVariants>("1 + 1239874").unwrap();
    let_assert!(EnumWithUnnamedVariants::SumOfInts(int1, plus, int2) = variant);
    check!(int1 == syn::parse_str::<LitInt>("1").unwrap());
    check!(plus == syn::parse_str::<syn::token::Plus>("+").unwrap());
    check!(int2 == syn::parse_str::<LitInt>("1239874").unwrap());
}

#[test]
fn second_variant_is_parsed_when_first_cannot_be_matched() {
    // this cannot be parsed as a sum of integer literals, so this must be parsed as an expr
    let variant = syn::parse_str::<EnumWithUnnamedVariants>("a+1239874").unwrap();
    let expression = syn::parse_str::<Expr>("a+1239874").unwrap();
    let_assert!(EnumWithUnnamedVariants::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}

#[test]
fn expressions_are_correctly_parsed() {
    let variant =
        syn::parse_str::<EnumWithUnnamedVariants>("this.looks(&like) == an + expression").unwrap();
    let expression = syn::parse_str::<Expr>("this.looks(&like) == an + expression").unwrap();
    let_assert!(EnumWithUnnamedVariants::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}

#[test]
    fn the_first_working_parse_is_chosen() {
    // this tests that expression will be parsed although the ident is just as suitable
    // this is because the order matters
    let variant = syn::parse_str::<EnumWithUnnamedVariants>("this_could_also_be_an_ident").unwrap();
    let expression = syn::parse_str::<Expr>("this_could_also_be_an_ident").unwrap();
    let_assert!(EnumWithUnnamedVariants::Expression(parsed_expression) = variant);
    check!(parsed_expression == expression);
}

#[test]
fn parsing_gives_error_if_none_of_the_variants_can_be_parsed() {
    let variant = syn::parse_str::<EnumWithUnnamedVariants>("---this cannot be parsed---");
    check!(variant.is_err());
}
