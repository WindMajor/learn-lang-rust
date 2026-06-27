// ================================================================
// walker 模块：目录遍历 + 多线程搜索
// ================================================================
//
// WHAT: 使用 walkdir 递归遍历目录，rayon 多线程并行处理
// WHY:  充分利用多核 CPU，大幅提升大目录的搜索速度
//
// 核心设计:
//   1. walkdir 收集所有文件路径（单线程，快速）
//   2. rayon par_iter 并行对每个文件执行过滤检查（多线程）
//   3. Arc 共享 FilterChain 配置（避免每个线程复制过滤器）
//
// CONTRAST:
//   - Go:     用 goroutine + channel 实现（你手动管理 pool）
//   - C++:    用 std::thread + 手动 work queue
//   - TS:     Worker Threads 或 async/await
//   - Java:   ExecutorService + ForkJoinPool
//   Rust rayon 优势: par_iter 自动分割工作，零样板代码

use crate::error::SearchError;
use crate::searcher::filter::FilterChain;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// 单个搜索结果
///
/// WHAT: 包含文件路径、匹配的行、文件大小等元信息
/// CONTRAST:
///   - TS:   type SearchResult = { path: string, matches: string[] }
///   - Go:   type SearchResult struct { ... }
///   - C++:  struct SearchResult { ... }
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 文件的绝对路径
    pub path: PathBuf,
    /// 文件大小（字节）
    pub size: u64,
    /// 如果内容匹配，包含匹配的行（最多显示 3 行）
    pub matched_lines: Vec<String>,
}

impl SearchResult {
    /// 创建搜索结果
    pub fn new(path: PathBuf, size: u64) -> Self {
        SearchResult {
            path,
            size,
            matched_lines: Vec::new(),
        }
    }

    /// 添加匹配行
    pub fn add_match(&mut self, line: String) {
        if self.matched_lines.len() < 3 {
            self.matched_lines.push(line);
        }
    }

    /// 是否有内容匹配
    pub fn has_content_match(&self) -> bool {
        !self.matched_lines.is_empty()
    }
}

/// 搜索器配置
///
/// WHAT: 封装搜索的所有配置参数
/// WHY:  通过结构体传递配置，明确且可扩展
pub struct SearchConfig {
    /// 搜索的根目录
    pub root_dir: PathBuf,
    /// 最大搜索深度（None = 无限）
    pub max_depth: Option<usize>,
    /// 是否跨越文件系统边界
    pub follow_links: bool,
    /// 正则表达式模式（文件名匹配）
    pub name_pattern: Option<String>,
    /// 内容搜索模式
    pub content_pattern: Option<String>,
    /// 最大结果数（None = 无限）
    pub max_results: Option<usize>,
}

/// 搜索统计信息
#[derive(Debug)]
pub struct SearchStats {
    /// 遍历的文件总数
    pub files_scanned: usize,
    /// 匹配的文件数
    pub files_matched: usize,
    /// 搜索耗时（毫秒）
    pub duration_ms: u64,
}

/// 执行搜索的主函数
///
/// WHAT: 这是搜索器的入口——收集文件 → 构建过滤器 → 并行匹配
/// WHY: 将遍历和匹配分离，匹配阶段可以并行化
///
/// # 参数
/// * `config` - 搜索配置
///
/// # 返回
/// 匹配的文件列表和搜索统计信息
///
/// # Panics
/// 如果正则表达式编译失败（由 FilterChain 的 new 方法传播）
pub fn search(config: &SearchConfig) -> Result<(Vec<SearchResult>, SearchStats), SearchError> {
    let start = Instant::now();

    // ─── 1. 验证目录存在 ───
    if !config.root_dir.exists() {
        return Err(SearchError::dir_not_found(
            &config.root_dir.display().to_string(),
        ));
    }

    // ─── 2. 构建过滤器链 ───
    let filter_chain = build_filter_chain(config)?;

    // ─── 3. 收集所有文件 ───
    let file_paths = collect_files(config)?;
    let files_scanned = file_paths.len();

    // ─── 4. 并行匹配（核心性能点） ───
    //
    // WHAT: 使用 rayon 的 par_iter 将文件列表并行处理
    // WHY: 每个文件独立检查，天然可并行化
    //      不需要手动创建线程池、分配任务、收集结果
    //
    // CONTRAST:
    //   - Go:    var wg sync.WaitGroup; results := make(chan Result)
    //           for _, path := range paths { wg.Add(1); go func() { ... }() }
    //   - C++:   手动 thread pool + queue
    //   - TS:    Promise.all(paths.map(p => process(p)))
    //   Rust rayon: 一行 par_iter() 替代 20 行的线程池代码
    use rayon::prelude::*;

    let filter_ref = &filter_chain;
    let needs_content = filter_ref.needs_content();

    // Arc 共享 filter（虽然这里不需要跨线程共享所有权，
    // 但展示了 Arc 在多线程场景下的典型用法）
    // 实际上由于 filter_chain 生命周期足够，不需要 Arc——这里保留用于教学
    let mut results: Vec<SearchResult> = file_paths
        .par_iter()
        .filter_map(|path| {
            // ─── 尝试读取文件内容（如果需要） ───
            let content = if needs_content {
                read_file_content(path).ok()
            } else {
                None
            };

            // ─── 执行过滤 ───
            if filter_ref.matches(path, content.as_deref()) {
                // ─── 创建搜索结果 ───
                let metadata = fs::metadata(path).ok()?;
                let size = metadata.len();
                let mut result = SearchResult::new(path.clone(), size);

                // ─── 如果有内容匹配，提取匹配行 ───
                if let Some(ref text) = content {
                    // 提取包含匹配内容的行
                    for line in text.lines() {
                        // 简单检测：如果行中包含搜索关键词
                        if let Some(ref content_pattern) = config.content_pattern {
                            if line.contains(content_pattern.as_str()) {
                                result.add_match(line.trim().to_string());
                            }
                        }
                    }
                }

                Some(result)
            } else {
                None
            }
        })
        .collect();

    // ─── 5. 限制结果数量 ───
    if let Some(max) = config.max_results {
        results.truncate(max);
    }

    let files_matched = results.len();
    let duration_ms = start.elapsed().as_millis() as u64;

    let stats = SearchStats {
        files_scanned,
        files_matched,
        duration_ms,
    };

    Ok((results, stats))
}

