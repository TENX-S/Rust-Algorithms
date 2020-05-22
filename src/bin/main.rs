#![allow(non_snake_case)]



/// Code that exists only for debugging,
/// if you want to see algorithms, please move to algorthms/

fn main() {
    use rand::prelude::*;

    // let mut nums: Vec<i32> = vec![8, 4, 5, 7, 1, 3, 6, 2];
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (1..50_0000).collect();
    nums.shuffle(&mut rng);
    divide(&mut nums, 0, 49_9998);

    // println!("{:?}", nums);
}

// Combine two ordered arrays into one ordered array

fn merge(arr: &mut Vec<i32>, first: usize, mid: usize, last: usize) {
    let mut i = first;
    let mut j = mid + 1;
    let m = mid;
    let n = last;
    let mut temp: Vec<i32> = Vec::with_capacity(arr.len());

    while i <= m && j <= n {
        if arr[i] < arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }

    while i <= m {
        temp.push(arr[i]);
        i += 1;
    }

    while j <= m {
        temp.push(arr[j]);
        j += 1;
    }

    for i in 0..temp.len() {
        arr[first + i] = temp[i];
    }


}


fn divide(arr: &mut Vec<i32>, first: usize, last: usize) {
    if first < last {
        let mid = (first + last) / 2;
        divide(arr, first, mid);
        divide(arr, mid+1, last);
        merge(arr, first, mid, last);
    }
}

