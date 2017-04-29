# lazy_cat [![Crates listing](https://img.shields.io/crates/v/lazy_cat.svg)](https://crates.io/crates/lazy_cat)
Lazy concatenation of strings and other things in Rust.

[Documentation](https://xaeroxe.github.io/lazy_cat/lazy_cat/index.html)

Example:

```Rust
#[macro_use]
extern crate lazy_cat;

fn main() {
  assert_eq!("Hello world!", lazy_cat!("Hello", " world!").to_string());
  assert_eq!("Hello John Doe!", lazy_cat!("Hello ", "John ", "Doe!").to_string());
  assert_eq!("123Hello", lazy_cat!(1, 2, 3, "Hello").to_string());
}

```

