fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_vec = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;
  
    while i < arr1.len() && j < arr2.len() {
      if arr1[i] <= arr2[j] {
        merged_vec.push(arr1[i]);
        i += 1;
      } else {
        merged_vec.push(arr2[j]);
        j += 1;
      }
    }
  
    // Append remaining elements from either array (if any)
    merged_vec.extend_from_slice(&arr1[i..]);
    merged_vec.extend_from_slice(&arr2[j..]);
  
    merged_vec
  }
  