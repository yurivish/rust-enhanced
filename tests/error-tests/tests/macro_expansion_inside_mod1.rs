#[macro_export]
macro_rules! example_bad_syntax {
    () => {
        enum E {
            // This is somewhat of an odd example, since rustc gives two
            // syntax errors.
            Kind(x: u32)
//                ^ERR /expected one of .*, found `:`/
//                ^ERR(>=1.41.0-beta) /expected one of .* tokens/
//                ^ERR(>=1.18.0,<1.41.0-beta) /expected one of .* here/
//                ^MSG(>=1.20.0) See Also: macro-expansion-inside-1.rs:6
//                ^ERR(<1.34.0-beta) /expected one of .*, found `:`/
//                ^ERR(>=1.18.0,<1.34.0-beta) expected one of
//                ^MSG(>=1.20.0,<1.34.0-beta) See Also: macro-expansion-inside-1.rs:6
        }
    }
}
