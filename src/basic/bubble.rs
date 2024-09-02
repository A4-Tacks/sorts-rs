use std::mem::swap;

use crate::cmp;

/// Bubble sort
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::bubble_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// bubble_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn bubble_sort<T, F>(mut arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let mut edited = true;

    while edited {
        edited = false;

        arr.iter_mut()
            .reduce(|a, b|
        {
            if cmp!(lt(a,>b)) {
                swap(a, b);
                edited = true;
            }
            b
        });
        (_, arr) = arr.split_last_mut().unwrap();
    }
}
