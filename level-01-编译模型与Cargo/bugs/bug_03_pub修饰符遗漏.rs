// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: pub 修饰符遗漏 —— 模块内类型字段默认私有        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   在模块中定义了一个结构体，结构体本身是 pub 的，但其中的字段没有 pub。
//   外部模块可以"看到"这个类型，但无法构造它（因为没有公开字段）。
//   这是 Rust 独有的设计：类型可见性和成员可见性是分开的。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_pub修饰符遗漏.rs`:
//
//   error[E0451]: field `name` of struct `Person` is private
//     --> bug_03_pub修饰符遗漏.rs:XX:XX
//      |
//   XX |     let p = my_mod::Person { name: String::from("Alice"), age: 30 };
//      |                              ^^^^ private field
//
// 【为什么会这样】
//   Rust 设计哲学：类型公开 ≠ 字段公开。你可以选择性暴露字段，
//   这正是封装的基础。在 C++/Java 中你通过 private 关键字实现，
//   在 Rust 中通过"不加 pub"实现。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:  struct 默认 public，class 默认 private（你用 class 会同样遇到此错）
//   - Go:   结构体字段首字母大写=public，小写=private（与 Rust 同哲理）
//   - TS:   class 成员默认 public（与 Rust 相反！）
//   关键差异: struct 在 C++ 中是 public-default，在 Rust 中是 private-default
//
// 【如何修复】
//   给需要公开的字段加 pub 前缀:
//   pub name: String,
//   或者提供构造函数（new 关联函数）来间接设置私有字段

mod my_mod {
    /// 人员信息结构体
    pub struct Person {
        name: String,  // ❌ 字段是私有的！外部不能直接访问
        age: u32,      // ❌ 同样私有
    }

    impl Person {
        /// 构造函数 —— 提供受控的创建方式
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }

        /// 公有 getter
        pub fn name(&self) -> &str {
            &self.name
        }

        /// 公有 getter
        pub fn age(&self) -> u32 {
            self.age
        }
    }
}

fn main() {
    // ❌ 编译错误: error[E0451]: field `name` of struct `Person` is private
    // let p = my_mod::Person {
    //     name: String::from("Alice"),
    //     age: 30,
    // };

    // ✅ 修复: 通过构造函数创建
    let p = my_mod::Person::new(String::from("Alice"), 30);
    println!("name: {}, age: {}", p.name(), p.age());
    // 注意: p.name 仍然不能直接访问，必须通过 p.name() 方法

    println!("注: 取消注释结构体字面量创建代码可触发 E0451 错误");
    println!("对比: C++ struct 默认 public，class 默认 private");
    println!("      Rust struct 统一默认 private —— 更安全的默认值");
}
