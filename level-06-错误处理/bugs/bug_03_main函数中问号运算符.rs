// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: main() 函数中使用 ? 运算符 — 返回值类型不对    ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   在 main() 函数中使用 ? 运算符，但 main 返回 () 而非 Result。
//   Rust 不允许 ? 在返回 () 的函数中使用。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_main函数中问号运算符.rs`:
//
//   error[E0277]: the `?` operator can only be used in a function
//                 that returns `Result` or `Option`
//     --> bug_03_main函数中问号运算符.rs:XX:XX
//      |
//   XX |     let file = File::open("hello.txt")?;
//      |                ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?`
//      |                operator in a function that returns `()`
//      |
//      = help: the trait `FromResidual<Result<Infallible, io::Error>>`
//              is not implemented for `()`
//
//   help: consider changing the return type of `main` to
//         `Result<(), Box<dyn std::error::Error>>`
//      |
//   XX | fn main() -> Result<(), Box<dyn std::error::Error>> {
//      |             ++++++++++++++++++++++++++++++++++++++++
//
// 【为什么会这样】
//   ? 本质上是"将错误向上传播"。如果调用者（main）不返回 Result，
//   错误无处可去。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:     main 返回 int，不能 throw（会调用 std::terminate）
//   - Go:      main 不返回 error，用 log.Fatal/os.Exit 替代
//   - TS:      main 中 await 需要顶层 await，否则用 .catch()
//
// 【如何修复】
//   方案1: main 返回 Result<(), Box<dyn Error>>
//   方案2: 在 main 中用 match/unwrap 替代 ?

fn main() {
    // ❌ 编译错误: ? in function returning ()
    // let content = std::fs::read_to_string("hello.txt")?;

    // ✅ 方案1: 改用 match
    match std::fs::read_to_string("hello.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => println!("错误: {e}"),
    }

    // ✅ 方案2: 使用 let-else (Rust 1.65+)
    let Ok(content) = std::fs::read_to_string("hello.txt") else {
        println!("读取文件失败");
        return;
    };
    println!("{}", content);

    println!();
    println!("═══ 推荐写法 ═══");
    println!("  如果 main 中需要 ? 运算符:");
    println!();
    println!("  fn main() -> Result<(), Box<dyn std::error::Error>> {");
    println!("      let content = std::fs::read_to_string(\"hello.txt\")?;");
    println!("      println!(\"{content}\");");
    println!("      Ok(())");
    println!("  }");
    println!();
    println!("  这样，main 也可以优雅地传播错误了！");
}

// ✅ 正确写法示例（取消注释以测试）:
// fn main_alt() -> Result<(), Box<dyn std::error::Error>> {
//     let content = std::fs::read_to_string("hello.txt")?;
//     println!("{content}");
//     Ok(())
// }
