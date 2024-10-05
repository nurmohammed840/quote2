use crate::QuoteFn;
use proc_macro2::TokenStream;

pub fn quote_rep<'a, I, T>(
    iter: I,
    f: impl Fn(&mut TokenStream, T) + 'a,
) -> QuoteFn<impl Fn(&mut TokenStream) + 'a>
where
    I: IntoIterator<Item = T> + 'a,
    I: Clone,
{
    crate::quote(move |t| {
        for val in iter.clone().into_iter() {
            f(t, val);
        }
    })
}
