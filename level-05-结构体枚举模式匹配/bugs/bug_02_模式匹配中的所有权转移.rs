// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 模式匹配中的所有权转移 —— match 拿走所有权       ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   match 解构 enum 时，变体内的 String 被移动，导致之后不能使用原值。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_模式匹配中的所有权转移.rs`:
//
//   error[E0382]: use of partially moved value: `msg`
//     --> bug_02_模式匹配中的所有权转移.rs:XX:XX
//      |
//   XX |     let msg = Message::Write(String::from("hello"));
//      |         --- move occurs because `msg` has type `Message`,
//      |             which does not implement the `Copy` trait
//   XX |     match msg {
//   XX |         Message::Write(s) => println!("写入: {s}"),
//      |                         - `msg` partially moved here
//   ...   }
//   XX |     println!("{:?}", msg);
//      |                      ^^^ value borrowed here after partial move
//
// 【为什么会这样】
//   match 解构时如果变体包含拥有所有权的数据（如 String），
//   数据会被移动到绑定变量 s 中。之后 msg 处于"部分移动"状态。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    variant<Quit, string> msg = string("hello");
//             if (holds_alternative<string>(msg)) {
//               auto& s = get<string>(msg); // 引用，所有权不变
//             }
//             // 仍然可以使用 msg
//   - TS:     const msg: { type:'A', data:string } = ...;
//             if (msg.type === 'A') { const s = msg.data; }
//             // msg 仍然可以使用（引用语义）
//
// 【如何修复】
//   方案1: match 时使用 ref 关键字借用而非移动
//   方案2: 先 match 获取结果，再让 msg 被消耗

fn main() {
    // ❌ 错误写法: match 移动了数据
    // let msg = write_message("hello");
    // match msg {
    //     Write(s) => println!("写入: {s}"), // s 取走所有权
    // }
    // println!("{:?}", msg); // ❌ msg 部分移动

    // ✅ 方案1: match 时用 ref 借用
    #[derive(Debug)]
    enum Msg {
        Write(String),
        Quit,
    }

    let msg = Msg::Write(String::from("hello"));
    match &msg {
        // &msg 借用整个 enum
        Msg::Write(s) => println!("写入: {s}"), // s 是 &String
        Msg::Quit => println!("退出"),
    }
    println!("msg 仍可用: {:?}", msg); // ✅

    // ✅ 方案2: match 后不再使用原值
    let msg2 = Msg::Write(String::from("world"));
    let result = match msg2 {
        Msg::Write(s) => s, // 所有权从 msg2 转移到 result
        Msg::Quit => String::new(),
    };
    println!("result: {result}");
    // msg2 之后不再使用

    println!();
    println!("核心: match 解构时默认移动所有权，用 & 或 ref 避免");
    println!("对比: C++ variant get<> 返回引用，TS 是引用语义");
    println!("      Rust 需要你显式选择：所有权移动 vs 借用");
}
