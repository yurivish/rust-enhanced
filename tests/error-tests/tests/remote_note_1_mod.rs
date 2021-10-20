pub fn f() {
    return;
//  ^^^^^^ERR(>=1.39.0-beta) any code following
    println!("Paul is dead");
//  ^^^^^^^^^^^^^^^^^^^^^^^^ERR(>=1.57.0-beta) unreachable statement
//  ^^^^^^^^^^^^^^^^^^^^^^^^ERR(>=1.57.0-beta) this error originates in a macro outside of the current crate
//  ^^^^^^^^^^^^^^^^^^^^^^^^MSG(>=1.57.0-beta) See Also: remote_note_1.rs:1
//  ^^^^^^^^^^^^^^^^^^^^^^^^ERR(>=1.57.0-beta) unreachable statement
//  ^^^^^^^^^^^^^^^^^^^^^^^^^ERR(<1.57.0-beta) unreachable statement
//  ^^^^^^^^^^^^^^^^^^^^^^^^^ERR(<1.57.0-beta) this error originates in a macro outside of the current crate
//  ^^^^^^^^^^^^^^^^^^^^^^^^^MSG(<1.57.0-beta) See Also: remote_note_1.rs:1
//  ^^^^^^^^^^^^^^^^^^^^^^^^^ERR(>=1.39.0-beta,<1.57.0-beta) unreachable statement
}
