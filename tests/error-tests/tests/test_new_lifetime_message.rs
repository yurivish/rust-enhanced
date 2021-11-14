// 1.21 added new support for messages with multiple spans.
/*BEGIN*/fn foo(x: &mut Vec<&u32>, y: &u32) {
//                          ^^^^ERR(>=1.21.0)
//                                    ^^^^ERR(>=1.21.0) these two types
//                                    ^^^^MSG(>=1.21.0) See Primary: ↓:12
//             |HELP(>=1.58.0-beta) consider introducing
//             |HELP(>=1.58.0-beta) /Accept Replacement:.*<'a>/
//                           |HELP(>=1.58.0-beta) consider introducing
//                           |HELP(>=1.58.0-beta) /Accept Replacement:.*'a/
//                                     |HELP(>=1.58.0-beta) consider introducing
//                                     |HELP(>=1.58.0-beta) /Accept Replacement:.*'a/
  x.push(y);
//       ^ERR(>=1.21.0) lifetime mismatch
//       ^ERR(>=1.21.0) ...but data from
//       ^ERR(<1.21.0) lifetime of reference outlives
//       ^NOTE(>=1.58.0-beta) each elided lifetime
//       ^MSG(>=1.21.0) See Also: ↑:2
}/*END*/
// ~NOTE(<1.21.0) ...the reference is valid
// ~NOTE(<1.21.0) ...but the borrowed content
// ~HELP(<1.16.0) consider using an explicit lifetime
