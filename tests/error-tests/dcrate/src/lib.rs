#[macro_export]
macro_rules! example {
    ($submac:ident!( $($args:tt)* )) => (
        $submac!($($args)*)
    )
}

#[macro_export]
macro_rules! inner {
    ($x:expr) => ($x.missing())
//                   ^^^^^^^ERR(>=1.44.0-beta) no method named `missing`
//                   ^^^^^^^ERR(>=1.44.0-beta) method not found in
//                   ^^^^^^^MSG(>=1.44.0-beta) See Also: macro-expansion.rs:7
}

#[macro_export]
macro_rules! example_bad_syntax {
    () => {
        enum E {
            Kind(x: u32)
//                ^ERR(>=1.44.0-beta) /expected one of .*, found `:`/
//                ^ERR(>=1.44.0-beta) /expected one of .* tokens/
//                ^MSG(>=1.44.0-beta) See Also: macro-expansion-outside-1.rs:9

        }
    }
}

#[macro_export]
macro_rules! example_bad_value {
    () => (1i32)
//         ^^^^ERR(>=1.44.0-beta) mismatched types
//         ^^^^ERR(>=1.44.0-beta) expected `()`, found `i32`
//         ^^^^MSG(>=1.44.0-beta) See Also: macro-expansion-outside-2.rs:9
}
