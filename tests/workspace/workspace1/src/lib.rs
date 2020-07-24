mod anothermod;

/*BEGIN*/struct S {
//       ^^^^^^^^ERR(>=1.18.0) recursive type has infinite size
//       ^^^^^^^^ERR(>=1.18.0) recursive type `S` has infinite size
//       ^^^^^^^^HELP(>=1.18.0,<1.46.0-beta) insert indirection
    recursive: S
//  ^^^^^^^^^^^^ERR(>=1.19.0,<1.46.0-beta) recursive without indirection
//             ^ERR(>=1.46.0-beta) recursive without indirection
//             |HELP(>=1.46.0-beta) insert some indirection
//             |HELP(>=1.46.0-beta) /Accept Replacement:.*Box</
//              |HELP(>=1.46.0-beta) insert some indirection
//              |HELP(>=1.46.0-beta) /Accept Replacement:.*>/
}/*END*/
// ~ERR(<1.18.0) recursive type has infinite size
// ~ERR(<1.18.0) recursive type `S` has infinite size
// ~HELP(<1.18.0) insert indirection
