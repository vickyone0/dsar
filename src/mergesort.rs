pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {

    let len = arr.len();
    if len <= 1 {
        return arr;
    }

    let mid = len /2;

    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());

    merge(left,right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {

    let mut merged = Vec::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() &&j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i +=1;
        } else {
            merged.push(right[j]);
            j+=1;
        }
    }

    if i < left.len() {
        merged.extend_from_slice(&left[i..]);
    }

    if j < right.len() {
        merged.extend_from_slice(&right[j..]);
    }

    merged
}