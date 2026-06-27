// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: String vs &str 类型混淆 —— 函数参数错误        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   函数期望 String 但传入 &str，或反过来。
//   虽然 Deref trait 让 &String 可以自动转为 &str，但方向是单向的。
//   你需要明确知道什么时候该用 String，什么时候该用 &str。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_字符串字面量类型混淆.rs`:
//
//   error[E0308]: mismatched types
//     --> bug_03_字符串字面量类型混淆.rs:XX:XX
//      |
//   XX |     consume_string("hello");
//      |     --------------- ^^^^^^^ expected `String`, found `&str`
//      |     |
//      |     arguments to this function are incorrect
//      |
//   note: function defined here
//     --> bug_03_字符串字面量类型混淆.rs:XX:XX
//      |
//   XX  | fn consume_string(s: String) {
//       |    ^^^^^^^^^^^^^^^ ----------
//   help: call `String::from` or `.to_string()` to convert
//      |
//   XX |     consume_string(String::from("hello"));
//      |                    ++++++++++++++       +
//
// 【为什么会这样】
//   - "hello" 的类型是 &str（字符串字面量，编译期嵌入）
//   - String::from("hello") 才创建拥有的 String（运行时堆分配）
//   - Rust 不会隐式执行堆分配——这是 C++ 中常见的性能陷阱来源
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    void take(string s);  // 参数类型 string
//             take("hello");        // 隐式构造 string（堆分配！）
//             // 你无法从代码看出这里有隐式的堆分配
//
//   - Go:     func take(s string) {}
//             take("hello") // "hello" 已经是 string 类型，无转换
//
//   - TS:     function take(s: string) {}
//             take("hello"); // 原始值，直接使用
//
//   关键差异:
//   Rust 强迫堆分配显式化——你不会在无意中分配 10MB 的字符串副本
//
// 【如何修复】
//   方案1（最佳）: 函数参数改用 &str
//   方案2: 调用时显式转换 String::from("hello") 或 "hello".to_string()

fn consume_string(s: String) {
    println!("  拥有: \"{s}\"");

    // CONTRAST: 这个函数签名暗示"我需要拥有这个字符串的所有权"
    //           可能是想修改它，或者把它存到某处
    //           如果只是读取，应该用 &str
}

fn use_str(s: &str) {
    println!("  借用: \"{s}\"");
}

fn main() {
    // ❌ 编译错误: expected String, found &str
    // consume_string("hello");

    // ✅ 方案1: 显式转换
    consume_string(String::from("hello"));
    consume_string("world".to_string());

    // ✅ 方案2（推荐）: 改函数签名为 &str
    use_str("hello");
    use_str(&String::from("world")); // String 自动解引用为 &str
    use_str(&"literal");             // 字面量本身就是 &str

    println!();
    println!("═══ 函数签名选择指南 ═══");
    println!("  fn foo(s: &str)      — 通用推荐，只读取");
    println!("  fn foo(s: String)     — 需要所有权（修改或存储）");
    println!("  fn foo(s: &mut String) — 需要原地修改");
    println!("  fn foo(s: impl Into<String>)  — 接受多种类型（更灵活）");
    println!();
    println!("对比: C++ 中 void f(string s) 会隐式触发构造和拷贝");
    println!("      Rust 杜绝隐式开销——每一处堆分配都是显式的");
}
