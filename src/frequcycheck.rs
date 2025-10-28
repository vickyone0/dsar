use std::collections::HashMap;
pub fn frequency_count(arr:&[i32]) ->HashMap<i32, i32>{
    let mut freq = HashMap::new();

    for &num in arr {

        *freq.entry(num).or_insert(0) += 1;

    }

    freq
}