// pub fn max_sum_subarray(arr:&[i32],k:usize) -> Option<i32>{

//     if arr.len() <k || k == 0 {
//         return None;
//     } 


//     let mut window_sum = arr[..k].iter().sum::<i32>();


//     let mut max_sum = window_sum;

//     for i in k..arr.len() {
//         window_sum += arr[i]-arr[i-k];

//         if window_sum > max_sum {
//             max_sum = window_sum;
//         }
//     }
//     Some(max_sum)

// }

pub fn max_sum_subarray(arr:&[i32], k: usize)-> Option<i32>{

    if arr.len() < k || k == 0{
        return None;
    }

    let mut window_sum = arr[..k].iter().sum::<i32>();

    let mut max_sum = window_sum;


    for i in k..arr.len(){
        window_sum += arr[i] - arr[i-k];

        if window_sum > max_sum{
            max_sum = window_sum;
        }
    }
    Some(max_sum)
}