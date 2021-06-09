//! This crate allows us to parse tokens as one of the variants given by an `enum`.
//! This is achieved by deriving the `parse_variants::Parse` trait on the enumeration which will in turn
//! derive a [`syn::Parse`](https://docs.rs/syn/1.0/syn/parse/trait.Parse.html) implementation with
//! the desired logic.
//!
//! # Motivation
//! For a project, I was trying to parse tokens that could either be an integer literal or an identifier
//! from a [ParseBuffer](https://docs.rs/syn/1.0/syn/parse/struct.ParseBuffer.html). This inspired me
//! to write a custom derive macro for these kinds of use cases. It enables us to solve the problem
//! like so:
//! ```
//! #[derive(parse_variants::Parse)]
//! enum Number {
//!     Identifier(syn::Ident),
//!     Literal(syn::LitInt),
//! }
//! ```
//! We can now use it to parse like so:
//! ```
//! # #[derive(parse_variants::Parse)]
//! # enum Number {
//! #    Identifier(syn::Ident),
//! #    Literal(syn::LitInt),
//! # }
//! # use syn::parse::ParseBuffer;
//! # fn parse_number(input : &ParseBuffer) -> Result<(), syn::Error>{
//! // input : &ParseBuffer
//! let num : Number = input.parse()?;
//! # Ok(())
//! # }
//! ```
//! Or in any other context where we wish to parse this type.
//!
//! # Usage
//!
//! **Attention** This crate requires that you have the [syn crate](https://crates.io/crates/syn) as a dependency
//! and that you have not renamed it.
//!
//! ## General
//! * The custom derive can be applied to *enumerations*, which may contain struct like or
//! tuple like variants. Each variant may contain one or multiple fields.
//! * Every contained field must implement the `syn::parse::Parse` trait.
//! * Member fields for each variants are parsed in order of definition.
//! * The first variant (in order of declaration) that is successfully parsed from the input will
//! be returned. The input `ParseBuffer` is advanced accordingly.
//! * If no variant can be successfully parsed from the given input, a descriptive compile error
//! is returned.
//!
//! ## Caveats
//! The enum variants are parsed top to bottom. That means the first variant that can successfully
//! parsed will be returned. That means the order matters if one variant includes other variants.
//! Take this for example:
//! ```
//! // WRONG: this can never return the Identifier variant
//! #[derive(parse_variants::Parse)]
//! enum Number {
//!     Expression(syn::Expr),
//!     Identifier(syn::Ident)
//! }
//! ```
//! Since identifiers can be parsed as expressions, the `Expression` variant will always be chosen,
//! even if the given tokens could also have been parsed as an identifier.
//!
//! ```
//! // CORRECT: the most general pattern comes last
//! #[derive(parse_variants::Parse)]
//! enum Number {
//!     Identifier(syn::Ident),
//!     Expression(syn::Expr)
//! }
//! ```
//! This is why we have to pay attention to ordering the variants from least general to most general.
//!
//! ## Restrictions
//! The enumeration cannot contain unit variants (i.e. without member fields), because there is no
//! useful way to parse them.
//!
//! ## Example
//! It is possible to write pretty complex parsers for variants. See this very silly example:
//!
//! ```
//! use syn::Ident;
//! use syn::Expr;
//! # use assert2::let_assert;
//!
//! mod kw {
//!     syn::custom_keyword!(meters);
//! }
//!
//! #[derive(parse_variants::Parse)]
//! enum SillyEnum {
//!     ExpressionInMeters {
//!         first: syn::Expr,
//!         _meters: kw::meters,
//!     },
//!     IdentPlusPlus(Ident, syn::Token![+], syn::Token![+]),
//! }
//! # let_assert!(Ok(SillyEnum::ExpressionInMeters{..}) = syn::parse_str::<SillyEnum>("16 + 12*length meters"));
//! # let_assert!(Ok(SillyEnum::IdentPlusPlus(_,_,_)) = syn::parse_str::<SillyEnum>("C++"));
//! ```
//! This will e.g. parse the tokens `16 + 12*length meters` as the first and `C++` as the second variant.

pub use parse_variants_derive::Parse;

#[cfg(test)]
#[allow(clippy::large_enum_variant)]
mod test;
