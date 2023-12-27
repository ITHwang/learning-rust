// Each file in the `tests` directory is a separate crate,
// so we need to bring out library into each test crate's scope.
use ch11_automated_tests;

mod common;

#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(ch11_automated_tests::add_two(2), 4);
}

// To run all the tests in a particular integration test file,
// $ cargo test --test integration_test