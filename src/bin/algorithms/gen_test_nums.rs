

pub fn random_nums_set(cases_num: usize, max_length: i32) -> Vec<Vec<i32>> {

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
