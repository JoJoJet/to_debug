//! This crate exports the [`ToDebug`]  trait, which is an alternative to
//! [`ToString`] that uses [`Debug`] instead of [`Display`].
//!
//! This can be useful for writing doctests, as it allows you to inspect
//! the values of private fields:
//!
//! ```
//! # use to_debug::ToDebug;
//! mod private {
//!     #[derive(Debug)]
//!     pub struct Person { name: String, age: u16 }
//!     // constructor boilerplate...
//! #   impl Person {
//! #       pub fn new(name: impl Into<String>, age: u16) -> Self {
//! #           Self { name: name.into(), age }
//! #       }
//! #   }
//! }
//! let p = private::Person::new("Joseph", 20);
//! // assert_eq!(p.name, "Joseph"); // This would fail since `name` is private.
//! assert_eq!(p.to_debug(), r#"Person { name: "Joseph", age: 20 }"#);
//! ```
//!
//! [`Debug`]: core::fmt::Debug
//! [`Display`]: core::fmt::Display

use std::fmt;
/// A trait for converting a value to a `String` using the [`Debug`] trait.
///
/// This trait is automatically implemented for any type which implements the
/// [`Debug`] trait. As such, `ToDebug` shouldn’t be implemented directly:
/// [`Debug`] should be implemented instead, and you get the `ToDebug`
/// implementation for free.
///
/// [`Debug`]: core::fmt::Debug
pub trait ToDebug {
    /// Converts the given value to a `String` using the [`Debug`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// # use to_debug::ToDebug;
    /// #[derive(Debug)]
    /// struct Years(u64);
    /// 
    /// let y = Years(18);
    /// assert_eq!(y.to_debug(), "Years(18)");
    /// ```
    ///
    /// [`Debug`]: core::fmt::Debug
    fn to_debug(&self) -> String;
}
impl<T: fmt::Debug + ?Sized> ToDebug for T {
    #[inline]
    fn to_debug(&self) -> String {
        // Code shamelessly stolen from https://internals.rust-lang.org/t/to-debug-a-debug-counterpart-of-to-string/11228
        use fmt::Write;
        let mut buf = String::new();
        buf.write_fmt(format_args!("{:?}", self))
           .expect("a Debug implementation returned an error unexpectedly");
        buf.shrink_to_fit();
        buf
    }
}
