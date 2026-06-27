// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: unwrap() 滥用导致 panic!                        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   对 Option::None 或 Result::Err 调用 unwrap()，运行时 panic。
//   这是 Rust 新手最常见的运行时错误（编译期不报错）。
//
// 【运行时会报什么错】
//   执行 `rustc bug_01_unwrap滥用导致panic.rs && ./bug_01_xxx`:
//
//   thread 'main' panicked at bug_01_unwrap滥用导致panic.rs:XX:
//   called `Option::unwrap()` on a `None` value
//
//   如果使用 expect():
//
//   thread 'main' panicked at bug_01_unwrap滥用导致panic.rs:XX:
//   应该存在: called `Option::unwrap()` on a `None` value
//
// 【为什么会这样】
//   unwrap() 和 expect() 是"我知道这里不可能是 None/Err"的断言。
//   当假设错误时，程序 panic。生产代码中应该避免使用 unwrap。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    auto v = opt.value(); // C++17 optional::value()
//             // 如果 optional 为空，抛 std::bad_optional_access！
//             // 相当于运行时错误（但可以被 catch 捕获）
//
//   - Go:     v := m["key"]  // 如果 key 不存在，返回零值（静默！）
//             v, ok := m["key"] // 安全方式
//
//   - TS:     const v = obj!.prop  // non-null assertion
//             // 如果 obj 是 null，运行时 TypeError: Cannot read properties of null
//
// 【如何修复】
//   方案1: 使用 match 安全处理
//   方案2: 使用 unwrap_or / unwrap_or_else 提供默认值
//   方案3: 使用 ? 运算符向上传播错误
//   方案4: 确实确定时使用 expect() 并写清楚不变量（至少比 unwrap 好）

fn main() {
    // ─── 错误演示 ───
    {
        println!("═══ Option unwrap 错误 ───");
        let x: Option<i32> = None;
        // ❌ 运行时 panic: called `Option::unwrap()` on a `None` value
        // let val = x.unwrap();

        // ✅ unwrap_or: 提供默认值
        let val = x.unwrap_or(42);
        println!("  unwrap_or(42) = {val}");
    }

    // ─── expect 错误 ───
    {
        println!();
        println!("═══ Result expect 错误 ───");

        fn find_config(key: &str) -> Result<String, &str> {
            Err("配置文件未找到")
        }

        // ❌ 运行时 panic: "必须找到数据库配置: ..."
        // let db_url = find_config("database_url")
        //     .expect("必须找到数据库配置");

        // ✅ 用 match 安全处理
        match find_config("database_url") {
            Ok(url) => println!("  数据库: {url}"),
            Err(e) => println!("  警告: {e}，使用默认配置"),
        }
    }

    println!();
    println!("═══ 使用 unwrap 的时机 ═══");
    println!("  ✅ 测试代码: #[test] fn test() { let x = ... .unwrap(); }");
    println!("  ✅ 原型代码: 快速验证概念");
    println!("  ✅ 逻辑上不可能失败: (0..10).last().unwrap()  — 范围非空");
    println!("  ❌ 生产代码: IO 操作、网络请求、用户输入");
    println!();
    println!("对比: Go 中 val, _ := fn() 忽略 error——更危险，静默错误");
    println!("      Rust unwrap 至少有明确的 panic 信息");
}
