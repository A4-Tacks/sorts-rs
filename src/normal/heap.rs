use crate::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Error {
    IndexOutOfRange(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BHeap<'a, T>(pub &'a mut [T]);
impl<'a, T> BHeap<'a, T> {
    fn left_i(&self, idx: usize) -> usize {
        (idx << 1) + 1
    }

    fn right_i(&self, idx: usize) -> usize {
        (idx << 1) + 2
    }

    fn each_node_from_floor(&self) -> impl DoubleEndedIterator<Item = usize> {
        (0..=self.len() >> 1).rev()
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.0.get(idx)
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn max_leaf<F>(&self, idx: usize, mut lt: F) -> Option<(usize, &T)>
    where F: FnMut(&T, &T) -> bool,
    {
        let (left_i, right_i) = (self.left_i(idx), self.right_i(idx));
        self.get(left_i)
            .map(|left| {
                self.get(right_i)
                    .and_then(|right| {
                        cmp!(lt(right,> left)).then_some((right_i, right))
                    })
                    .unwrap_or((left_i, left))
            })
    }

    fn filter_down<F>(
        &mut self,
        idx: usize,
        mut lt: F,
    ) -> Result<(), Error>
    where F: FnMut(&T, &T) -> bool,
    {
        let Some((maxi, max)) = self.max_leaf(idx, &mut lt) else { return Ok(()) };
        let this = self.get(idx).ok_or(Error::IndexOutOfRange(idx))?;
        if cmp!(lt(this,< max)) {
            self.swap(idx, maxi);
            self.filter_down(maxi, lt)?;
        }
        Ok(())
    }

    fn pop_to<F>(&'a mut self, i: impl IntoIterator, mut lt: F)
    where F: FnMut(&T, &T) -> bool,
    {
        for _ in i {
            if self.len() == 0 { break }
            self.swap(0, self.len() - 1);
            let (_, rest) = self.0.split_last_mut().unwrap();
            self.0 = rest;
            self.filter_down(0, &mut lt).unwrap();
        }
    }

    fn make<F>(&mut self, mut lt: F)
    where F: FnMut(&T, &T) -> bool,
    {
        if self.len() < 2 { return }
        for idx in self.each_node_from_floor() {
            self.filter_down(idx, &mut lt).unwrap();
        }
    }
}

/// Heap Sort (Max-Top Binary Heap)
///
/// Heap is in-place
///
/// > 堆排序, 通过原地构造一个大顶堆再向堆尾依次弹出来实现元素的降序排列,
/// > 虽然时间复杂度为`O(n*log(n))`, 但是其堆操作本身的缓存不友好等原因,
/// > 导致远慢于其它同复杂度的排序算法,
/// > 不过堆排序优点在于其最坏复杂度也为`O(n*log(n))`, 且无需额外空间
///
/// # Example
/// ```
/// # use sorts_rs::normal::heap_sort;
/// let lt = i32::lt;
/// let mut arr = [0, 3, 1, 5, 2, 9, 6, 4];
/// heap_sort(&mut arr, lt);
/// assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn heap_sort<T, F>(arr: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    let mut bheap = BHeap(arr);
    bheap.make(&mut lt);
    bheap.pop_to(0.., &mut lt);
}

#[test]
fn max_leaf_test() {
    let lt = i32::lt;
    {
        let mut buf = [0];
        let bheap = BHeap(&mut buf);
        assert_eq!(bheap.max_leaf(0, lt), None);
        assert_eq!(bheap.max_leaf(1, lt), None);
        assert_eq!(bheap.max_leaf(2, lt), None);
    }
    {
        let mut buf = [1, 0];
        let bheap = BHeap(&mut buf);
        assert_eq!(bheap.max_leaf(0, lt), Some((1, &0)));
        assert_eq!(bheap.max_leaf(1, lt), None);
        assert_eq!(bheap.max_leaf(2, lt), None);
    }
}
