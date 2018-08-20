// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

#![warn(unused)]
// <- meta.annotation
//^^^^^^^^^^^^^^ meta.annotation
// ^^^^ support.function
//     ^ meta.group punctuation.definition.group.begin
//      ^^^^^^ meta.group
//            ^ meta.group punctuation.definition.group.end

#[macro_use]
// <- meta.annotation
//^^^^^^^^^^ meta.annotation

#[cfg(all(unix, not(target_os = "haiku")))]
// <- meta.annotation
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
//^^^ support.function
//    ^^^ meta.group support.function
//        ^^^^^^ meta.group meta.group
//              ^^^ meta.group meta.group support.function
//                  ^^^^^^^^^^ meta.group meta.group meta.group
//                            ^ meta.group meta.group meta.group keyword.operator
//                              ^^^^^^^ meta.group meta.group meta.group string.quoted.double

// Test highlighting/scope with struct field attributes
// https://github.com/rust-lang/sublime-rust/issues/120
pub struct Claim {
//  ^^^^^^^^ meta.struct
    pub claim_id: String,
//  ^^^ storage.modifier
    pub patient_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
//                               ^^^^^^^^^^^^^^^ string.quoted.double
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub referring: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
//    ^^^^^ support.function
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub drug: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub ndc: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub rendering: Option<String>,
    pub date: String,
}

enum E {
    #[allow(dead_code)]
//  ^^^^^^^^^^^^^^^^^^^ meta.enum meta.annotation
//    ^^^^^ support.function
    A(i32),
//    ^^^ meta.enum meta.struct meta.group storage.type
}

// Generic parameters.
unsafe impl<#[may_dangle] T: ?Sized> Drop for Box<T> { }
//          ^^^^^^^^^^^^^ meta.annotation
//         ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl meta.generic
