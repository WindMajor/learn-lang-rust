// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 所有权转移后使用 (Move after use)                  ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   String 类型不实现 Copy，赋值时发生 Move（所有权转移），
//   之后试图访问原变量导致编译错误。
//   这是 Rust 新手（尤其是来自 C++/GC 语言背景的）最常见的错误。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_所有权转移后使用.rs`:
//
//   error[E0382]: borrow of moved value: `s1`
//     --> bug_01_所有权转移后使用.rs:XX:XX
//      |
//   51 |     let s1 = String::from("hello");
//      |         -- move occurs because `s1` has type `String`,
//      |            which does not implement the `Copy` trait
//   52 |     let s2 = s1;
//      |              -- value moved here
//   53 |     println!("{}", s1);
//      |                    ^^ value borrowed here after move
//      |
//   help: consider cloning the value if the performance cost is acceptable
//      |
//   52 |     let s2 = s1.clone();
//      |                ++++++++
//
//   error[E0382]: use of moved value: `s1`
//     ...
//
// 【为什么会这样】
//   Rust 所有权规则 #2: 值在任一时刻有且只有一个所有者。
//   s1 的值被移动到 s2 后，s1 不再拥有该值，编译器禁止继续使用 s1。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    string s1 = "hello";
//             string s2 = s1;       // 拷贝构造！两个独立副本都能用
//             string s3 = std::move(s1); // 但此时 s1 处于未定义状态
//             cout << s1;  // 未定义行为！运行时可能崩溃、可能输出空串、可能正常
//
//   - Go:     s1 := "hello"
//             s2 := s1  // Go 字符串不可变，复制 (ptr, len)
//             两者都可以用——因为 Go string 是"浅拷贝"语义
//
//   - TS:     let s1 = "hello";
//             let s2 = s1;  // 两个变量指向同一不可变字符串
//
//   关键差异:
//   Rust 编译器在编译时就阻止了"使用已移动值" —— C++ 给你一个
//   "有效但未定义状态"的对象（等你运行时报错），GC 语言根本不需要操心。
//
// 【如何修复】
//   方案1 (推荐): 如果只需要读取，使用借用: let s2 = &s1;
//   方案2: 如果需要独立拥有，使用 clone: let s2 = s1.clone();
//   方案3: 转移所有权后不再使用 s1，只使用 s2
//   方案4: 在赋值前先使用 s1

fn main() {
    let s1 = String::from("hello");
    // ❌ 编译错误: error[E0382]: borrow of moved value: `s1`
    let s2 = s1;
    // println!("s1 = {}", s1);  // 取消注释触发编译错误
    println!("s2 = {}", s2); // ✅ s2 是当前所有者，可以使用

    println!();
    println!("═══ 修复方案对比 ═══");
    println!();

    // ─── 修复方案1: clone（显式深拷贝） ───
    {
        let s1 = String::from("方案1 clone");
        let s2 = s1.clone(); // 显式深拷贝
        println!("s1 = {s1}, s2 = {s2}"); // 两个变量各有一份数据
    }

    // ─── 修复方案2: 借用（见 Level 03） ───
    {
        let s1 = String::from("方案2 借用");
        let s2 = &s1; // 不可变借用，s1 仍拥有数据
        println!("s1 = {s1}, s2 = {s2}");
    }

    println!();
    println!("核心教训: Rust 默认 Move，C++ 默认 Copy。");
    println!("这是 Rust 与 C++ 最根本的哲学差异之一。");
}
