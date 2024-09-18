fn linear_search_helper(arr: &[i32], target: i32) -> Option<usize> {
  for (index, &item) in arr.iter().enumerate() {
    if item == target {
      return Some(index)
    }
  }
  None
}

pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
  linear_search_helper(arr, target)
}
