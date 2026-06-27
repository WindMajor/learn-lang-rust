# Level 12：毕业设计 —— 高性能 CLI 文件搜索器

## 通关标准

> 本关结束后，你将拥有一个完整的、可编译运行、可交付的 Rust 项目。
> 它把前面 11 关的所有知识点串联在一起，是一个真实可用的 CLI 工具。

## 项目简介

`rsearch` —— 一个类似 `fd` / `grep` 的递归文件搜索器，支持：

- **递归目录遍历**（`walkdir`）：自动跳过 `.git`、`node_modules` 等目录
- **正则表达式匹配**（`regex`）：按文件名模式搜索
- **文件内容搜索**：在匹配的文件中搜索内容
- **多线程并行搜索**（`rayon`）：充分利用多核 CPU
- **彩色输出**（`colored`）：搜索结果高亮显示
- **自定义 Trait** + **错误处理** + **单元测试**

## 知识点全覆盖

| 关卡 | 知识点 | 在项目中的体现 |
|------|--------|---------------|
| L01 | Cargo/模块 | 多文件模块组织 (`src/searcher/`) |
| L02 | 所有权 | 函数间传递 String/Vec/PathBuf |
| L03 | 借用 | 函数参数大量使用 `&str`/`&Path` |
| L04 | 字符串/切片 | 文件路径处理、内容切片 |
| L05 | 枚举/模式匹配 | 自定义错误 `SearchError`、`match` 处理 |
| L06 | Result/? | 全部函数返回 `Result<_, SearchError>` |
| L07 | Trait/泛型 | `SearchFilter` trait + `dyn` 动态分发 |
| L08 | 生命周期 | 结构体中的引用、函数返回引用 |
| L09 | 迭代器/闭包 | walkdir 迭代器、闭包回调 |
| L10 | 智能指针 | `Arc` 跨线程共享配置 |
| L11 | 并发 | `rayon` 多线程并行、`Arc` + `Mutex` |

## 编译与运行

```bash
# 开发编译
cargo build

# 发布编译（优化）
cargo build --release

# 运行示例
cargo run -- --help
cargo run -- . --name "\.rs$"                    # 搜索当前目录下所有 .rs 文件
cargo run -- src/ --name "\.rs$" --content "fn main"  # 搜索含 main 函数的 rs 文件
cargo run -- . --name "README" --max-depth 2     # 最大搜索深度 2 层
cargo run -- . --name "\.rs$" --workers 8        # 使用 8 个线程
cargo run -- . --name "\.md$" --no-color         # 关闭颜色输出

# 运行测试
cargo test
```

## 项目结构

```
level-12-毕业设计-CLI文件搜索器/
├── README.md
├── Cargo.toml
└── src/
    ├── main.rs           # CLI 入口（clap 参数解析）
    ├── lib.rs            # 库根模块
    ├── error.rs          # 自定义错误类型
    └── searcher/
        ├── mod.rs         # 搜索模块入口
        ├── walker.rs      # 目录遍历 + 多线程搜索
        └── filter.rs      # 文件名/内容匹配过滤
```

## 自检清单

- [ ] 能独立编译运行 `cargo run` 和 `cargo test`
- [ ] 能解释项目中 `Arc` 的使用场景（为什么需要跨线程共享？）
- [ ] 能解释自定义 `SearchError` 的设计（如何实现 `From` trait？）
- [ ] 能解释 `SearchFilter` trait 的作用（静态分发 vs 动态分发）
- [ ] 能独立扩展项目：添加一个新的 Filter 实现（如按文件大小过滤）
