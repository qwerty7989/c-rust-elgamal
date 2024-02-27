#[cfg(test)] // Runs code only during 'cargo test'
mod tests {

    use crate::util;

    use super::*;
    #[test] // Identifies a test function
    fn test_name() {
        // Assertions: Verify results
        assert_eq!(util::upperbound(5), 7);
    }
}
