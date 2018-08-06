// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

let inferred_closure = |i, j: u32| i + 1;
//  ^^^^^^^^^^^^^^^^ entity.name.function
//                     ^^^^^^^^^^^^^^^^^ meta.function.closure
//                     ^^^^^^^^^^^ meta.function.parameters
//                     ^ punctuation.definition.parameters.begin
//                      ^ variable.parameter
//                         ^ variable.parameter
//                          ^ punctuation.separator
//                            ^^^ storage.type
//                               ^ punctuation.definition.parameters.end
let closure = || -> i32 { | | 1 + 2 };
//  ^^^^^^^ entity.name.function
//            ^^^^^^^^^^^^^^^^^^^^^^^ meta.function.closure
//            ^^ meta.function.parameters
//            ^ punctuation.definition.parameters.begin
//             ^ punctuation.definition.parameters.end
//                  ^^^ storage.type
//                      ^^^^^^^^^^^^^ meta.block
//                      ^ meta.block punctuation.definition.block.begin
//                            ^ constant.numeric.integer.decimal
//                                ^ constant.numeric.integer.decimal
//                                  ^ meta.block punctuation.definition.block.end

// Make sure "or" is not confused with closure.
let c = a | b;
//        ^ keyword.operator

call_func(|c| 1 + 2 + c);
//        ^^^^^^^^^^^^^ meta.function.closure
//        ^^^ meta.function.parameters

// TODO: x and y should be parameters!
// Looks like tuples use meta.group so we'll use that for tuple arguments too
// fixes https://github.com/rust-lang/sublime-rust/issues/164
let f = |(x, y)| { x + y };
//      ^ punctuation.definition.parameters.begin
//       ^^^^^^ meta.group

fn lambdas() {
    let c = |foo,
//          ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.begin
//           ^^^ meta.function.parameters variable.parameter
             bar| {};
//           ^^^ meta.function.parameters variable.parameter
//              ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.end
    let c = |foo,  // weird, but should work
//          ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.begin
//           ^^^ meta.function.parameters variable.parameter
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^ comment.line
             bar| {};
//           ^^^ meta.function.parameters variable.parameter
//              ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.end
}
