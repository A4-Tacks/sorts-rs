//! 基础的排序方法, 通常它们被设计的足够简单
//!
//! 但是因其复杂度为`O(n^2)`, 难以胜任与较大数据量的情况
//!
//! 不过由于其常数项够低在较小区间的性能也是非常不错的

mod bubble;
mod cocktail;
mod select;
mod double_select;
mod binary_insert;
mod insert;

pub use bubble::*;
pub use cocktail::*;
pub use select::*;
pub use double_select::*;
pub use binary_insert::*;
pub use insert::*;
