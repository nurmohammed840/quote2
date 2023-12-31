#![doc = include_str!("../README.md")]
pub use proc_macro2;

use proc_macro2::*;
pub use quote2_macros::quote;
use std::{
    fmt, iter,
    ops::{Deref, DerefMut},
    str::FromStr,
};

impl Quote for TokenStream {
    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()))
    }

    fn add_tokens(&mut self, tts: impl IntoTokens) {
        tts.into_tokens(self)
    }
}

pub trait Quote {
    fn add_tokens(&mut self, _: impl IntoTokens);

    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;

    fn add_punct_join(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Joint));
    }

    fn add_punct(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Alone));
    }

    fn add_punct2(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Joint));
        self.add(Punct::new(ch, Spacing::Alone));
    }

    fn add_puncts(&mut self, ch: char, ch2: char) {
        self.add(Punct::new(ch, Spacing::Joint));
        self.add(Punct::new(ch2, Spacing::Alone));
    }

    fn add_idents(&mut self, names: &[&str]) {
        for name in names {
            self.add_ident(name);
        }
    }

    fn add_ident(&mut self, name: &str) {
        self.add(Ident::new(name, Span::call_site()));
    }

    fn add_group(&mut self, delimiter: char, f: impl FnOnce(&mut TokenStream)) {
        self.add(group(delimiter, f));
    }

    fn add_parsed_lit(&mut self, s: &str) {
        self.add(Literal::from_str(s).expect("invalid literal"));
    }

    fn add_ident_span(&mut self, name: &str, span: Span) {
        self.add(Ident::new(name, span));
    }
}

pub fn group(delimiter: char, f: impl FnOnce(&mut TokenStream)) -> Group {
    let mut stream = TokenStream::new();
    f(&mut stream);
    let delimiter = match delimiter {
        '{' => Delimiter::Brace,
        '[' => Delimiter::Bracket,
        '(' => Delimiter::Parenthesis,
        _ => Delimiter::None,
    };
    Group::new(delimiter, stream)
}

pub trait IntoTokens {
    fn into_tokens(self, s: &mut TokenStream);
}

impl<T: quote::ToTokens> IntoTokens for T {
    fn into_tokens(self, s: &mut TokenStream) {
        self.to_tokens(s)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Token<T>(pub T);

pub fn quote<F>(f: F) -> Token<F>
where
    F: FnOnce(&mut TokenStream),
{
    Token(f)
}

impl<F> IntoTokens for Token<F>
where
    F: FnOnce(&mut TokenStream),
{
    fn into_tokens(self, s: &mut TokenStream) {
        self.0(s)
    }
}

impl<T: IntoTokens> IntoTokens for Token<Option<T>> {
    fn into_tokens(self, s: &mut TokenStream) {
        if let Some(v) = self.0 {
            T::into_tokens(v, s)
        }
    }
}

impl IntoTokens for Token<Group> {
    fn into_tokens(self, s: &mut TokenStream) {
        s.extend(iter::once(TokenTree::Group(self.0)));
    }
}

impl IntoTokens for Token<TokenStream> {
    fn into_tokens(self, s: &mut TokenStream) {
        s.extend(iter::once(self.0));
    }
}

impl<T: fmt::Display> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(&self.0, f)
    }
}

impl<T> Deref for Token<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Token<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
