// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 非穷尽匹配 —— match 遗漏变体                      ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   match 表达式没有覆盖 enum 的所有变体，编译器报错。
//   Rust 的穷尽检查是编译期安全特性，来自 TS/Java/C/C++ 的开发者
//   一开始会觉得"烦人"，但很快会发现这是救命特性。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_非穷尽匹配.rs`:
//
//   error[E0004]: non-exhaustive patterns: `ChangeColor(_, _, _)` not covered
//     --> bug_01_非穷尽匹配.rs:XX:XX
//      |
//   XX | /     match msg {
//   XX | |         Message::Quit => "退出",
//   XX | |         Message::Move { .. } => "移动",
//   XX | |         Message::Write(_) => "写入",
//   XX | |     }
//      | |_____^ pattern `ChangeColor(_, _, _)` not covered
//      |
//      = note: the matched value is of type `Message`
//   help: ensure that all possible cases are being handled
//       by adding a match arm with a wildcard pattern
//      |
//   XX +         Message::ChangeColor(_, _, _) => todo!(),
//
// 【为什么会这样】
//   Rust 要求 match 覆盖所有可能情况。这是代数数据类型的核心价值：
//   编译器告诉你"你忘记处理这种情况了"——在你推向生产之前。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    switch(val) { case A: ..., case B: ... }
//             // 没有 default 也编译通过 —— 运行时到 C 分支则 fallthrough
//             // 或什么都不做（如果有 break），产生静默逻辑错误
//
//   - Go:     switch val { case A: ..., case B: ... }
//             // 不影响编译，运行时什么都不做
//
//   - TS:     type Msg = { type:'A' } | { type:'B' }
//             function handle(m: Msg) {
//               switch(m.type) {
//                 case 'A': return 'a'; // 只处理 A，不行！
//               }
//             }
//             // 需要 noImplicitReturns + never 类型才能检测
//             // 但不强制，容易遗漏
//
// 【如何修复】
//   方案1: 补全所有分支
//   方案2: 使用通配符 _ => { ... }

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let msg = Message::ChangeColor(255, 0, 0);

    // ❌ 编译错误: non-exhaustive patterns
    // let desc = match msg {
    //     Message::Quit => "退出",
    //     Message::Move { .. } => "移动",
    //     Message::Write(_) => "写入",
    //     // 忘记 ChangeColor!
    // };

    // ✅ 修复: 补全
    let desc = match msg {
        Message::Quit => "退出",
        Message::Move { .. } => "移动",
        Message::Write(_) => "写入",
        Message::ChangeColor(_, _, _) => "改变颜色",
    };
    println!("{desc}");

    // ✅ 也可以使用通配符（但如果后续添加变体，编译器不再警告）
    let desc2 = match Message::Write(String::from("hi")) {
        Message::Quit => "退出",
        _ => "其他", // 通配符捕获其余所有
    };
    println!("{desc2}");

    println!();
    println!("对比: C++ switch 忘记 case 也不报错 —— 静默 bug");
    println!("      TS discriminated union 有穷尽检查但依赖 never 类型");
    println!("      Rust: 强制穷尽 —— 这是编译器帮你写代码");
}
