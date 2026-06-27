// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 索引字符串 —— Rust 不允许直接用 [] 索引         ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   尝试使用 s[i] 直接索引 String/&str。Rust 不允许！
//   来自 C/C++/Go/TS 背景的开发者几乎人人都会踩这个坑。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_索引字符串.rs`:
//
//   error[E0277]: the type `str` cannot be indexed by `{integer}`
//     --> bug_01_索引字符串.rs:XX:XX
//      |
//   XX |     let first_char = s[0];
//      |                       ^^^ string indices are ranges of bytes
//      |
//      = help: the trait `std::ops::Index<{integer}>` is not
//              implemented for `str`
//
//   error[E0277]: the type `str` cannot be indexed by `{integer}`
//     ...
//
// 【为什么会这样】
//   Rust 的 String 是 UTF-8 编码。s[i] 的 i 是字节索引（O(1)），
//   但字符 ≠ 字节！"世"占用 3 个字节，s[0] 返回什么？
//   - 返回第 0 个字节？可能得到一个无效的 UTF-8 片段
//   - 返回第 0 个字符？需要 O(n) 扫描
//
//   Rust 选择不提供这种有歧义的 API。
//   你必须显式选择：
//   - s.as_bytes()[i] → 第 i 个字节
//   - s.chars().nth(i) → 第 i 个字符（O(n)）
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    string s = "Hello世界";
//             s[0]  // 'H' —— 返回 char（1字节）
//             s[6]  // 不是 '世'！是 '世' 的 UTF-8 第一个字节！
//             // 静默错误！
//
//   - Go:     s := "Hello世界"
//             s[0]  // 返回 72（字节值，不是字符！）
//             s[6]  // 返回 UTF-8 片段字节，同样坑
//
//   - TS:     s[0] 返回 "H"，s[6] 返回 "世"（自动按码点）
//
//   关键差异:
//   C++ 和 Go 都允许按字节索引，但都不做 UTF-8 验证，
//   轻松产生无效字符。Rust 直接禁止，强迫你明确意图。
//
// 【如何修复】
//   方案1: s.chars().nth(i) —— 按字符索引（O(n)）
//   方案2: s.as_bytes()[i] —— 按字节索引
//   方案3: &s[i..j] —— 按字节切片

fn main() {
    let s = String::from("Hello世界");

    // ❌ 编译错误: the type `str` cannot be indexed by `{integer}`
    // let first_char = s[0];

    // ✅ 按字节索引（得到 u8）
    let first_byte = s.as_bytes()[0];
    println!("  s.as_bytes()[0] = {first_byte} (0x{:02x})", first_byte);

    // ✅ 按字符索引（得到 char，O(n)）
    if let Some(first_char) = s.chars().nth(0) {
        println!("  s.chars().nth(0) = '{first_char}'");
    }
    if let Some(sixth_char) = s.chars().nth(5) {
        println!("  s.chars().nth(5) = '{sixth_char}'");
    }

    // ✅ 按字节范围切片（得到 &str，O(1)）
    println!("  &s[0..5] = \"{}\"", &s[0..5]);
    // WARNING: &s[0..6] 会 panic！—— 切割了 UTF-8 字符 '世' 的中间

    println!();
    println!("对比: C++ s[0] 返回 char 但不保证 UTF-8 有效性");
    println!("      Go s[0] 返回 byte 同样不保证");
    println!("      Rust: 强制显式选择字节 vs 字符 vs 切片");
}
