

mod algorithms;

#[cfg(test)]
mod tests {

    use super::algorithms::{sort_methods::*, assert_alg};

    #[test]
    // Test your sort algorithms
    // Please design your function signature as fn your_algorithms(&mut Vec<i32>) -> ()
    fn test_sort_alg() {
        assert_alg(insertion_sort_1);
        // assert_alg(merge_sort);
    }
}