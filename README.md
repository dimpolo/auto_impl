# `auto_impl` [![Join the chat at https://gitter.im/auto-impl-rs/Lobby](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/auto-impl-rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) [![Build Status](https://travis-ci.org/auto-impl-rs/auto_impl.svg?branch=master)](https://travis-ci.org/auto-impl-rs/auto_impl) [![Crates.io](https://img.shields.io/crates/v/auto_impl.svg)](https://crates.io/crates/auto_impl) [![docs](https://docs.rs/auto_impl/badge.svg)](https://docs.rs/auto_impl)

A proc-macro attribute for automatically implementing a trait for references,
some common smart pointers and closures.

# Usage

This library requires Rust 1.30.0 or newer.

Add `auto_impl` to your `Cargo.toml` and just use it in your crate:

```rust
// In Rust 2015 you still need `extern crate auto_impl;` at your crate root
use auto_impl::auto_impl;
```

Add an `auto_impl` attribute to traits you want to automatically implement for wrapper types. Here is a small example:

```rust
// This will generate two additional impl blocks: one `for &T` and one
// `for Box<T>` where `T: Foo`.
#[auto_impl(&, Box)]
trait Foo {
    fn foo(&self);
}

impl Foo for i32 {
    fn foo(&self) {}
}

fn requires_foo(_: impl Foo) {}


requires_foo(0i32);  // works: through the impl we defined above
requires_foo(&0i32); // works: through the generated impl
requires_foo(Box::new(0i32)); // works: through the generated impl
```

For more explanations, please see [**the documentation**](https://docs.rs/auto_impl) and for more examples, see [the examples folder](https://github.com/auto-impl-rs/auto_impl/tree/master/examples).


---

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
