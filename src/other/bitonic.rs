use crate::cmp;

fn assert_bin_exp(n: usize) {
    if n != 0 {
        assert!(n.is_power_of_two(), "expect {n} is 2^n or 0 (n: uint)");
    }
}

fn bitonic_merge_unchecked_inner<T, F>(arr: &mut [T], lt: &mut F)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let mid = arr.len() >> 1;

    for i in 0..mid {
        if cmp!(lt(arr[i],> arr[i+mid])) {
            arr.swap(i, i+mid)
        }
    }

    bitonic_merge_unchecked_inner(&mut arr[..mid], lt);
    bitonic_merge_unchecked_inner(&mut arr[mid..], lt);
}

fn bitonic_sort_unchecked_inner<T, F>(
    arr: &mut [T],
    lt: &mut F,
    rev: bool,
)
where F: FnMut(&T, &T) -> bool,
{
    if arr.len() < 2 { return }
    let mid = arr.len() >> 1;

    bitonic_sort_unchecked_inner(&mut arr[..mid], &mut *lt, true);
    bitonic_sort_unchecked_inner(&mut arr[mid..], &mut *lt, false);

    if rev {
        bitonic_merge_unchecked(arr, cmp!(lt(>=)))
    } else {
        bitonic_merge_unchecked(arr, lt)
    }
}


/// Bitonic Merge
///
/// > 双调归并, 类似 [`bitonic_merge`], 但是不检查数组长度是否有效
fn bitonic_merge_unchecked<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    bitonic_merge_unchecked_inner(arr, &mut lt)
}

/// Bitonic Merge
///
/// > 双调归并, 假定输入是一个双调序列, 那么将会将其变为升序单调序列
///
/// # Panics
/// `arr.len() != 0 && !arr.len().is_power_of_two()`
pub fn bitonic_merge<T, F>(arr: &mut [T], lt: F)
where F: FnMut(&T, &T) -> bool,
{
    assert_bin_exp(arr.len());
    bitonic_merge_unchecked(arr, lt)
}

/// Bitonic Merge
///
/// > 双调归并, 类似 [`bitonic_merge`], 但是不检查数组长度是否有效
fn bitonic_sort_unchecked<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    bitonic_sort_unchecked_inner(arr, &mut lt, false)
}

/// Bitonic Sort
///
/// > 双调排序, 一种归并排序, 也是一种归并排序网络
/// > 其优点在于比较顺序和输入数据无关, 所以其比较操作是固定的
/// > 这么做的好处是可以通过简单暴力的巨量并行来换取效率,
/// > 在特殊情况下可以用GPU来进行高效排序
/// >
/// > 目前实现使用单线程实现
///
/// # Panics
/// `arr.len() != 0 && !arr.len().is_power_of_two()`
pub fn bitonic_sort<T, F>(arr: &mut [T], lt: F)
where F: FnMut(&T, &T) -> bool,
{
    assert_bin_exp(arr.len());
    bitonic_sort_unchecked(arr, lt)
}

#[test]
fn assert_2pow_test() {
    let nums = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 256,
    ];
    for num in nums {
        assert_bin_exp(num)
    }

    let fails = [
        3, 5, 6, 7, 9, 11, 12, 13, 14, 15, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36,
    ];
    for num in fails {
        let res = std::panic::catch_unwind(|| {
            assert_bin_exp(num)
        });
        assert!(res.is_err(), "{res:?}");
    }
}
