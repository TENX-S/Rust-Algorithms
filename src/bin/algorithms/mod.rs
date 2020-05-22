pub mod sort_methods;


pub fn assert_alg(alg_func: fn(&mut Vec<i32>)) {
    let mut nums_set = random_nums_set(100, 200);

    for num in &mut nums_set {
        let mut num_copy = num.clone();
        num_copy.sort();

        alg_func(num);
        assert_eq!(num_copy, *num);
    }
}

fn random_nums_set(cases_num: usize, max_length: i32) -> Vec<Vec<i32>> {

    use rand::prelude::*;
    let mut nums_set: Vec<Vec<i32>> = vec![];
    let mut rng = thread_rng();
    for _i in 0..cases_num {
        let upper = rng.gen_range(10, max_length) as i32;
        let mut nums: Vec<i32> = (1..upper).collect();
        nums.shuffle(&mut rng);
        nums_set.push(nums);
    }
    let special_cases: Vec<Vec<i32>> = vec![vec![0], vec![1, 1, 1]];
    for case in special_cases {
        nums_set.push(case);
    }
    nums_set
}