// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"
// This file is for various expressions that don't fit into other more
// specific categories.

let big_n =
//        ^ keyword.operator
    if n < 10 && n > -10 {
//       ^ keyword.operator
//            ^^ keyword.operator
//                 ^ keyword.operator
//                   ^ keyword.operator
//                    ^^ constant.numeric.integer.decimal
//                       ^ meta.block punctuation.definition.block.begin
        10 * n
//         ^ meta.block keyword.operator
    } else {
//  ^ meta.block punctuation.definition.block.end
//         ^ meta.block punctuation.definition.block.begin
        n / 2
//        ^ meta.block keyword.operator
    };
//  ^ meta.block punctuation.definition.block.end

let tuple = (1.0, 0i32, "Hello");
//          ^^^^^^^^^^^^^^^^^^^^ meta.group
//          ^ punctuation.definition.group.begin
//           ^^^ constant.numeric.float
//                ^ constant.numeric.integer.decimal
//                 ^^^ storage.type
//                      ^^^^^^^ string.quoted.double
//                             ^ punctuation.definition.group.end

// Tuple access.
let x = tuple.1;
//            ^ constant.numeric.integer.decimal.rust

// Array access.
let x = arr[1];
//         ^^^ meta.group
//         ^ punctuation.definition.group.begin
//          ^ constant.numeric.integer.decimal
//           ^ punctuation.definition.group.end

// Borrow expression.
let xsl = &xs;
//        ^ keyword.operator
let a = &&  10;
//      ^^ keyword.operator
let a = & & 10;
//      ^ keyword.operator
//        ^ keyword.operator
let y = &mut 9;
//      ^ keyword.operator
//       ^^^ storage.modifier

// Dereference.
assert_eq!(*x, 7);
//         ^ meta.group keyword.operator
   *y = 11;
// ^ keyword.operator

// Block expression.
let z = {
//      ^ meta.block punctuation.definition.block.begin
    2 * 5
//  ^ constant.numeric.integer.decimal
//    ^ keyword.operator
//      ^ constant.numeric.integer.decimal
};
// <- meta.block punctuation.definition.block.end

// Various operators.
let x = !6;
//      ^ keyword.operator
//       ^ constant.numeric.integer.decimal
let a = 1 + 2 - 3 * 4 / 6 % 7 & 8 | 9 ^ a << b >> c;
//        ^ keyword.operator
//            ^ keyword.operator
//                ^ keyword.operator
//                    ^ keyword.operator
//                        ^ keyword.operator
//                            ^ keyword.operator
//                                ^ keyword.operator
//                                    ^ keyword.operator
//                                        ^^ keyword.operator
//                                             ^^ keyword.operator
 a == b != c > d < e >= f <= g
// ^^ keyword.operator
//      ^^ keyword.operator
//           ^ keyword.operator
//               ^ keyword.operator
//                   ^^ keyword.operator
//                        ^^ keyword.operator
 a || b && c
// ^^ keyword.operator
//      ^^ keyword.operator
a += b;
//^^ keyword.operator
a -= b;
//^^ keyword.operator
a *= b;
//^^ keyword.operator
a /= b;
//^^ keyword.operator
a %= b;
//^^ keyword.operator
a &= b;
//^^ keyword.operator
a |= b;
//^^ keyword.operator
a ^= b;
//^^ keyword.operator
a <<= b;
//^^^ keyword.operator
a >>= b;
//^^^ keyword.operator
