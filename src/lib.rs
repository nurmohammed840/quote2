#![doc = include_str!("../README.md")]

mod common;
#[cfg(feature = "proc-macro")]
pub mod proc_macro;
#[cfg(feature = "proc-macro2")]
pub mod proc_macro2;

pub use quote2_macros::quote;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub trait Quote {
    fn add_punct_join(&mut self, ch: char);
    fn add_punct(&mut self, ch: char);
    fn add_ident(&mut self, name: &str);
    fn add_group(&mut self, delimiter: char, f: impl FnOnce(&mut Self));
    fn add_parsed_lit(&mut self, s: &str);

    fn add_tokens(&mut self, tokens: impl IntoTokens<Self>)
    where
        Self: Sized,
    {
        tokens.into_tokens(self);
    }

    fn add_puncts(&mut self, p: impl AddPuncts)
    where
        Self: Sized,
    {
        p.add_puncts(self)
    }

    fn add_idents(&mut self, names: &[&str]) {
        for name in names {
            self.add_ident(name);
        }
    }
}

pub trait AddPuncts {
    fn add_puncts(self, this: &mut impl Quote);
}

impl AddPuncts for char {
    fn add_puncts(self, this: &mut impl Quote) {
        this.add_punct_join(self);
        this.add_punct(self);
    }
}

impl AddPuncts for (char, char) {
    fn add_puncts(self, this: &mut impl Quote) {
        this.add_punct_join(self.0);
        this.add_punct(self.1);
    }
}

pub trait IntoTokens<Q: Quote> {
    fn into_tokens(self, s: &mut Q);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Token<T>(pub T);

pub fn quote<F, Q: Quote>(f: F) -> Token<F>
where
    F: FnOnce(&mut Q),
{
    Token(f)
}

impl<F, Q: Quote> IntoTokens<Q> for Token<F>
where
    F: FnOnce(&mut Q),
{
    fn into_tokens(self, s: &mut Q) {
        self.0(s)
    }
}

impl<T: IntoTokens<Q>, Q: Quote> IntoTokens<Q> for Token<Option<T>> {
    fn into_tokens(self, s: &mut Q) {
        if let Some(v) = self.0 {
            T::into_tokens(v, s)
        }
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
