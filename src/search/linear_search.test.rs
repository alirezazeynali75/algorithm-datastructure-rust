#[cfg(test)]
use crate::search::linear_search::linear_search;

#[test]
fn linear_search_returns_some_index_for_negative_target_present() {
    let arr = [-5, 3, 0, 8, -2, 1];
    let target = -2;
    assert_eq!(linear_search(&arr, target), Some(4));
}

#[test]
fn linear_search_returns_none_for_not_included_target() {
  let arr = [-5, 3, 0, 8, 1];
  let target = -2;
  assert_eq!(linear_search(&arr, target), None);
}

#[test]
fn linear_search_returns_none_for_empty_target() {
  let arr = [];
  let target = 0;
  assert_eq!(linear_search(&arr, target), None);
}
