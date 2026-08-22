# `mayhaps`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/mayhaps.svg)](https://github.com/orgrinrt/mayhaps/stargazers)
[![Crates.io](https://img.shields.io/crates/v/mayhaps)](https://crates.io/crates/mayhaps)
[![docs.rs](https://img.shields.io/docsrs/mayhaps)](https://docs.rs/mayhaps)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/mayhaps.svg)](https://github.com/orgrinrt/mayhaps/issues)
![License](https://img.shields.io/github/license/orgrinrt/mayhaps?color=%23009689)

> Convenience macros for those uncertain times.

</div>

## Usage

The crate exports one macro, `maybe!`. It runs a block when an `Option` holds a value, binding the unwrapped value to the given name:

```rust
use mayhaps::maybe;

let opt = Some(5);
maybe!(val from (opt) exists {
    println!("got {}", val); // runs only when opt is Some
});
```

An `else` variant runs a second block when the `Option` is empty:

```rust
use mayhaps::maybe;

let opt: Option<i32> = None;
maybe!(val from (opt) exists {
    println!("got {}", val);
} else {
    println!("nothing there"); // runs when opt is None
});
```

The bound name is yours and cannot collide with anything: the intermediate binding the
macro introduces carries the macro's own hygiene, so `maybe!(held from (opt) exists {..})`
works even where `held` is also the caller's variable and also what the macro calls its
own. `tests/hygiene.rs` uses the same word for all three at once.

---

## Features

Neither changes what the crate does, because there is nothing here to change: it is
`macro_rules!` and nothing else, and what it expands into is a `let` and an `if let`.

| Feature | Effect |
|---|---|
| `no_std` | Adds `#![no_std]`. |
| `no_alloc` | Implies `no_std`. States what is already true. |

They exist so a consumer whose workspace turns them on everywhere can name them, and so
the claim is checked rather than believed. `tests/feature_matrix.rs` builds under each
selection, and compiles a `#![no_std]` consumer crate against it to confirm the macro
still expands there, with a control confirming that consumer really is without `std`.

The crate has no dependencies. `paste` was the only one, and it was there to build a
uniquely-named intermediate binding that macro hygiene already guarantees.

---

## Examples

```text
cargo run --example both_arms
cargo run --example config_lookup
```

The first is each form in each state, four cases. The second reads a configuration where
half the keys are missing, which is where the `else` form earns its keep: the fallback is a
block, so it can compute a default from another setting rather than only supply a constant.
Both are run by `cargo test`, in `tests/examples_run.rs`, which checks what they print.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/mayhaps/blob/dev/LICENSE)
