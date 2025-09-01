use std::collections::HashMap;

// pub fn has_zero_sum_subarray(arr: &[i32]) -> bool {


//     let mut prefix_sum = 0;

//     let mut seen = HashMap::new();

//     seen.insert(0, false);

//     for &num in arr {
//         prefix_sum += num;

//         if seen.contains_key(&prefix_sum) {
//             return  true;
//         }
//         seen.insert(prefix_sum, false);
//     }
//     false

// }

// pub fn count_zero_sum_subarrays(arr: &[i32]) -> i32 {
//     let mut prefix_sum = 0;
//     let mut count = 0;
//     let mut freq = HashMap::new();

//     freq.insert(0,1);

//     for &num in arr {
//         prefix_sum += num;

//         if let Some(&c) = freq.get(&prefix_sum){
//             count += c;

//         }

//         *freq.entry(prefix_sum).or_insert(0) +=1;
//     }
//     count
// }

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

pub fn longest_consecutive(nums: &[i32]) -> usize {
    let mut map: HashMap<i32, bool> = HashMap::new();

    // Fill map with numbers, marked as not visited
    for &num in nums {
        map.insert(num, false);
    }

    let mut longest = 0;

    for &num in nums {
        // Only start from numbers that have not been visited
        if let Some(visited) = map.get_mut(&num) {
            if *visited {
                continue; // Skip if already part of a counted sequence
            }

            *visited = true;

            // Expand in both directions
            let mut length = 1;
            let mut left = num - 1;
            let mut right = num + 1;

            // Expand left
            while let Some(v) = map.get_mut(&left) {
                if *v {
                    break;
                }
                *v = true;
                length += 1;
                left -= 1;
            }

            // Expand right
            while let Some(v) = map.get_mut(&right) {
                if *v {
                    break;
                }
                *v = true;
                length += 1;
                right += 1;
            }

            longest = longest.max(length);
        }
    }

    longest
}

pub fn has_zero_subarray(arr: &[i32]) -> bool {
    let mut prefered_sum = 0;

    let sum: HashMap<i32, usize> = HashMap::new();


    for (i, &num) in arr.iter().enumerate(){
        if i == arr.len()-3 {
            return false;
        }
        //[5,50,-55,32,34

        prefered_sum += num;

        if prefered_sum ==0 && i >= 2 {
            return true;
        }

        if i > 2 && sum.contains_key(&prefered_sum) {
            return true;

        }
    }

    false
}

pub fn count_zero_sum_subarrays(arr:&[i32]) -> i32{

    let mut prefer_sum = 0;
    let mut count = 0;
    let mut sum_counter = HashMap::new();

    sum_counter.insert(0, 1);

    for &num in arr.iter(){
        prefer_sum += num;

        if let Some(freq) = sum_counter.get(&prefer_sum) {
            count += freq;
            
        }

        *sum_counter.entry(prefer_sum).or_insert(0)+=1;



    }

    count


}