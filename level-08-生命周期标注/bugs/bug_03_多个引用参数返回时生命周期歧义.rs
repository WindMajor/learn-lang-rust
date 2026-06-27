// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 多个引用参数返回时生命周期歧义                   ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   函数有多个引用参数，返回其中一个引用。
//   编译器无法推断返回值的生命周期应该关联哪个输入参数。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_多个引用参数返回时生命周期歧义.rs`:
//
//   error[E0106]: missing lifetime specifier
//     --> bug_03_多个引用参数返回时生命周期歧义.rs:XX:XX
//      |
//   XX | fn pick(x: &str, y: &str, flag: bool) -> &str {
//      |            ----     ----              ^ expected named
//      |                                        lifetime parameter
//      |
//      = help: this function's return type contains a borrowed value,
//              but the signature does not say whether it is borrowed
//              from `x` or `y`
//   help: consider introducing a named lifetime parameter
//      |
//   XX | fn pick<'a>(x: &'a str, y: &'a str, flag: bool) -> &'a str {
//
// 【为什么会这样】
//   生命周期省略规则只适用于"单个输入生命周期"的情况。
//   两个及以上输入时，编译器不知道该关联哪个。
//
// 【在 C++ 中对应的行为】
//   const string& pick(const string& x, const string& y, bool flag) {
//     return flag ? x : y;
//   }
//   // 编译通过！无任何警告！
//   // 调用 pick(temp1, temp2, true) 时返回的引用可能指向临时对象
//
// 【如何修复】
//   手动标注生命周期关系

fn main() {
    // ❌ 编译错误: E0106
    // fn pick(x: &str, y: &str, flag: bool) -> &str {
    //     if flag { x } else { y }
    // }

    // ✅ 修复: 标注生命周期
    fn pick<'a>(x: &'a str, y: &'a str, flag: bool) -> &'a str {
        if flag {
            x
        } else {
            y
        }
    }

    let s1 = String::from("first");
    let s2 = String::from("second");
    let result = pick(&s1, &s2, true);
    println!("选择了: {result}");

    // 也可以标注不同的生命周期（如果只返回其中一个）
    fn first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x // 明确只返回 x
    }
    let result2 = first(&s1, &s2);
    println!("first: {result2}");

    println!();
    println!("核心: 两个及以上引用参数时，编译器不知道返回哪个");
    println!("     必须手动标注生命周期关系");
    println!("对比: C++ 可以编译通过但产生悬垂引用——UB 无声无息");
}
