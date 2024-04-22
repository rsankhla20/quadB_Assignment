fn is_palindrome(text: &str) -> bool {
    let mut chars = text.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = chars.len() - 1;
  
    while left < right {
      if chars[left].to_lowercase() != chars[right].to_lowercase() {
        return false;
      }
      left += 1;
      right -= 1;
    }
  
    true
  }

