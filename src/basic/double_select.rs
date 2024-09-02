use crate::cmp;

/// Selection sort
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::select_doubled_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// select_doubled_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn select_doubled_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let (mut l, mut r) = (0, arr.len() - 1);
    while l < r {
        let [mut min, mut max] = [&arr[l], &arr[r]];
        let [mut mini, mut maxi] = [l, r];
        (l..=r).for_each(|i| {
            if cmp!(lt(arr[i],< min)) {
                (mini, min) = (i, &arr[i]);
            }
            if cmp!(lt(arr[i],> max)) {
                (maxi, max) = (i, &arr[i]);
            }
        });
        if maxi == l { maxi = mini }
        arr.swap(l, mini);
        arr.swap(r, maxi);
        l += 1;
        r -= 1;
    }
}
