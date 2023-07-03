use proc_macro::*;
use std::iter;

impl Tokens for TokenStream {
    fn add<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()))
    }
}

pub trait Tokens {
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

    fn group(&mut self, delimiter: Delimiter, f: impl FnOnce(&mut TokenStream)) {
        let mut stream = TokenStream::new();
        f(&mut stream);
        self.add(Group::new(delimiter, stream));
    }

    fn char(&mut self, ch: char) {
        self.add(Literal::character(ch));
    }
}
