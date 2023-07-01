An alternative lightweight version of [quote](https://github.com/dtolnay/quote).

Unlike `quote', this library avoids cloning whenever possible. 


```rust
use quote2::{proc_macro2::TokenStream, quote, TokensExt};

let mut tokens = TokenStream::new();
let body = quote(|tokens| {
    for i in 0..3 {
        quote!(tokens, {
            println!("{}", #i);
        });
    }
});
quote!(tokens, {
    fn name() {
        #body
    }
});
```

Generated code:

```rust
fn name() {
    println!("{}", 0i32);
    println!("{}", 1i32);
    println!("{}", 2i32);
}
```