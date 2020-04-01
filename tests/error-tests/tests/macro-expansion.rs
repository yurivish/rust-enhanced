#[macro_use]
extern crate dcrate;

fn main() {
    // This ensures that the expansion of nested macros works correctly.

    example!(inner!(b" "));
//  ^^^^^^^^^^^^^^^^^^^^^^^ERR(<1.44.0-beta) no method named `missing`
//  ^^^^^^^^^^^^^^^^^^^^^^^ERR(<1.44.0-beta) this error originates in a macro outside of the current crate
//  ^^^^^^^^^^^^^^^^^^^^^^^ERR(>=1.39.0-beta,<1.44.0-beta) method not found
//  ^^^^^^^^^^^^^^^^^^^^^^^HELP(>=1.44.0-beta) in this macro invocation
//  ^^^^^^^^^^^^^^^^^^^^^^^MSG(>=1.44.0-beta) See Primary: lib.rs:10
}
