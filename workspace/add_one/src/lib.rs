//! # add_one
//!
//! `add_one` 是一个简单的数学工具库，提供将数字加一的功能。
//! 这个库是 workspace 示例项目的一部分，展示了如何在 Rust 工作空间中
//! 创建和使用共享的库 crate。
//!
//! ## 功能特性
//!
//! - 提供安全的整数加一操作
//! - 简单易用的 API
//! - 完整的测试覆盖
//!
//! ## 使用示例
//!
//! ```rust
//! use add_one::add_one;
//!
//! let result = add_one(5);
//! assert_eq!(result, 6);
//! ```

/// 将给定的整数加一
///
/// 这是一个简单的数学函数，接受一个 32 位有符号整数，
/// 并返回该数字加一后的结果。
///
/// # 参数
///
/// * `x` - 要进行加一操作的 32 位有符号整数
///
/// # 返回值
///
/// 返回输入数字加一后的结果
///
/// # 示例
///
/// ```rust
/// use add_one::add_one;
///
/// let result = add_one(5);
/// assert_eq!(result, 6);
///
/// let result = add_one(-1);
/// assert_eq!(result, 0);
///
/// let result = add_one(0);
/// assert_eq!(result, 1);
/// ```
///
/// # 注意事项
///
/// 此函数不检查整数溢出。如果输入是 `i32::MAX`，
/// 结果将会溢出并导致 panic（在 debug 模式下）或环绕（在 release 模式下）。
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
