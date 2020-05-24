

mod algorithms;

#[cfg(test)]
mod tests {
    use super::algorithms::{sort_methods::*, assert_alg};
    #[test]
    fn test_sort_alg() {
        assert_alg(insertion_sort_1);
        // assert_alg(merge_sort);
    }
}