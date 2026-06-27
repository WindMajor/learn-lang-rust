// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 错误的字符串切片边界 —— 在 UTF-8 字符中间切割  ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   切片操作 &s[i..j] 的边界必须落在 UTF-8 字符边界上，
//   否则会 panic! 而不是静默产生无效 UTF-8。
//   Rust 宁愿崩溃也不接受无效的字符串。
//
// 【运行时会报什么错】（这是运行时 panic，不是编译期错误）
//   执行 `rustc bug_02_错误的字符串切片边界.rs && ./bug_02_xxx`:
//
//   thread 'main' panicked at bug_02_错误的字符串切片边界.rs:XX:
//   byte index 1 is not a char boundary; it is inside '世'
//   (bytes 0..3) of `Hello世界`
//
// 【为什么会这样】
//   "Hello世界" 的 UTF-8 字节: H(1) e(1) l(1) l(1) o(1) 世(3) 界(3)
//   字节索引:  0→H  1→e  2→l  3→l  4→o  5/6/7→世  8/9/10→界
//   字符边界: 0, 1, 2, 3, 4, 5, 8, 11
//   &s[0..6] 的结束位置 6 落在 "世" 的中间（5~8），非法！
//
// 【在 C++/Go 中对应的行为】
//   - C++:    string_view sv(s.data() + 0, 6); // 包含"世"的3个字节 + 1个无效片段
//             // 编译器不阻止，静默产生无效 UTF-8
//
//   - Go:     s[0:6]  // 包含不完整的 UTF-8 字符
//             // Go 不验证切片边界的 UTF-8 有效性
//             // 输出的字符串可能显示乱码
//
//   关键差异:
//   Rust 的选择: "要么给我有效的 UTF-8 字符串，要么崩溃——绝不需要无效的"
//   这确保了 String/&str 类型的"UTF-8 有效"不变量始终成立
//
// 【如何修复】
//   方案1: 确定边界在 UTF-8 字符边界上
//   方案2: 使用 floor_char_boundary() 方法（Rust 1.79+）安全切片
//   方案3: 先计算字符边界再切片

fn main() {
    let s = String::from("Hello世界");

    println!("═══ 字符串字节分析 ═══");
    println!("  s = {:?}", s);
    println!("  s.len() = {} 字节", s.len());
    println!("  s.chars().count() = {} 字符", s.chars().count());
    println!();

    // 打印每个字符的字节范围和长度
    println!("  字符  字节索引  字节长度");
    for (i, c) in s.char_indices() {
        println!("  '{c}'  {i}      {}", c.len_utf8());
    }
    println!();

    // ✅ 安全切片：边界在字符边界上
    println!("  &s[0..5] = {:?}", &s[0..5]);   // "Hello" — 边界在 5 (H=1,e=1,l=1,l=1,o=1)
    println!("  &s[5..]  = {:?}", &s[5..]);    // "世界"  — 5 是 "世" 的起始

    // ❌ 以下会在运行时 panic:
    // println!("{:?}", &s[0..6]); // panic! 6 落在 "世" 的中间

    // ✅ 安全方案：使用 floor_char_boundary（Rust 1.79+）
    let end = (0..7).filter_map(|i| {
        if s.is_char_boundary(i) { Some(i) } else { None }
    }).last().unwrap_or(0);
    println!("  &s[0..{end}] = {:?}", &s[0..end]); // 自动回退到最近的字符边界

    // ✅ 更好的方式：遍历字符收集
    let safe_sub: String = s.chars().take(6).collect();
    println!("  前6个字符: {:?}", safe_sub);

    println!();
    println!("核心教训: Rust 保证 String/&str 始终是有效 UTF-8");
    println!("对比: C++ string 可以包含任意字节，不保证 UTF-8");
    println!("      C++20 char8_t 试图改善但生态不完善");
}
