//! # minigrep
//!
//! `minigrep` 是一个简单的文本搜索工具，类似于 Unix 系统中的 `grep` 命令。
//! 它可以在指定的文件中搜索包含特定字符串的行，并支持大小写敏感和不敏感的搜索。
//!
//! ## 功能特性
//!
//! - 在文件中搜索指定的查询字符串
//! - 支持大小写敏感和不敏感的搜索
//! - 通过命令行参数或环境变量控制搜索行为
//! - 错误处理和用户友好的错误信息
//!
//! ## 使用方法
//!
//! ### 基本用法
//!
//! ```bash
//! cargo run -- searchstring example-filename.txt
//! ```
//!
//! ### 大小写不敏感搜索
//!
//! ```bash
//! # 通过环境变量
//! IGNORE_CASE=1 cargo run -- searchstring example-filename.txt
//!
//! # 通过命令行参数
//! cargo run -- searchstring example-filename.txt true
//! ```
//!
//! ## 示例
//!
//! ```rust
//! use minigrep::{Config, run, search, search_case_insensitive};
//!
//! // 创建配置
//! let args = vec![
//!     "minigrep".to_string(),
//!     "rust".to_string(),
//!     "example.txt".to_string(),
//! ];
//! let config = Config::build(args.into_iter()).unwrap();
//!
//! // 搜索文本
//! let content = "Rust is fast\nPython is easy\nRust is safe";
//! let results = search("Rust", content);
//! assert_eq!(results, vec!["Rust is fast", "Rust is safe"]);
//! ```

use std::fs;
use std::error::Error;
use std::env;

/// 应用程序的配置结构体
///
/// `Config` 包含了运行 minigrep 应用程序所需的所有配置信息，
/// 包括搜索查询、目标文件路径以及是否忽略大小写的标志。
///
/// # 字段
///
/// * `query` - 要搜索的字符串
/// * `file_path` - 要搜索的文件路径
/// * `ignore_case` - 是否进行大小写不敏感搜索
///
/// # 示例
///
/// ```rust
/// use minigrep::Config;
///
/// let args = vec![
///     "minigrep".to_string(),
///     "rust".to_string(),
///     "example.txt".to_string(),
/// ];
/// let config = Config::build(args.into_iter()).unwrap();
/// assert_eq!(config.query, "rust");
/// assert_eq!(config.file_path, "example.txt");
/// ```
pub struct Config {
    /// 要搜索的查询字符串
    pub query: String,
    /// 要搜索的文件路径
    pub file_path: String,
    /// 是否忽略大小写进行搜索
    pub ignore_case: bool,
}

