use crate::cmp;

fn quick_sort_inner<T, F>(arr: &mut [T], lt: &mut F)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let last = arr.len()-1;
    let (mut l, mut r) = (0, last);
    while l < r {
        if cmp!(lt(arr[l],> arr[last])) {
            r -= 1;
            arr.swap(l, r);
        } else {
            l += 1;
        }
    }
    arr.swap(l, arr.len()-1);
    quick_sort_inner(&mut arr[..l], lt);
    quick_sort_inner(&mut arr[r+1..], lt);
}

/// Quick sort
///
/// # Example
/// ```
/// # use sorts_rs::normal::quick_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// quick_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn quick_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    quick_sort_inner(arr, &mut lt)
}
