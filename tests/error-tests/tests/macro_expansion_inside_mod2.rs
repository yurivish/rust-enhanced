#[macro_export]
macro_rules! example_bad_value {
    () => (1i32)
//         ^^^^ERR mismatched types
//         ^^^^ERR(>=1.41.0-beta) expected `()`, found `i32`
//         ^^^^ERR(<1.41.0-beta) expected (), found i32
//         ^^^^NOTE(<1.41.0-beta) expected type `()`
//         ^^^^NOTE(<1.16.0) found type `i32`
//         ^^^^MSG See Also: macro-expansion-inside-2.rs:7
}
