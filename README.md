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

# Contributing

I welcome contributions and alterations to this project! [Here's some info to help you get started.](https://help.github.com/articles/about-pull-requests/)

- If you would consider your change substantial open an issue on the issues tab so we can discuss it before you build it.
- If you're fixing a bug please provide a unit test for the bug fixed so we don't do it again.
- If applicable, new features should have some basic unit tests added too.
- Please format your code with the most recent stable release of rustfmt before submitting your PR.
- I don't have a regular release schedule, if you want something you've added put on crates.io right away make sure to
bump the version number for the project in your pull request.
