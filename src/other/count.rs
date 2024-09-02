use std::fmt::Debug;

use crate::utils::IterMinMax;

/// Counting sort
///
/// **is new value sort**
///
/// # Panics
/// - `max(arr) - base >= buf.len()`
/// - `max(arr) - base < 0`
///
/// # Examples
/// ```
/// # use sorts_rs::other::count_sort_with_buf;
/// let mut arr = [3, 1, 5, 2, 9, 6, 4];
/// let mut buf = [0; 10];
/// count_sort_with_buf(&mut arr, 0, &mut buf);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 9]);
/// ```
/// use base index, to minimal buffer
/// ```
/// # use sorts_rs::other::count_sort_with_buf;
/// let mut arr = [3, 1, 5, 2, 9, 6, 4];
/// let mut buf = [0; 9];
/// count_sort_with_buf(&mut arr, 1, &mut buf);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn count_sort_with_buf<T>(
    mut arr: &mut [T],
    base: usize,
    buf: &mut [usize],
)
where T: TryInto<usize> + TryFrom<usize> + Copy + Debug,
      <T as TryInto<usize>>::Error: Debug,
      <T as TryFrom<usize>>::Error: Debug,
{
    buf.fill(0);
    for &mut ele in &mut *arr {
        buf[ele.try_into().unwrap() - base] += 1;
    }
    for (i, count) in buf.iter().copied().enumerate() {
        let (fill, rest) = arr.split_at_mut(count);
        fill.fill((i + base).try_into().unwrap());
        arr = rest;
    }
}

/// Counting sort
///
/// **is new value sort**
///
/// Alloc buffer, and set base index
///
/// # Example
/// ```
/// # use sorts_rs::other::count_sort;
/// let mut arr = [3, 1, 5, 2, 9, 6, 4];
/// count_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn count_sort<T>(arr: &mut [T])
where T: TryInto<usize> + TryFrom<usize> + Copy + Debug,
      <T as TryInto<usize>>::Error: Debug,
      <T as TryFrom<usize>>::Error: Debug,
{
    if arr.len() < 2 { return }
    let (base, max) = arr.iter()
        .map(|&n| <T as TryInto<usize>>::try_into(n).unwrap())
        .minmax()
        .unwrap();
    let mut buf = vec![0; max - base + 1];
    count_sort_with_buf(arr, base, &mut buf)
}
