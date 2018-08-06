// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

fn my_func(x: i32)
// <- storage.type.function
// ^^^^^^^ entity.name.function
//        ^^^^^^^^ meta.function.parameters
//        ^ punctuation.definition.parameters.begin
//         ^ variable.parameter
//          ^ punctuation.separator
//               ^ punctuation.definition.parameters.end
{
// <-  meta.function meta.block punctuation.definition.block.begin

}
// <-  meta.function meta.block punctuation.definition.block.end

fn foo<A>(i: u32, b: i64) -> u32 {
// <- storage.type.function
// ^^^ entity.name.function
//    ^ punctuation.definition.generic.begin
//      ^ punctuation.definition.generic.end
//       ^^^^^^^^^^^^^^^^ meta.function.parameters
//                           ^^^ storage.type
//                               ^ meta.block punctuation.definition.block.begin

}
// <- meta.block punctuation.definition.block.end


fn my_other_func(e: OperatingSystem) -> &'a f64 {
// ^^^^^^^^^^^^^ entity.name.function
//               ^ variable.parameter
//                ^ punctuation.separator
//                                      ^ meta.function meta.function.return-type keyword.operator
//                                       ^^ meta.function meta.function.return-type storage.modifier.lifetime
//                                          ^^^ meta.function meta.function.return-type storage.type
}


pub fn pub_function() -> bool
// <- storage.modifier
//  ^^ storage.type.function
//     ^^^^^^^^^^^^ entity.name.function
{
// <- meta.function
    return true
//  ^^^^^^ meta.function meta.block keyword.control
}

pub unsafe extern "C" fn __sync_synchronize() { }
// <- storage.modifier
//  ^^^^^^ storage.modifier
//         ^^^^^^ keyword.other
//                ^^^ string.quoted.double
//                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function
//                    ^^ storage.type.function
//                       ^^^^^^^^^^^^^^^^^^ entity.name.function
//                                         ^ meta.function.parameters punctuation.definition.parameters.begin
//                                          ^ meta.function.parameters punctuation.definition.parameters.end

let f: extern "C" fn () = mem::transmute(0xffff0fa0u32);
//     ^^^^^^ keyword.other
//            ^^^ string.quoted.double
//                ^^ storage.type.function
//                   ^^ meta.group
//                      ^ keyword.operator
//                                       ^^^^^^^^^^ meta.group constant.numeric.integer.hexadecimal
//                                                 ^^^ meta.group storage.type.numeric
