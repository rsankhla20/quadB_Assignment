fn reverse_string(text: &str) -> String {
    let mut reversed = String::new();
    for ch in text.chars().rev() {
      reversed.push(ch);
    }
    reversed
  }
  