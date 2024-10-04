use crate::Token;
use proc_macro2::TokenStream;

pub fn quote_rep<'a, I, T>(
    iter: I,
    mut f: impl FnMut(&mut TokenStream, T) + 'a,
) -> Token<impl FnOnce(&mut TokenStream) + 'a>
where
    I: IntoIterator<Item = T> + 'a,
{
    crate::quote(move |t| {
        for val in iter.into_iter() {
            f(t, val);
        }
    })
}
