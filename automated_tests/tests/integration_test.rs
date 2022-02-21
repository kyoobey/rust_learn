

use automated_tests;

mod common;

#[test]
fn integration_test_greeting() {
	assert!(automated_tests::greeting("Joe").contains("Joe"));
}

#[test]
fn integration_test_this() {
	common::setup();
	assert!(2+2==4);
}