use std::cmp::max;

pub fn max_volume(arr: &[i32])-> i32{

    let len = arr.len();

    let mut ans = 0;

    for i in 0..len{

        for j in i+1 .. len{
            let heigh = max(arr[i], arr[j]);
            let length = (j -i) as i32;
            
            
            ans = max(ans, heigh* length);
        
        }
    }
    ans
    
}