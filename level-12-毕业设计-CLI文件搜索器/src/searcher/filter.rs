// ================================================================
// filter 模块：文件过滤与内容匹配
// ================================================================
//
// WHAT: 定义 SearchFilter trait 和具体实现（文件名匹配、内容匹配）
// WHY:
//   - 通过 trait 抽象"过滤"概念，支持组合和扩展
//   - 使用 dyn trait 动态分发（运行时选择不同的 filter）
//
// CONTRAST:
//   - Go:     用 interface 定义 Filter（隐式实现）
//   - TS:     用 interface 定义 Filter（结构匹配）
//   - C++:    用纯虚类定义 Filter（虚函数表）
//   - Java:   用 interface 定义 Filter（类似）
//   关键差异: Rust 的 trait 可以有默认实现，实现与定义分离

use regex::Regex;
use std::path::Path;

// ─── 自定义 Trait ───
/// 搜索过滤器特征
///
/// 任何实现了 matches 方法的类型都可以作为搜索过滤器。
/// 这是 Rust trait 系统的典型应用——定义行为契约。
///
/// CONTRAST:
///   - TS:   interface SearchFilter { matches(path: string): boolean }
///   - Go:   type SearchFilter interface { Matches(path string) bool }
///   - Java: interface SearchFilter { boolean matches(String path); }
pub trait SearchFilter: Send + Sync {
    /// 检查文件是否匹配过滤条件
    ///
    /// # 参数
    /// * `path` - 文件的完整路径
    /// * `content` - 文件内容（如果已读取，可选优化）
    ///
    /// # 返回
    /// `true` 如果文件匹配过滤条件
    fn matches(&self, path: &Path, content: Option<&str>) -> bool;

    /// 过滤器名称（用于调试和统计）
    fn name(&self) -> &str;

    /// 是否需要读取文件内容（优化：如果不需要，跳过文件读取）
    /// 默认不需要
    fn needs_content(&self) -> bool {
        false
    }
}

// ─── 文件名过滤器 ───
/// 基于正则表达式的文件名匹配器
///
/// WHAT: 使用编译好的正则表达式匹配文件名
/// WHY:  regex::Regex 编译时生成 DFA，匹配极快
///
/// CONTRAST:
///   - Go:      regexp.MustCompile(pattern)
///   - TS:      new RegExp(pattern)
///   - C++:     std::regex（性能较慢，推荐 RE2/boost::regex）
///   Rust 的 regex crate 使用基于 DFA 的引擎，编译期优化
pub struct FileNameFilter {
    /// 编译好的正则表达式
    pattern: Regex,
    /// 过滤器名称
    name: String,
}

impl FileNameFilter {
    /// 创建文件名过滤器
    ///
    /// # 参数
    /// * `pattern` - 正则表达式模式（如 `\.rs$` 匹配所有 .rs 文件）
    ///
    /// # 错误
    /// 如果正则表达式语法错误，返回 SearchError
    pub fn new(pattern: &str) -> Result<Self, crate::SearchError> {
        let re = Regex::new(pattern)?;
        Ok(FileNameFilter {
            pattern: re,
            name: format!("文件名匹配: /{}/", pattern),
        })
    }
}

impl SearchFilter for FileNameFilter {
    fn matches(&self, path: &Path, _content: Option<&str>) -> bool {
        // 获取文件名（如果获取不到，默认不匹配）
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| self.pattern.is_match(name))
            .unwrap_or(false)
    }

    fn name(&self) -> &str {
        &self.name
    }

    // 文件名过滤不需要读取文件内容
    fn needs_content(&self) -> bool {
        false
    }
}

// ─── 内容过滤器 ───
/// 基于文件内容的搜索器
///
/// WHAT: 在文件内容中搜索指定模式（类似 grep）
/// WHY:  需要在匹配文件名的文件中进一步搜索内容时才使用
pub struct ContentFilter {
    /// 内容搜索的正则表达式
    pattern: Regex,
    /// 过滤器名称
    name: String,
}

