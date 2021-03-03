// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(3 > 1);
    }
    
    #[test]
    fn assert_eq() {
        assert_eq!(9, 3_u8.pow(2));
    }

    #[test]
    #[should_panic(expected = "PANIC !")]
    fn you_can_assert_and_panic() {
        assert!(panic!("PANIC !"));
    }
}
