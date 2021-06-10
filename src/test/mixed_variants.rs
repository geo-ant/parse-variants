use assert2::check;
use assert2::let_assert;
use syn::Expr;
use syn::Ident;

mod keywords {
    syn::custom_keyword!(lebowski);
}

#[derive(crate::Parse)]
enum EnumWithMixedVariants {
    TwoExpressionsSeparatedByKeyword {
        first: syn::Expr,
        _the_dude: keywords::lebowski,
        second: syn::Expr,
    },
    IdentifierPlusPlus(Ident, syn::token::Add, syn::token::Add),
}

#[test]
fn parsing_struct_like_variants_work() {
    let variant =
        syn::parse_str::<EnumWithMixedVariants>("jeffrey() lebowski el.duderino(&his_dudeness)")
            .unwrap();
    let expected_first = syn::parse_str::<Expr>("jeffrey()").unwrap();
    let expected_second = syn::parse_str::<Expr>("el.duderino(&his_dudeness)").unwrap();
    let_assert!(
        EnumWithMixedVariants::TwoExpressionsSeparatedByKeyword {
            first,
            _the_dude,
            second
        } = variant
    );
    check!(first == expected_first);
    check!(second == expected_second);
}

#[test]
fn parsing_stuple_like_variants_work() {
    let variant = syn::parse_str::<EnumWithMixedVariants>("C++").unwrap();
    let expected_ident = syn::parse_str::<Ident>("C").unwrap();
    let_assert!(EnumWithMixedVariants::IdentifierPlusPlus(ident, _, _) = variant);
    check!(ident == expected_ident);
}
