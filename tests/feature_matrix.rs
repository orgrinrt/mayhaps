//! Every feature selection this crate claims to support, built, plus the claim that
//! `no_std` is real rather than decorative.
//!
//! A feature that is never built is a claim nobody checked, and this crate's two are
//! easier to get wrong than most, because neither is supposed to change anything. A
//! `no_std` that does not actually forbid `std` looks identical to one that does, from
//! inside a crate that never reaches for `std` either way.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Builds the crate under one feature selection.
fn check(label: &str, args: &[&str]) {
    let output = Command::new(env!("CARGO"))
        .arg("check")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"))
        .output()
        .unwrap_or_else(|e| panic!("could not run cargo for {label}: {e}"));

    assert!(
        output.status.success(),
        "{label} does not build\n--- cargo said\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
}

/// Builds a throwaway crate that uses `body`, against this crate at `features`.
///
/// Whether the macro still expands is a question about a consumer rather than about this
/// crate, since nothing here invokes it outside the tests.
fn consumer_compiles(name: &str, features: &str, attrs: &str, body: &str) -> (bool, String) {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers")).join(name);
    fs::create_dir_all(root.join("src")).expect("the consumer directory");

    let features_list = if features.is_empty() {
        String::new()
    } else {
        features.split(',').map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(", ")
    };

    fs::write(
        root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[dependencies.mayhaps]
path = "{crate_dir}"
default-features = false
features = [{features_list}]

[workspace]
"#,
            crate_dir = env!("CARGO_MANIFEST_DIR"),
        ),
    )
    .expect("the consumer manifest");

    fs::write(
        root.join("src").join("lib.rs"),
        format!("{attrs}\nuse mayhaps::maybe;\n\npub fn exercise() {{\n{body}\n}}\n"),
    )
    .expect("the consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--quiet"])
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers/target"))
        .output()
        .expect("cargo runs");

    (output.status.success(), String::from_utf8_lossy(&output.stderr).to_string())
}

#[test]
fn the_default_selection_builds() {
    check("default", &[]);
}

#[test]
fn no_std_builds() {
    check("no_std", &["--no-default-features", "--features", "no_std"]);
}

#[test]
fn no_alloc_builds_and_implies_no_std() {
    check("no_alloc", &["--no-default-features", "--features", "no_alloc"]);

    // `no_alloc = ["no_std"]`, so naming both is naming the same thing and must not
    // conflict. A consumer whose workspace turns `no_std` on everywhere arrives here.
    check("no_alloc + no_std", &["--no-default-features", "--features", "no_alloc,no_std"]);
}

#[test]
fn the_macro_still_expands_in_a_no_std_consumer() {
    // The thing the features exist for. A `no_std` consumer is where an accidental `std`
    // path in the expansion would show up, and nothing inside this crate can see it,
    // because this crate is not the one that gets compiled without `std`.
    let (ok, err) = consumer_compiles(
        "no_std_consumer",
        "no_alloc",
        "#![no_std]",
        r#"    let opt = Some(5u8);
    let mut seen = 0u8;
    maybe!(v from (opt) exists { seen = v; });
    let _ = seen;

    let absent: Option<u8> = None;
    let which;
    maybe!(v from (absent) exists { let _ = v; which = 1u8; } else { which = 2u8; });
    let _ = which;"#,
    );
    assert!(ok, "the macro expands in a `#![no_std]` consumer:\n{err}");
}

#[test]
fn a_no_std_consumer_really_is_without_std() {
    // The control for the test above. Without it that test would pass just as well against
    // a consumer where `#![no_std]` did nothing, and the whole point of the feature is
    // that it does.
    let (ok, err) = consumer_compiles(
        "no_std_control",
        "no_alloc",
        "#![no_std]",
        r#"    let _ = std::vec::Vec::<u8>::new();"#,
    );
    assert!(!ok, "a `#![no_std]` consumer must not reach `std::vec`");
    assert!(
        err.contains("std") || err.contains("no_std"),
        "the error is about `std` being absent:\n{err}",
    );
}

#[test]
fn the_crate_has_no_dependencies() {
    // `paste` was the only one, and it was there to build a uniquely-named intermediate
    // binding that macro hygiene already guarantees. Pinned, because a dependency is easy
    // to add back and this crate's value in a `no_std` build is partly that it has none.
    let manifest = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("the manifest");

    let deps = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("a dependencies section")
        // Stop at the next section header.
        .split("\n[")
        .next()
        .expect("the section body");

    let named: Vec<&str> = deps
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    assert!(named.is_empty(), "the crate has picked up dependencies: {named:?}");
}
