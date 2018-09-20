// Testing a Clippy warning (char_lit_as_u8).

pub fn clippy_example() {
    println!("libf1");
    'x' as u8;
//  ^^^^^^^^^WARN casting character literal
//  ^^^^^^^^^NOTE char_lit_as_u8
//  ^^^^^^^^^HELP Consider using a byte
//  ^^^^^^^^^HELP for further information
//  ^^^^^^^^^^WARN statement with no effect
//  ^^^^^^^^^^NOTE no_effect
//  ^^^^^^^^^^HELP for further information
}