impl Config {
    /// 从命令行参数构建 Config 实例
    ///
    /// 此方法解析命令行参数并创建一个 `Config` 实例。它期望至少有两个参数：
    /// 查询字符串和文件路径。第三个可选参数用于控制大小写敏感性。
    ///
    /// # 参数
    ///
    /// * `args` - 命令行参数的迭代器，通常来自 `std::env::args()`
    ///
    /// # 返回值
    ///
    /// * `Ok(Config)` - 成功解析参数时返回配置实例
    /// * `Err(String)` - 参数不足或解析失败时返回错误信息
    ///
    /// # 参数格式
    ///
    /// 1. 程序名称（自动忽略）
    /// 2. 查询字符串（必需）
    /// 3. 文件路径（必需）
    /// 4. 大小写标志（可选，任何可解析为 bool 的值）
    ///
    /// 如果没有提供大小写标志，将检查 `IGNORE_CASE` 环境变量。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use minigrep::Config;
    ///
    /// let args = vec![
    ///     "minigrep".to_string(),
    ///     "rust".to_string(),
    ///     "example.txt".to_string(),
    ///     "true".to_string(),
    /// ];
    /// let config = Config::build(args.into_iter()).unwrap();
    /// assert_eq!(config.query, "rust");
    /// assert_eq!(config.file_path, "example.txt");
    /// assert_eq!(config.ignore_case, true);
    /// ```
    ///
    /// # 错误
    ///
    /// 如果提供的参数少于 3 个（包括程序名），此方法将返回错误。
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        // ignore first value which is executable file
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err(String::from("Didn't get a query string")),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err(String::from("Didn't get a file path string")),
        };

        let ignore_case: bool;
        if let Some(flag) = args.next() {
            ignore_case = flag.parse::<bool>().is_ok();
        } else {
            ignore_case = env::var("IGNORE_CASE").is_ok();
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// 运行 minigrep 应用程序的主要逻辑
///
/// 此函数接受一个 `Config` 实例，读取指定的文件，执行搜索操作，
/// 并将匹配的行打印到标准输出。
///
/// # 参数
///
/// * `config` - 包含搜索配置的 `Config` 实例
///
/// # 返回值
///
/// * `Ok(())` - 成功执行搜索操作
/// * `Err(Box<dyn Error>)` - 文件读取失败或其他 I/O 错误
///
/// # 行为
///
/// 1. 读取 `config.file_path` 指定的文件内容
/// 2. 根据 `config.ignore_case` 选择合适的搜索函数
/// 3. 将所有匹配的行打印到标准输出
///
/// # 示例
///
/// ```rust,no_run
/// use minigrep::{Config, run};
///
/// let args = vec![
///     "minigrep".to_string(),
///     "rust".to_string(),
///     "example.txt".to_string(),
/// ];
/// let config = Config::build(args.into_iter()).unwrap();
/// 
/// match run(config) {
///     Ok(()) => println!("搜索完成"),
///     Err(e) => eprintln!("运行错误: {}", e),
/// }
/// ```
///
/// # 错误
///
/// 如果无法读取指定的文件，此函数将返回相应的 I/O 错误。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let match_lines = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    for line in match_lines {
        println!("{line}");
    }

    Ok(())
}

/// 在文本内容中搜索包含查询字符串的行（大小写敏感）
///
/// 此函数在给定的文本内容中搜索包含指定查询字符串的所有行，
/// 并返回这些行的引用向量。搜索是大小写敏感的。
///
/// # 参数
///
/// * `query` - 要搜索的字符串
/// * `content` - 要搜索的文本内容
///
/// # 返回值
///
/// 返回一个包含所有匹配行引用的向量。如果没有找到匹配项，返回空向量。
///
/// # 生命周期
///
/// 返回的字符串切片的生命周期与输入的 `content` 参数相同。
///
/// # 示例
///
/// ```rust
/// use minigrep::search;
///
/// let content = "Rust:\nsafe, fast, productive.\nPick three.\nrust is great";
/// let results = search("Rust", content);
/// assert_eq!(results, vec!["Rust:"]);
///
/// let results = search("rust", content);
/// assert_eq!(results, vec!["rust is great"]);
/// ```
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 在文本内容中搜索包含查询字符串的行（大小写不敏感）
///
/// 此函数在给定的文本内容中搜索包含指定查询字符串的所有行，
/// 并返回这些行的引用向量。搜索是大小写不敏感的。
///
/// # 参数
///
/// * `query` - 要搜索的字符串
/// * `content` - 要搜索的文本内容
///
/// # 返回值
///
/// 返回一个包含所有匹配行引用的向量。如果没有找到匹配项，返回空向量。
///
/// # 生命周期
///
/// 返回的字符串切片的生命周期与输入的 `content` 参数相同。
///
/// # 实现细节
///
/// 此函数将查询字符串和每一行都转换为小写进行比较，
/// 但返回的是原始行的引用，保持原有的大小写格式。
///
/// # 示例
///
/// ```rust
/// use minigrep::search_case_insensitive;
///
/// let content = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
/// let results = search_case_insensitive("rUsT", content);
/// assert_eq!(results, vec!["Rust:", "Trust me."]);
///
/// let results = search_case_insensitive("TRUST", content);
/// assert_eq!(results, vec!["Trust me."]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}