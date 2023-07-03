mod tokens;
use std::mem;

use proc_macro::*;
use tokens::Tokens;

#[proc_macro]
pub fn quote(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    let mut input = input.into_iter();

    let var = input.next().expect("expected: ident");
    input.next().expect("expected `,`");

    let input = match input.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => panic!("expected `{{`"),
    };
    expend(input, &mut output, var);
    output
}

fn expend(input: TokenStream, o: &mut TokenStream, var: TokenTree) {
    let mut input = input.into_iter();

    let mut idents_len: u16 = 0; // 0 = None, 1 = Mono, 2 = Poly;
    let mut idents = TokenStream::new();

    let mut peek = input.next();
    while let Some(tt) = peek.take() {
        peek = input.next();

        if let TokenTree::Ident(name) = tt {
            // Poly
            if idents_len > 0 {
                idents.punct(',');
            }
            idents.add(Literal::string(&name.to_string()));
            if idents_len < 2 {
                idents_len += 1;
            }
            continue;
        }
        if idents_len != 0 {
            let idents = mem::replace(&mut idents, TokenStream::new());

            o.add(var.clone());
            o.punct('.');

            match idents_len {
                1 => {
                    o.ident("ident");
                    o.add(Group::new(Delimiter::Parenthesis, idents))
                }
                _ => {
                    o.ident("idents");
                    o.group(Delimiter::Parenthesis, |o| {
                        o.punct('&');
                        o.add(Group::new(Delimiter::Bracket, idents));
                    })
                }
            }
            o.punct(';');
            idents_len = 0;
        }

        o.add(var.clone());
        o.punct('.');
        match tt {
            TokenTree::Ident(_) => {}
            TokenTree::Punct(cur) => {
                let ch = cur.as_char();
                match peek {
                    Some(TokenTree::Ident(_)) if ch == '#' => {
                        let v = mem::replace(&mut peek, input.next());
                        o.ident("tokens");
                        o.group(Delimiter::Parenthesis, |o| o.extend(v));
                    }
                    Some(TokenTree::Punct(next))
                        if cur.spacing() == Spacing::Joint && next.spacing() == Spacing::Alone =>
                    {
                        let next_ch = next.as_char();
                        let is_same = ch == next_ch;
                        peek = input.next();
                        o.ident(if is_same { "punct2" } else { "punct_joined" });
                        o.group(Delimiter::Parenthesis, |o| {
                            o.char(ch);
                            if !is_same {
                                o.punct(',');
                                o.char(next_ch);
                            }
                        });
                    }
                    _ => {
                        match cur.spacing() {
                            Spacing::Joint => o.ident("punct_join"),
                            Spacing::Alone => o.ident("punct"),
                        }
                        o.group(Delimiter::Parenthesis, |o| o.char(ch));
                    }
                }
            }
            TokenTree::Group(group) => {
                o.ident("group");
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
                        expend(group.stream(), o, var.into());
                    });
                });
            }
            TokenTree::Literal(lit) => {
                o.ident("tokens");
                o.group(Delimiter::Parenthesis, |o| o.add(lit));
            }
        }
        o.punct(';');
    }
}
