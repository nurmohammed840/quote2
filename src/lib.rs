#![doc = include_str!("../README.md")]

mod common;
pub mod proc_macro;
pub mod proc_macro2;

pub use quote2_macros::quote;
use std::fmt;

pub trait Quote {
    fn add_punct_join(&mut self, ch: char);
    fn add_punct(&mut self, ch: char);
    fn add_ident(&mut self, name: &str);
    fn add_group(&mut self, delimiter: char, f: impl FnOnce(&mut Self));
    fn add_parsed_lit(&mut self, s: &str);

    fn add_tokens(&mut self, tokens: impl IntoTokens<Stream = Self>) {
        tokens.into_tokens(self);
    }

    fn add_punct2(&mut self, ch: char) {
        self.add_punct_join(ch);
        self.add_punct(ch);
    }

    fn add_puncts(&mut self, ch: char, ch2: char) {
        self.add_punct_join(ch);
        self.add_punct(ch2);
    }

    fn add_idents(&mut self, names: &[&str]) {
        for name in names {
            self.add_ident(name);
        }
    }
}

pub trait IntoTokens {
    type Stream;
    fn into_tokens(self, s: &mut Self::Stream);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Token<T>(pub T);

// pub fn quote<F, Q: Quote>(f: F) -> Token<F>
// where
//     F: FnOnce(&mut Q),
// {
//     Token(f)
// }

impl<F, Q: Quote> IntoTokens for Token<F>
where
    F: FnOnce(&mut Q),
{
    type Stream = Q;
    fn into_tokens(self, s: &mut Q) {
        self.0(s)
    }
}

// impl<T: IntoTokens<Q>, Q: Quote> IntoTokens<Q> for Token<Option<T>> {
//     fn into_tokens(self, s: &mut Q) {
//         if let Some(v) = self.0 {
//             T::into_tokens(v, s)
//         }
//     }
// }

impl<T: fmt::Display> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(&self.0, f)
    }
}

impl<T> std::ops::Deref for Token<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Token<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
