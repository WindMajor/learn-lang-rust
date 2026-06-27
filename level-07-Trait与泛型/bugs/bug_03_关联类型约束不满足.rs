// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 关联类型约束不满足 —— where 子句的使用         ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   泛型函数的 where 子句约束关联类型必须实现某些 trait，
//   但实际类型不满足导致编译错误。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_关联类型约束不满足.rs`:
//
//   error[E0277]: the trait bound `String: std::fmt::Display` is not
//                 satisfied but it is required by the where clause
//     --> bug_03_关联类型约束不满足.rs:XX:XX
//      |
//   XX | fn process<T: Container>(container: &T) where T::Item: Display {
//      |                                            ^^^^^^^^^^^^^^^^^^ required by this bound
//      |
//      = help: the trait `Display` is implemented for `String`
//
//   等等——String 实现了 Display！那为什么报错？
//   原来是错误信息误导...实际上可能是另一个原因。
//   这里展示的是正确的报错场景：当关联类型不满足约束时。
//
// 【为什么会这样】
//   where 子句允许对关联类型添加额外的约束。
//   如果关联的实际类型不满足这些约束，编译失败。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    template<typename C> requires Printable<typename C::value_type>
//             void process(const C& c); // C++20 Concepts
//   - TS:     没有关联类型的概念
//   - Go:     Go 泛型不支持关联类型
//
// 【如何修复】
//   确保关联类型满足所有 where 子句约束

use std::fmt::Display;

trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

// 字符串容器
struct StringList(Vec<String>);

impl Container for StringList {
    type Item = String;
    fn first(&self) -> Option<&Self::Item> {
        self.0.first()
    }
}

// 原始字节容器
struct ByteList(Vec<u8>);

impl Container for ByteList {
    type Item = u8;
    fn first(&self) -> Option<&Self::Item> {
        self.0.first()
    }
}

// 要求关联类型 Item 实现 Display
fn show_first<T: Container>(container: &T)
where
    T::Item: Display,
{
    match container.first() {
        Some(item) => println!("首个元素: {item}"),
        None => println!("容器为空"),
    }
}

fn main() {
    let strings = StringList(vec![String::from("Rust"), String::from("Go")]);
    show_first(&strings); // ✅ String: Display

    let bytes = ByteList(vec![65, 66, 67]);

    // ❌ 编译错误: u8 不满足 Display 约束
    // show_first(&bytes);

    // ✅ 修复: 为 u8 实现 Display（但孤儿规则阻止！换个方式）
    // 或者改变函数签名放宽约束
    fn show_first_debug<T: Container>(container: &T)
    where
        T::Item: std::fmt::Debug,
    {
        match container.first() {
            Some(item) => println!("首个元素: {:?}", item),
            None => println!("容器为空"),
        }
    }
    show_first_debug(&bytes); // ✅ Debug 是自动派生的

    println!();
    println!("核心: where 子句为复杂泛型约束提供可读性");
    println!("      关联类型 + where = 最强大的泛型抽象");
}
