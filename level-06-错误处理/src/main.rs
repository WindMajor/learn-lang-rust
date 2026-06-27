// ================================================================
// Level 06: 错误处理 —— Result 与 Option
// 目标: 理解 Result/Option/?/panic!/unwrap 的设计哲学
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 理念        │ Rust         │ Go           │ Java         │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 可恢复错误  │ Result<T,E>  │ (T, error)   │ Checked Ex  │
// │ 不可恢复错误│ panic!       │ panic()      │ Error/Runtime│
// │ 传播机制    │ ? 运算符     │ if err!=nil  │ throws       │
// │ 强制处理    │ 编译期强制   │ 可忽略(_)    │ 声明+try    │
// │ 空值处理    │ Option<T>    │ nil 指针     │ Optional<T> │
// └────────────┴──────────────┴──────────────┴──────────────┘

use std::fs;
use std::io;
use std::num::ParseIntError;

// WHAT: Option 和 Result 本身就是普通枚举！
// 这不是内置语法，就是标准库定义的 enum
//
// enum Option<T> { Some(T), None }
// enum Result<T, E> { Ok(T), Err(E) }
//
// CONTRAST:
//   Go:  error 是内置接口 interface { Error() string }
//   TS:  Promise 是内置的异步抽象
//   Java: Throwable 是类层级

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 06: 错误处理                  ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. Option<T> 的使用 ───
    println!("━━━ 1. Option<T>：替代 null ━━━");
    {
        // WHAT: Option 强迫你在编译期处理"可能存在也可能不存在"的情况
        // WHY: Rust 没有 null！这是 Tony Hoare 的"十亿美元错误"的解决方案
        // CONTRAST:
        //   - TS: string | null —— 需要显式 null check（strictNullChecks 模式下）
        //   - Kotlin: String? —— 编译期空安全（与 Rust Option 最接近）
        //   - Swift: String? —— Optional（语法糖，底层是 Optional<T> 枚举）
        //   - Go: *string —— 可以 nil，编译器不检查
        //   - Java: @Nullable String —— 注解，非语言特性
        //   - C++: std::optional<T> (C++17)

        fn find_user(id: u32) -> Option<String> {
            match id {
                1 => Some(String::from("Alice")),
                2 => Some(String::from("Bob")),
                _ => None,
            }
        }

        // 方式1: match（最安全，编译器强制覆盖所有情况）
        match find_user(1) {
            Some(name) => println!("  找到用户: {name}"),
            None => println!("  用户不存在"),
        }

        // 方式2: if let
        if let Some(name) = find_user(3) {
            println!("  找到: {name}");
        } else {
            println!("  用户 3 不存在");
        }

        // 方式3: unwrap_or —— 提供默认值
        let name = find_user(5).unwrap_or(String::from("匿名用户"));
        println!("  unwrap_or: {name}");

        // 方式4: 组合子（combinators）—— 函数式风格
        let upper_name = find_user(1)
            .map(|n| n.to_uppercase())
            .unwrap_or_else(|| String::from("未知"));
        println!("  map + unwrap_or_else: {upper_name}");

        // WARNING: unwrap() 在 None 上会 panic!
        // let name = find_user(99).unwrap(); // panic! if None
    }
    println!();

    // ─── 2. Result<T, E> 的使用 ───
    println!("━━━ 2. Result<T, E>：替代异常 ━━━");
    {
        // WHAT: Result 强迫调用者处理成功和失败两种情况
        // CONTRAST:
        //   - Go:    val, err := doSomething()
        //            if err != nil { return err } —— 与 Rust 最接近，但是可选的
        //   - TS:    try { const val = await doSomething() }
        //            catch(e) { ... } —— 异常可在栈上传播
        //   - Java:  方法签名声明 throws，调用者必须 try-catch 或继续 throws
        //   - C++:   try { ... } catch(...) { ... } —— 零成本异常（但实现复杂）

        fn parse_number(s: &str) -> Result<i32, ParseIntError> {
            s.parse() // parse() 返回 Result<i32, ParseIntError>
        }

        // 方式1: match
        match parse_number("42") {
            Ok(n) => println!("  解析成功: {n}"),
            Err(e) => println!("  解析失败: {e}"),
        }

        // 方式2: is_ok / is_err（注意：不会提取值）
        let r = parse_number("42");
        println!("  is_ok: {}", r.is_ok());

        // 方式3: unwrap / expect
        let n = parse_number("100").unwrap(); // 失败则 panic
        println!("  unwrap: {n}");

        let m = parse_number("200")
            .expect("BUG: '200' 应该是合法数字"); // 失败则 panic 并显示自定义消息
        println!("  expect: {m}");

        // 方式4: unwrap_or 提供默认值
        let n = parse_number("abc").unwrap_or(0);
        println!("  unwrap_or 默认: {n}");
    }
    println!();

    // ─── 3. ? 运算符 —— Rust 错误处理的核心 ───
    println!("━━━ 3. ? 运算符：错误传播语法糖 ━━━");
    {
        // WHAT: ? 是 match { Ok(v) => v, Err(e) => return Err(e.into()) } 的语法糖
        // WHY: 减少样板代码，让错误处理链条清晰

        fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
            // ? 运算符自动向上传播错误
            let content = fs::read_to_string(path)?; // io::Error → 自动转换
            let trimmed = content.trim();
            let num: i32 = trimmed.parse()?; // ParseIntError → 自动转换
            Ok(num)
        }

        match read_and_parse("nonexistent.txt") {
            Ok(n) => println!("  读取的数字: {n}"),
            Err(e) => println!("  出错: {e}"),
        }

        // CONTRAST:
        // Go 等价代码:
        //   func readAndParse(path string) (int, error) {
        //     content, err := os.ReadFile(path)
        //     if err != nil { return 0, err }
        //     num, err := strconv.Atoi(strings.TrimSpace(string(content)))
        //     if err != nil { return 0, err }
        //     return num, nil
        //   }
        //
        // Rust ? 消除了 if err != nil { return err } 的样板代码
        // 同时保留了"显式错误传播"的可见性
    }
    println!();

    // ─── 4. panic! vs Result ───
    println!("━━━ 4. panic!：不可恢复 vs 可恢复错误 ━━━");
    {
        // WHAT: panic! 用于不可恢复的错误（类似 Go panic、Java Error）
        // WHY: 有些错误不该被"处理"——数组越界、断言失败等
        //
        // 经验法则:
        // - 用 Result:  文件不存在、网络超时、解析失败（调用者可能想恢复）
        // - 用 panic!:  内部逻辑错误、违反了不变量（无法恢复）
        // - 用 unwrap:  原型/测试中快速写代码
        // - 用 expect:  unwrap 的升级版，带错误信息

        // 示例：数组索引用 get() 返回 Option
        let arr = [10, 20, 30];
        match arr.get(5) {
            Some(v) => println!("  arr[5] = {v}"),
            None => println!("  索引 5 越界（安全处理）"),
        }
        // arr[5]  // 这会 panic!（直接索引）
    }
    println!();

    // ─── 5. 自定义错误类型 ───
    println!("━━━ 5. 自定义错误类型 ━━━");
    {
        // WHAT: 实现 std::error::Error trait 来自定义错误
        // CONTRAST:
        //   - Go:    type MyError struct{...} + func (e *MyError) Error() string
        //   - Java:  class MyException extends Exception
        //   - TS:    class MyError extends Error

        #[derive(Debug)]
        enum MyError {
            NotFound(String),
            PermissionDenied(String),
            IoError(io::Error),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::NotFound(msg) => write!(f, "未找到: {msg}"),
                    MyError::PermissionDenied(msg) => write!(f, "权限不足: {msg}"),
                    MyError::IoError(e) => write!(f, "IO错误: {e}"),
                }
            }
        }

        impl std::error::Error for MyError {}

        fn risky_operation() -> Result<(), MyError> {
            Err(MyError::NotFound(String::from("配置文件")))
        }

        match risky_operation() {
            Ok(()) => println!("  操作成功"),
            Err(e) => println!("  错误: {e}"),
        }
    }
    println!();

    // ─── 6. 对比总结 ───
    println!("━━━ 6. 错误处理跨语言对比 ━━━");
    println!();
    println!("  场景: 打开文件 → 读取内容 → 解析数字 → 计算");
    println!();
    println!("  Rust:");
    println!("    fn process() -> Result<i32, Error> {{");
    println!("      let s = fs::read_to_string(path)?;     // ? 错误传播");
    println!("      let n: i32 = s.trim().parse()?;        // ? 错误传播");
    println!("      Ok(n * 2)");
    println!("    }}");
    println!();
    println!("  Go:");
    println!("    func process() (int, error) {{");
    println!("      b, err := os.ReadFile(path)            // if err != nil");
    println!("      if err != nil {{ return 0, err }}");
    println!("      n, err := strconv.Atoi(              // if err != nil");
    println!("        strings.TrimSpace(string(b)))");
    println!("      if err != nil {{ return 0, err }}");
    println!("      return n * 2, nil");
    println!("    }}");
    println!();
    println!("  TS:");
    println!("    async function process(): Promise<number> {{");
    println!("      const s = await fs.readFile(path, 'utf8'); // throws");
    println!("      const n = parseInt(s.trim());             // NaN 静默 ");
    println!("      return n * 2;                             // NaN*2=NaN");
    println!("    }}");
    println!();
    println!("  Rust 的 ? 运算符 = Go 的样板代码消除 + 编译期强制检查");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 06 通关！继续 Level 07        ║");
    println!("╚══════════════════════════════════════╝");
}
