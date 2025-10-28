pub fn count_vowels_in_string(val: &str) -> i32{
    let vowels = ['a','e','i','o','u'];
    let count = val
                                .to_lowercase()
                                .chars()
                                .filter(|c| vowels.contains(c))
                                .count() as i32;
                            count
}