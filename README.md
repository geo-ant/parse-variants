# parse-variants
![build](https://github.com/geo-ant/parse-variants/workflows/build/badge.svg?branch=main)
![lints](https://github.com/geo-ant/parse-variants/workflows/lints/badge.svg?branch=main)
![tests](https://github.com/geo-ant/parse-variants/workflows/tests/badge.svg?branch=main)

Derive the `syn::parse::Parse` trait for enumerations and use it to comfortably parse 
a variant of the enumeration.

# Motivation
For a little project, I was trying to parse tokens that could either be an integer literal or an identifier
from a [ParseBuffer](https://docs.rs/syn/1.0/syn/parse/struct.ParseBuffer.html). This inspired me
to write a custom derive macro for these kinds of use cases. We can now write
```rust
#[derive(parse_variants::Parse)]
enum Number {
    Identifier(syn::Ident),
    Literal(syn::LitInt),
}
```
and then use this type to parse either variant from a parse buffer like so:

```rust
// input : &ParseBuffer
let num : Number = input.parse()?;
```
Parsing returns the first variant that can be parsed from the contents of the parse buffer.
If none of the variants can be parsed, a compile error is returned. We can use this in any context
where we wish to parse this type. The custom derive macro can also be used on
much more general `enum` types, enabling pretty powerful parsing of variant types.

# Advanced Use Cases
Enumerations do not have to be as simple as in the example above. This crate will let you
use the custom derive on enumerations with struct-like or tuple-like variants (or any
combination of them). See this silly example for a more advanced use case:
```rust
#[derive(parse_variants::Parse)]
enum SillyEnum {
    ExpressionInMeters {
        first: syn::Expr,
        _meters: kw::meters,
    },
    IdentPlusPlus(Ident, syn::Token![+], syn::Token![+]),
}
# let_assert!(Ok(SillyEnum::ExpressionInMeters{..}) = syn::parse_str::<SillyEnum>("16 + 12*length meters"));
# let_assert!(Ok(SillyEnum::IdentPlusPlus(_,_,_)) = syn::parse_str::<SillyEnum>("C++"));
```
This parses the tokens `16 + 12*length meters` as the first and `C++` as the second variant.

Consult the crate documentation for more information on how to use this macro and what to watch out for.