impl ContentFilter {
    /// 创建内容过滤器
    pub fn new(pattern: &str) -> Result<Self, crate::SearchError> {
        let re = Regex::new(pattern)?;
        Ok(ContentFilter {
            pattern: re,
            name: format!("内容匹配: /{}/", pattern),
        })
    }
}

impl SearchFilter for ContentFilter {
    fn matches(&self, _path: &Path, content: Option<&str>) -> bool {
        match content {
            Some(text) => self.pattern.is_match(text),
            None => false, // 没有内容 → 不匹配
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    // 内容过滤需要读取文件内容
    fn needs_content(&self) -> bool {
        true
    }
}

// ─── 组合过滤器 ───
/// 过滤器链：组合多个过滤器（逻辑与）
///
/// WHAT: 类似于责任链模式，所有 filter 都必须返回 true
/// WHY:  可以组合文件名过滤 + 内容过滤 + 更多约束
pub struct FilterChain {
    filters: Vec<Box<dyn SearchFilter>>,
}

impl FilterChain {
    /// 创建空的过滤器链
    pub fn new() -> Self {
        FilterChain {
            filters: Vec::new(),
        }
    }

    /// 添加一个过滤器
    pub fn add(&mut self, filter: Box<dyn SearchFilter>) {
        self.filters.push(filter);
    }

    /// 链中是否有过滤器需要读取内容
    pub fn needs_content(&self) -> bool {
        self.filters.iter().any(|f| f.needs_content())
    }

    /// 链是否为空
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// 执行所有过滤器的检查
    pub fn matches(&self, path: &Path, content: Option<&str>) -> bool {
        self.filters.iter().all(|f| f.matches(path, content))
    }
}

impl Default for FilterChain {
    fn default() -> Self {
        Self::new()
    }
}

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_name_filter() {
        let filter = FileNameFilter::new(r"\.rs$").unwrap();
        assert!(filter.matches(&PathBuf::from("main.rs"), None));
        assert!(!filter.matches(&PathBuf::from("main.go"), None));
        assert!(!filter.matches(&PathBuf::from("README"), None));
    }

    #[test]
    fn test_file_name_filter_nested_path() {
        let filter = FileNameFilter::new(r"\.rs$").unwrap();
        assert!(filter.matches(&PathBuf::from("src/searcher/walker.rs"), None));
        assert!(!filter.matches(&PathBuf::from("src/main.go"), None));
    }

    #[test]
    fn test_content_filter() {
        let filter = ContentFilter::new(r"fn main").unwrap();
        assert!(filter.matches(&PathBuf::from("main.rs"), Some("fn main() {\n    println!();\n}")));
        assert!(!filter.matches(&PathBuf::from("lib.rs"), Some("pub fn helper() {}")));
    }

    #[test]
    fn test_filter_chain() {
        let mut chain = FilterChain::new();
        chain.add(Box::new(FileNameFilter::new(r"\.rs$").unwrap()));
        chain.add(Box::new(ContentFilter::new(r"TODO").unwrap()));

        let path = PathBuf::from("lib.rs");

        // 文件名匹配 + 内容匹配
        assert!(chain.matches(&path, Some("// TODO: implement")));
        // 文件名匹配 + 内容不匹配
        assert!(!chain.matches(&path, Some("fn ready() {}")));
        // 文件名不匹配
        assert!(!chain.matches(&PathBuf::from("main.go"), Some("// TODO")));
    }

    #[test]
    fn test_needs_content() {
        let mut chain = FilterChain::new();
        chain.add(Box::new(FileNameFilter::new(r"\.rs$").unwrap()));
        assert!(!chain.needs_content());

        chain.add(Box::new(ContentFilter::new(r"TODO").unwrap()));
        assert!(chain.needs_content());
    }
}
