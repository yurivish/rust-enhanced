// Tests for cycling through the errors from a test run.

#[test]
fn passing() {
}

#[test]
fn basic_error1() {
    assert!(false);
}

#[test]
fn basic_error2() {
    assert_eq!(1, 0);
}

#[test]
fn basic_error3() {
    assert_ne!(1, 1);
}

#[test]
fn custom_message() {
    assert!(false, "Custom message");
}

#[test]
fn manual_panic() {
    panic!("manual panic");
}

#[test]
#[should_panic(expected="xyz")]
fn expected_panic1() {
    panic!("xyz");
}

#[test]
#[should_panic(expected="xyz")]
fn expected_panic2() {
}

#[test]
#[ignore]
fn ignored_test() {
    panic!("ignored");
}

#[test]
#[ignore]
fn slow() {
    // This just prints a simple warning.
    std::thread::sleep(std::time::Duration::from_secs(65));
}

#[test]
fn panic_outside_crate() {
    // This (unfortunately) should not show up as an error when running tests
    // since Rust prints the path to src/libcore/result.rs. (This has been fixed in 1.42.)
    Err::<i32, &str>("err").unwrap();
}
