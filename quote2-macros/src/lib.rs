mod tokens;
use std::mem;

use proc_macro::*;
use tokens::Tokens;

#[proc_macro]
pub fn quote(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    let mut input = input.into_iter();

    let Some(TokenTree::Ident(var)) = input.next() else { panic!("expected `ident`") };
    input.next().expect("expected `,`");

    let input = match input.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => panic!("expected `{{`"),
    };
    expend(input, &mut output, var);
    output
}

fn expend(input: TokenStream, o: &mut TokenStream, var: Ident) {
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
            idents.take().write(o, var.clone());
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
                        o.group(Delimiter::Parenthesis, |o| o.extend(v));
                    }
                    Some(TokenTree::Punct(next))
                        if curr.spacing() == Spacing::Joint
                            && next.spacing() == Spacing::Alone
                            && next.as_char() != '#' =>
                    {
                        let next_ch = next.as_char();
                        peek = input.next();
                        o.ident("add_puncts");
                        o.group(Delimiter::Parenthesis, |o| {
                            if ch == next_ch {
                                o.char(ch);
                            } else {
                                o.group(Delimiter::Parenthesis, |o| {
                                    o.char(ch);
                                    o.punct(',');
                                    o.char(next_ch);
                                })
                            }
                        });
                    }
                    _ => {
                        match curr.spacing() {
                            Spacing::Joint => o.ident("add_punct_join"),
                            Spacing::Alone => o.ident("add_punct"),
                        }
                        o.group(Delimiter::Parenthesis, |o| o.char(ch));
                    }
                }
            }
            TokenTree::Group(group) => {
                o.ident("add_group");
                o.group(Delimiter::Parenthesis, |o| {
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
                        expend(group.stream(), o, var);
                    });
                });
            }
            TokenTree::Literal(lit) => {
                let s = lit.to_string();
                let (method_name, lit) = if s.chars().next().is_some_and(char::is_numeric) {
                    ("add_parsed_lit", Literal::string(&s))
                } else {
                    ("add_tokens", lit)
                };
                o.ident(method_name);
                o.group(Delimiter::Parenthesis, |o| o.add(lit));
            }
            TokenTree::Ident(_) => {}
        }
        o.punct(';');
    }
    if !idents.is_empty() {
        idents.write(o, var);
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

    fn write(self, o: &mut TokenStream, var: Ident) {
        o.add(var);
        o.punct('.');

        match self.size {
            1 => {
                o.ident("add_ident");
                o.add(Group::new(Delimiter::Parenthesis, self.stream))
            }
            _ => {
                o.ident("add_idents");
                o.group(Delimiter::Parenthesis, |o| {
                    o.punct('&');
                    o.add(Group::new(Delimiter::Bracket, self.stream));
                })
            }
        }
        o.punct(';');
    }
}
