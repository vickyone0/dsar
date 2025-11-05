pub fn reverse_array(arr: &mut Vec<i32>, k: usize) {
    let n = arr.len();
    let k= K%n;

    arr.reverse();
    arr[..k].reverse();
    arr[k..].reverse();
}