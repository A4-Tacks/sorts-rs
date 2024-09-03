/// Bitonic Sort
///
/// NOTE: **unimplemented**
///
/// > 双调排序, 一种归并排序, 也是一种归并网络排序
/// > 其优点在于比较顺序和输入数据无关, 所以其比较操作是固定的
/// > 这么做的好处是可以通过简单暴力的巨量并行来换取效率,
/// > 在特殊情况下可以用GPU来进行高效排序
pub fn bitonic_sort<T, F>(arr: &mut [T], lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let _ = (arr, lt);
    unimplemented!()
}
