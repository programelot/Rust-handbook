// These are intergated tests.
// It only will be tested when it passes every unit tests.
// Intergrated tests can be tested one by one with following command.
// cargo test --test file_name
// e.x) cargo test --test 1_tests

#[test]
fn test_in_test_directory1() {
    println!("test in test directory");
}

#[test]
#[should_panic]
fn test_in_test_directory2() {
    println!("test in test directory");
    panic!("fail");
}
