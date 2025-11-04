pub fn zero_to_end(nums: &mut Vec<i32>) {
    let mut insert_pos = 0;

    for i in 0..nums.len() {
        if nums[i] !=0 {
            nums[insert_pos] = nums[i];
            insert_pos +=1;
        }
    }

    while insert_pos < nums.len() {
        nums[insert_pos] = 0;
        insert_pos +=1;
    }
}