// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

struct GenVal<T>(T,);
//     ^^^^^^^^^ meta.generic
//     ^^^^^^ entity.name.struct
//           ^ punctuation.definition.generic.begin - entity.name.struct
//             ^ punctuation.definition.generic.end

// Ensure that `mut` is a storage modifier.
impl<A> Thing for &'a mut A {}
//            ^^^ meta.impl keyword.other
//                ^ meta.impl keyword.operator
//                 ^^ meta.impl storage.modifier.lifetime
//                    ^^^ meta.impl storage.modifier
//                        ^ meta.impl entity.name.impl

// Various tests on `where`.
fn f<'b: 'a>(self) -> &'b mut [i32] where 'a: 'b { }
//                 ^^^^^^^^^^^^^^ meta.function meta.function.return-type
//                            ^ meta.function meta.function.return-type punctuation.definition.group.begin
//                             ^^^ meta.function meta.function.return-type storage.type
//                                ^ meta.function meta.function.return-type punctuation.definition.group.end
//                                  ^^^^^ meta.function meta.where keyword.other
//                                        ^^ meta.function meta.where storage.modifier.lifetime
//                                          ^ meta.function meta.where punctuation.separator
//                                            ^^ meta.function meta.where storage.modifier.lifetime
//                                               ^ meta.function meta.block punctuation.definition.block.begin
//                                                 ^ meta.function meta.block punctuation.definition.block.end

fn f<F>(func: F) -> usize
//               ^^ meta.function meta.function.return-type punctuation.separator
//                  ^^^^^ meta.function meta.function.return-type storage.type
    where F: Fn(usize) -> usize {}
//  ^^^^^ meta.function meta.where keyword.other
//        ^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//         ^ punctuation.separator
//           ^^ support.type
//             ^ punctuation.definition.type.begin
//              ^^^^^ storage.type
//                   ^ punctuation.definition.type.end
//                     ^^ meta.function.return-type punctuation.separator
//                        ^^^^^ meta.function.return-type storage.type
//                              ^ meta.function meta.block punctuation.definition.block.begin
//                               ^ meta.function meta.block punctuation.definition.block.end

fn f<L, R>(lhs: L, rhs: R)
    where L: IntoIterator<Item=(&'a i32, &'a i32)>,
//  ^^^^^ meta.function meta.where keyword.other
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//           ^^^^^^^^^^^^ support.type
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//                       ^ punctuation.definition.generic.begin
//                             ^ punctuation.definition.type.begin
//                              ^ keyword.operator
//                               ^^ storage.modifier.lifetime
//                                  ^^^ storage.type
//                                       ^ keyword.operator
//                                        ^^ storage.modifier.lifetime
//                                           ^^^ storage.type
//                                              ^ punctuation.definition.type.end
//                                               ^ punctuation.definition.generic.end
          R: IntoIterator<Item=(&'a i32, &'a i32)>, {}
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//                                                  ^ meta.function meta.block punctuation.definition.block.begin
//                                                   ^ meta.function meta.block punctuation.definition.block.end
fn f<F: Fn(usize) -> usize>(func: f) {}
//  ^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//  ^ meta.generic punctuation.definition.generic.begin
//    ^ meta.generic punctuation.separator
//      ^^ meta.generic support.type
//        ^ meta.generic punctuation.definition.type.begin
//         ^^^^^ meta.generic storage.type
//              ^ meta.generic punctuation.definition.type.end
//                   ^^^^^ meta.generic meta.function.return-type storage.type
//                        ^ meta.generic punctuation.definition.generic.end
//                         ^ meta.function meta.function.parameters punctuation.definition.parameters.begin
//                          ^^^^ meta.function meta.function.parameters variable.parameter
fn f<L: IntoIterator<Item=(&'a i32, &'a i32)>>(lhs: L) {}
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//  ^ punctuation.definition.generic.begin
//                  ^ punctuation.definition.generic.begin
//                        ^ punctuation.definition.type.begin
//                          ^^ storage.modifier.lifetime
//                             ^^^ storage.type
//                                          ^ punctuation.definition.generic.begin
//                                           ^ punctuation.definition.generic.end
//                                            ^ meta.function meta.function.parameters punctuation.definition.parameters.begin

