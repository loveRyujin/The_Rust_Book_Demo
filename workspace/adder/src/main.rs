//! # adder 演示程序
//!
//! 这是一个演示程序，展示了如何在 Rust 工作空间中使用多个库 crate。
//! 程序使用 `add_one` 和 `add_two` 库来执行简单的数学运算，
//! 并将结果打印到控制台。
//!
//! ## 功能
//!
//! - 演示工作空间中库之间的依赖关系
//! - 展示如何导入和使用外部 crate
//! - 提供简单的数学运算示例
//!
//! ## 运行方法
//!
//! ```bash
//! cargo run -p adder
//! ```

use add_one;
use add_two;

/// 程序的主入口点
///
/// 此函数演示了如何使用工作空间中的 `add_one` 和 `add_two` 库。
/// 它对数字 10 执行加一和加二操作，并将结果打印到控制台。
///
/// # 输出示例
///
/// ```text
/// the result of add_one is 11
/// the result of add_two is 12
/// ```
///
/// # 工作空间依赖
///
/// 此程序依赖于：
/// - `add_one` - 提供加一功能
/// - `add_two` - 提供加二功能
fn main() {
    let num = 10;
    println!("the result of add_one is {}", add_one::add_one(num));
    println!("the result of add_two is {}", add_two::add_two(num));
}
