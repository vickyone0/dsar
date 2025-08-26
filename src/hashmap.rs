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