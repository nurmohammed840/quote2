mod tokens;
use proc_macro::{token_stream::IntoIter, *};
use tokens::Tokens;

#[proc_macro]
pub fn quote(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    let mut input = input.into_iter();

    let target = input.next().expect("expected: ident");
    input.next().expect("expected `,`");

    let input = match input.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => panic!("expected `{{`"),
    };
    output.group('{', |s| {
        expend(input.into_iter(), s, target);
    });
    output
}

fn expend(input: IntoIter, s: &mut TokenStream, targer: TokenTree) {
    let mut input = input.peekable();
    while let Some(tt) = input.next() {
        let targer = &targer;
        s.add(targer.clone());
        s.punct('.');
        match tt {
            TokenTree::Group(group) => {
                s.ident("group");
                s.group('(', |s| {
                    s.ch(match group.delimiter() {
                        Delimiter::None => '_',
                        Delimiter::Brace => '{',
                        Delimiter::Bracket => '[',
                        Delimiter::Parenthesis => '(',
                    });
                    s.punct(',');
                    s.punct('|');
                    s.ident("__s");
                    s.punct('|');
                    s.group('{', |s| {
                        let targer = Ident::new("__s", Span::call_site());
                        expend(group.stream().into_iter(), s, targer.into());
                    });
                });
            }
            TokenTree::Ident(ident) => {
                s.ident("ident");
                s.group('(', |s| s.str(&ident.to_string()))
            }
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                if ch == '#' {
                    if let Some(TokenTree::Ident(_)) = input.peek() {
                        let Some(TokenTree::Ident(var)) = input.next() else { unreachable!() };
                        s.ident("tokens");
                        s.group('(', |s| s.add(var));
                        s.punct(';');
                        continue;
                    }
                }
                match punct.spacing() {
                    Spacing::Joint => s.ident("punct_join"),
                    Spacing::Alone => s.ident("punct"),
                }
                s.group('(', |s| s.ch(ch));
            }
            TokenTree::Literal(lit) => {
                s.ident("tokens");
                s.group('(', |s| s.add(lit));
            }
        }
        s.punct(';');
    }
}
