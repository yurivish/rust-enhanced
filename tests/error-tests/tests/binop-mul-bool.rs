// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:`*` cannot be applied to type `bool`

fn main() { let x = true * false; }
//                  ^^^^ERR(<1.19.0) binary operation
//                  ^^^^NOTE(<1.19.0) an implementation of
//                  ^^^^^^^^^^^^ERR(>=1.19.0,<1.35.0-beta) binary operation
//                  ^^^^^^^^^^^^NOTE(>=1.19.0,<1.35.0-beta) an implementation of
//                       ^ERR(>=1.35.0-beta,<1.42.0-beta) binary operation
//                       ^ERR(>=1.42.0-beta) cannot multiply
//                       ^NOTE(>=1.35.0-beta,<1.43.0-beta) an implementation of
//                  ^^^^ERR(>=1.35.0-beta) bool
//                         ^^^^^ERR(>=1.35.0-beta) bool