struct A<T>(T) where T: AsRef<str>;
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.struct
//                            ^^^ meta.struct meta.where storage.type
//                                ^ punctuation.terminator
//             ^^^^^ meta.struct meta.where keyword.other
pub struct A<T>(T)
//  ^^^^^^^^^^ meta.struct
//  ^^^^^^ meta.struct storage.type
where
//^^^ meta.struct meta.where keyword.other
    T: AsRef<str>;
//^^^^^^^^^^^^^^^ meta.struct
//           ^^^ meta.struct meta.where storage.type
//               ^ punctuation.terminator

pub struct IterHolder<A> where A: Number {
//                   ^^^ meta.struct meta.generic
//                       ^^^^^ meta.struct meta.where keyword.other
//                                ^^^^^^ meta.struct meta.where
//                                       ^ meta.struct punctuation.definition.block.begin
    num: A
}

pub struct IterHolder<A>
//                   ^^^ meta.struct meta.generic
where
//   <- meta.struct meta.where keyword.other
    A: Number {
//     ^^^^^^ meta.struct meta.where
    num: A
}

pub fn new<T>() -> Fibonacci<T>
    where T: One + Zero,
//  ^^^^^ keyword.other
    for <'a> &'a T: Add<Output = T>,
//  ^^^ keyword.other
//      ^ punctuation.definition.generic.begin
//       ^^ storage.modifier.lifetime
//         ^ punctuation.definition.generic.end
//           ^ keyword.operator
//            ^^ storage.modifier.lifetime
{
    unimplemented!();
}

pub fn new<T>() -> Fibonacci<T>
    where for <'a> &'a T: Add<Output = T>,
//  ^^^^^ meta.where keyword.other
//        ^^^ keyword.other
//            ^ punctuation.definition.generic.begin
//             ^^ storage.modifier.lifetime
//               ^ punctuation.definition.generic.end
//                 ^ keyword.operator
//                  ^^ storage.modifier.lifetime
{
    unimplemented!();
}

impl<T> Fibonacci<T>
    where for <'a> &'a T: Add<Output = T>,
//  ^^^^^ keyword.other
//        ^^^ keyword.other
//            ^ punctuation.definition.generic.begin
//             ^^ storage.modifier.lifetime
//               ^ punctuation.definition.generic.end
//                 ^ keyword.operator
//                  ^^ storage.modifier.lifetime
{
    unimplemented!();
}

impl<T> Iterator for Fibonacci<T>
    where T: Clone,
//  ^^^^^ keyword.other
    for <'a> &'a T: Add<Output = T>,
//  ^^^ keyword.other
//      ^ punctuation.definition.generic.begin
//       ^^ storage.modifier.lifetime
//         ^ punctuation.definition.generic.end
//           ^ keyword.operator
//            ^^ storage.modifier.lifetime
{
    unimplemented!();
}

pub const FOO: Option<[i32; 1]> = Some([1]);
//                    ^ punctuation.definition.group.begin
//                           ^ punctuation.definition.group.end

#[derive(Clone)]
pub struct GobletMiddleware<B: Backend + ?Sized + 'static> {
//         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//                                                ^^^^^^^ storage.modifier.lifetime
    pub derp: Arc<Api<B>>,
}

impl<B: Backend + ?Sized + 'static> GobletMiddleware<B> {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^ storage.modifier.lifetime
//                                  ^^^^^^^^^^^^^^^^^^^ meta.generic
    pub fn new(api: Arc<Api<B>>) -> GobletMiddleware<B> {
        GobletMiddleware { derp: api }
    }
}

impl<B: Backend + ?Sized + 'static> Key for GobletMiddleware<B> {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^ storage.modifier.lifetime
//                                      ^^^ keyword.other
//                                          ^^^^^^^^^^^^^^^^ entity.name.impl
//                                                          ^^^ meta.generic
    type Value = Arc<Api<B>>;
}

impl<T> From<AsRef<T>> for CliError<T> { }
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^^ entity.name.impl
//                                 ^^^ meta.generic

fn legal_dates_iter() -> Box<Iterator<Item = Date<UTC>>> {
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.return-type meta.generic
//                                         ^ keyword.operator
    unimplemented!()
}

fn numbers() -> impl Iterator<Item = u64> {
//              ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.return-type
//              ^^^^ meta.function.return-type storage.type.impl
//                   ^^^^^^^^ meta.function.return-type support.type
//                           ^^^^^^^^^^^^ meta.function.return-type meta.generic
    Generator(move || for a in (0..10) { yield a; } })
