//! 普通的排序方法, 能够胜任于多数排序需求
//!
//! 可以应用在有一定规模的数据上

mod quick;
mod merge;
mod heap;
mod shell;
mod comb;

pub use quick::*;
pub use merge::*;
pub use heap::*;
pub use shell::*;
pub use comb::*;
