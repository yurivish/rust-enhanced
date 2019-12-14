#[macro_use]
extern crate dcrate;

// This is an example of an error in a macro from an external crate.  It
// should include a message that it originates in a macro outside of the
// current crate.

fn f() {
    let x: () = example_bad_value!();
//              ^^^^^^^^^^^^^^^^^^^^ERR mismatched types
//              ^^^^^^^^^^^^^^^^^^^^ERR this error originates in a macro outside
//              ^^^^^^^^^^^^^^^^^^^^ERR(>=1.41.0-beta) expected `()`, found `i32`
//              ^^^^^^^^^^^^^^^^^^^^ERR(<1.41.0-beta) expected (), found i32
//         ^^ERR(>=1.41.0-beta) expected due to this
//              ^^^^^^^^^^^^^^^^^^^^NOTE(<1.41.0-beta) expected type
//              ^^^^^^^^^^^^^^^^^^^^NOTE(<1.16.0) found type
}
