// Test can be done by following command.
// cargo test (test all)
// Programmer can use flag for testing binary by writing flags next to --.
// In other world, it works as belows.
// cargo test {cargo flag} -- {test binary flag}
// e.x) cargo test -- --show-output

// Test can be done with functions with certain word in the name.
// e.x) cargo test success

// #[cfg(test)] denotes this will build when only using cargo test.
// cfg is a acronym of configuration.
// It is not included to the build process (cargo run or cargo build)

// Belows are unit tests.
// It can be included in a module.
// It can tests parent's private functions.

// There are intergrated tests.
// It located in /tests/

fn private_function() {
    println!("private function");
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {} // This test always success.

    #[test]
    fn failure_panic() {
        // This test always fails.
        // panic!("failed");
    }

    // It should be failed.
    // Howver it denoted as should_panic.
    // Therefore, program consider failure of this test as okay.
    // Failure tests below are written with should_panic.
    // Please comment it out how it works.
    #[test]
    #[should_panic]
    fn failure_should_panic() {
        panic!("panic");
    }

    #[test]
    #[should_panic]
    fn failure_assert() {
        // This test fails since f is false.
        let f = false;
        // assert works similar with c++.
        // It panic when it get false value.
        assert!(f);
    }

    #[test]
    #[should_panic]
    fn failure_assert_eq() {
        // This test fails since hello is not Hello world!
        let hello = "hello";
        assert_eq!(hello, "Hello world!");
    }
    // ---- tests::failure_assert_eq stdout ----
    // thread 'tests::failure_assert_eq' panicked at src\bin\18_test.rs:63:9:
    // assertion `left == right` failed
    // left: "hello"
    // right: "Hello world!"
    // It shows why test failed.

    #[test]
    fn success_assert_ne() {
        // This test success since hello is not bye
        let hello = "hello";
        assert_ne!(hello, "bye");
    }

    #[test]
    #[should_panic]
    fn failure_assert_with_message() {
        // This test fails and prints message.
        let nine = 10;
        assert_eq!(
            nine, 9,
            "Checked that nine was nine but it wasn't. The value was {0}",
            nine
        );
    }
    // thread 'tests::failure_assert_with_message' panicked at src\bin\18_test.rs:84:9:
    // assertion `left == right` failed: Checked that nine was nine but it wasn't. The value was 10
    // left: 10
    // right: 9

    // Should panic doesn't care about which panic it will be.
    // Therefore, programmer can set a part of the message.
    // This test will pass if debug message includes panic.
    #[test]
    #[should_panic(expected = "panic")]
    fn failure_should_panic_with_expect() {
        panic!("panic");
    }

    // Testing with Result can not use #[should_panic]
    #[test]
    fn failure_Result_T_E() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        Err(String::from("Error"))
    }

    // If test fails, it gives other outputs either.
    // It doesn't show output when it succeed unless it uses --show-output flag like using following command.
    // cargo test -- --show-output
    #[test]
    #[should_panic]
    fn failure_println() {
        println!("Test done.");
        panic!("panic");
    }
    // ---- tests::failure_println stdout ----
    // Test done.
    // thread 'tests::failure_println' panicked at src\bin\18_test.rs:120:9:
    // panic

    #[test]
    fn success_println() {
        println!("Test done.");
    }
    // Ouput with --show-output flag is as follows.

    // ---- tests::success_println stdout ----
    // Test done.

    // Some tests can be ignored unless it is manually called.
    // These tests can be called with following commands.
    // cargo test -- --ignored
    // cargo test -- --include-ignored
    // The first command will only test with ignored cases.
    // The second command will test every thing includes ignored ones.
    #[test]
    #[ignore]
    fn success_println_ignored() {
        println!("Test done.");
    }

    #[test]
    fn success_private_function() {
        super::private_function();
    }
}

//It will not be used.
fn main() {}
