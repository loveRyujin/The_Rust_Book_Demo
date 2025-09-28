//! # minigrep 命令行工具
//!
//! 这是 minigrep 应用程序的主入口点。
//! 它解析命令行参数，创建配置，并运行搜索操作。

use std::env;
use std::process;
use minigrep::Config;

/// 应用程序的主入口点
///
/// 解析命令行参数，创建配置实例，并执行文本搜索操作。
/// 如果发生错误，程序将打印错误信息并以非零状态码退出。
fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err_string| {
        eprintln!("Problems parsing arguments: {err_string}!");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application run error: {e}!");
        process::exit(1);
    }
}