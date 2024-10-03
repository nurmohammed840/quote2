use quote::ToTokens;

#[macro_export]
macro_rules! quote {
    ($($tts: tt)*) => {{
        let mut _o = TokenStream::new();
        quote2::quote!(_o, {
            $($tts)*
        });
        _o
    }};
}

#[macro_export]
macro_rules! quote_spanned {
    ($span:tt => $($tts: tt)*) => {{
        use quote2::Quote;
        let mut g = TokenStream::new();
        quote2::quote_spanned!($span, g, {
            $($tts)*
        });
        g
    }};
}

pub struct QuoteIter<T>(pub T);

impl<T> From<T> for QuoteIter<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<I> quote2::IntoTokens for QuoteIter<I>
where
    I: IntoIterator,
    I::Item: quote::ToTokens,
{
    fn into_tokens(self, tokens: &mut proc_macro2::TokenStream) {
        for tt in self.0.into_iter() {
            tt.to_tokens(tokens);
        }
    }
}
