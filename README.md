mayhaps
============
[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/mayhaps.svg)](https://github.com/orgrinrt/mayhaps/stargazers) 
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/mayhaps.svg)](https://github.com/orgrinrt/mayhaps/issues) 
[![Current Version](https://img.shields.io/badge/version-0.1.0-orange.svg)](https://github.com/orgrinrt/mayhaps) 

Convenience macros for those uncertain times.

---
## Buy me a coffee

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

---

## Usage

The crate exports one macro, `maybe!`. It runs a block when an `Option` holds a value, binding the unwrapped value to the given name:

```rust
use mayhaps::maybe;

let opt = Some(5);
maybe!(val from (opt) exists {
    println!("got {}", val); // runs only when opt is Some
});
```

The source also contains an `else` variant of `maybe!`, but it does not currently compile; only the form above is usable.

---

## License
>You can check out the full license [here](https://github.com/orgrinrt/mayhaps/blob/main/LICENSE)

This project is licensed under the terms of the **MIT** license.
