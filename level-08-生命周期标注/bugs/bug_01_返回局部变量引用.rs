// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 返回局部变量引用 —— does not live long enough   ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   函数返回局部变量的引用，违反生命周期规则。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_返回局部变量引用.rs`:
//
//   error[E0515]: cannot return reference to local variable `result`
//     --> bug_01_返回局部变量引用.rs:XX:XX
//      |
//   XX |     let result = String::from("hello");
//      |         ------ `result` is borrowed here
//   XX |     &result
//      |     ^^^^^^^ returns a reference to data owned by the
//      |             current function
//
// 【在 C++ 中对应的行为】
//   完全相同的问题在 C++ 中:
//     const string& bad() {
//       string result = "hello";
//       return result;  // 编译器警告，但代码可编译运行——UB
//     }
//   GCC/Clang: warning: returning reference to local temporary
//   但这只是一个 WARNING，代码可以正常编译链接执行——未定义行为
//
// 【如何修复】
//   返回 String 所有权，而不是引用

fn main() {
    // ❌ 错误版本（编译不通过）
    // fn bad_return() -> &str {
    //     let result = String::from("hello");
    //     &result // result 在函数结束时析构，返回的引用指向已释放内存
    // }

    // ✅ 正确版本
    fn good_return() -> String {
        let result = String::from("hello");
        result // 所有权移出函数
    }

    let s = good_return();
    println!("{s}");

    println!();
    println!("核心: Rust 在编译期拒绝返回局部引用");
    println!("对比: C++ 只给 WARNING（甚至没有），运行时 UB");
    println!("      Go 自动逃逸分析 → 堆分配（隐式开销）");
    println!("      GC 语言根本不需要关心这个问题");
}
