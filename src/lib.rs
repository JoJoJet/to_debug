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
