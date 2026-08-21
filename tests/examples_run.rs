//! The examples are built by `cargo test` and never run by it, so they are run here.
//!
//! An example that compiles and then prints the wrong thing is an example that lies to
//! whoever copies it, and for a macro crate that is the entire risk: the wrong arm running
//! is exactly the defect this macro shipped with once.

use std::process::Command;

/// Runs one example and returns what it printed.
fn run_example(name: &str) -> String {
    let output = Command::new(env!("CARGO"))
        .args(["run", "-q", "--example", name])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/examples"))
        .output()
        .unwrap_or_else(|e| panic!("could not run example {name}: {e}"));

    assert!(
        output.status.success(),
        "example {name} exited {}\n--- stderr\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8(output.stdout).expect("example printed something that is not utf-8")
}

#[test]
fn both_arms_runs_each_arm_exactly_when_it_should() {
    let out = run_example("both_arms");

    assert!(out.contains("Some(5) -> ran with 5"), "the present arm did not run:\n{out}");
    assert!(out.contains("None -> ran the else"), "the else arm did not run:\n{out}");

    // The defect this macro shipped with: the else body ran unconditionally, so `Some`
    // printed both. One line, both texts, is what that looked like.
    assert!(
        !out.contains("Some(5) -> ran with 5ran the else"),
        "the else body ran on the present arm too:\n{out}",
    );

    // And the plain form runs nothing at all for `None`, rather than running the block
    // with some default.
    assert!(out.contains("None -> \n"), "the plain form ran on an absent value:\n{out}");

    // Hygiene, from the third section.
    assert!(out.contains("bound `held` is 7"), "{out}");
    assert!(out.contains("the outer `held` is still 99"), "{out}");
}

#[test]
fn config_lookup_takes_the_file_value_or_the_default_per_key() {
    let out = run_example("config_lookup");

    // Two keys the file had, two it did not, so both arms are exercised twice and a macro
    // that always took one arm fails here rather than in only one direction.
    assert!(out.contains("port     8443 (from the file)"), "{out}");
    assert!(out.contains("retries  3 (from the file)"), "{out}");
    assert!(out.contains("host     localhost (default"), "{out}");
    assert!(out.contains("tls      true (derived from the port"), "{out}");

    assert!(out.contains("connecting to localhost:8443, tls true, up to 3 retries"), "{out}");

    // The plain form over three values, two present.
    assert!(out.contains("flag     verbose"), "{out}");
    assert!(out.contains("flag     dry-run"), "{out}");
    assert_eq!(out.matches("flag     ").count(), 2, "the absent flag printed:\n{out}");
}
