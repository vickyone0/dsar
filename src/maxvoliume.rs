use std::cmp::max;

// pub fn max_volume(arr: &[i32])-> i32{

//     let len = arr.len();

//     let mut ans = 0;

//     for i in 0..len{

//         for j in i+1 .. len{
//             let heigh = max(arr[i], arr[j]);
//             let length = (j -i) as i32;
            
            
//             ans = max(ans, heigh* length);
        
//         }
//     }
//     ans
    
// }

pub fn max_volume(arr:&[i32]) -> i32 {

    let mut sum = 0;

    for i in 0..arr.len(){
        for j in i+1..arr.len() {
            let height = max(arr[i], arr[j]);
            let length = (j-i) as i32;
            sum = max(sum, height*length);
        }
    }
    sum

}