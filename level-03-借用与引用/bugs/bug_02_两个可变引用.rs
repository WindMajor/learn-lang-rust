// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 两个可变引用 —— 违反"排他可变"规则              ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   尝试在同一作用域内创建两个可变引用 &mut。Rust 严格禁止。
//   这是防止数据竞争（data race）的编译期机制。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_两个可变引用.rs`:
//
//   error[E0499]: cannot borrow `s` as mutable more than once at a time
//     --> bug_02_两个可变引用.rs:XX:XX
//      |
//   XX |     let r1 = &mut s;
//      |              ------ first mutable borrow occurs here
//   XX |     let r2 = &mut s;
//      |              ^^^^^^ second mutable borrow occurs here
//   XX |     println!("r1 = {}", r1);
//      |                          -- first borrow later used here
//
// 【为什么会这样】
//   借用规则 #1: 同一时间只能有一个可变引用。
//   这背后是更根本的原则: Rust 编译期杜绝数据竞争。
//   如果两个 &mut 可以同时存在，两个不同的修改者可能并发修改同一数据，
//   导致数据竞争（即使在单线程中也可能因为"叠加修改"导致逻辑错误）。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    string s = "hello";
//             string& r1 = s;
//             string& r2 = s;  // 编译通过！
//             r1.push_back('x');
//             r2.push_back('y');
//             // 结果: s = "helloxy" —— 这在你预期之内吗？
//             // 在更复杂的场景下会导致"谁先谁后"的不确定性
//
//   - Go: 多个 goroutine 并发修改同一变量 → data race（运行时检测）
//
//   关键差异:
//   C++ 允许你同时拥有多个可变别名，这本身不是 UB，
//   但它会悄悄导致逻辑错误（多个地方的修改互相干扰）。
//   Rust 直接把这种模式看作编译错误——"想同时可变借用两次？不行。"
//
// 【如何修复】
//   方案1: 顺序使用（用完一个再创建另一个，依赖 NLL）
//   方案2: 用作用域隔离

fn main() {
    // ─── 错误演示 ───
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // r1.push_str(" world"); // ❌ error[E0499]
    // println!("{}", r2);

    // ─── 修复方案: 顺序使用 ───
    {
        println!("═══ 修复: 顺序使用可变引用 ───");
        let mut s = String::from("hello");

        let r1 = &mut s;
        r1.push_str(" world");
        println!("  r1 修改后: {r1}");
        // r1 在这里之后不再使用（NLL 结束借用）

        let r2 = &mut s; // ✅ 此时只有一个可变引用
        r2.push_str("!");
        println!("  r2 修改后: {r2}");
    }

    // ─── 深入理解: 这不是 Rust 的"限制" ───
    println!();
    println!("═══ 为什么这是好事? ───");
    println!("  考虑 C++ 场景:");
    println!("    void append(string& s1, string& s2) {");
    println!("      s1.push_back('!'); s2.push_back('?');");
    println!("    }");
    println!("    string s; append(s, s); // 同一个 s 被修改两次！");
    println!();
    println!("  Rust 编译器在编译期就拒绝这种模式——");
    println!("  不是因为"做不到"，而是因为它想保护你不产生逻辑错误。");
}