/// 构建过滤器链
fn build_filter_chain(config: &SearchConfig) -> Result<FilterChain, SearchError> {
    use crate::searcher::filter::{ContentFilter, FileNameFilter};

    let mut chain = FilterChain::new();

    // 添加文件名过滤器
    if let Some(ref pattern) = config.name_pattern {
        chain.add(Box::new(FileNameFilter::new(pattern)?));
    }

    // 添加内容过滤器
    if let Some(ref pattern) = config.content_pattern {
        chain.add(Box::new(ContentFilter::new(pattern)?));
    }

    Ok(chain)
}

/// 收集目录下所有文件路径
///
/// WHAT: 使用 walkdir 递归遍历目录
/// WHY: walkdir 自动处理符号链接、权限错误、目录跳过
fn collect_files(config: &SearchConfig) -> Result<Vec<PathBuf>, SearchError> {
    // 构建 walker（根据是否有 max_depth 决定配置）
    let mut walkdir_builder = WalkDir::new(&config.root_dir)
        .follow_links(config.follow_links);

    if let Some(depth) = config.max_depth {
        walkdir_builder = walkdir_builder.max_depth(depth);
    }

    let walker = walkdir_builder.into_iter();

    let mut paths = Vec::new();

    for entry in walker {
        let entry = entry.map_err(|e| {
            SearchError::IoError {
                path: e.path().unwrap_or(Path::new("unknown")).display().to_string(),
                source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
            }
        })?;

        // 只收集文件，跳过目录
        if entry.file_type().is_file() {
            paths.push(entry.path().to_path_buf());
        }
    }

    Ok(paths)
}

/// 读取文件内容
///
/// WHAT: 读取整个文件为 String
/// WHY: UTF-8 文本文件的内容搜索需要
///
/// WARNING: 对于二进制文件，read_to_string 可能失败（返回错误），
///          这是预期行为——我们只搜索文本文件
fn read_file_content(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    /// 创建临时测试目录结构
    fn setup_test_dir() -> (tempfile::TempDir, Vec<PathBuf>) {
        let dir = tempfile::tempdir().unwrap();
        let mut files = Vec::new();

        // 创建测试文件
        let test_files = [
            ("main.rs", "fn main() {\n    println!(\"hello\");\n}"),
            ("lib.rs", "pub fn add(a: i32, b: i32) -> i32 { a + b }"),
            ("README.md", "# Test Project\n\nHello world"),
            ("data.txt", "some data"),
        ];

        for (name, content) in &test_files {
            let path = dir.path().join(name);
            let mut file = fs::File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            files.push(path);
        }

        (dir, files)
    }

    #[test]
    fn test_search_by_name() {
        let (dir, _files) = setup_test_dir();
        let config = SearchConfig {
            root_dir: dir.path().to_path_buf(),
            max_depth: Some(1),
            follow_links: false,
            name_pattern: Some(r"\.rs$".to_string()),
            content_pattern: None,
            max_results: None,
        };

        let (results, stats) = search(&config).unwrap();
        assert_eq!(results.len(), 2); // main.rs + lib.rs
        assert_eq!(stats.files_scanned, 4);
        assert_eq!(stats.files_matched, 2);
        assert!(stats.duration_ms < 1000);
    }

    #[test]
    fn test_search_by_content() {
        let (dir, _files) = setup_test_dir();
        let config = SearchConfig {
            root_dir: dir.path().to_path_buf(),
            max_depth: Some(1),
            follow_links: false,
            name_pattern: None,
            content_pattern: Some("Hello".to_string()),
            max_results: None,
        };

        let (results, _stats) = search(&config).unwrap();
        // README.md 包含 "Hello world"
        assert!(!results.is_empty());
        let readme = results.iter().find(|r| r.path.ends_with("README.md"));
        assert!(readme.is_some());
    }

    #[test]
    fn test_search_nonexistent_dir() {
        let config = SearchConfig {
            root_dir: PathBuf::from("/nonexistent/path"),
            max_depth: None,
            follow_links: false,
            name_pattern: None,
            content_pattern: None,
            max_results: None,
        };

        let result = search(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_name_and_content() {
        let (dir, _files) = setup_test_dir();
        let config = SearchConfig {
            root_dir: dir.path().to_path_buf(),
            max_depth: Some(1),
            follow_links: false,
            name_pattern: Some(r"\.rs$".to_string()),
            content_pattern: Some("fn main".to_string()),
            max_results: None,
        };

        let (results, _stats) = search(&config).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].path.ends_with("main.rs"));
    }
}
