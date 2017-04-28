use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// Structure created by lazy_cat! macro.
///
/// Implements Display and ToString.  It is recommended to use this in generics,
/// as the generic arguments A and B are typically recursive.
/// This structure will consume everything given to the lazy_cat! macro
/// then when a string version of this is requested it will be built on the fly by using the
/// Display implementations of everything it has consumed.
pub struct LazyStr<A: Display + Sized, B: Display + Sized> {
    a: A,
    b: B,
}

impl<A: Display + Sized, B: Display + Sized> Debug for LazyStr<A, B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.a.fmt(f)?;
        self.b.fmt(f)?;
        Ok(())
    }
}

impl<A: Display + Sized, B: Display + Sized> Display for LazyStr<A, B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.a.fmt(f)?;
        self.b.fmt(f)?;
        Ok(())
    }
}

impl<A: Display + Sized, B: Display + Sized> LazyStr<A, B> {
    pub fn new(a: A, b: B) -> LazyStr<A, B> {
        LazyStr {
            a,
            b,
        }
    }
}

/// Creates a LazyStr from several things that are Display + Sized.
///
/// Examples:
/// ```Rust
/// assert_eq!("Hello world!", lazy_cat!("Hello", " world!").to_string());
/// assert_eq!("Hello John Doe!", lazy_cat!("Hello ", "John ", "Doe!").to_string());
/// assert_eq!("123Hello", lazy_cat!(1, 2, 3, "Hello").to_string());
/// ```
#[macro_export]
macro_rules! lazy_cat {
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        LazyStr::new($x, lazy_cat!($($y),+))
    )
}

#[cfg(test)]
mod tests {
    use LazyStr;
    #[test]
    fn it_works() {
        assert_eq!("Hello world!", lazy_cat!("Hello", " world!").to_string());
        assert_eq!("Hello John Doe!", lazy_cat!("Hello ", "John ", "Doe!").to_string());
        assert_eq!("123Hello", lazy_cat!(1, 2, 3, "Hello").to_string());
    }
}
