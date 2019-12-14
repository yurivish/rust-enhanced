mod somemod;

trait Trait {}

pub fn f() {
    let x = Box::new(0u32);
    let y: &dyn Trait = x;
//                      ^ERR(<1.29.0-beta) expected &Trait, found
//                      ^ERR(>=1.41.0-beta) expected `&dyn Trait`, found
//                      ^ERR(>=1.29.0-beta,<1.41.0-beta) expected &dyn Trait, found
//                      ^ERR mismatched types
//         ^^^^^^^^^^ERR(>=1.41.0-beta) expected due to this
//                      ^NOTE(<1.29.0-beta) expected type `&Trait`
//                      ^NOTE(>=1.41.0-beta) expected reference `&dyn Trait`
//                      ^NOTE(>=1.29.0,<1.41.0-beta) expected type `&dyn Trait`
//                      ^NOTE(<1.16.0) found type
//                      ^HELP(>=1.18.0,<1.24.0) try with `&x`
//                      ^HELP(>=1.24.0-beta) consider borrowing here
//                      ^HELP(>=1.24.0-beta) &x
}
