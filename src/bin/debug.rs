// use std::fmt::Debug;
//
// struct Builder;
//
// impl Builder {
//     fn build<I: Debug>(&self) -> fn(I) -> I {
//         fn input<I: Debug>(x: I) -> I { x }
//         input
//     }
// }
//

// fn test<F, T: Debug>(gen: F)
//     where F: Fn(Builder) -> T
// {
//     let builder = Builder;
//     println!("{:?}", gen(builder));
// }
//
// fn main() {
//     test(|builder| {
//         builder.build()(10);
//         builder.build()(10.0)
//     });
// }

fn main() {
    let mut case = vec![9, 8, 7];
    merge_sort(&mut case);
    println!("{:?}", case);
}


pub fn merge_sort(arr: &mut Vec<i32>) {

    let first = 0;
    let last = arr.len() - 1;

    if first < last {
        let mid = (first + last) / 2;
        merge_sort(&mut arr[first..mid+1].to_vec());
        merge_sort(&mut arr[mid+1..last+1].to_vec());
        merge_two_arr(arr, first, mid, last);
    }
}

fn merge_two_arr(arr: &mut Vec<i32>, first: usize, mid: usize, last: usize) {
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