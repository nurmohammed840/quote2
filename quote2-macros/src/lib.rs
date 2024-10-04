mod tokens;
use std::mem;

use proc_macro::*;
use tokens::Tokens;

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
    let mut input = input.into_iter();
    let mut idents = Idents::default();
    let mut peek = input.next();

    while let Some(tt) = peek.take() {
        peek = input.next();

        if let TokenTree::Ident(name) = tt {
            idents.add(&name.to_string());
            continue;
        }
        if !idents.is_empty() {
            idents.take().write(o, span, var.clone());
        }

        o.add(var.clone());
        o.punct('.');
        match tt {
            TokenTree::Punct(curr) => {
                let ch = curr.as_char();
                match peek {
                    Some(TokenTree::Ident(_)) if ch == '#' => {
                        let v = mem::replace(&mut peek, input.next());
                        o.ident("add_tokens");
                        o.group(Delimiter::Parenthesis, |o| {
                            o.punct('&');
                            o.extend(v)
                        });
                    }
                    Some(TokenTree::Punct(next))
                        if curr.spacing() == Spacing::Joint
                            && next.spacing() == Spacing::Alone
                            && next.as_char() != '#' =>
                    {
                        let next_ch = next.as_char();
                        let is_punct2 = ch == next_ch;
                        peek = input.next();

                        o.ident(match (is_punct2, span.is_some()) {
                            (true, true) => "add_punct2_span",
                            (true, false) => "add_punct2",
                            (false, true) => "add_puncts_span",
                            (false, false) => "add_puncts",
                        });
                        o.group(Delimiter::Parenthesis, |o| {
                            add_span(o, span);
                            o.char(ch);
                            if !is_punct2 {
                                o.punct(',');
                                o.char(next_ch);
                            }
                        });
                    }
                    _ => {
                        o.ident(match (curr.spacing(), span.is_some()) {
                            (Spacing::Joint, true) => "add_punct_join_span",
                            (Spacing::Alone, true) => "add_punct_span",
                            (Spacing::Joint, false) => "add_punct_join",
                            (Spacing::Alone, false) => "add_punct",
                        });
                        o.group(Delimiter::Parenthesis, |o| {
                            add_span(o, span);
                            o.char(ch)
                        });
                    }
                }
            }
            TokenTree::Group(group) => {
                o.ident(if span.is_some() {
                    "add_group_span"
                } else {
                    "add_group"
                });
                o.group(Delimiter::Parenthesis, |o| {
                    add_span(o, span);

                    let var = Ident::new("__o", Span::call_site());
                    o.char(match group.delimiter() {
                        Delimiter::None => '_',
                        Delimiter::Brace => '{',
                        Delimiter::Bracket => '[',
                        Delimiter::Parenthesis => '(',
                    });
                    o.punct(',');
                    o.punct('|');
                    o.add(var.clone());
                    o.punct('|');
                    o.group(Delimiter::Brace, |o| {
                        expend(group.stream(), o, span, var);
                    });
                });
            }
            TokenTree::Literal(lit) => {
                o.ident(if span.is_some() {
                    "add_parsed_lit_span"
                } else {
                    "add_parsed_lit"
                });
                o.group(Delimiter::Parenthesis, |o| {
                    add_span(o, span);
                    o.add(Literal::string(&lit.to_string()));
                });
            }
            TokenTree::Ident(_) => {}
        }
        o.punct(';');
    }
    if !idents.is_empty() {
        idents.write(o, span, var);
    }
}

fn add_span(o: &mut TokenStream, span: Option<&Ident>) {
    if let Some(spanned) = span {
        o.add(spanned.clone());
        o.punct(',');
    }
}

#[derive(Default)]
struct Idents {
    // 0 = None, 1 = Mono, 2 = Poly;
    size: u8,
    stream: TokenStream,
}

impl Idents {
    fn add(&mut self, s: &str) {
        // Poly
        if self.size > 0 {
            self.stream.punct(',');
        }
        self.stream.add(Literal::string(s));
        if self.size < 2 {
            self.size += 1;
        }
    }

    fn take(&mut self) -> Self {
        mem::take(self)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn write(self, o: &mut TokenStream, span: Option<&Ident>, var: Ident) {
        o.add(var);
        o.punct('.');

        match self.size {
            1 => {
                o.ident(if span.is_some() {
                    "add_ident_span"
                } else {
                    "add_ident"
                });
                o.group(Delimiter::Parenthesis, |o| {
                    add_span(o, span);
                    o.extend(self.stream);
                });
            }
            _ => {
                o.ident(if span.is_some() {
                    "add_idents_span"
                } else {
                    "add_idents"
                });
                o.group(Delimiter::Parenthesis, |o| {
                    add_span(o, span);
                    o.punct('&');
                    o.add(Group::new(Delimiter::Bracket, self.stream));
                });
            }
        }
        o.punct(';');
    }
}
