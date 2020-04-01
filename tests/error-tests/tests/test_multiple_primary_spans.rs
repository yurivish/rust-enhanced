// This test is for a message with multiple primary spans
// (both spans have no label).

#[repr(C, u64)]
//     ^WARN(<1.42.0-beta) conflicting representation hints
//        ^^^WARN(<1.42.0-beta) conflicting representation hints
//     ^ERR(>=1.42.0-beta) conflicting representation hints
//        ^^^ERR(>=1.42.0-beta) conflicting representation hints
//     ^NOTE(>=1.42.0-beta) `#[deny(conflicting_repr_hints)]` on by default
//     ^WARN(>=1.42.0-beta) this was previously
//     ^NOTE(>=1.42.0-beta) for more information
pub enum C { C }
