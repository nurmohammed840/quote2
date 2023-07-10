#![allow(warnings)]

pub use proc_macro2::*;

// super::common::impl_quote!();

use crate::Token;

// impl<Q: Quote> IntoTokens<Q> for Token<Group> {
//     fn into_tokens(self, s: &mut Q) {
//         s.add_group(delimiter, f)
//         // s.add_group(delimiter, f)
//     }
//     // fn into_tokens(self, s: &mut TokenStream) {
//     //     s.extend(iter::once(TokenTree::Group(self.0)));
//     // }
// }

// impl IntoTokens for Token<TokenStream> {
//     fn into_tokens(self, s: &mut TokenStream) {
//         s.extend(iter::once(self.0));
//     }
// }
