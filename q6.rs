fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
      return "".to_string();
    }
  
    let mut prefix = strs[0].clone();
  
    for word in strs.iter().skip(1) {
      let mut i = 0;
      while i < prefix.len() && i < word.len() && prefix.chars().nth(i).unwrap() == word.chars().nth(i).unwrap() {
        i += 1;
      }
      prefix.truncate(i);
    }
  
    prefix
  }
  