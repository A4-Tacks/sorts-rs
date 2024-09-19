use std::{fmt::Debug, mem};

fn bit_high(n: usize) -> u32 {
    match n {
        0 => 0,
        n => n.ilog2(),
    }
}

/// Radix Sort (LSD) binary
///
/// # Example
/// ```
/// # use sorts_rs::other::radix_sort;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// radix_sort(&mut arr);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn radix_sort<T>(arr: &mut [T])
where T: TryInto<usize> + Default + Copy,
      T::Error: Debug,
{
    radix_sort_with_buf(arr, &mut Vec::with_capacity(arr.len()>>1))
}

/// Radix Sort (LSD) binary
///
/// # Example
/// ```
/// # use sorts_rs::other::radix_sort_with_buf;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// radix_sort_with_buf(&mut arr, &mut Vec::with_capacity(8));
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn radix_sort_with_buf<T>(arr: &mut [T], buf: &mut Vec<T>)
where T: TryInto<usize> + Default + Copy,
      T::Error: Debug,
{
    if arr.len() < 2 { return }
    buf.clear();
    buf.reserve(arr.len() - arr.len() >> 2);

    let radix = arr.iter()
        .map(|&n| bit_high(n.try_into().unwrap()))
        .max()
        .unwrap_or_default();

    for d in 0..radix+1 {
        let mut j = 0;
        for i in 0..arr.len() {
            let cur = &mut arr[i];
            if (*cur).try_into().unwrap() >> d & 1 == 1 {
                buf.push(mem::take(cur))
            } else {
                arr[j] = mem::take(cur);
                j += 1;
            }
        }

        debug_assert_eq!(buf.len(), arr.len() - j);

        buf.drain(..).zip(&mut arr[j..])
            .for_each(|(ele, cur)| *cur = ele);
    }
}
