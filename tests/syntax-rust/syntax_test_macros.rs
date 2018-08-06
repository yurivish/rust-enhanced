// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

String my_var = format!("Hello {0}", "World");
// ^^^ support.type
//            ^ keyword.operator
//              ^^^^^^^ support.macro
//                     ^ punctuation.definition.group.begin
//                     ^^^^^^^^^^^^^^^^^^^^^^ meta.group
//                      ^^^^^^^^^^^ string.quoted.double
//                             ^^^ constant.other.placeholder
//                                          ^ punctuation.definition.group.end

pub fn macro_tests() {
    println!();
//  ^^^^^^^^ support.macro
    println!("Example");
//  ^^^^^^^^ support.macro
//          ^ punctuation.definition.group.begin
//           ^^^^^^^^^ string.quoted.double
//                    ^ punctuation.definition.group.end
    println!("Example {} {message}", "test", message="hi");
//                    ^^ constant.other.placeholder
//                       ^^^^^^^^^ constant.other.placeholder
    panic!();
//  ^^^^^^ support.macro
    panic!("Example");
//  ^^^^^^ support.macro
//        ^ punctuation.definition.group.begin
//         ^^^^^^^^^ string.quoted.double
//                  ^ punctuation.definition.group.end
    panic!("Example {} {message}", "test", message="hi");
//                  ^^ constant.other.placeholder
//                     ^^^^^^^^^ constant.other.placeholder
    format_args!("invalid type: {}, expected {}", unexp, exp);
//  ^^^^^^^^^^^^ support.macro
//                              ^^ constant.other.placeholder
//                                           ^^ constant.other.placeholder
    unreachable!("{:?}", e);
//  ^^^^^^^^^^^^ support.macro
//                ^^^^ constant.other.placeholder
    unimplemented!("{:?}", e);
//  ^^^^^^^^^^^^^^ support.macro
//                  ^^^^ constant.other.placeholder
}

my_var = format!("Hello {name}, how are you?",
//     ^ keyword.operator
//       ^^^^^^^ support.macro
//              ^ punctuation.definition.group.begin
//              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.group
//               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double
//                      ^^^^^^ constant.other.placeholder
    name="John");
// ^^^^^^^^^^^^^ meta.group
//      ^ keyword.operator
//       ^^^^^^ string.quoted.double
//             ^ punctuation.definition.group.end

        write!(f, "{}", self.0)
//      ^^^^^^ support.macro
//            ^^^^^^^^^^^^^^^^^ meta.group
//            ^ punctuation.definition.group.begin
//                ^^^^ string.quoted.double
//                 ^^ constant.other.placeholder
//                            ^ punctuation.definition.group.end
        write!(f, "{:10}", self.0)
//                 ^^^^^ constant.other.placeholder
        eprint!("{:^10}", self.0)
//      ^^^^^^^ support.macro
//               ^^^^^^ constant.other.placeholder
        eprintln!("{:+046.89?}", self.0)
//      ^^^^^^^^^ support.macro
//                 ^^^^^^^^^^^ constant.other.placeholder
        assert!(true, "{:-^#10x}", self.0)
//      ^^^^^^^ support.macro
//                     ^^^^^^^^^ constant.other.placeholder
        debug_assert!(true, "{4j:#xf10}", self.0)
//      ^^^^^^^^^^^^^ support.macro
//                           ^^^^^^^^^^ string.quoted.double
        write!(f, "{{}}", self.0)
//                 ^^^^ constant.character.escape
        write!(get_writer(), "{}", "{}")
//      ^^^^^^ support.macro
//            ^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.group
//             ^^^^^^^^^^ support.function
//                           ^^^^ string.quoted.double
//                            ^^ constant.other.placeholder
//                                 ^^^^ string.quoted.double
//            ^ punctuation.definition.group.begin
//                                     ^ punctuation.definition.group.end
        writeln!(w)
//      ^^^^^^^^ support.macro
//              ^^^ meta.group
//              ^ punctuation.definition.group.begin
//                ^ punctuation.definition.group.end
        println!()
//      ^^^^^^^^ support.macro
//              ^^ meta.group
//              ^ punctuation.definition.group.begin
//               ^ punctuation.definition.group.end

