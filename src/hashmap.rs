use std::collections::HashMap;

pub fn has_zero_sum_subarray(arr: &[i32]) -> bool {


    let mut prefix_sum = 0;

    let mut seen = HashMap::new();

    seen.insert(0, false);

    for &num in arr {
        prefix_sum += num;

        if seen.contains_key(&prefix_sum) {
            return  true;
        }
        seen.insert(prefix_sum, false);
    }
    false

}

pub fn count_zero_sum_subarrays(arr: &[i32]) -> i32 {
    let mut prefix_sum = 0;
    let mut count = 0;
    let mut freq = HashMap::new();

    freq.insert(0,1);

    for &num in arr {
        prefix_sum += num;

        if let Some(&c) = freq.get(&prefix_sum){
            count += c;

        }

        *freq.entry(prefix_sum).or_insert(0) +=1;
    }
    count
}

pub fn longest_zero_sum_subarray(arr: &[i32]) -> usize {
    let mut prefix_sum = 0;
    let mut max_len = 0;
    let mut map = HashMap::new();

    map.insert(0, -1);

    for (i, &num) in arr.iter().enumerate() {
        prefix_sum += num;

        if let Some(&first_index) = map.get(&prefix_sum) {

            let length = i as isize - first_index;
            if length as usize > max_len {
                max_len = length as usize;
            }

        }
        else {
            map.insert(prefix_sum, i as isize);
        }
    }
    max_len
}