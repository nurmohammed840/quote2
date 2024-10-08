use proc_macro2::*;
use std::str::FromStr;

fn ident_maybe_raw(id: &str, span: Span) -> Ident {
    if let Some(id) = id.strip_prefix("r#") {
        Ident::new_raw(id, span)
    } else {
        Ident::new(id, span)
    }
}

#[inline]
pub fn punct_join(ch: char) -> TokenTree {
    Punct::new(ch, Spacing::Joint).into()
}

pub fn punct_join_span(span: Span, ch: char) -> TokenTree {
    let mut t = Punct::new(ch, Spacing::Joint);
    t.set_span(span);
    t.into()
}

#[inline]
pub fn punct(ch: char) -> TokenTree {
    Punct::new(ch, Spacing::Alone).into()
}

pub fn punct_span(span: Span, ch: char) -> TokenTree {
    let mut p = Punct::new(ch, Spacing::Alone);
    p.set_span(span);
    p.into()
}

#[inline]
pub fn ident(name: &str) -> TokenTree {
    ident_maybe_raw(name, Span::call_site()).into()
}

#[inline]
pub fn ident_span(span: Span, name: &str) -> TokenTree {
    ident_maybe_raw(name, span).into()
}

#[inline]
pub fn parsed_lit(s: &str) -> TokenTree {
    Literal::from_str(s).expect("invalid literal").into()
}

pub fn parsed_lit_span(span: Span, s: &str) -> TokenTree {
    let mut l = Literal::from_str(s).expect("invalid literal");
    l.set_span(span);
    l.into()
}

#[inline]
pub fn group(delimiter: char, f: impl FnOnce(&mut TokenStream)) -> TokenTree {
    _group(delimiter, f).into()
}

pub fn group_span(span: Span, delimiter: char, f: impl FnOnce(&mut TokenStream)) -> TokenTree {
    let mut g = _group(delimiter, f);
    g.set_span(span);
    g.into()
}

fn _group(delimiter: char, f: impl FnOnce(&mut TokenStream)) -> Group {
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
