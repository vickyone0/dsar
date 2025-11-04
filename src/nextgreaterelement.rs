pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {

    let n = nums.len();
    let mut res = vec![-1;n];
    let mut stack: Vec<i32> = Vec::new();

    for i in (0..n).rev() {
        while let Some(&top) = stack.last() {
            if top <= nums[i] {
                stack.pop();
            } else {
                break;
            }
        }
    

    if let Some(&next) = stack.last() {
        res[i] = next;
    }

    stack.push(nums[i]);
}
res
}
