// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 未声明模块引用 —— 忘记 use 或 mod                ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   在代码中使用了 HashMap，但既没有 use std::collections::HashMap，
//   也没有通过完整路径引用。这是 Rust 新手最常见的编译错误之一。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_未声明模块引用.rs`:
//
//   error[E0433]: failed to resolve: use of undeclared type `HashMap`
//     --> bug_02_未声明模块引用.rs:67:13
//      |
//   67 |     let mut scores: HashMap<&str, i32> = HashMap::new();
//      |                     ^^^^^^^ use of undeclared type `HashMap`
//      |
//   help: consider importing this struct
//      |
//    1 + use std::collections::HashMap;
//
//   error[E0433]: failed to resolve: use of undeclared type `HashMap`
//     --> bug_02_未声明模块引用.rs:67:44
//      |
//   67 |     let mut scores: HashMap<&str, i32> = HashMap::new();
//      |                                            ^^^^^^^ not found in this scope
//
// 【为什么会这样】
//   Rust 的 prelude（自动导入集）非常小，只包含最常用的类型：
//   - std::option::Option
//   - std::result::Result
//   - std::vec::Vec
//   - std::string::String
//   - Box, Drop, Clone, Copy, Send, Sync 等 trait
//   HashMap 不在 prelude 中，需要显式 import
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++: std::unordered_map 需要 #include <unordered_map>
//   - Go:  map[K]V 是内置类型，不需要 import（更便捷但模糊了实现细节）
//   - TS:  Map<K,V> 是全局类型，无需 import
//   差异: Rust 的 prelude 策略是"显式优于隐式"哲学的体现
//
// 【如何修复】
//   在文件顶部添加: use std::collections::HashMap;
//   或使用完整路径: let mut scores: std::collections::HashMap<&str, i32> = ...

fn main() {
    // ❌ 编译错误: error[E0433]: failed to resolve: use of undeclared type `HashMap`
    // let mut scores: HashMap<&str, i32> = HashMap::new();

    // ✅ 修复方案1: 添加 use 语句（在文件顶部）
    // ✅ 修复方案2: 使用完整路径
    let mut scores: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    scores.insert("Rust", 100);
    println!("修复后: {:?}", scores);

    println!("注: 取消注释第67-68行并注释第72-74行可体验错误");
    println!("对比: Go 的 map 是内置类型不需要 import，");
    println!("      Rust 显式 import 让依赖关系一目了然");
}
