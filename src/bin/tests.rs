

mod algorithms;



#[cfg(test)]
mod tests {
    use super::algorithms::sort_methods::*;


    #[test]
    fn test_insertion_sort() {
        
        let mut L = vec![9, 8, 7, 6, 4];
        insertion_sort(&mut L);
        println!("{:?}", L);
    }

}