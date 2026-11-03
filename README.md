# `mayhaps`

[![Crates.io](https://img.shields.io/crates/v/mayhaps)](https://crates.io/crates/mayhaps) [![docs.rs](https://img.shields.io/docsrs/mayhaps)](https://docs.rs/mayhaps)

> Convenience macros for uncertain times, one `macro_rules!` for working with an `Option`.

`maybe!` runs a block when an `Option` holds a value and binds the unwrapped value to the name you give it, with an `else` form for the empty case. It is `macro_rules!` and nothing else, expanding to a `let` and an `if let`, with no dependencies.

## Usage

Not published to crates.io yet, so it is a git dependency for now.

```toml
[dependencies]
mayhaps = { git = "https://github.com/orgrinrt/mayhaps", branch = "dev" }
```

```rust
use mayhaps::maybe;

let opt = Some(5);
maybe!(val from (opt) exists {
    println!("got {}", val); // runs only when opt is Some
});

let none: Option<i32> = None;
maybe!(val from (none) exists {
    println!("got {}", val);
} else {
    println!("nothing there"); // runs when none is None
});
```

The intermediate binding the macro introduces carries the macro's own hygiene, so the name you pick can't collide with anything, and `tests/hygiene.rs` uses the same word for the caller's variable, the bound name and the macro's own. The `else` block can compute a fallback from something else, and `cargo run --example config_lookup` reads a configuration with half its keys missing.

The `no_std` and `no_alloc` features only name what is already true, and `tests/feature_matrix.rs` builds under each and compiles a `#![no_std]` consumer against it.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/mayhaps/blob/dev/LICENSE)
