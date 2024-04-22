fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
      return None;
    }
  
    let mid_idx = len / 2;
    if len % 2 == 0 {
      // if even number of elements, median is average of middle two elements
      let median = (arr[mid_idx - 1] as f64 + arr[mid_idx] as f64) / 2.0;
      return Some(median);
    } else {
      // if odd number of elements, median is the middle element
      let median = arr[mid_idx] as f64;
      return Some(median);
    }
  }
  