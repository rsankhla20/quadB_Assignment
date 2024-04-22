fn find_kth_smallest_sorted(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
      return None;
    }
  
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    return Some(sorted_arr[k - 1]);
  }
  