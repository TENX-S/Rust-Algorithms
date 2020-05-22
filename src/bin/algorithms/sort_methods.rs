

/// Insertion sort algorithm
const USIZE_MAX:usize = std::usize::MAX;

pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j-1;

        while i != USIZE_MAX && arr[i] > key {
            arr[i+1] = arr[i];
            i = i.wrapping_sub(1);
        }

        match i {
            USIZE_MAX => { arr[0] = key; },
            _ => { arr[i+1] = key; },
        }

        // if i == USIZE_MAX {
        //     arr[0] = key
        // } else {
        //     arr[i+1] = key
        // }
    }
}



pub fn merge(arr: &mut Vec<i32>, first: usize, mid: usize, last: usize) {
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


pub fn divide(arr: &mut Vec<i32>) {

    let first = 0;
    let last = arr.len();
    if first < last {
        let mid = (first + last) / 2;
        divide(&mut arr[first..mid+1].to_vec());
        divide(&mut arr[mid+1..last].to_vec());
        merge(arr, first, mid, last);
    }
}
