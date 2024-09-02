use std::mem::swap;

use crate::cmp;

/// Bubble sort
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::cocktail_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// cocktail_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn cocktail_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let mut edited = true;

    while edited {
        edited = false;

        arr.iter_mut().reduce(|a, b| {
            if cmp!(lt(a,>b)) {
                swap(a, b);
                edited = true;
            }
            b
        });

        arr.iter_mut().rev().reduce(|a, b| {
            if cmp!(lt(a,<b)) {
                swap(a, b);
                edited = true;
            }
            b
        });
    }
}
