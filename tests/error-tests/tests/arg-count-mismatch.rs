// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: parameters were supplied

/*BEGIN*/fn f(x: isize) {
//       ^^^^^^^^^^^^^^ERR(>=1.24.0-beta,<1.49.0-beta) defined here
//       ^^^^^^^^^^^^^^MSG(>=1.24.0-beta,<1.49.0-beta) See Primary: ↓:25
//          ^NOTE(>=1.49.0-beta) defined here
//            ^^^^^^^^NOTE(>=1.49.0-beta)
//          ^MSG(>=1.49.0-beta) See Primary: ↓:25
}/*END*/
// ~ERR(<1.24.0-beta) defined here
// ~MSG(<1.24.0-beta) See Primary: ↓:25

// children without spans, spans with no labels
// Should display error (with link) and a note of expected type.
fn main() { let i: (); i = f(); }
//                         ^^^ERR(<1.43.0-beta) this function takes 1 parameter
//                         ^^^ERR(<1.43.0-beta) expected 1 parameter
//                         ^^^MSG(<1.24.0-beta) See Also: ↑:16
//                         ^^^MSG(>=1.24.0-beta,<1.43.0-beta) See Also: ↑:13
//                         ^ERR(>=1.43.0-beta) this function takes 1
//                          ^^ERR(>=1.43.0-beta,<1.63.0-beta) supplied 0
//                          ^^ERR(>=1.43.0-beta,<1.63.0-beta) expected 1
//                          ^^ERR(>=1.63.0-beta) an argument of type `isize` is missing
//                         ^^^HELP(>=1.63.0,<1.65.0-beta) provide the argument
//                         ^^^HELP(>=1.63.0,<1.65.0-beta) /Accept Replacement:.*/
//                          ^^HELP(>=1.65.0-beta) provide the argument
//                          ^^HELP(>=1.65.0-beta) /Accept Replacement:.*/
//                         ^MSG(>=1.43.0-beta) See Also: ↑:13
