
// pub fn two_sum_values(nums: Vec<i32>, target: i32) -> Option<(i32,i32)> {
//     let mut map = HashMap::new();

//     for &num in &nums {
//         let complement = terget -num;

//         if map.contains_key(&complement) {
//             return Some((complement, num));
//         }
//         map.insert(num, true);
//     }
//     None
// }