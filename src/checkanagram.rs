pub fn check_anagram(s1: &str, s2: &str) -> bool {

    let mut chars1:Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();

    chars1.sort_unstable();
    chars2.sort_unstable();

    let is_anagram = chars1==chars2;

    is_anagram



}