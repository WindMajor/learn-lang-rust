// ================================================================
// Level 01: Rust 编译模型与 Cargo 生态
// 目标: 理解 Rust 的一体化工具链和模块系统
// ================================================================
//
// WHAT: 本文件演示 Rust 项目的核心组织结构
// WHY: Rust 的模块系统与 C/C++ 的 #include、Go 的 package、TS 的 import/export
//      有本质差异——理解这些差异是后续所有关卡的基础
//
// CONTRAST 核心差异地图:
// ┌──────────┬──────────────┬──────────────┬──────────────┐
// │ 概念      │ Rust         │ C/C++        │ Go           │
// ├──────────┼──────────────┼──────────────┼──────────────┤
// │ 编译单位  │ crate        │ 翻译单元     │ package      │
// │ 模块声明  │ mod (显式)   │ #include     │ 目录=package │
// │ 可见性    │ 默认私有     │ 默认公开     │ 首字母大写   │
// │ 路径分隔  │ ::           │ :: 或 /      │ .            │
// │ 构建系统  │ Cargo(内置)  │ 自选         │ go build     │
// │ 包管理    │ Cargo(内置)  │ 无标准       │ go mod       │
// └──────────┴──────────────┴──────────────┴──────────────┘

// ─── 模块声明 ───
// WHAT: mod 声明模块。Rust 会在以下位置查找模块体:
//   1. 同文件内嵌: mod foo { ... }
//   2. src/foo.rs
//   3. src/foo/mod.rs
// CONTRAST: C/C++ 用 #include "foo.h"（文本包含，无模块语义）
//           Go 用目录路径决定 package 名（自动匹配）
//           TS 用 import from './foo'（文件路径决定）
mod math_utils;      // 在 src/math_utils.rs 或 src/math_utils/mod.rs 中定义
mod string_tools;    // 另一个模块

// WHAT: use 将外部路径引入当前作用域，相当于创建别名
// CONTRAST: C++ using namespace、Go import、TS import
//           Rust 的 use 可以重命名: use std::collections::HashMap as Map;
use math_utils::add;
use math_utils::multiply;
use string_tools::greet;

// WARNING: Rust 的 main 函数不接受命令行参数（参数通过 std::env::args() 获取）
// CONTRAST: C/C++: int main(int argc, char* argv[])
//           Go: func main() { os.Args }
//           TS: process.argv
fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 01: 编译模型与 Cargo 生态     ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 模块间调用 ───
    println!("━━━ 1. 模块间函数调用 ━━━");
    let sum = add(3, 4);
    let product = multiply(3, 4);
    println!("  add(3, 4) = {sum}");
    println!("  multiply(3, 4) = {product}");
    greet();
    println!();

    // ─── 2. Cargo 生态：使用标准库 ───
    println!("━━━ 2. 标准库类型速览 ━━━");

    // Vec<T> —— Rust 的动态数组
    // CONTRAST: C++ std::vector<T>（大小可增长）
    //           Go []T（slice，引用底层数组）
    //           TS Array<T>（GC 管理，无内存布局控制）
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    // WHAT: dbg!() 宏 —— 打印表达式及其值到 stderr，返回所有权
    // WHY: 调试时不用写 println!("{:?}", x)，且保留值的所有权（与 borrow 不同）
    // CONTRAST: C++ 没有内置等价的调试宏
    //           Go 没有内置，需用 fmt.Printf("%+v", x)
    //           TS console.log(x) 类似但无所有权概念
    dbg!(&numbers);
    println!("  numbers 长度: {}, 容量: {}", numbers.len(), numbers.capacity());
    println!();

    // HashMap —— Rust 的哈希表
    // CONTRAST: C++ std::unordered_map、Go map[K]V、TS Map<K,V>
    //           Rust 不在 prelude 中，需显式 use
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    // WHAT: entry().or_insert() —— 存在则返回引用，不存在则插入默认值
    // WHY: 避免两次哈希查找（Rust API 设计追求零成本）
    // CONTRAST: Go: if _, ok := m[k]; !ok { m[k] = v }
    //           TS: m[k] ??= v  但 TS 没有"一次查找"的保证
    scores.entry("Charlie").or_insert(60);
    dbg!(&scores);
    println!();

    // ─── 3. 理解 Cargo 项目结构 ───
    println!("━━━ 3. Cargo 项目结构 ━━━");

    // WHAT: 使用 include_str!() 编译期嵌入文件内容
    // WHY: 编译时将文件内容嵌入二进制，运行时零开销读取
    // CONTRAST: C/C++ 的 #include 是预处理阶段文本替换
    //           Go 用 embed 包（Go 1.16+ 编译期嵌入）
    //           TS 没有等价的编译期嵌入机制
    let cargo_toml = include_str!("../Cargo.toml");
    println!("  Cargo.toml 内容（{} 字节）:", cargo_toml.len());
    println!("  ────────────────────────────");
    for line in cargo_toml.lines().take(6) {
        println!("  {}", line);
    }
    println!("  ... (省略)");

    // ─── 4. 编译时与运行时 ───
    println!();
    println!("━━━ 4. 编译时 vs 运行时 ━━━");

    // WHAT: env!("CARGO_PKG_NAME") —— 编译时读取环境变量/Cargo.toml 元数据
    // WHY: 零运行时开销，信息嵌入二进制
    // CONTRAST: Go 用 runtime/debug.ReadBuildInfo() 运行时读取
    //           TS 用 process.env 运行时读取
    //           C/C++ 用 #define 宏或 __FILE__/__LINE__ 预处理器常量
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    println!("  包名: {pkg_name}");
    println!("  版本: {pkg_version}");

    // WHAT: compile_error!() —— 编译期断言失败
    // WHY: 在编译时阻止不符合条件的代码编译（比 C/C++ 的 static_assert 更灵活）
    // WARNING: 下面的代码如果取消注释会导致编译失败
    // compile_error!("这条消息会在编译时显示，阻止生成二进制文件");

    // WHAT: cfg!() —— 编译期条件判断的运行时版本
    let is_debug = cfg!(debug_assertions);
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;
    println!("  Debug 模式: {is_debug}");
    println!("  目标系统: {target_os}");
    println!("  目标架构: {target_arch}");

    // ─── 5. 与 C/C++/Go 的关键差异 ───
    println!();
    println!("━━━ 5. 编译模型核心差异 ━━━");
    println!("  C/C++:  头文件(.h) + 源文件(.c/.cpp) → 编译器 → .o → 链接器 → 可执行文件");
    println!("          问题: ODR 违规、循环依赖、编译慢、ABI 不稳定");
    println!();
    println!("  Go:     .go 文件 → 编译器 → 静态链接可执行文件");
    println!("          特点: 极快编译速度、无头文件、隐式接口");
    println!();
    println!("  Rust:   .rs 文件 → Cargo 解析模块树 → rustc 逐 crate 编译 → 静态链接");
    println!("          特点: 无头文件、编译期借用检查、Link-Time Optimization、");
    println!("                编译器错误信息极其友好（这是Rust的最大优势之一）");
    println!();
    println!("  Rust 一体化工具链 = Go 的简洁 + C++ 的性能 + 编译器级别的安全保证");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 01 通关！继续 Level 02        ║");
    println!("╚══════════════════════════════════════╝");
}
