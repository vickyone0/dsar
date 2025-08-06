use std::option;

pub fn two_sum_sorted(arr: &[i32], tar: i32) -> Option<(usize, usize)>{

    let (mut left, mut right) = (0, arr.len()-1);


    while left< right{

        let mut sum = arr[left] + arr[right];

        if sum == tar {
            return Some((left+1, right+1));
        }
        else if sum < tar {

            left +=1;

            
        }
        else {
            right -=1;
        }
    }
    
    None

}