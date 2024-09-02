use std::{
    collections::HashMap, fmt::Debug, hash::Hash, panic::{catch_unwind, AssertUnwindSafe}
};
use rand::random;

use crate::*;

const TEST_LEN: usize = 500;

fn shuffle<T>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let j = random::<usize>() % arr.len();
        arr.swap(i, j)
    }
}

fn run_sorts<T>(arr: &[T])
where T: Ord + Clone + Debug + Eq + Hash + Default,
{
    macro_rules! sorts {
        ($($path:path),+ $(,)?) => {(
            [
                $(
                    $path,
                )+
            ], [
                $(
                    stringify!($path),
                )+
            ]
        )};
    }
    let (sorts, names) = sorts![
        basic::bubble_sort,
        basic::cocktail_sort,
        basic::insert_sort,
        basic::binary_insert_sort,
        basic::select_sort,
        basic::select_doubled_sort,
        normal::comb_sort,
        normal::shell_sort,
        normal::sedgewick_sort,
        normal::quick_sort,
        normal::merge_sort,
        normal::heap_sort,
    ];
    let mut table: HashMap<Vec<T>, Vec<_>> = HashMap::new();
    sorts.into_iter()
        .enumerate()
        .map(|(i, f)|
    {
        let mut sort_arr = arr.to_vec();
        let mut_slice = AssertUnwindSafe(sort_arr.as_mut_slice());
        if let Err(e) = catch_unwind(|| {
            f({mut_slice}.0, T::lt);
        }) {
            eprintln!("{arr:?}");
            panic!("failed {} {e:?}", names[i]);
        };
        (names[i], sort_arr)
    }).for_each(|(name, arr)| {
        let names = table
            .entry(arr)
            .or_default();
        names.push(name);
    });
    if table.len() <= 1 { return }
    for (arr, names) in table.into_iter() {
        println!("{names:#?}: {arr:?}");
    }
    panic!();
}

#[test]
fn test_random_cases() {
    let mut buf = vec![];
    let mut bound_lens = [0, 1, 2, 3].into_iter();
    for _ in 0..200 {
        let len = bound_lens.next().unwrap_or_else(|| {
            rand::random::<usize>() % TEST_LEN
        });
        buf.resize(len, 0);
        for ele in &mut buf {
            *ele = random::<usize>() % TEST_LEN >> 1;
        }
        shuffle(&mut buf);
        run_sorts(&buf);
    }
}
