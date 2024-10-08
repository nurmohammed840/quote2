use proc_macro::*;
use std::mem;

/// # Example
/// ```rust
/// use quote2::{proc_macro2::TokenStream, quote, Quote};
/// let body = quote(|tokens| {
///     for i in 0..3 {
///         quote!(tokens, {
///             println!("{}", #i);
///         });
///     }
/// });
/// let mut tokens = TokenStream::new();
/// quote!(tokens, {
///     fn name() {
///         #body
///     }
/// });
/// ```
///
/// ## Generated Code
///
/// ```rust
/// fn name() {
///     println!("{}", 0i32);
///     println!("{}", 1i32);
///     println!("{}", 2i32);
/// }
/// ```
#[proc_macro]
pub fn quote(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let var = parse_arg(&mut input, "expected `ident`");

    let input = match input.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => panic!("expected `{{`"),
    };

    let mut output = TokenStream::new();
    expend(input, &mut output, None, var);
    output
}

fn parse_arg(input: &mut token_stream::IntoIter, msg: &str) -> Ident {
    let Some(TokenTree::Ident(var)) = input.next() else {
        panic!("{msg}")
    };
    input.next().expect("expected `,`");
    var
}

/// ## Example
///
/// ```rust
/// use quote2::{
///     proc_macro2::{Span, TokenStream},
///     quote_spanned, Quote,
/// };
/// let span = Span::call_site();
/// let mut tokens = TokenStream::new();
/// quote_spanned!(span, tokens, {
///     fn add(a: i32, b: i32) -> i32 {
///         a + b
///     }
/// });
/// ```
#[proc_macro]
pub fn quote_spanned(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let span = parse_arg(&mut input, "expected `span`");
    let var = parse_arg(&mut input, "expected `ident`");

    let input = match input.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => panic!("expected `{{`"),
    };

    let mut output = TokenStream::new();
    expend(input, &mut output, Some(&span), var);
    output
}

fn expend(input: TokenStream, o: &mut TokenStream, span: Option<&Ident>, var: Ident) {
    let mut input = input.into_iter().peekable();
    let mut items = TokenStream::new();

    while let Some(tree) = input.next() {
        match tree {
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                if ch == '#' && matches!(input.peek(), Some(TokenTree::Ident(_))) {
                    write_extender(&mut items, o, &var);
                    let v = input.next();

                    o.extend([
                        tt(var.clone()),
                        tt::punct('.'),
                        tt::ident("add_tokens"),
                        tt::group('(', |o| {
                            add(o, tt::punct('&'));
                            o.extend(v);
                        }),
                        tt::punct(';'),
                    ]);
                } else {
                    let varient_ty = match (punct.spacing(), span.is_some()) {
                        (Spacing::Joint, true) => "punct_join_span",
                        (Spacing::Alone, true) => "punct_span",
                        (Spacing::Joint, false) => "punct_join",
                        (Spacing::Alone, false) => "punct",
                    };
                    varient(&mut items, varient_ty, |o| {
                        add_span(o, span);
                        add(o, tt::char(ch));
                    });
                }
            }
            TokenTree::Group(group) => {
                let varient_ty = if span.is_some() {
                    "group_span"
                } else {
                    "group"
                };
                varient(&mut items, varient_ty, |o| {
                    add_span(o, span);

                    let var = Ident::new("__o", Span::call_site());
                    o.extend([
                        tt::char(match group.delimiter() {
                            Delimiter::None => '_',
                            Delimiter::Brace => '{',
                            Delimiter::Bracket => '[',
                            Delimiter::Parenthesis => '(',
                        }),
                        tt::punct(','),
                        tt::punct('|'),
                        tt(var.clone()),
                        tt::punct('|'),
                        tt::group('{', |o| {
                            expend(group.stream(), o, span, var);
                        }),
                    ]);
                });
            }
            TokenTree::Ident(ident) => {
                let varient_ty = if span.is_some() {
                    "ident_span"
                } else {
                    "ident"
                };
                varient(&mut items, varient_ty, |o| {
                    add_span(o, span);
                    add(o, Literal::string(&ident.to_string()));
                });
            }
            TokenTree::Literal(lit) => {
                let varient_ty = if span.is_some() {
                    "parsed_lit_span"
                } else {
                    "parsed_lit"
                };
                varient(&mut items, varient_ty, |o| {
                    add_span(o, span);
                    add(o, Literal::string(&lit.to_string()));
                });
            }
        }
    }
    write_extender(&mut items, o, &var);
}

fn write_extender(items: &mut TokenStream, o: &mut TokenStream, var: &Ident) {
    if !items.is_empty() {
        let items = mem::take(items);
        o.extend([
            tt(var.clone()),
            tt::punct('.'),
            tt::ident("extend"),
            tt::group('(', |o| {
                add(o, Group::new(Delimiter::Bracket, items));
            }),
            tt::punct(';'),
        ]);
    }
}

fn add(o: &mut TokenStream, t: impl Into<TokenTree>) {
    o.extend(Some(t.into()));
}

fn add_span(o: &mut TokenStream, span: Option<&Ident>) {
    if let Some(spanned) = span {
        o.extend([tt(spanned.clone()), tt::punct(',')]);
    }
}

fn varient(t: &mut TokenStream, varient_ty: &str, f: impl FnOnce(&mut TokenStream)) {
    t.extend([
        tt::ident("quote2"),
        tt::punct_joined(':'),
        tt::punct(':'),
        tt::ident("tt"),
        tt::punct_joined(':'),
        tt::punct(':'),
        tt::ident(varient_ty),
        tt::group('(', f),
        tt::punct(','),
    ]);
}

fn tt<T: Into<TokenTree>>(tt: T) -> TokenTree {
    tt.into()
}

mod tt {
    use super::*;
    pub fn punct_joined(ch: char) -> TokenTree {
        TokenTree::from(Punct::new(ch, Spacing::Joint))
    }
    pub fn punct(ch: char) -> TokenTree {
        TokenTree::from(Punct::new(ch, Spacing::Alone))
    }
    pub fn ident(string: &str) -> TokenTree {
        TokenTree::from(Ident::new(string, Span::call_site()))
    }
    pub fn char(ch: char) -> TokenTree {
        TokenTree::from(Literal::character(ch))
    }

    pub fn group(delimiter: char, f: impl FnOnce(&mut TokenStream)) -> TokenTree {
        let mut stream = TokenStream::new();
        f(&mut stream);
        let delimiter = match delimiter {
            '{' => Delimiter::Brace,
            '[' => Delimiter::Bracket,
            '(' => Delimiter::Parenthesis,
            _ => Delimiter::None,
        };
        Group::new(delimiter, stream).into()
    }
}
