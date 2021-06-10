#[derive(parse_variants::Parse)]
enum EnumWithMixedVariants {
    TwoExpressionsSeparatedByKeyword {
        first: syn::Expr,
        _the_dude: keywords::lebowski,
        second: syn::Expr,
    },
    IdentifierPlusPlus(Ident, syn::token::Add, syn::token::Add),
}
