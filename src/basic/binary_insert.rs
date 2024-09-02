use crate::cmp;

/// Search insert point
///
/// return first greater than index
fn binary_search<T, F>(arr: &[T], ele: &T, mut lt: F) -> usize
where F: FnMut(&T, &T) -> bool,
{
    let (mut start, mut end) = (0, arr.len());

    while start < end {
        let mid = start + ((end - start) >> 1);
        if cmp!(lt(arr[mid],<= *ele)) {
            start = mid+1;
        } else {
            end = mid;
        }
    }

    debug_assert_eq!(start, end);
    start
}

fn insert_point<T, F>(arr: &mut [T], lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let (ele, sorted) = arr.split_last().unwrap();
    let idx = binary_search(sorted, ele, lt);
    arr[idx..].rotate_right(1)
}

/// Insertion sort, use binary search
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::binary_insert_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// binary_insert_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn binary_insert_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let (rng, _) = arr.split_at_mut(i+1);
        insert_point(rng, &mut lt)
    }
}

#[cfg(test)]
#[test]
fn binary_search_test() {
    let lt = i32::lt;
    assert_eq!(binary_search(&[1, 3, 4, 4], &5, lt), 4);
    assert_eq!(binary_search(&[1, 3, 4, 4], &4, lt), 4);
    assert_eq!(binary_search(&[1, 3, 4, 4], &3, lt), 2);
    assert_eq!(binary_search(&[1, 3, 4, 4], &2, lt), 1);
    assert_eq!(binary_search(&[1, 3, 4, 4], &1, lt), 1);
    assert_eq!(binary_search(&[1, 3, 4, 4], &0, lt), 0);
}
