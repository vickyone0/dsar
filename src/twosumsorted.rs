use std::option;

// pub fn two_sum_sorted(arr: &[i32], tar: i32) -> Option<(usize, usize)>{

//     let (mut left, mut right) = (0, arr.len()-1);


//     while left< right{

//         let mut sum = arr[left] + arr[right];

//         if sum == tar {
//             return Some((left+1, right+1));
//         }
//         else if sum < tar {

//             left +=1;

            
//         }
//         else {
//             right -=1;
//         }
//     }
    
//     None

// }

// pub fn two_sum_sorted(arr: &[i32], tar: i32) -> Option<(usize, usize)> {

//     let (mut left, mut right) = (0, arr.len()-1);

//     while left < right {

//         let sum = arr[left] + arr[right];

//         if sum == tar{
//             return Some((left+1,right+1));
//         }
//         else if sum < tar {
//             left +=1;
            
//         }
//         else {
//             right -=1;
//         }
//     }
//         None
//     }


pub fn two_sum(arr: &[i32], tar: i32) -> (usize,usize){
        
        if arr.len()==1 {
            return (0,0);
        }
        for i in 0..arr.len(){
            for j in 0..arr.len(){
                if i != j {
                if arr[i] + arr[j]== tar{
                    return (i,j);
                }
            }
            }
        }
        (0,0)
}

pub fn two_sum_sorted(arr: &[i32],tar: i32) -> (usize,usize) {

    let mut i = 0;
    let mut j = arr.len()-1;

    while i < j {

        let sum = arr[i] + arr[j];

        if sum < tar {
            i +=1;
        }
        else if sum > tar {
            j -=1;
            
        }
        else if sum == tar{
            return (i,j);
        }

    }

    (0,0)
}