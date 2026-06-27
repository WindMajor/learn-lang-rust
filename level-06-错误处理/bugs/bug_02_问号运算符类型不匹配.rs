// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: ? 运算符类型不匹配 —— 错误类型不同              ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   ? 运算符要求当前函数的错误类型与传播的错误类型兼容。
//   如果类型不匹配，编译错误。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_问号运算符类型不匹配.rs`:
//
//   error[E0277]: `?` couldn't convert the error to `ParseIntError`
//     --> bug_02_问号运算符类型不匹配.rs:XX:XX
//      |
//   XX |     let content = fs::read_to_string(path)?;
//      |                   ------------------------- the trait
//      |                   `From<io::Error>` is not implemented for `ParseIntError`
//      |
//      = note: the question mark operation (`?`) implicitly performs
//              a conversion on the error value using the `From` trait
//      = help: the following other types implement trait `From<T>`:
//                <ParseIntError as From<...>>
//   help: consider using `.map_err` to convert the error
//
// 【为什么会这样】
//   ? 运算符自动调用 .into() 转换错误类型。如果目标错误类型没有
//   实现 From<源错误类型>，编译失败。
//
// 【如何修复】
//   方案1: 使用 Box<dyn Error> 作为返回类型（类型擦除）
//   方案2: 使用 .map_err() 手动转换
//   方案3: 自定义错误类型并实现 From trait

use std::fs;
use std::num::ParseIntError;

fn main() {
    // ❌ 编译错误: ? couldn't convert io::Error to ParseIntError
    // fn read_and_parse_wrong(path: &str) -> Result<i32, ParseIntError> {
    //     let content = fs::read_to_string(path)?; // io::Error → ? → ParseIntError 失败
    //     let num: i32 = content.trim().parse()?;  // ParseIntError — 这个可以
    //     Ok(num)
    // }

    // ✅ 方案1: 使用 Box<dyn Error>（类型擦除）
    fn read_and_parse_erased(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?; // io::Error → Box<dyn Error> ✅
        let num: i32 = content.trim().parse()?; // ParseIntError → Box<dyn Error> ✅
        Ok(num)
    }
    match read_and_parse_erased("nonexistent.txt") {
        Ok(n) => println!("{n}"),
        Err(e) => println!("错误: {e}"),
    }

    // ✅ 方案2: 使用 map_err 手动转换
    fn read_and_parse_convert(path: &str) -> Result<i32, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("读取失败: {e}"))?;
        let num: i32 = content.trim().parse().map_err(|e| format!("解析失败: {e}"))?;
        Ok(num)
    }
    match read_and_parse_convert("nonexistent.txt") {
        Ok(n) => println!("{n}"),
        Err(e) => println!("{}", e),
    }

    println!();
    println!("核心: ? 运算符自动调用 From trait 转换，需要兼容的错误类型");
    println!("Box<dyn Error> 是快速原型中最常用的返回类型");
    println!("生产代码推荐: thiserror 或 anyhow crate");
}
