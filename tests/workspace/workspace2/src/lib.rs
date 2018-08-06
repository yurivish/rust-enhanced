mod somemod;

trait Trait {}

pub fn f() {
    let x = Box::new(0u32);
    let y: &Trait = x;
//                  ^ERR(<1.29.0-beta) expected &Trait, found
//                  ^ERR(>=1.29.0-beta) expected &dyn Trait, found
//                  ^ERR mismatched types
//                  ^NOTE(<1.29.0-beta) expected type `&Trait`
//                  ^NOTE(>=1.29.0-beta) expected type `&dyn Trait`
//                  ^NOTE(<1.16.0) found type
//                  ^HELP(>=1.18.0,<1.24.0) try with `&x`
//                  ^HELP(>=1.24.0-beta) consider borrowing here
//                  ^HELP(>=1.24.0-beta) &x
}
