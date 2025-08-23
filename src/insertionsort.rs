pub fn insertion_sort(arr: &mut [i32]) {

    let len = arr.len();

    for i in 1..len {

        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j -1] > key {
            arr[j] = arr[j -1];

            j -= 1;

        }
        arr[j] = key;
    }

}