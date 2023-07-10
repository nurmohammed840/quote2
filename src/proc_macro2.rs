#![allow(warnings)]

pub use proc_macro2::*;
super::common::impl_quote!();

impl<T: quote::ToTokens> IntoTokens<TokenStream> for T {
    fn into_tokens(self, s: &mut TokenStream) {
        T::into_tokens(self, s)
    }
}
