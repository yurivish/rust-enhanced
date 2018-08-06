// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

enum OperatingSystem
// <- storage.type.enum
// ^^^^^^^^^^^^^^^^^ meta.enum
//   ^^^^^^^^^^^^^^^ entity.name.enum
{
// <- meta.enum meta.block punctuation.definition.block.begin
    Osx,
    Windows,
    Linux,
    Bsd(String),
    //  ^^^^^^ support.type
    Info { field: i32, value: str }
    //   ^ meta.block meta.block punctuation.definition.block.begin
    //            ^^^ storage.type
    //                        ^^^ storage.type
    //                            ^ meta.block meta.block punctuation.definition.block.end
}
// <- meta.enum meta.block punctuation.definition.block.end

let q = Message::Quit;
//      ^^^^^^^ storage.type.source
//             ^^ meta.path
//               ^^^^ storage.type.source
//                   ^ punctuation.terminator
let w = Message::WriteString("Some string".to_string());
//               ^^^^^^^^^^^ storage.type.source
//                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.group
//                           ^^^^^^^^^^^^^ string.quoted.double
//                                         ^^^^^^^^^ support.function
let m = Message::Move { x: 50, y: 200 };
//                    ^^^^^^^^^^^^^^^^^ meta.block
//                         ^^ constant.numeric.integer.decimal
//                                ^^^ constant.numeric.integer.decimal
