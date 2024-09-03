use std::mem::swap;

use crate::cmp;

fn bubble_sort_by_step<T, F>(mut arr: &mut [T], step: usize, mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    while arr.len() > step {
        arr.iter_mut()
            .step_by(step)
            .reduce(|a, b|
        {
            if cmp!(lt(a,>b)) {
                swap(a, b);
            }
            b
        });
        (arr, _) = arr.split_at_mut(arr.len() - step);
    }
}

/// Comb sort, is grouped bubble sort
///
/// > 来自冒泡排序的分组优化版本, 类似 Shell Sort,
/// > 复杂度似乎也是`O(n*log(n))`, 但是快不到哪去就是了
///
/// **is stable sort**
/// # Example
/// ```
/// # use sorts_rs::normal::comb_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// comb_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn comb_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let mut step = arr.len() >> 1;

    while step > 0 {
        for i in 0..step {
            bubble_sort_by_step(&mut arr[i..], step, &mut lt)
        }
        step = (step as f64 * 0.801711847137793) as usize
    }
}
