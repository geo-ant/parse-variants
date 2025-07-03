enum EnumWithMixedVariants {
    TwoExpressionsSeparatedByKeyword {
        first: syn::Expr,
        _the_dude: keywords::lebowski,
        second: syn::Expr,
    },
    IdentifierPlusPlus(Ident, syn::token::Add, syn::token::Add),
}
impl ::syn::parse::Parse for EnumWithMixedVariants {
    fn parse(
        input: &::syn::parse::ParseBuffer,
    ) -> ::std::result::Result<Self, ::syn::Error> {
        use ::syn::parse::discouraged::Speculative;
        let fork = input.fork();
        if let Ok(variant) = (|| {
            Ok(EnumWithMixedVariants::TwoExpressionsSeparatedByKeyword {
                first: fork.parse()?,
                _the_dude: fork.parse()?,
                second: fork.parse()?,
            }) as ::std::result::Result<EnumWithMixedVariants, ::syn::Error>
        })() {
            input.advance_to(&fork);
            return Ok(variant);
        }
        let fork = input.fork();
        if let Ok(variant) = (|| {
            Ok(
                EnumWithMixedVariants::IdentifierPlusPlus(
                    fork.parse()?,
                    fork.parse()?,
                    fork.parse()?,
                ),
            ) as ::std::result::Result<EnumWithMixedVariants, ::syn::Error>
        })() {
            input.advance_to(&fork);
            return Ok(variant);
        }
        Err(
            syn::Error::new(
                input.span(),
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["parse error: tokens cannot be parsed as any variant of "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &"EnumWithMixedVariants",
                                ),
                            ],
                        ),
                    );
                    res
                },
            ),
        )
    }
}
