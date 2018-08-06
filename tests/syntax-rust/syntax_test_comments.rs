// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

// Line comments
// <- comment.line.double-slash
// ^^^^^^^^^^^^^^ comment.line.double-slash
/// Line doc comments
// <- comment.line.documentation
// ^^^^^^^^^^^^^ comment.line.documentation

/*!
// <- comment.block.documentation
 // <- comment.block.documentation
//^ comment.block.documentation
Block doc comments
// ^^^^^^^^^^^^^^^^ comment.block.documentation
/* Nested comments */
// ^^^^^^^^^^^^^^^^^^ comment.block.documentation comment.block
*/

// Verify comment is cleared.
mod a {}
// ^^^^^ -comment

fn foo<F: FnMut(i32, i32 /*asd*/) -> i32>(f: F) {
//                       ^^^^^^^ meta.generic comment
    let lam = |time: i32 /* comment */, other: i32| {
//                       ^^^^^^^^^^^^^ meta.function.parameters comment
    };
}
