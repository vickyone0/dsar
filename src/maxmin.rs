pub fn max_min(arr: &[i32]) -> Option<(i32,i32)> {
    let min =arr.iter().min()?;
    let max = arr.iter().max()?;
    Some((*min, *max))
}