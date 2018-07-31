// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"
// Random things that don't deserve a file on their own.

// Unsafe
let x = unsafe { }
//      ^^^^^^ storage.modifier
unsafe impl<T> Send for Interned<T> {}
// <- storage.modifier
//^^^^ storage.modifier
pub unsafe trait Alloc { }
//  ^^^^^^ storage.modifier
