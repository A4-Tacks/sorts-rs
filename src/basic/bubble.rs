use std::mem::swap;

use crate::{cmp, utils::IterBufEach};

/// Bubble sort
///
/// > 冒泡排序, 几乎是最简单的排序
/// > 最简单的实现就是两两查看, 顺序颠倒则交换
/// > 这样可以在每一轮将一个最大值送到末尾
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::basic::bubble_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// bubble_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn bubble_sort<T, F>(mut arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let mut edited = true;

    while edited {
        edited = false;

        arr.iter_mut().buf_each(|a, [b]| {
            if cmp!(lt(a,>b)) {
                swap(a, b);
                edited = true;
            }
        });
        (_, arr) = arr.split_last_mut().unwrap();
    }
}
