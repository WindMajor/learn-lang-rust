// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: Trait 未实现 —— 泛型约束未满足                   ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   泛型函数要求 T: SomeTrait，但传入的类型没有实现该 trait。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_Trait未实现.rs`:
//
//   error[E0277]: the trait bound `i32: CustomDisplay` is not satisfied
//     --> bug_02_Trait未实现.rs:XX:XX
//      |
//   XX |     show(42);
//      |     ---- ^^ the trait `CustomDisplay` is not implemented for `i32`
//      |     |
//      |     required by a bound introduced by this call
//      |
//      = help: the trait `CustomDisplay` is not implemented for `i32`
//
// 【为什么会这样】
//   Rust 泛型是编译期检查的。函数签名中的 T: Trait 是契约，
//   编译器在调用点检查是否满足。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    template<typename T> void show(const T& val) {
//               val.display();  // 编译错误时错误信息又长又臭
//             }
//             SFINAE/Concepts 之前，C++ 模板的错误信息是灾难级的
//             C++20 Concepts 提供了类似 Rust trait bound 的能力
//
//   - Go:     func show(v CustomDisplay) { v.Display() }
//             Go 是动态检查（接口值包含方法表），编译时不报错
//
//   - TS:     function show<T extends CustomDisplay>(val: T) {}
//             编译期检查，但类型擦除后运行时无保证
//
// 【如何修复】
//   方案1: 为 i32 实现 CustomDisplay trait
//   方案2: 改变函数签名以接受不要求 trait 的类型

trait CustomDisplay {
    fn display(&self) -> String;
}

fn show<T: CustomDisplay>(val: &T) {
    println!("{}", val.display());
}

fn main() {
    // ❌ 编译错误: i32 没有实现 CustomDisplay
    // show(&42);

    // ✅ 修复: 为 i32 实现 CustomDisplay
    impl CustomDisplay for i32 {
        fn display(&self) -> String {
            format!("整数: {}", self)
        }
    }

    show(&42); // ✅ 现在可以了

    // 也可以为其他类型实现
    impl CustomDisplay for f64 {
        fn display(&self) -> String {
            format!("浮点: {:.2}", self)
        }
    }
    show(&3.14);

    println!();
    println!("对比: C++ 模板错误信息（C++17 之前）可能长达数百行");
    println!("      Rust trait bound 的错误信息精准指出缺失的 trait");
    println!("      C++20 Concepts 终于赶上了 Rust 2015 的错误信息质量");
}
