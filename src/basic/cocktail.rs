use std::mem::swap;

use crate::{cmp, utils::IterBufEach};

/// Bubble sort
///
/// > 鸡尾酒排序, 也称双向冒泡排序, 其流程对于冒泡排序来说多了一趟往回走
/// > 可以同时确定一个最小值和最大值
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
pub fn cocktail_sort<T, F>(mut arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let mut edited = true;

    while edited && arr.len() > 1 {
        edited = false;

        arr.iter_mut().buf_each(|a, [b]| {
            if cmp!(lt(a,>b)) {
                swap(a, b);
                edited = true;
            }
        });

        arr.iter_mut().rev().buf_each(|a, [b]| {
            if cmp!(lt(a,<b)) {
                swap(a, b);
                edited = true;
            }
        });

        let range = 1..arr.len()-1;
        arr = &mut arr[range];
    }
}
