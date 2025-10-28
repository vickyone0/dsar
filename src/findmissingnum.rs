pub fn find_missing(arr:&[i32]) -> i32 {
    let n = arr.len() as i32 +1;
    let expeted_sum = n* (n+1) /2;
    let actual_sum: i32 = arr.iter().sum();
    expeted_sum - actual_sum
}