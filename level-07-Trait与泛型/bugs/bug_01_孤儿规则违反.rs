// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 孤儿规则违反 —— 为外部类型实现外部 Trait         ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   尝试为外部类型（Vec<T>）实现外部 trait（std::fmt::Display）。
//   孤儿规则禁止这样做——这是 Rust 编译器强制 API 隔离的核心机制。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_孤儿规则违反.rs`:
//
//   error[E0117]: only traits defined in the current crate can be
//                 implemented for arbitrary types
//     --> bug_01_孤儿规则违反.rs:XX:XX
//      |
//   XX | impl std::fmt::Display for Vec<String> {
//      | ^^^^^^^^^^^^^^^^^^^^^^^ -------- `Vec` is not defined
//      | |                      in the current crate
//      | `Display` is not defined in the current crate
//      |
//      = note: define and implement a trait or new type instead
//
// 【为什么会这样】
//   孤儿规则: 你只能在"你的 trait + 别人的类型"或"别人的 trait + 你的类型"
//   这两种组合中实现 trait，不能"别人的 trait + 别人的类型"。
//   这防止了两个不同的 crate 为同一组合提供冲突的实现。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:  没有孤儿规则——你可以为 std::vector 重载 operator<<，
//          但这可能导致 ODR 违规（多个翻译单元定义不同版本）
//
//   - Go:   你不能为内置类型添加方法，但可以为本地类型添加方法
//
//   - TS:   你可以通过 declaration merging 扩展任何类型
//           （运行时可能冲突但编译期不拦）
//
// 【如何修复】
//   方案1（推荐）: newtype 模式——包装外部类型
//   方案2: 定义你自己的 trait 然后为外部类型实现

fn main() {
    // ❌ 编译错误: E0117 orphan rule violation
    // impl std::fmt::Display for Vec<i32> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "[{:?}]", self)
    //     }
    // }

    // ✅ 方案: newtype 模式
    struct MyVec(Vec<i32>);

    impl std::fmt::Display for MyVec {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "我的数组: {:?}", self.0)
        }
    }

    let v = MyVec(vec![1, 2, 3]);
    println!("{v}");

    // ✅ 也可以通过 Deref trait 让包装类型"透明"地使用内部方法
    use std::ops::Deref;
    impl Deref for MyVec {
        type Target = Vec<i32>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    println!("元素数: {}", v.len()); // 自动解引用到 Vec 的方法
    println!("首元素: {}", v[0]);

    println!();
    println!("核心教训:");
    println!("  孤儿规则 = Rust 的 API 边界守门人");
    println!("  newtype 模式 = 零成本包装（编译期优化掉）");
    println!("对比: C++ 允许重叠实现 → ODR 违规的深渊");
    println!("      TS 的 declaration merging 是运行时炸弹");
}
