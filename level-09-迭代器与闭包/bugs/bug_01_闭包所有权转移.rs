// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: FnOnce 闭包多次调用 —— 所有权已被消费           ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   闭包通过 move 获取了某个值的所有权，只能调用一次（FnOnce）。
//   尝试第二次调用时编译器报错。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_闭包所有权转移.rs`:
//
//   error[E0382]: use of moved value: `consumer`
//     --> bug_01_闭包所有权转移.rs:XX:XX
//      |
//   XX |     let consumer = || {
//      |         -------- move occurs because `consumer` has type
//      |                  `[closure@...]`, which does not implement
//      |                  the `Copy` trait
//   XX |         let _s = owned;
//      |                   ----- `owned` moved into closure
//   XX |         "done"
//   XX |     };
//   XX |     consumer();
//      |     -------- `consumer` moved due to this call
//   XX |     consumer();  // ❌ 第二次调用
//      |     ^^^^^^^^ value used here after move
//
//   note: closure cannot be invoked more than once because it moves
//         the variable `owned` out of its environment
//
// 【为什么会这样】
//   闭包消耗了捕获变量的所有权（将 owned 移动进闭包内部）。
//   这使闭包成为 FnOnce —— 只能调用一次。
//
// 【如何修复】
//   方案1: 不获取所有权，改为借用
//   方案2: Clone 数据而不是移动

fn main() {
    let owned = String::from("hello");

    // ❌ FnOnce —— 获取了 owned 的所有权
    // let consumer = || {
    //     let _captured = owned; // 消耗了 owned
    //     "done"
    // };
    // consumer();
    // consumer(); // ❌ 第二次调用失败

    // ✅ 修复1: 借用而不是移动
    let borrower = || {
        let _ref = &owned; // 借用
        format!("借用了: {owned}")
    };
    println!("{}", borrower());
    println!("{}", borrower()); // ✅ 可以多次调用

    // ✅ 修复2: 如果确实需要所有权，clone 后移动
    let owned2 = String::from("world");
    let cloner = {
        let owned2 = owned2; // 移入作用域
        move || {
            // move 关键字: 强制闭包获取所有权
            let _captured = owned2;
            "消耗"
        }
    };
    println!("{}", cloner());
    // cloner(); // FnOnce，只能一次

    println!();
    println!("对比: C++ lambda [=]复制捕获／[&]引用捕获需要手动指定");
    println!("      Rust 编译器自动推断 Fn/FnMut/FnOnce");
}
