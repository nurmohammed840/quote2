#![doc = include_str!("../README.md")]
pub use proc_macro2;

use proc_macro2::*;
pub use quote2_macros::quote;
use std::{
    iter,
    ops::{Deref, DerefMut},
};

impl Quote for TokenStream {
    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()))
    }

    fn tokens(&mut self, tts: impl IntoTokens) {
        tts.into_tokens(self)
    }
}

pub trait Quote {
    fn tokens(&mut self, _: impl IntoTokens);

    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;

    fn punct_join(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Joint));
    }

    fn punct(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Alone));
    }

    fn punct2(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Joint));
        self.add(Punct::new(ch, Spacing::Alone));
    }

    fn punct_joined(&mut self, ch: char, ch2: char) {
        self.add(Punct::new(ch, Spacing::Joint));
        self.add(Punct::new(ch2, Spacing::Alone));
    }

    fn idents(&mut self, names: &[&str]) {
        for name in names {
            self.ident(name);
        }
    }

    fn ident(&mut self, name: &str) {
        self.add(Ident::new(name, Span::call_site()));
    }

    fn group(&mut self, delimiter: char, f: impl FnOnce(&mut TokenStream)) {
        self.add(group(delimiter, f).0);
    }

    fn ident_span(&mut self, name: &str, span: Span) {
        self.add(Ident::new(name, span));
    }
}

pub fn group(delimiter: char, f: impl FnOnce(&mut TokenStream)) -> Owned<Group> {
    let mut stream = TokenStream::new();
    f(&mut stream);
    let delimiter = match delimiter {
        '{' => Delimiter::Brace,
        '[' => Delimiter::Bracket,
        '(' => Delimiter::Parenthesis,
        _ => Delimiter::None,
    };
    Owned(Group::new(delimiter, stream))
}

pub trait IntoTokens {
    fn into_tokens(self, s: &mut TokenStream);
}

impl<T: quote::ToTokens> IntoTokens for T {
    fn into_tokens(self, s: &mut TokenStream) {
        self.to_tokens(s)
    }
}

#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct Owned<T>(pub T);

impl<T> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoTokens for Owned<Group> {
    fn into_tokens(self, s: &mut TokenStream) {
        s.extend(iter::once(TokenTree::Group(self.0)));
    }
}

impl IntoTokens for Owned<TokenStream> {
    fn into_tokens(self, s: &mut TokenStream) {
        s.extend(iter::once(self.0));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuoteFn<F>(F)
where
    F: for<'a> FnOnce(&'a mut TokenStream);

impl<F> IntoTokens for QuoteFn<F>
where
    F: for<'a> FnOnce(&'a mut TokenStream),
{
    fn into_tokens(self, s: &mut TokenStream) {
        self.0(s)
    }
}

pub fn quote<F>(f: F) -> QuoteFn<F>
where
    F: for<'a> FnOnce(&'a mut TokenStream),
{
    QuoteFn(f)
}
