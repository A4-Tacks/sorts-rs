use crate::cmp;

fn insert_sort_by_step<T, F>(arr: &mut [T], step: usize, mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    for i in (step..arr.len()).step_by(step) {
        let _ = (0..i).step_by(step).rev()
            .try_fold(i, |i, j|
        {
            if cmp!(lt(arr[j],> arr[i])) {
                arr.swap(i, j);
                Ok(j)
            } else {
                Err(())
            }
        });
    }
}

/// Shell sort, is grouped insert sort
///
/// > 标准的 ShellSort, 最坏时间复杂度为`O(n^2)`,
/// > 但在中等规模的数据下工作还是很不错的
///
/// # Example
/// ```
/// # use sorts_rs::normal::shell_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// shell_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn shell_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let mut step = arr.len() >> 1;
    while step > 0 {
        for i in 0..step {
            insert_sort_by_step(&mut arr[i..], step, &mut lt);
        }
        step >>= 1;
    }
}

/// Sedgewick increment expr
///
/// # Example
/// ```
/// # use sorts_rs::normal::sedgewick_expr;
/// let steps = (0..10).map(sedgewick_expr).collect::<Vec<_>>();
/// assert_eq!(steps, [1,5,19,41,109,209,505,929,2161,3905]);
/// ```
pub fn sedgewick_expr(i: usize) -> usize {
    let u = (i >> 1).try_into().unwrap();
    let b4pow = |n: u32| ((1<<n)*(1<<n)) as usize;
    if i & 1 == 0 {
        9 * (b4pow(u) - (1<<u)) + 1
    } else {
        b4pow(u+2) - 3 * (1<<u+2) + 1
    }
}

/// Shell sort (use Sedgewick)
///
/// > 步长使用 Sedgewick 增量序列的 ShellSort, 平均复杂度`O(n^1.3)`
///
/// Use step expr:
/// - `i % 2 == 0`: `9 * (4^u - 2^u) + 1`
/// - `i % 2 == 1`: `4^(u+2) - 3 * 2^(u+2) + 1`
///
/// `u = i >> 1`
///
/// # Example
/// ```
/// # use sorts_rs::normal::sedgewick_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// sedgewick_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn sedgewick_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let stepi = (0..).map(sedgewick_expr)
        .position(|step| step >= arr.len())
        .and_then(|step| step.checked_sub(1))
        .unwrap_or_default();
    (0..=stepi).rev().map(sedgewick_expr).for_each(|step| {
        for i in 0..step {
            insert_sort_by_step(&mut arr[i..], step, &mut lt);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_sort_by_step_test() {
        let lt = i32::lt;
        {
            let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            insert_sort_by_step(&mut arr, 2, lt);
            assert_eq!(arr, [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]);
        }
        {
            let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            insert_sort_by_step(&mut arr[1..], 2, lt);
            assert_eq!(arr, [9, 0, 7, 2, 5, 4, 3, 6, 1, 8]);
        }
        {
            let mut arr = [];
            insert_sort_by_step(&mut arr, 2, lt);
            assert_eq!(arr, []);
        }
        {
            let mut arr = [1];
            insert_sort_by_step(&mut arr, 2, lt);
            assert_eq!(arr, [1]);
        }
    }
}
