pub fn intersection(num1: Vec<i32>, num2:Vec<i32>) -> Vec<i32>{
    use std::collections::HashSet;

    let set1: HashSet<_> = num1.into_iter().collect();
    let set2: HashSet<_> = num2.into_iter().collect();

    set1.intersection(&set2).cloned().collect()
}