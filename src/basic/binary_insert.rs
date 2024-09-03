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
/// > 使用二分查找确定位置来进行插入的插入排序, 在比较函数代价较大时,
/// > 可使比较次数更加均衡且具有对数增长速度而不是线性速度
/// > 由于空元素或单元素数组被认为是有序的, 且插入元素后依然是有序的,
/// > 所以已经遍历插入完毕的部分总是有序的, 所以能够应用二分查找
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
