// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: Rc<T> 不能跨线程 —— 非 Send 类型                ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   Rc<T> 不是 Send （引用计数非原子），不能传递到另一个线程。
//   Rust 编译器在编译期就阻止这种错误！
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_Rc跨线程.rs`:
//
//   error[E0277]: `Rc<i32>` cannot be sent between threads safely
//     --> bug_01_Rc跨线程.rs:XX:XX
//      |
//   XX |     thread::spawn(move || {
//      |     ------------- ^^^^^^^ `Rc<i32>` cannot be sent between
//      |     |                     threads safely
//      |     required by a bound introduced by this call
//      |
//      = help: the trait `Send` is not implemented for `Rc<i32>`
//      = note: use `Arc<i32>` instead of `Rc<i32>`
//
// 【为什么会这样】
//   Rc 使用非原子操作更新引用计数，多线程并发修改会导致
//   引用计数错乱 → use-after-free 或 double-free。
//   Rust 的 Send/Sync trait 在编译期阻止了这种 UB。
//
// 【在 C++/Go 中对应的行为】
//   - C++:    shared_ptr<int> p = make_shared<int>(42);
//             thread t([p]() { ... }); // 编译通过！
//             // shared_ptr 使用原子引用计数，安全但隐式开销
//             // 你不知道你的智能指针是不是原子的
//
//   - Go:     Go 没有 Rc，所有数据都可以通过 channel 传递
//
//   关键差异:
//   C++ shared_ptr 把单/多线程的开销都统一为原子操作
//   Rust 让你选择: Rc(非原子,快) vs Arc(原子,线程安全)
//   你不会多付一毫秒的原子操作开销如果你不需要

use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new(42);

    // ❌ error[E0277]: Rc<i32> 不是 Send
    // let handle = thread::spawn(move || {
    //     println!("{}", rc);
    // });

    // ✅ 修复: 使用 Arc 替代 Rc
    use std::sync::Arc;
    let arc = Arc::new(42);
    let arc_clone = Arc::clone(&arc);

    let handle = thread::spawn(move || {
        println!("子线程: {}", arc_clone);
    });
    handle.join().unwrap();

    println!("主线程: {}", arc);
    println!();
    println!("核心: Rc 不实现 Send — 编译期阻止跨线程使用");
    println!("      需要跨线程? 用 Arc");
    println!("对比: C++ shared_ptr 总是原子的——你无法选择");
    println!("      Rust 让你显式选择: Rc vs Arc");
}
