# to_debug

This crate exports the [`ToDebug`]  trait, which is an alternative to
[`ToString`] that uses [`Debug`] instead of [`Display`].

This can be useful for writing doctests, as it allows you to inspect
the values of private fields:

```rust
mod private {
    #[derive(Debug)]
    pub struct Person { name: String, age: u16 }
    // constructor boilerplate...
}
let p = private::Person::new("Joseph", 20);
// assert_eq!(p.name, "Joseph"); // This would fail since `name` is private.
assert_eq!(p.to_debug(), r#"Person { name: "Joseph", age: 20 }"#);
```

[`Debug`]: core::fmt::Debug
[`Display`]: core::fmt::Display

License: MIT
