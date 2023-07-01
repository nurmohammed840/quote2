#![doc = include_str!("../README.md")]
pub use proc_macro2;

use proc_macro2::*;
pub use quote2_macros::quote;
use std::{borrow::Cow, iter, rc::Rc};

impl TokensExt for TokenStream {
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

pub trait TokensExt {
    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;

    fn punct_join(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Joint));
    }
    fn punct(&mut self, ch: char) {
        self.add(Punct::new(ch, Spacing::Alone));
    }

    fn ident(&mut self, name: &str) {
        self.add(Ident::new(name, Span::call_site()));
    }

    fn ident_span(&mut self, name: &str, span: Span) {
        self.add(Ident::new(name, span));
    }

    fn group(&mut self, delimiter: char, f: impl FnOnce(&mut TokenStream)) {
        let delimiter = match delimiter {
            '{' => Delimiter::Brace,
            '[' => Delimiter::Bracket,
            '(' => Delimiter::Parenthesis,
            _ => Delimiter::None,
        };
        self.add(group(delimiter, f));
    }

    fn tokens(&mut self, _: impl IntoTokens);
}

pub fn group(delimiter: Delimiter, f: impl FnOnce(&mut TokenStream)) -> Group {
    let mut stream = TokenStream::new();
    f(&mut stream);
    Group::new(delimiter, stream)
}

pub trait IntoTokens {
    fn into_tokens(self, s: &mut TokenStream);
}

impl<'a, T> IntoTokens for &'a T
where
    T: ?Sized + IntoTokens + Copy,
{
    fn into_tokens(self, s: &mut TokenStream) {
        T::into_tokens(*self, s)
    }
}

impl<T: IntoTokens> IntoTokens for Option<T> {
    fn into_tokens(self, s: &mut TokenStream) {
        if let Some(v) = self {
            T::into_tokens(v, s)
        }
    }
}

macro_rules! impl_for {
    [@lit $($lit: ty => $name: ident)*] => {$(
        impl IntoTokens for $lit {
            fn into_tokens(self, s: &mut TokenStream) {
                s.add(Literal::$name(self));
            }
        }
    )*};
    [@struct $($ty: ty)*] => {$(
        impl IntoTokens for $ty {
            fn into_tokens(self, s: &mut TokenStream) {
                s.add(self)
            }
        }
    )*};
}

impl_for! {@struct Group Ident Punct Literal TokenTree }
impl_for! {
    @lit
    u8 => u8_suffixed
    u16 => u16_suffixed
    u32 => u32_suffixed
    u64 => u64_suffixed
    u128 => u128_suffixed

    i8 => i8_suffixed
    i16 => i16_suffixed
    i32 => i32_suffixed
    i64 => i64_suffixed
    i128 => i128_suffixed

    f32 => f32_suffixed
    f64 => f64_suffixed

    &str => string
    char => character
    &[u8] => byte_string
}

impl IntoTokens for bool {
    fn into_tokens(self, s: &mut TokenStream) {
        let word = if self { "true" } else { "false" };
        s.add(Ident::new(word, Span::call_site()));
    }
}

impl IntoTokens for TokenStream {
    fn into_tokens(self, s: &mut TokenStream) {
        s.extend(iter::once(self));
    }
}

impl<'a, T: ?Sized + IntoTokens + Copy> IntoTokens for &'a mut T {
    fn into_tokens(self, s: &mut TokenStream) {
        T::into_tokens(*self, s)
    }
}

impl<'a, T: ?Sized + IntoTokens + Clone> IntoTokens for Cow<'a, T> {
    fn into_tokens(self, s: &mut TokenStream) {
        match self {
            Cow::Borrowed(v) => T::into_tokens(T::clone(v), s),
            Cow::Owned(v) => T::into_tokens(v, s),
        }
    }
}

impl<T: IntoTokens> IntoTokens for Box<T> {
    fn into_tokens(self, s: &mut TokenStream) {
        T::into_tokens(*self, s)
    }
}

impl<T: IntoTokens + Clone> IntoTokens for Rc<T> {
    fn into_tokens(self, s: &mut TokenStream) {
        T::into_tokens(T::clone(&self), s)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quote<F>(F)
where
    F: for<'a> FnOnce(&'a mut TokenStream);

impl<F> IntoTokens for Quote<F>
where
    F: for<'a> FnOnce(&'a mut TokenStream),
{
    fn into_tokens(self, s: &mut TokenStream) {
        self.0(s)
    }
}

pub fn quote<F>(f: F) -> Quote<F>
where
    F: for<'a> FnOnce(&'a mut TokenStream),
{
    Quote(f)
}
