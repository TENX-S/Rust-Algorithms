

/// Insertion sort algorithm
const USIZE_MAX:usize = std::usize::MAX;


pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j-1;

        while i != USIZE_MAX && arr[i] > key {
            arr[i+1] = arr[i];
            // Or write code like follows ⬇️
            // arr.swap(i+1, i);
            i = i.wrapping_sub(1);
        }

        match i {
            USIZE_MAX => { arr[0] = key; },
            _ => { arr[i+1] = key },
        }
    }
}

pub fn merge_sort(arr: &mut Vec<i32>, first: usize, last: usize) {

    if first < last {
        let mid = (first + last) / 2;
        merge_sort(arr, first, mid);
        merge_sort(arr, mid+1, last);
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

    while j <= n {
        temp.push(arr[j]);
        j += 1;
    }

    for i in 0..temp.len() {
        arr[first + i] = temp[i];
    }
}
