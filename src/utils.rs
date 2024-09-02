use std::cmp::Ordering;

pub use crate::{cmp, i};

/// use `lessThan` function compare
///
/// # Example
/// ```
/// # use sorts_rs::cmp;
/// let lt = i32::lt;
///
/// assert_eq!(cmp!(lt<)(&1, &2), true);
///
/// assert_eq!(cmp!(lt(1,< 2)), true);
/// assert_eq!(cmp!(lt(1,< 1)), false);
/// assert_eq!(cmp!(lt(1,< 0)), false);
///
/// assert_eq!(cmp!(lt(1,<= 2)), true);
/// assert_eq!(cmp!(lt(1,<= 1)), true);
/// assert_eq!(cmp!(lt(1,<= 0)), false);
///
/// assert_eq!(cmp!(lt(2,> 1)), true);
/// assert_eq!(cmp!(lt(1,> 1)), false);
/// assert_eq!(cmp!(lt(0,> 1)), false);
///
/// assert_eq!(cmp!(lt(2,>= 1)), true);
/// assert_eq!(cmp!(lt(1,>= 1)), true);
/// assert_eq!(cmp!(lt(0,>= 1)), false);
/// ```
#[macro_export]
macro_rules! cmp {
    ($lt:ident<) => {
        |a, b| $lt(a, b)
    };
    ($lt:ident>) => {
        |a, b| $lt(b, a)
    };
    ($lt:ident<=) => {
        |a, b| !$lt(b, a)
    };
    ($lt:ident>=) => {
        |a, b| !$lt(a, b)
    };

    ($lt:ident($a:expr ,< $b:expr)) => {
        $lt(&$a, &$b)
    };
    ($lt:ident($a:expr ,> $b:expr)) => {
        $lt(&$b, &$a)
    };
    ($lt:ident($a:expr ,<= $b:expr)) => {
        !$lt(&$b, &$a)
    };
    ($lt:ident($a:expr ,>= $b:expr)) => {
        !$lt(&$a, &$b)
    };
}

/// Quick I op, like `i++` `++i`
///
/// # Examples
/// ```
/// # use sorts_rs::i;
/// let mut a = 0;
/// let mut b = 0;
/// assert_eq!((i!(++a), i!(b++)), (1, 0));
/// assert_eq!((a, b), (1, 1));
/// ```
#[macro_export]
macro_rules! i {
    (++$i:ident) => {{ $i += 1; $i }};
    (--$i:ident) => {{ $i -= 1; $i }};
    ($i:ident++) => {{ let __tmp = $i; $i += 1; __tmp }};
    ($i:ident--) => {{ let __tmp = $i; $i -= 1; __tmp }};
}

pub trait IterMinMax: Iterator<Item: Clone> + Sized {
    fn minmax(self) -> Option<(Self::Item, Self::Item)>
    where Self::Item: Ord,
    {
        self.minmax_by(<Self::Item as Ord>::cmp)
    }

    fn minmax_by_key<K, F>(self, mut f: F) -> Option<(Self::Item, Self::Item)>
    where K: Ord,
          F: FnMut(&Self::Item) -> K,
    {
        self.minmax_by(|a, b| {
            f(a).cmp(&f(b))
        })
    }

    fn minmax_by<F>(mut self, mut cmp: F) -> Option<(Self::Item, Self::Item)>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let ele = self.next()?;
        self.fold((ele.clone(), ele), |(min, max), ele| {
            (
                if cmp(&ele, &min).is_lt() { ele.clone() } else { min },
                if cmp(&ele, &max).is_gt() { ele } else { max },
            )
        }).into()
    }
}
impl<T: Iterator<Item: Clone> + Sized> IterMinMax for T { }
