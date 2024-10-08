#![doc = include_str!("../README.md")]
use core::fmt;

#[doc(hidden)]
pub mod tt;
pub mod utils;
pub use proc_macro2;

use proc_macro2::{TokenStream, TokenTree};
pub use quote::{format_ident, ToTokens};
pub use quote2_macros::{quote, quote_spanned};

pub trait Quote: Extend<TokenTree> {
    fn add_tokens(&mut self, _: impl ToTokens);
}

impl Quote for TokenStream {
    #[inline]
    fn add_tokens(&mut self, t: impl ToTokens) {
        t.to_tokens(self);
    }
}

#[derive(Clone, Copy)]
pub struct QuoteFn<T>(pub T);

impl<T> From<T> for QuoteFn<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[inline]
pub fn quote<F>(f: F) -> QuoteFn<F>
where
    F: Fn(&mut TokenStream),
{
    QuoteFn(f)
}

impl<F> quote::ToTokens for QuoteFn<F>
where
    F: Fn(&mut TokenStream),
{
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (self.0)(tokens)
    }
}

impl<F> fmt::Debug for QuoteFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuoteFn").finish()
    }
}

impl<T> std::ops::Deref for QuoteFn<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for QuoteFn<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
