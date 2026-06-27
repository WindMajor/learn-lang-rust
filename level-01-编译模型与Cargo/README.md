# Level 01：编译模型与 Cargo 生态

## 通关标准

> 本关结束后，你能独立创建 Rust 项目、管理依赖、理解模块可见性，并能向 C/C++/Go/TS 开发者解释 Rust 的编译模型与之的本质差异。

## 核心概念速查

| Rust 概念 | 对比 C/C++ | 对比 Go | 对比 TS/Node |
|-----------|-----------|--------|-------------|
| `cargo new` | `cmake`/手动 Makefile | `go mod init` | `npm init`/`yarn init` |
| `Cargo.toml` | `CMakeLists.txt` | `go.mod` | `package.json` |
| `Cargo.lock` | 无直接对应（手动锁版本） | `go.sum` | `package-lock.json`/`yarn.lock` |
| `cargo build` | `cmake --build`/`make` | `go build` | `tsc`/`ncc` |
| `cargo run` | 手动编译+运行 | `go run` | `ts-node`/`node` |
| `cargo test` | 无内置（依赖 gtest 等） | `go test` | `jest`/`vitest` |
| `mod`/`pub`/`use` | `#include`+头文件（完全不同） | `package`/`import` | `import`/`export` |
| crate | `.a`/`.so`（静态/动态库） | module（包） | npm package |
| crates.io | 无中心仓库（各发行版自管） | pkg.go.dev | npmjs.com |

## 核心差异

1. **Cargo = 构建系统 + 包管理器 + 测试运行器 + 文档生成器 + lint 工具** —— 一体化工具链
2. **模块系统 ≠ 文件系统**：`mod` 声明的是逻辑层次，文件只是物理载体
3. **`cargo check` 比 `cargo build` 快 3-5 倍** —— 只做类型检查不生成代码，开发时必用
4. **crates.io 上的包都可以直接 `cargo doc --open` 生成本地文档**

## 编译/运行命令

```bash
# 编译（debug 模式，含调试信息，未优化）
cargo build

# 编译并运行
cargo run

# 仅做类型/借用检查（不生成二进制，速度更快）
cargo check

# 发布模式（优化，去除调试信息）
cargo build --release

# 生成并打开文档
cargo doc --open

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 静态分析
cargo clippy
```

## 模块可见性速查

| 关键字 | 含义 |
|--------|------|
| 无修饰 | 私有（仅当前模块及其子模块可见） |
| `pub` | 公开（外部可访问） |
| `pub(crate)` | 仅在当前 crate 内可见 |
| `pub(super)` | 仅在父模块可见 |
| `pub(self)` | 等价于私有 |
| `pub(in path)` | 仅在指定路径可见 |

## 自检清单

- [ ] 能手写 `cargo new` 创建项目，不查文档
- [ ] 能解释 `Cargo.toml` 中 `[dependencies]` 的版本号语法（`^`、`~`、`=`）
- [ ] 能独立修复 `bugs/` 目录下的 2 个错误（并读懂 rustc 错误信息）
- [ ] 能解释为什么 Rust 不需要 CMake 这样的独立构建系统
- [ ] 能向一个 Go 开发者解释 `mod` 与 Go package 的本质差异
