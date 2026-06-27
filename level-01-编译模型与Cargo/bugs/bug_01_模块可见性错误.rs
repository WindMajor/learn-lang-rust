// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 模块可见性错误 —— 尝试访问私有函数              ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   在 main 中尝试调用 math_utils 模块的私有函数 internal_helper()
//   Rust 模块的默认可见性是私有的，不加 pub 则外部不可见
//
// 【编译后会报什么错】
//   尝试在本文件所在目录执行 `rustc bug_01_模块可见性错误.rs` 会失败，
//   因为本文件只是一个示例。正确的检查方式是：
//   在 level-01 根目录执行 `cargo build` 时，
//   如果 src/main.rs 中存在以下代码会报错:
//
//   error[E0603]: function `internal_helper` is private
//     --> src/main.rs:XX:XX
//      |
//   XX |     math_utils::internal_helper();
//      |                   ^^^^^^^^^^^^^^^ private function
//      |
//   note: the function `internal_helper` is defined here
//     --> src/math_utils.rs:XX:1
//      |
//   XX | fn internal_helper() -> &'static str {
//      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//
// 【为什么会这样】
//   Rust 选择了"默认私有"策略。这在大型项目中至关重要：
//   - 公开 API 表面积极小化 = 变更影响范围小
//   - 编译器帮你强制 API 边界
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++: 类的 private 成员外部不可见（编译错误），但顶层函数默认 public
//   - Go:  小写开头函数包外不可见（编译错误）—— 与 Rust 最接近
//   - TS:  不 export 的函数文件外不可用（编译错误）
//   差异: Rust 是少数将"模块私有"作为默认的语言之一
//
// 【如何修复】
//   方案1: 在函数前加 pub（如果确实需要暴露）:
//         pub fn internal_helper() -> &'static str { ... }
//   方案2: 在 main.rs 中删除对外部私有函数的调用

// 模拟不正确的调用（如果这是 main.rs 的内容）:
mod math_utils {
    // 私有函数 —— 故意不加 pub
    fn internal_helper() -> &'static str {
        "内部辅助"
    }
}

fn main() {
    println!("尝试访问私有函数:");
    // ❌ 编译错误: error[E0603]: function `internal_helper` is private
    // math_utils::internal_helper();
    // 取消上一行注释即可触发真实错误

    println!("注: 取消注释第XX行触发 E0603 错误");
    println!("对比: Go 中 func internalHelper() 小写开头同样是包私有");
    println!("      C++ 顶层函数默认 public，这是 Rust 与 C++ 的重要差异");
}
