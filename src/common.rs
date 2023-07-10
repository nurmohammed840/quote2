#[macro_export]
macro_rules! impl_quote {
    () => {
        use super::{IntoTokens, Quote, Token};
        use std::str::FromStr;

        impl Quote for TokenStream {
            fn add_punct_join(&mut self, ch: char) {
                self.extend(Some(TokenTree::Punct(Punct::new(ch, Spacing::Joint))));
            }

            fn add_punct(&mut self, ch: char) {
                self.extend(Some(TokenTree::Punct(Punct::new(ch, Spacing::Alone))));
            }

            fn add_ident(&mut self, name: &str) {
                self.extend(Some(TokenTree::Ident(Ident::new(name, Span::call_site()))));
            }

            fn add_group(&mut self, delimiter: char, f: impl FnOnce(&mut Self)) {
                self.extend(Some(TokenTree::Group(group(delimiter, f))));
            }

            fn add_parsed_lit(&mut self, s: &str) {
                self.extend(Some(TokenTree::Literal(
                    Literal::from_str(s).expect("invalid literal"),
                )));
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

        impl IntoTokens<TokenStream> for Token<Group> {
            fn into_tokens(self, s: &mut TokenStream) {
                s.extend(Some(TokenTree::Group(self.0)))
            }
        }

        impl IntoTokens<TokenStream> for Token<TokenStream> {
            fn into_tokens(self, s: &mut TokenStream) {
                s.extend(Some(self.0));
            }
        }
    };
}
pub(crate) use impl_quote;
