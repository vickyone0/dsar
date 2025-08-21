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

pub fn rsa_min_value(arr: &[i32]) -> i32{

    let mut left = 0;
    let mut right = arr.len()-1;

    while left < right {
        let mid = left + (right - left)/2;

        if arr[mid] > arr[right] {
            left = mid + 1;
        }
        else {
            right = mid;
        }
    }

    arr[left]

}