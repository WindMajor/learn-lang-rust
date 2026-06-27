// ================================================================
// searcher 模块：核心搜索逻辑
// ================================================================
//
// WHAT: 搜索器的主要入口和公共类型
// WHY: 模块化组织——walker 负责遍历，filter 负责匹配

pub mod filter;
pub mod walker;

pub use filter::{ContentFilter, FileNameFilter, SearchFilter};
pub use walker::{search, SearchResult};
