//! 其它排序, 通常是较为特殊的排序
//!
//! - 非通用排序, 如针对整数的排序,
//!   这类排序的复杂度通常不会受到`O(n*log(n))`下界的影响,
//!   如计数排序的复杂度就是`O(n)`
//!
//! - 有特殊要求的排序, 如双调排序.

mod count;
mod radix;
mod backet;
mod bitonic;

pub use count::*;
pub use radix::*;
pub use backet::*;
pub use bitonic::*;
