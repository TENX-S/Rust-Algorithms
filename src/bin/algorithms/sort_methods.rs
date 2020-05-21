

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

