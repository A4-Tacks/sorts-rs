/// Radix Sort (LSD)
///
/// Generic param `N` is bits
///
/// # Example
/// ```
/// # use sorts_rs::other::radix_sort;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// radix_sort::<1>(&mut arr);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn radix_sort<T, const N: usize>(arr: &mut [T])
where T: TryInto<usize>,
{
    assert_ne!(N, 0);
    let bnum = 1 << N;
    let bmask = bnum - 1;
    //let backet = vec![vec![]; bnum];
    let high = unimplemented!();
}
