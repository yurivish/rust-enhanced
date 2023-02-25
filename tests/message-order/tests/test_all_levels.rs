trait Trait {}

// This triggers a warning about missing `dyn`
fn Foo(x: &Fn()) {}

pub fn f() {
    let x = Box::new(0u32);
    // This is an error with a note and help.
    let y: &dyn Trait = x;
}
