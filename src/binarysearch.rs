pub fn binary_search(arr: &[i32], tar: i32) -> Option<usize>{

    let mut left = 0;
    let mut right = arr.len();

    while left < right{

        let mid = left + (right - left)/2;

        if arr[mid] == tar {
            return Some(mid);
        }
        else if arr[mid] < tar {
            left = mid+ 1;
            
        }else{
            right = mid;
        }
    }
    None
}