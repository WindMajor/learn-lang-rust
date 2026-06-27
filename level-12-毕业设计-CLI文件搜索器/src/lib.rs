// ================================================================
// rsearch: 高性能 CLI 文件搜索器 —— 毕业设计
// ================================================================
//
// WHAT: 一个多线程递归文件搜索器，支持文件名正则匹配和内容搜索
// WHY: 串联前面 11 关的所有知识点，构建完整 Rust 项目
//
// 架构:
//   main.rs → Cli (clap) → Searcher → walker (遍历) + filter (过滤) → Result
//
// 使用的 Rust 核心概念:
//   - 所有权: 函数间传递 String/Vec 所有权
//   - 借用: 大量 &str, &Path 引用参数
//   - Trait: SearchFilter trait 定义过滤接口
//   - 泛型: FilterChain 组合多个 filter
//   - Result/? 错误处理
//   - 并发: rayon 多线程 + Arc 共享
//   - 迭代器: walkdir + 组合子链式处理
//   - 生命周期: 引用在结构体中的标注

pub mod error;
pub mod searcher;

// 重新导出常用类型，方便 main.rs 使用
pub use error::SearchError;
pub use searcher::walker::SearchResult;
