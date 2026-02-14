


pub fn two_sum(nums:&[i32], target:i32) -> (usize,usize) {

    let mut map = HashMap::new();

    for (i, num) in arr.iter().enumarate() {
        let complement = target - num;
        
        if let Some(&j) =map.get(&complement) {
            return ( i, j);
        }
        map.insert(num, i);
    }
    ()
}