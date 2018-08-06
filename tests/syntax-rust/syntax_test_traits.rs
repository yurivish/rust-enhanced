// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

pub trait Animal {
// <- storage.modifier
//  ^^^^^^^^^^^^^^ meta.trait
//               ^ meta.block punctuation.definition.block.begin
    fn noise(quiet: bool) {
        // Comment
    }
}
// <- meta.trait meta.block punctuation.definition.block.end

impl<'a, T: MyTrait + OtherTrait> PrintInOption for T where
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//   ^^ storage.modifier.lifetime
//        ^ punctuation.separator
//                  ^ keyword.operator
//                                              ^^^ keyword.other
//                                                  ^ entity.name.impl
//                                                    ^^^^^ meta.where keyword.other
    Option<T>: Debug {
//^^^^^^^^^^^^^^^^^^^^ meta.impl
    fn print_in_option(self) {
//     ^^^^^^^^^^^^^^^ entity.name.function
        println!("{:?}", Some(self));
    }
}

impl fmt::Display for PrintableStruct {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
// <- storage.type.impl
//^^ storage.type.impl
//   ^^^^^ meta.path
//                ^^^ keyword.other
//                    ^^^^^^^^^^^^^^^ entity.name.impl
//                                    ^ meta.block punctuation.definition.block.begin
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function
//  ^^ storage.type.function
//     ^^^ entity.name.function
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.parameters
//        ^ punctuation.definition.parameters.begin
//         ^ keyword.operator
//          ^^^^ variable.parameter
//                ^ variable.parameter
//                 ^ punctuation.separator
//                   ^ keyword.operator
//                    ^^^ storage.modifier
//                        ^^^^^ meta.path
//                                      ^ punctuation.definition.parameters.end
//                                        ^^ punctuation.separator
//                                           ^^^^^ meta.path
//                                                       ^ meta.block punctuation.definition.block.begin
    }
// ^^ meta.function meta.block
//  ^ punctuation.definition.block.end
}
// <- meta.block punctuation.definition.block.end

impl !Send for Point {}
//^^^^^^^^^^^^^^^^^^^^^ meta.impl
//   ^ meta.impl keyword.operator meta.impl.opt-out
