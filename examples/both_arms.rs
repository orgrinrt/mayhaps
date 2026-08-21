//! What `maybe!` does, in both of its forms.
//!
//! ```text
//! cargo run --example both_arms
//! ```

use mayhaps::maybe;

fn main() {
    println!("The plain form runs the block only when the Option holds something.\n");

    for opt in [Some(5u8), None] {
        // Nothing runs for `None`, so the line below is the whole report for that case.
        print!("{opt:?} -> ");
        maybe!(value from (opt) exists {
            print!("ran with {value}");
        });
        println!();
    }

    println!("\nThe else form has somewhere to go when it does not.\n");

    for opt in [Some(5u8), None] {
        print!("{opt:?} -> ");
        maybe!(value from (opt) exists {
            print!("ran with {value}");
        } else {
            print!("ran the else");
        });
        println!();
    }

    println!("\nThe bound name is yours, and it does not collide with anything.\n");

    // `held` is what the macro calls its own intermediate binding. Using it here for the
    // caller's variable and for the bound name at once is fine: the macro's identifier
    // carries the macro's hygiene, and these carry this file's.
    let held = 99u8;
    let opt = Some(7u8);
    maybe!(held from (opt) exists {
        println!("bound `held` is {held}, from the Option");
    });
    println!("the outer `held` is still {held}");
}
