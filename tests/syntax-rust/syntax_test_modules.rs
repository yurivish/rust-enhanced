// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

extern crate foo;
//<- keyword.other
//^^^^ keyword.other
//     ^^^^^ keyword.other
//           ^^^ source
//              ^ punctuation.terminator

extern crate std as ruststd;
//^^^^ keyword.other
//     ^^^^^ keyword.other
//           ^^^^ source
//               ^^ keyword.operator
//                  ^^^^^^^ source
//                         ^ punctuation.terminator

mod bar;
// <- meta.module storage.type.module
//^^^^^^ meta.module
//  ^^^ entity.name.module

pub mod my_mod {
//  ^^^^^^^^^^^^ meta.module
// <- storage.modifier
//  ^^^ storage.type.module
//      ^^^^^^ entity.name.module
//             ^ meta.block punctuation.definition.block.begin
}
// <- meta.module meta.block punctuation.definition.block.end

pub use self::trafile::*;
// <- storage.modifier
//  ^^^ keyword.other
//      ^^^^ variable.language
//      ^^^^^^^^^^^^^^^ meta.path
//                     ^ keyword.operator
//                      ^ punctuation.terminator

use std::fmt;
// <- keyword.other
//  ^^^^^ meta.path
//       ^^^ - meta.path
//          ^ punctuation.terminator
use foo::i32;
//  ^^^^^ meta.path
//       ^^^ - meta.path storage.type
use foo::Bar;
//  ^^^^^ meta.path
//       ^^^ storage.type.source

use foo::{Baz, QUX, quux};
//  ^^^^^ meta.path
//       ^^^^^^^^^^^^^^^^ meta.block
//       ^ punctuation.definition.block.begin
//        ^^^ storage.type.source
//             ^^^ constant.other
//                  ^^^^ meta.block
//                      ^ punctuation.definition.block.end
//                       ^ punctuation.terminator

use std::{
// <- keyword.other
//  ^^^^^ meta.path
//       ^ meta.block punctuation.definition.block.begin
    fs::{self, read_dir},
//  ^^^^ meta.block meta.path
//      ^ meta.block meta.block punctuation.definition.block.begin
//       ^^^^ meta.block meta.block variable.language
//             ^^^^^^^^ meta.block meta.block
//                     ^ meta.block meta.block punctuation.definition.block.end
    path::{Path, PathBuf},
//  ^^^^^^ meta.block meta.path
//        ^ meta.block meta.block punctuation.definition.block.begin
//         ^^^^ meta.block meta.block storage.type.source
//               ^^^^^^^ meta.block meta.block storage.type.source
//                      ^ meta.block meta.block punctuation.definition.block.end
   };
// ^ meta.block punctuation.definition.block.end
//  ^ punctuation.terminator

extern {
// <- keyword.other
//^^^^ keyword.other
//     ^ meta.block punctuation.definition.block.begin
    fn foo(x: i32, ...);
}
// <- meta.block punctuation.definition.block.end

extern "stdcall" { }
// <- keyword.other
//     ^^^^^^^^^ string.quoted.double
