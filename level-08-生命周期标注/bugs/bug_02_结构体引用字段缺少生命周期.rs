// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 结构体引用字段缺少生命周期标注                  ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   结构体包含引用字段时，必须标注生命周期参数。
//   Rust 编译期需要知道:"这个引用字段不能比结构体本身活得更短"。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_结构体引用字段缺少生命周期.rs`:
//
//   error[E0106]: missing lifetime specifier
//     --> bug_02_结构体引用字段缺少生命周期.rs:XX:XX
//      |
//   XX |     part: &str,
//      |           ^ expected named lifetime parameter
//      |
//   help: consider introducing a named lifetime parameter
//      |
//   XX ~ struct Excerpt<'a> {
//   XX ~     part: &'a str,
//
// 【为什么会这样】
//   Rust 不允许"裸"引用字段出现在结构体中。
//   必须通过生命周期约束来确保引用在结构体整个生命周期内都有效。
//
// 【在 C++ 中对应的行为】
//   struct Excerpt {
//     const char* part; // 编译通过！无任何警告！
//   };
//   // 你可以这样构造：
//   //   char local[] = "hello";
//   //   Excerpt e{local};
//   //   delete[] local;
//   //   e.part[0]; // UB — 使用已释放内存
//
// 【如何修复】
//   添加生命周期参数

fn main() {
    // ❌ 编译错误
    // struct Excerpt {
    //     part: &str,  // error[E0106]: missing lifetime specifier
    // }

    // ✅ 修复: 添加生命周期参数
    struct Excerpt<'a> {
        part: &'a str,
    }

    let text = String::from("Rust 生命周期标注");
    let excerpt = Excerpt {
        part: &text,
    };
    println!("摘录: {}", excerpt.part);

    // 但是：
    // let dangling: Excerpt<'static>;
    // {
    //     let local = String::from("local");
    //     dangling = Excerpt { part: &local };
    //     // ❌ local 活得不够久！
    // }

    println!();
    println!("核心: 结构体中引用字段必须标注生命周期");
    println!("对比: C++ 允许裸指针字段 —— 内存安全靠程序员记忆");
    println!("      Rust 强制声明生命周期关系 —— 编译器替你记忆");
}
