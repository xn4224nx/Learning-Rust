use external_testing;
mod common;

#[test]
fn it_adds_two() {
	
	common::setup();
	assert_eq!(4, external_testing::add(2, 2));
}
