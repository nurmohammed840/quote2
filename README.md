## quote2

An alternative lightweight version of [quote](https://github.com/dtolnay/quote).

[![Crates.io][crates-badge]][crates-url]
[![Documentation](https://docs.rs/quote2/badge.svg)](https://docs.rs/quote2)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

[crates-badge]: https://img.shields.io/crates/v/quote2.svg
[crates-url]: https://crates.io/crates/quote2

## Features

- Very lightweight and produces extremely lean, minimal code compare to `quote`
- Unlike `quote`, `quote2` allows direct mutation of tokens without creating new
  `TokenStream` instances, enhancing runtime performance.

  similar to [write](https://doc.rust-lang.org/std/macro.write.html) macro.

### Example

Add it as a dependency to your Rust project by adding the following line to your
`Cargo.toml` file:

```toml
[dependencies]
quote2 = "0.8"
```

```rust
use quote2::{proc_macro2::TokenStream, quote, Quote};

let body = quote(|t| {
    for n in 1..7 {
        if n % 2 == 0 {
            quote!(t, {
                println!("{}", #n);
            });
        }
    }
});

let mut t = TokenStream::new();
quote!(t, {
    fn main() {
        #body
    }
});
```

Generated code:

```rust
fn main() {
    println!("{}", 2i32);
    println!("{}", 4i32);
    println!("{}", 6i32);
}
```
