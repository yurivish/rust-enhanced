#[macro_use]
extern crate dcrate;

// This is an example of an error in a macro from an external crate.  It
// should include a message that it originates in a macro outside of the
// current crate.

fn f() {
    let x: () = example_bad_value!();
//              ^^^^^^^^^^^^^^^^^^^^ERR(<1.44.0-beta) mismatched types
//              ^^^^^^^^^^^^^^^^^^^^ERR(<1.44.0-beta) this error originates in a macro outside
//              ^^^^^^^^^^^^^^^^^^^^ERR(>=1.41.0-beta,<1.44.0-beta) expected `()`, found `i32`
//              ^^^^^^^^^^^^^^^^^^^^ERR(<1.41.0-beta) expected (), found i32
//         ^^ERR(>=1.41.0-beta) expected due to this
//              ^^^^^^^^^^^^^^^^^^^^NOTE(<1.41.0-beta) expected type
//              ^^^^^^^^^^^^^^^^^^^^NOTE(<1.16.0) found type
//              ^^^^^^^^^^^^^^^^^^^^HELP(>=1.44.0-beta) in this macro invocation
//              ^^^^^^^^^^^^^^^^^^^^MSG(>=1.44.0-beta) See Primary: lib.rs:31
}
