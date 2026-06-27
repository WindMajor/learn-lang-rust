// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 悬垂引用(Dangling Reference) —— 借用比被借用者  ║
// ║          活得久，Rust 编译期拒绝                           ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   试图返回一个指向局部变量的引用。函数返回后局部变量被释放，
//   引用指向已释放的内存——典型的"悬垂引用"。
//   Rust 编译器在编译期就发现并拒绝。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_悬垂引用.rs`:
//
//   error[E0515]: cannot return reference to local variable `s`
//     --> bug_03_悬垂引用.rs:XX:XX
//      |
//   XX |     let s = String::from("hello");
//      |         - `s` is borrowed here
//   XX |     &s
//      |     ^^ returns a reference to data owned by the current function
//
//   error[E0515]: cannot return reference to local variable `result`
//     ...
//
// 【为什么会这样】
//   借用规则 #2: 引用必须总是有效的（不能比被引用者活得更久）。
//   局部变量 s 在函数返回时被释放，&s 指向已释放的内存。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    const string& dangling() {
//               string s = "hello";
//               return s;  // 返回引用指向栈上的局部变量！
//             }
//             // GCC/Clang 警告: returning reference to local
//             // 但代码可以编译通过！运行结果未定义（可能崩溃、可能"正常"）
//             // 这是 C++ 中最常见的悬垂引用 bug 之一
//
//   - Go:     func dangling() *string {
//               s := "hello"
//               return &s  // Go 编译器检测到逃逸，自动将 s 分配到堆！
//             }
//             // Go 的逃逸分析(escape analysis)自动救了程序员
//             // 代价: 隐式的堆分配（性能开销不透明）
//
//   - TS:     函数不能返回局部变量的"地址"（没有这个概念）
//
//   关键差异:
//   C++ 给你警告但允许编译 —— "你可能是故意的"（但通常不是）
//   Go 自动将变量分配到堆 —— "我来帮你擦屁股"（但有隐式开销）
//   Rust 编译期拒绝 —— "这不可能正确，我不编译"
//
// 【如何修复】
//   方案1: 返回 String（所有权转移），让调用者拥有数据
//   方案2: 使用 Box<String> 显式堆分配
//   方案3: 接收一个生命周期适当的引用参数

fn main() {
    // ─── 错误演示 ───
    // fn create_dangling() -> &String {
    //     let s = String::from("hello");
    //     &s  // ❌ 返回局部变量的引用
    // }

    // ─── 修复方案1: 返回所有权 ───
    fn create_owned() -> String {
        let s = String::from("hello");
        s // 所有权移出函数，安全
    }

    fn create_owned_impl() -> String {
        String::from("hello") // 更简洁
    }

    let s1 = create_owned();
    let s2 = create_owned_impl();
    println!("{} {}", s1, s2);

    // ─── 修复方案2: 接收引用参数 ───
    fn first_char<'a>(s: &'a str) -> &'a str {
        &s[0..1] // 返回的引用不超出输入引用的生命周期
    }

    let text = String::from("Rust");
    let first = first_char(&text);
    println!("第一个字符: {first}"); // ✅ 安全，text 还活着

    println!();
    println!("═══ 跨语言对比 ═══");
    println!();
    println!("  场景: 将局部变量的引用返回给调用者");
    println!();
    println!("  C++:   编译通过（只有警告），运行时 UB");
    println!("  Go:    编译器自动逃逸分析 → 堆分配（有隐式 GC 成本）");
    println!("  Rust:  编译期拒绝！你必须显式选择所有权转移或堆分配");
    println!();
    println!("  Rust 哲学: 内存管理决策必须显式、可见、零成本——");
    println!("  不像 Go 那样悄悄帮你堆分配，也不像 C++ 让你在 UB 中裸奔。");
}