macro_rules! forward_ref_binop [
//                             ^ meta.macro meta.group punctuation.definition.group.begin
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
//        ^^^^ variable.parameter
//             ^^^^^ storage.type
//                    ^^^^^^^ variable.parameter
//                            ^^^^^ storage.type
//                                      ^^ variable.parameter
//                                         ^^ storage.type
//                                             ^^ variable.parameter
//                                                ^^ storage.type
//                                                    ^^ keyword.operator
//                                                       ^ meta.macro meta.group meta.block punctuation.definition.block.begin
        impl<'a, 'b> $imp<&'a $u> for &'b $t {
//      ^^^^ storage.type.impl
//          ^^^^^^^^ meta.generic
//           ^^ storage.modifier.lifetime
//               ^^ storage.modifier.lifetime
//                   ^^^^ variable.other
//                       ^^^^^^^^ meta.generic
//                        ^ keyword.operator
//                         ^^ storage.modifier.lifetime
//                            ^^ variable.other
//                                ^^^ keyword.other
//                                    ^ keyword.operator
//                                     ^^ storage.modifier.lifetime
//                                        ^^ variable.other
//                                           ^ meta.macro meta.group meta.block meta.impl meta.block punctuation.definition.block.begin
            type Output = <$t as $imp<$u>>::Output;
//                        ^^^^^^^^^^^^^^^^ meta.generic
//                                        ^^ meta.path

            #[inline]
//          ^^^^^^^^^ meta.annotation
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
//          ^^ storage.type.function
//             ^^^^^^^ variable.other
//                     ^^^^ variable.language
//                                  ^ keyword.operator
//                                   ^^ storage.modifier.lifetime
//                                      ^^ variable.other
//                                          ^^ punctuation.separator
//                                             ^^^^^^^^^^^^^^^^ meta.generic
//                                                             ^^ meta.path
//                                                                      ^ meta.macro meta.group meta.block meta.impl meta.block meta.block punctuation.definition.block.begin
                $imp::$method(*self, *other)
//              ^^^^ variable.other
//                    ^^^^^^^ variable.other
//                            ^ keyword.operator
//                             ^^^^ variable.language
//                                   ^ keyword.operator
            }
        }
    }
]

macro_rules! alternate_group (
//                           ^ meta.macro meta.group punctuation.definition.group.begin
    ($a:expr) => (
//   ^^ variable.parameter
//      ^^^^ storage.type
        println!("Test {}!", $a)
    )
)

macro_rules! kleene_star {
    ($($arg:tt)+) => (
//   ^ meta.macro meta.block meta.group keyword.operator
//    ^ meta.macro meta.block meta.group punctuation.definition.group.begin
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^ meta.macro meta.block meta.group
//              ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                ^ meta.macro meta.block keyword.operator
        println!($($arg));
    ),
    ($($arg:tt)*) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^ meta.macro meta.block meta.group
//              ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                ^ meta.macro meta.block keyword.operator
        println!($($arg)*);
    ),
    ($($arg:tt);+) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^^ meta.macro meta.block meta.group
//               ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                 ^ meta.macro meta.block keyword.operator
        println!($($arg));
    ),
    ($($arg:tt),*) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^^ meta.macro meta.block meta.group
//               ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                 ^ meta.macro meta.block keyword.operator
        println!($($arg)*);
    )
}

macro_rules! eat_numbers {
    ($count:expr, /*$comment:ident,*/  $msg:expr) => {{
    //            ^^^^^^^^^^^^^^^^^^^ meta.macro meta.block meta.group comment.block
    //                                               ^ meta.macro meta.block meta.block punctuation.definition.block.begin
    //                                                ^ meta.macro meta.block meta.block meta.block punctuation.definition.block.begin
        let parse_err = concat!("Err parsing value in ", $msg);
        try!{ eat_numbers(&mut lines, $count, parse_err, missing_err, too_many) }
    //  ^^^^ support.macro
    //      ^ meta.macro meta.block meta.block meta.block meta.block punctuation.definition.block.begin
    }}
};
 // <- -meta.macro
