/*BEGIN*/pub fn f() {
//       ^^^^^^^^^^ERR(>=1.24.0-beta,<1.49.0-beta) defined here
//       ^^^^^^^^^^MSG(>=1.24.0-beta,<1.49.0-beta) See Primary: error_across_mod.rs:4
//              ^NOTE(>=1.49.0-beta) defined here
//              ^MSG(>=1.49.0-beta) See Primary: error_across_mod.rs:4
}/*END*/
// ~ERR(<1.24.0-beta) defined here
// ~MSG(<1.24.0-beta) See Primary: error_across_mod.rs:4
