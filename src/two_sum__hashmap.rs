pub fn two_sum_hashmap(arr:&[i32],tar:i32) -> (usize,usize) {

    let value_to_index = HashMap::new();

    for (i, num) in arr.iter().enumarate() {
        let  complement = tar - num;
        if value_to_index[complement].is_excit() {
            let index = value_to_index[complement];
            (i, index)
        }

        value_to_index(complement, i);
    }
    (0, 0)
}