//                                       ^^^^^ keyword.control
}

fn collect_vec() {
    let _: Vec<(usize, usize)> = (0..10).enumerate().collect::<Vec<_>>();
//         ^^^^^^^^^^^^^^^^^^^ meta.generic
//             ^ punctuation.definition.type.begin
//              ^^^^^ storage.type
//                     ^^^^^ storage.type
//                          ^ punctuation.definition.type.end
//                                                            ^^^^^^^^ meta.generic
//                                                             ^^^^^^ meta.generic meta.generic
//                                                                 ^ keyword.operator
    let _: Vec<(usize, usize)> = vec!();
//                               ^^^^ support.macro
    let _: Vec<(usize, usize)> = vec!{};
//                               ^^^^ support.macro
    let _: Vec<(usize, usize)> = vec![];
//                               ^^^^ support.macro
}


pub fn next_lex<T:PartialOrd>(/* block */data: &mut [T] // line {
//                            ^^^^^^^^^^^ source meta.function meta.function.parameters comment.block
//                                                      ^^^^^^^^^ source meta.function meta.function.parameters comment.line.double-slash
    /* block2 */ data2: &mut [T]  // line
//  ^^^^^^^^^^^^ source meta.function meta.function.parameters comment.block
//                                ^^^^^^^ source meta.function meta.function.parameters comment.line.double-slash
    ) -> bool {
    unimplemented!();
}

pub fn next_lex2</* block */T/* comments */:/* everywhere */
//               ^^^^^^^^^^^ comment.block
//                           ^^^^^^^^^^^^^^ comment.block
//                                          ^^^^^^^^^^^^^^^^ comment.block
    // Many comments
//  ^^^^^^^^^^^^^^^^ comment.line.double-slash
    /* help */ PartialOrd // Possibly too many comments
//  ^^^^^^^^^^ comment.block
//                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ comment.line.double-slash
> (
    /* block2 */ data2: &mut [T]  // line
//  ^^^^^^^^^^^^ source meta.function meta.function.parameters comment.block
//                                ^^^^^^^ source meta.function meta.function.parameters comment.line.double-slash
    ) -> bool {
    unimplemented!();
}

// impl of GenVal for a generic type `T`
impl <'a, T> GenVal<T> {
//   ^ punctuation.definition.generic.begin
//    ^^ storage.modifier.lifetime
//         ^ punctuation.definition.generic.end
//           ^^^^^^ entity.name.impl
//                 ^^^ - entity.name.impl
//                 ^ punctuation.definition.generic.begin
//                   ^ punctuation.definition.generic.end
    fn value(&self) -> &T { &self.0 }
    //       ^ keyword.operator
    //                 ^ keyword.operator
}

fn print_debug<T: Debug> (t: &T) {
// ^^^^^^^^^^^ entity.name.function
//            ^ punctuation.definition.generic.begin - entity.name.function
//              ^ punctuation.separator
//                     ^ punctuation.definition.generic.end
//                        ^ variable.parameter
//                           ^ keyword.operator
    println!("{:?}", t);
//            ^^^^ constant.other.placeholder
}

pub fn from_buf_reader<T>(s: io::BufReader<T>) -> Result<isize, &'static str>
//                                                              ^ keyword.operator
//                                                               ^^^^^^^ meta.function meta.function.return-type meta.generic storage.modifier.lifetime
    where T: io::Read
//  ^^^^^ keyword.other
{}

// HRTB in various positions.
fn f<F: for<'c> Fn(&'c mut Self)>() {}
//      ^^^ meta.generic keyword.other
//         ^ meta.generic meta.generic punctuation.definition.generic.begin
//          ^^ meta.generic meta.generic storage.modifier.lifetime
//            ^ meta.generic meta.generic punctuation.definition.generic.end
//              ^^ meta.generic support.type
fn f(a: for<'a, 'b> fn() -> String) {}
//   ^ variable.parameter
//   ^^^^^^^^^^^^^^ meta.function meta.function.parameters
//      ^^^ keyword.other
//         ^ meta.generic punctuation.definition.generic.begin
//          ^^ meta.generic storage.modifier.lifetime
//            ^ meta.generic punctuation.separator
//              ^^ meta.generic storage.modifier.lifetime
//                ^ meta.function meta.function.parameters meta.generic punctuation.definition.generic.end
