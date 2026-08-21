//! The macro's own binding cannot collide with anything the caller has.
//!
//! `maybe!` binds the expression to an intermediate before matching on it. That binding has
//! a literal name in the macro body, so it carries the macro's hygiene while `$name` and
//! everything at the call site carry the call site's. The two are separate, which is what
//! makes a `paste!`-built unique name unnecessary and is why this crate has no
//! dependencies.
//!
//! It is one word, `held`, and these use it for all three at once.

use mayhaps::maybe;

#[test]
fn the_bound_name_may_be_the_macros_own_word() {
    let opt = Some(7u8);
    let mut seen = 0u8;

    // `$name` is `held`, which is also what the macro calls its intermediate. If they were
    // one binding this would see the `Option` rather than the value inside it, and would
    // not compile.
    maybe!(held from (opt) exists { seen = held; });

    assert_eq!(seen, 7);
}

#[test]
fn a_callers_variable_of_the_same_name_survives() {
    let held = 99u8;
    let opt = Some(7u8);
    let mut seen = 0u8;

    maybe!(held from (opt) exists { seen = held; });

    assert_eq!(seen, 7, "the macro's binding shadowed the bound name");
    assert_eq!(held, 99, "the macro's binding clobbered the caller's variable");
}

#[test]
fn the_else_arm_has_the_same_property() {
    let held = 99u8;
    let opt: Option<u8> = None;
    // Uninitialised on purpose: both arms assign, so this compiling at all is the
    // compiler proving the expansion has no path that assigns neither.
    let which;

    maybe!(held from (opt) exists {
        let _ = held;
        which = "present";
    } else {
        // The caller's `held` is what is visible here, because the absent branch binds
        // nothing.
        which = if held == 99 { "absent" } else { "absent, but clobbered" };
    });

    assert_eq!(which, "absent");
    assert_eq!(held, 99);
}

#[test]
fn the_expression_may_name_the_callers_variable_of_that_name() {
    // The expression is evaluated at the call site, so it sees the call site's `held`
    // rather than the one the macro is about to introduce. Without hygiene this would be
    // a use-before-definition or, worse, would silently read the wrong one.
    let held = Some(4u8);
    let mut seen = 0u8;

    maybe!(v from (held) exists { seen = v; });

    assert_eq!(seen, 4);
}

#[test]
fn nesting_one_inside_another_keeps_them_apart() {
    // Two expansions in the same scope, each with its own intermediate under the same
    // literal name. The inner one must not disturb the outer one's.
    let outer = Some(1u8);
    let inner = Some(2u8);
    let mut sum = 0u8;

    maybe!(a from (outer) exists {
        maybe!(b from (inner) exists {
            sum = a + b;
        });
        // `a` is still the outer binding after the inner expansion has run.
        sum += a;
    });

    assert_eq!(sum, 4, "1 + 2, then + 1");
}
