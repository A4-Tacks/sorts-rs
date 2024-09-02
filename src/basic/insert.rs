use crate::cmp;

#[inline]
fn insert_point<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let (ele, sorted) = arr.split_last().unwrap();
    let idx = sorted.iter()
        .rposition(|cur| cmp!(lt(cur,<= ele)))
        .map(|i| i+1)
        .unwrap_or_default();
    arr[idx..].rotate_right(1)
}

/// Insertion sort
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::insert_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// insert_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn insert_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let (rng, _) = arr.split_at_mut(i+1);
        insert_point(rng, &mut lt)
    }
}

#[cfg(test)]
#[test]
fn insert_point_test() {
    let lt = i32::lt;
    {
        let mut arr = [0];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [0]);
    }
    {
        let mut arr = [0, 1];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [0, 1]);
    }
    {
        let mut arr = [2, 1];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [1, 2]);
    }
    {
        let mut arr = [1, 2, 3];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [1, 2, 3]);
    }
    {
        let mut arr = [1, 3, 2];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [1, 2, 3]);
    }
    {
        let mut arr = [1, 3, 4, 2];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [1, 2, 3, 4]);
    }
    {
        let mut arr = [1, 2, 3, 0];
        insert_point(&mut arr, lt);
        assert_eq!(arr, [0, 1, 2, 3]);
    }
}
