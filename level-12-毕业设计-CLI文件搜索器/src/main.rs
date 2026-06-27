// ================================================================
// rsearch — 高性能 CLI 文件搜索器（毕业设计）
// ================================================================
//
// WHAT: 命令行入口，使用 clap 解析参数，调用 searcher 执行搜索
// WHY: 将所有模块串联成一个完整的 CLI 工具
//
// 使用方法:
//   cargo run -- . --name "\.rs$"
//   cargo run -- src/ --name "\.rs$" --content "fn main"
//   cargo run -- . --name "README" --max-depth 2
//   cargo run -- . --name "\.rs$" --workers 8
//   cargo run -- . --name "\.md$" --no-color
//
// CONTRAST:
//   - Go:     flag.Parse() / cobra
//   - TS:     commander / yargs
//   - Python: argparse
//   Rust clap derive: 通过结构体 + 属性宏定义参数，编译期类型安全

use clap::Parser;
use colored::*;
use rsearch::searcher::walker::SearchConfig;
use std::process;

// ─── CLI 参数定义 ───
/// rsearch — 高性能文件搜索器
///
/// 递归搜索目录树，支持正则匹配文件名和文件内容。
/// 所有知识点（所有权、借用、Trait、错误处理、并发）的毕业设计。
#[derive(Parser, Debug)]
#[command(
    name = "rsearch",
    version = "1.0.0",
    author = "Rust 闯关者",
    about = "高性能 CLI 文件搜索器 | Rust 学习项目毕业设计"
)]
struct Cli {
    /// 搜索的根目录
    #[arg(default_value = ".")]
    directory: String,

    /// 文件名正则匹配模式（如 `\.rs$` 匹配 .rs 文件）
    #[arg(short = 'n', long = "name")]
    name: Option<String>,

    /// 文件内容搜索模式
    #[arg(short = 'c', long = "content")]
    content: Option<String>,

    /// 最大搜索深度（层级）
    #[arg(short = 'd', long = "max-depth")]
    max_depth: Option<usize>,

    /// 并行线程数（默认使用所有 CPU 核心）
    #[arg(short = 'w', long = "workers")]
    workers: Option<usize>,

    /// 最大结果数
    #[arg(short = 'm', long = "max-results")]
    max_results: Option<usize>,

    /// 是否跨越符号链接
    #[arg(short = 'L', long = "follow-links")]
    follow_links: bool,

    /// 禁用彩色输出
    #[arg(long = "no-color")]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();

    // ─── 构建搜索配置 ───
    let config = SearchConfig {
        root_dir: std::path::PathBuf::from(&cli.directory),
        max_depth: cli.max_depth,
        follow_links: cli.follow_links,
        name_pattern: cli.name,
        content_pattern: cli.content,
        max_results: cli.max_results,
    };

    // ─── 设置 rayon 线程数 ───
    if let Some(w) = cli.workers {
        rayon::ThreadPoolBuilder::new()
            .num_threads(w)
            .build_global()
            .unwrap_or_else(|e| {
                eprintln!("{} 设置线程数失败: {}", "警告:".yellow(), e);
            });
    }

    // ─── 执行搜索 ───
    match rsearch::searcher::walker::search(&config) {
        Ok((results, stats)) => {
            display_results(&results, &stats, cli.no_color);

            // 如果没有结果，返回非零退出码（Unix 约定）
            if results.is_empty() {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "错误:".red().bold(), e);
            process::exit(1);
        }
    }
}

/// 显示搜索结果
///
/// WHAT: 格式化输出搜索结果，支持彩色输出
/// WHY: 让结果可读性更好——文件路径、大小、匹配行清晰展示
fn display_results(results: &[rsearch::SearchResult], stats: &rsearch::searcher::walker::SearchStats, no_color: bool) {
    // ─── 头部统计信息 ───
    if !no_color {
        println!("{}", "═".repeat(60).dimmed());
        println!(
            "{} {} {} {} {}",
            "搜索完成".green().bold(),
            "| 扫描:".dimmed(),
            stats.files_scanned.to_string().cyan(),
            "| 匹配:".dimmed(),
            stats.files_matched.to_string().green().bold()
        );
        println!(
            "{} {} {}",
            "耗时:".dimmed(),
            format_duration(stats.duration_ms).yellow(),
            "| 线程:".dimmed()
        );
        println!("{}", "═".repeat(60).dimmed());
        println!();
    } else {
        println!("══════════════════════════════════════");
        println!("搜索完成 | 扫描: {} | 匹配: {} | 耗时: {}",
            stats.files_scanned, stats.files_matched, format_duration(stats.duration_ms));
        println!("══════════════════════════════════════");
        println!();
    }

    // ─── 文件列表 ───
    if results.is_empty() {
        if !no_color {
            println!("{}", "没有找到匹配的文件".yellow());
        } else {
            println!("没有找到匹配的文件");
        }
        return;
    }

    for (i, result) in results.iter().enumerate() {
        let path_str = result.path.display().to_string();

        if !no_color {
            // 彩色输出：序号 + 路径 + 大小
            print!(
                "{:>4}. {} {}",
                (i + 1).to_string().cyan(),
                path_str.white().bold(),
                format_size(result.size).dimmed()
            );
        } else {
            print!(
                "{:>4}. {} {}",
                i + 1,
                path_str,
                format_size(result.size)
            );
        }

        println!();

        // ─── 匹配行（如果有内容匹配） ───
        for line in &result.matched_lines {
            if !no_color {
                // 高亮匹配行中的关键词（简化版：整行变色）
                println!("       {}  {}", "│".dimmed(), line.magenta());
            } else {
                println!("       │  {}", line);
            }
        }
    }
}

/// 格式化文件大小
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("({:.1} MB)", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("({:.1} KB)", bytes as f64 / KB as f64)
    } else {
        format!("({} B)", bytes)
    }
}

/// 格式化持续时间
fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        format!("{:.2}s", ms as f64 / 1000.0)
    } else {
        let mins = ms / 60_000;
        let secs = (ms % 60_000) as f64 / 1000.0;
        format!("{}m{:.0}s", mins, secs)
    }
}
