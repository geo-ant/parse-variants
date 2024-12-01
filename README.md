# parse-variants
![build](https://github.com/geo-ant/parse-variants/actions/workflows/build.yml/badge.svg?branch=main)
![lints](https://github.com/geo-ant/parse-variants/actions/workflows/lints.yml/badge.svg?branch=main)
![tests](https://github.com/geo-ant/parse-variants/actions/workflows/tests.yml/badge.svg?branch=main)
![approval-tests](https://github.com/geo-ant/parse-variants/actions/workflows/approval-tests.yml/badge.svg?branch=main)
![maintenance-status](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)
[![crates](https://raw.githubusercontent.com/geo-ant/user-content/refs/heads/main/ko-fi-support.svg)](https://ko-fi.com/geoant)

Derive the `syn::parse::Parse` trait for enumerations and use it to comfortably parse 
a variant of the enumeration.

## Motivation
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

This operation returns the first variant (in order of declaration) that can 
be successfully parsed from the contents of the parse buffer.
If none of the variants can be parsed, a compile error is returned. We can use this in any context
where we wish to parse this type. The custom derive macro can also be used on
much more general `enum` types, enabling pretty powerful parsing of variant types.

## Advanced Use Cases
Enumerations do not have to be as simple as in the example above, because this crate will let you
use the custom derive on enumerations with struct-like or tuple-like variants (or any
combination of them). See this silly example for a more advanced use case:
```rust
mod kw {
    syn::custom_keyword!(meters);
}

#[derive(parse_variants::Parse)]
enum SillyEnum {
    ExpressionInMeters {
        first: syn::Expr,
        _meters: kw::meters,
    },
    IdentPlusPlus(Ident, syn::Token![+], syn::Token![+]),
}
```
This parses the tokens `16 + 12*length meters` as the first and `C++` as the second variant.

Consult the crate documentation for more information on how to use this macro and what to watch out for.
