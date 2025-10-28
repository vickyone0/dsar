use std::collections::HashSet;
pub fn unique(arr:&[i32]) -> Vec<i32>{
    let mut seen = HashSet::new();

    let unique: Vec<_> = arr.iter().filter(|&x| seen.insert(*x)).cloned().collect();
    unique
}