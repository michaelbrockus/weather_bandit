//
// file: test_cases.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
// USE CASE:
//
// The use case of this file is to contain the test cases needed by this
// project since its important to test once implementation against a set
// of common test cases
//

//
// list of all the test cases for this project
//
#[cfg(test)]
mod tests {

    #[test]
    fn test_equal() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn test_greet() {
    //     assert_eq!("Hello, Rust Developer.", program::greet());
    // }
}
