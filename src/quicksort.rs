pub fn quick_sort(arr: &mut [i32]) {

    let len =  arr.len();

    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);

    quick_sort(&mut arr[pivot_index+1..]);


}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = arr[pivot_index];

    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}