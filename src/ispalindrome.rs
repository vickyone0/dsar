pub fn is_palindrome(val: &str) -> bool{

    let cleaned: String= val.chars().filter(|c| c.is_alphabetic())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let is_palindrome = cleaned == cleaned.chars().rev().collect::<String>();

    is_palindrome

}