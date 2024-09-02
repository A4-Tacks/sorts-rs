use std::mem::swap;

use crate::cmp;

fn min_by<'a, T, I, F>(i: I, ele: &T, mut lt: F) -> Option<&'a mut T>
where F: FnMut(&T, &T) -> bool,
      I: IntoIterator<Item = &'a mut T>
{
    i.into_iter()
        .reduce(|a, b| {
            cmp!(lt(b,< a)).then_some(b).unwrap_or(a)
        })
        .filter(|cur| cmp!(lt(cur,< ele)))
}

/// Selection sort
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::select_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// select_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn select_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    for i in 0..arr.len() {
        let (sorted, unsorted) = arr.split_at_mut(i+1);
        let ele = sorted.last_mut().unwrap();
        if let Some(less) = min_by(unsorted, ele, &mut lt) {
            swap(ele, less)
        }
    }
}
