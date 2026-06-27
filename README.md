# learn-lang-rust：闯关式 Rust 语言学习项目

> 12 关，每关 30~90 分钟，打通 Rust 核心语法与系统级编程能力。
> 专为已掌握 TS/C/C++/Go/Kotlin/Swift/Java/Python 的资深开发者设计。
> **不是学 Rust 语法，而是建立 Rust 与你已知语言的"差异地图"。**

---

## 环境要求

- **Rust 1.75+ Stable**：通过 `rustup` 安装
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update stable
  rustc --version  # 确保 >= 1.75.0
  ```
- **编辑器**：VSCode + rust-analyzer 插件
- **操作系统**：macOS（本教程默认，Windows/Linux 同样适用）

## 项目结构总览

```
learn-lang-rust/
├── README.md                          # ← 你在这里
├── level-01-编译模型与Cargo/          # 1.5h  Cargo 生态、模块系统
├── level-02-所有权与移动语义/         # 1.5h  所有权三规则、Move/Copy/Clone
├── level-03-借用与引用/               # 1.5h  &T、&mut T、借用规则
├── level-04-字符串与集合基础/         # 1h    String/&str、Vec、切片
├── level-05-结构体枚举模式匹配/       # 1.5h  struct、enum、match
├── level-06-错误处理/                 # 1.5h  Result、Option、?、panic!
├── level-07-Trait与泛型/              # 2h    trait、泛型、孤儿规则
├── level-08-生命周期标注/             # 1.5h  生命周期、Elision、'static
├── level-09-迭代器与闭包/             # 1.5h  Iterator、Fn/FnMut/FnOnce
├── level-10-智能指针与内部可变性/     # 1.5h  Box/Rc/RefCell/Arc/Mutex
├── level-11-并发与线程安全/           # 2h    thread、channel、Send/Sync
└── level-12-毕业设计-CLI文件搜索器/   # 3h    完整项目：多线程文件搜索器
```

## 学习路线图：从已知看未知

```
你已掌握的              →  Rust 对应物                关级
══════════════════════════════════════════════════════════════
C 编译链接/Makefile      →  rustc/Cargo/crates.io       Level 01
C++ RAII/移动语义        →  Ownership/Move/Copy/Drop    Level 02
C++ 指针/引用/const      →  &T / &mut T / 借用规则      Level 03
C/C++ 字符串/数组        →  String/&str/Vec/切片        Level 04
TS discriminated union   →  enum + match（穷尽检查）     Level 05
Go error/Haskell Maybe   →  Result/Option/? 运算符       Level 06
TS interface/Go 隐式接口  →  Trait（显式 impl+孤儿规则） Level 07
C++ 悬垂指针（运行时炸）  →  生命周期标注（编译期拒绝）  Level 08
Python 推导式/STL 迭代器  →  Iterator/闭包/Fn 系列       Level 09
C++ shared_ptr/mutex     →  Rc/RefCell/Arc/Mutex        Level 10
Go goroutine/Java 线程    →  thread/channel/Send/Sync   Level 11
──────────────────────────────────────────────────────────────
全部融会贯通              →  毕业设计：CLI 文件搜索器    Level 12
```

## 使用方法

### 每关标准流程

```bash
# 1. 进入关卡
cd level-01-编译模型与Cargo/

# 2. 阅读本关 README（5 分钟）
cat README.md

# 3. 运行主程序，观察输出
cargo run

# 4. 阅读 src/main.rs 中的详细注释（重点看 CONTRAST 标记）

# 5. 尝试修改代码，制造错误，观察编译器报错

# 6. 学习 bugs/ 目录下的错误案例
cd bugs/
rustc bug_01_xxx.rs  # 观察编译错误
# 阅读注释中的修复方案

# 7. 回到 README 完成自检清单
```

### 通关标准（每关通用）

- [ ] 能独立默写本关核心代码结构
- [ ] 能修复 bugs/ 目录下的所有错误
- [ ] 能向 C++/Go/TS 开发者解释本关 Rust 概念的本质差异
- [ ] 能预测对应 C++/Go 写法中的运行时风险

## 核心心法

1. **Rust 编译器的报错就是最好的老师**——比任何教程都精准
2. **每次编译通过都是一次"内存安全检查通过"的证明**——C/C++ 永远做不到这一点
3. **所有权不是限制，是指南**——它迫使你思考：谁拥有这份数据？谁负责释放？
4. **零成本抽象不是口号**——泛型单态化、Trait 静态分发、迭代器链优化后与手写循环等价
5. **Rust 没有 GC，但也没有段错误**——编译期保证了 C/C++ 需要运行时检查才能保证的事

## 毕业设计预览

**level-12** 将带你从零构建一个**多线程 CLI 文件搜索器**（类似 `fd` 的简化版），涵盖：

- `clap` 命令行参数解析
- `walkdir` 递归目录遍历  
- `regex` 正则表达式匹配
- `rayon` 多线程并行搜索
- `colored` 彩色输出
- 自定义 Trait + 错误处理 + 单元测试

---

开始闯关吧！→ `cd level-01-编译模型与Cargo`
