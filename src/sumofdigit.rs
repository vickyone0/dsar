pub fn sum_of_digit(mut num: i32) ->i32 {
    let mut sum = 0;
    
    while num > 0 {
        sum += num %10;
        num /= 10;
    }
    sum
}