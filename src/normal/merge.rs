use std::{iter, mem::take};

use crate::{cmp, i};

/// Merge sorted `arr[..len]` and `arr[len..]`
///
/// 涉及到元素的双重存在, 需要使用unsafe且通过guard保证其UnwindSafe,
/// 这过于复杂, 虽然有能力实现但并不好阅读, 所以将约束Default并使用take
///
/// # Examples
/// ```
/// # use sorts_rs::normal::merge;
/// let mut buf = vec![];
/// let mut arr = [5, 6, 7, 1, 2, 3, 4];
/// merge(&mut arr, 3, &mut buf, i32::lt);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
///
/// let mut arr = [1, 5, 7, 2, 3, 4, 6];
/// merge(&mut arr, 3, &mut buf, i32::lt);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
///
/// let mut arr = [1, 5, 7, 2, 3, 4, 6, 8];
/// merge(&mut arr, 3, &mut buf, i32::lt);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
///
/// let mut arr = [1, 5, 7, 8, 2, 3, 4, 6];
/// merge(&mut arr, 4, &mut buf, i32::lt);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
///
/// let mut arr = [1, 3, 5, 7, 8, 2, 4, 6];
/// merge(&mut arr, 5, &mut buf, i32::lt);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn merge<T, F>(
    arr: &mut [T],
    len: usize,
    buf: &mut Vec<T>,
    mut lt: F,
)
where F: FnMut(&T, &T) -> bool,
      T: Default,
{
    buf.clear();
    let rest = arr.len() - len;
    if len <= rest {
        // forward
        buf.extend(arr[..len].iter_mut().map(take));
        let mut i = buf.iter_mut()
            .map(take)
            .peekable();
        let mut j = len;
        let mut k = 0;
        while let (Some(a), Some(b)) = (i.peek(), arr.get(j)) {
            if cmp!(lt(a,<= b)) {
                arr[i!(k++)] = i.next().unwrap();
            } else {
                arr[i!(k++)] = take(&mut arr[i!(j++)]);
            }
        }
        for ele in i {
            arr[i!(k++)] = ele;
        }
    } else {
        // backward
        buf.extend(arr[len..].iter_mut().map(take));
        let mut i = iter::from_fn(|| buf.pop())
            .peekable();
        let mut j = len;
        let mut k = arr.len();
        while i.peek().is_some() && j > 0 {
            let (a, b) = (i.peek().unwrap(), &arr[j-1]);
            if cmp!(lt(a,>= b)) {
                arr[i!(--k)] = i.next().unwrap();
            } else {
                arr[i!(--k)] = take(&mut arr[i!(--j)]);
            }
        }
        for ele in i {
            arr[i!(--j)] = ele;
        }
    }
}

fn merge_sort_with_buf_inner<T, F>(
    arr: &mut [T],
    buf: &mut Vec<T>,
    lt: &mut F,
)
where F: FnMut(&T, &T) -> bool,
      T: Default,
{
    if arr.len() < 2 { return }
    let mid = arr.len() >> 1;
    merge_sort_with_buf_inner(&mut arr[..mid], buf, lt);
    merge_sort_with_buf_inner(&mut arr[mid..], buf, lt);

    merge(arr, mid, buf, lt)
}

/// Merge Sort, like [`merge_sort`], using extern buffer
pub fn merge_sort_with_buf<T, F>(
    arr: &mut [T],
    buf: &mut Vec<T>,
    mut lt: F,
)
where F: FnMut(&T, &T) -> bool,
      T: Default,
{
    merge_sort_with_buf_inner(arr, buf, &mut lt)
}

/// Merge sort
///
/// > 归并排序, 相当均衡通用的一种排序算法, 最好最坏复杂度均为`O(n*log(n))`.
/// > 并且不像堆排序缓存不友好, 归并排序的访问基本上非常连续, 缓存较为容易命中.
/// > 这个排序还是稳定的, 也就是排序前后相等元素相对顺序不变
/// > 其唯一缺点也就是其归并算法需要开辟额外空间,
/// > 在双向归并算法的优化下这个空间最多需要`ceil(n/2)`.
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::normal::merge_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// merge_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn merge_sort<T, F>(
    arr: &mut [T],
    lt: F,
)
where F: FnMut(&T, &T) -> bool,
      T: Default,
{
    let capacity = arr.len() >> 1; // 一个期望的容量
    let mut buf = Vec::with_capacity(capacity);
    merge_sort_with_buf(arr, &mut buf, lt);
}
