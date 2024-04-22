fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut current_sum = i32::MIN;
    let mut max_sum = i32::MIN;
  
    for num in arr.iter() {
      // Update current_sum to consider the current element or start a new subarray
      current_sum = std::cmp::max(*num, current_sum + *num);
      // Update max_sum if the current subarray sum is larger
      max_sum = std::cmp::max(max_sum, current_sum);
    }
  
    max_sum
  }
  