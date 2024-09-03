use crate::cmp;

/// Selection sort
///
/// > 双重选择排序, 相对普通选择排序, 它会同时选择最大值和最小值
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
pub fn select_doubled_sort<T, F>(mut arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    while arr.len() > 1 {
        let tail = arr.len()-1;
        let [mut mini, mut maxi] = [0, tail];
        let [mut min, mut max] = [&arr[mini], &arr[maxi]];

        for (i, ele) in arr.iter().enumerate() {
            if cmp!(lt(ele,< min)) {
                (mini, min) = (i, ele);
            }
            if cmp!(lt(ele,> max)) {
                (maxi, max) = (i, ele);
            }
        }

        if maxi == 0 { maxi = mini }
        arr.swap(0, mini);
        arr.swap(tail, maxi);
        arr = &mut arr[1..tail];
    }
}
