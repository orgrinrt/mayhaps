//! Convenience macros for those uncertain times.
//!
//! One macro, [`maybe!`], which runs a block when an `Option` holds a value and binds the
//! value to a name you choose.
//!
//! # Allocation
//!
//! Both positions are features, and neither changes what the crate does, because there is
//! nothing here to change: it is `macro_rules!` and nothing else, it names no type, calls
//! no function, and touches neither `std` nor an allocator at any point. What it expands
//! into is a `let` and an `if let`, which are whatever the surrounding code already was.
//!
//! | Feature | Effect |
//! |---|---|
//! | `no_std` | Adds `#![no_std]`. |
//! | `no_alloc` | Implies `no_std`. States what is already true. |
//!
//! They exist so a consumer whose workspace turns them on everywhere can name them, and so
//! the claim is checked rather than believed: `tests/feature_matrix.rs` builds under each
//! selection and asserts the macro still expands.
//!
//! The crate has no dependencies.

#![cfg_attr(feature = "no_std", no_std)]

/// Runs a block when an `Option` holds a value, binding the value to `$name`.
///
/// ```
/// use mayhaps::maybe;
///
/// let opt = Some(5);
/// let mut seen = 0;
/// maybe!(val from (opt) exists {
///     seen = val;
/// });
/// assert_eq!(seen, 5);
/// ```
///
/// With an `else`, which runs when the `Option` is empty:
///
/// ```
/// use mayhaps::maybe;
///
/// let opt: Option<i32> = None;
/// let mut which = "";
/// maybe!(val from (opt) exists {
///     let _ = val;
///     which = "present";
/// } else {
///     which = "absent";
/// });
/// assert_eq!(which, "absent");
/// ```
///
/// The expression is evaluated once, before the match, so a call in that position happens
/// whether or not the block runs, and happens only once:
///
/// ```
/// use mayhaps::maybe;
///
/// let mut calls = 0;
/// {
///     let mut evaluate = |v| { calls += 1; v };
///     maybe!(val from (evaluate(Some(1))) exists { let _ = val; });
///     maybe!(val from (evaluate(None::<i32>)) exists { let _ = val; });
/// }
///
/// assert_eq!(calls, 2, "once per invocation, present or not");
/// ```
#[macro_export]
macro_rules! maybe {
    ($name:ident from ($opt_var:expr) exists $body:block) => {
        // The intermediate binding is the macro's own, so its name cannot collide with
        // `$name` or with anything the caller has: a literal identifier here carries this
        // macro's hygiene, and `$name` carries the call site's. `tests/hygiene.rs` uses the
        // same word for all three and pins it.
        //
        // This used to go through `paste!` to build a `maybe_$name` binding, which is what
        // that dependency was for and the only thing it was for. Hygiene was already doing
        // the work, so the crate has no dependencies now.
        let held = $opt_var;
        if let Some($name) = held $body
    };
    ($name:ident from ($opt_var:expr) exists $body:block else $else_body:block) => {
        let held = $opt_var;
        if let Some($name) = held $body else $else_body
    };
}
