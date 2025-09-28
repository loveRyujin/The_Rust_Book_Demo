//! # add_two
//!
//! `add_two` 是一个简单的数学工具库，提供将数字加二的功能。
//! 这个库是 workspace 示例项目的一部分，展示了如何在 Rust 工作空间中
//! 创建和使用多个相关的库 crate。
//!
//! ## 功能特性
//!
//! - 提供安全的整数加二操作
//! - 简单易用的 API
//! - 完整的测试覆盖
//! - 与 `add_one` 库配套使用
//!
//! ## 使用示例
//!
//! ```rust
//! use add_two::add_two;
//!
//! let result = add_two(5);
//! assert_eq!(result, 7);
//! ```
//!
//! ## 与其他库的关系
//!
//! 此库可以与同一工作空间中的 `add_one` 库一起使用，
//! 提供不同的数学运算选项。

/// 将给定的整数加二
///
/// 这是一个简单的数学函数，接受一个 32 位有符号整数，
/// 并返回该数字加二后的结果。
///
/// # 参数
///
/// * `x` - 要进行加二操作的 32 位有符号整数
///
/// # 返回值
///
/// 返回输入数字加二后的结果
///
/// # 示例
///
/// ```rust
/// use add_two::add_two;
///
/// let result = add_two(5);
/// assert_eq!(result, 7);
///
/// let result = add_two(-2);
/// assert_eq!(result, 0);
///
/// let result = add_two(0);
/// assert_eq!(result, 2);
/// ```
///
/// # 实现细节
///
/// 此函数内部使用 `add_one` 库的功能来实现加二操作，
/// 展示了工作空间中库之间的依赖关系。
///
/// # 注意事项
///
/// 此函数不检查整数溢出。如果输入接近 `i32::MAX`，
/// 结果可能会溢出并导致 panic（在 debug 模式下）或环绕（在 release 模式下）。
pub fn add_two(x: i32) -> i32 {
    add_one::add_one(add_one::add_one(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
