//! A worked case: reading settings that may or may not be there.
//!
//! Several `Option`s in a row, each with a fallback, which is where the else form earns
//! its keep. The alternative is `if let` with the same three lines of ceremony each time,
//! or `unwrap_or`, which cannot run a block.
//!
//! ```text
//! cargo run --example config_lookup
//! ```

use mayhaps::maybe;

/// What a partially-filled configuration file might parse to.
struct Config {
    port:    Option<u16>,
    host:    Option<&'static str>,
    retries: Option<u8>,
    tls:     Option<bool>,
}

fn main() {
    let from_file = Config {
        port:    Some(8443),
        host:    None,
        retries: Some(3),
        tls:     None,
    };

    println!("Reading a configuration where half the keys are missing.\n");

    let port;
    maybe!(value from (from_file.port) exists {
        println!("port     {value} (from the file)");
        port = value;
    } else {
        println!("port     8080 (default, the file did not say)");
        port = 8080;
    });

    let host;
    maybe!(value from (from_file.host) exists {
        println!("host     {value} (from the file)");
        host = value;
    } else {
        println!("host     localhost (default, the file did not say)");
        host = "localhost";
    });

    let retries;
    maybe!(value from (from_file.retries) exists {
        println!("retries  {value} (from the file)");
        retries = value;
    } else {
        println!("retries  1 (default, the file did not say)");
        retries = 1;
    });

    let tls;
    maybe!(value from (from_file.tls) exists {
        println!("tls      {value} (from the file)");
        tls = value;
    } else {
        // The default depends on another setting, which is the case `unwrap_or` cannot
        // reach without computing the fallback whether or not it is needed.
        tls = port == 443 || port == 8443;
        println!("tls      {tls} (derived from the port, since the file did not say)");
    });

    println!("\nconnecting to {host}:{port}, tls {tls}, up to {retries} retries");

    println!("\nThe plain form, where a missing value simply means nothing happens.\n");

    // No else, because there is nothing to do when a setting is absent.
    let overrides: [Option<&str>; 3] = [Some("verbose"), None, Some("dry-run")];
    for over in overrides {
        maybe!(flag from (over) exists {
            println!("flag     {flag}");
        });
    }
}
