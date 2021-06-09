use assert2::check;
use assert2::let_assert;
use syn::{Expr};
use syn::{LitInt,Ident};

mod keywords {
    syn::custom_keyword!(lebowski);
}

#[derive(crate::Parse)]
enum EnumWithMixedVariants {
    TwoExpressionsSeparatedByKeyword {
        first: syn::Expr,
        _the_dude : keywords::lebowski,
        second : syn::Expr,
    },
    Identifier(Ident),
}

#[test]
fn parsing_struct_like_variants_work() {
    let variant = syn::parse_str::<EnumWithMixedVariants>("jeffrey() lebowski el.duderino(&his_dudeness)").unwrap();
    let expected_first = syn::parse_str::<Expr>("jeffrey()").unwrap();
    let expected_second = syn::parse_str::<Expr>("el.duderino(&his_dudeness)").unwrap();
    let_assert!(EnumWithMixedVariants::TwoExpressionsSeparatedByKeyword{first,_the_dude,second} = variant);
    check!(first == expected_first);
    check!(second == expected_second);
}

#[test]
fn parsing_stuple_like_variants_work() {
    let variant = syn::parse_str::<EnumWithMixedVariants>("jeffrey_leboswki").unwrap();
    let expected_ident = syn::parse_str::<Ident>("jeffrey_leboswki").unwrap();
    let_assert!(EnumWithMixedVariants::Identifier(ident) = variant);
    check!(ident == expected_ident);
}
