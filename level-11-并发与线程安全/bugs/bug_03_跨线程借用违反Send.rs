// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 跨线程借用违反 Send —— RefCell 非 Sync         ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   尝试在线程间共享 RefCell 的引用，但 RefCell 不实现 Sync。
//   RefCell 的运行时借用检查不是线程安全的。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_跨线程借用违反Send.rs`:
//
//   error[E0277]: `RefCell<i32>` cannot be shared between threads safely
//     --> bug_03_跨线程借用违反Send.rs:XX:XX
//      |
//   XX |     let handle = thread::spawn(move || {
//      |                   ------------- ^^^^^^^ `RefCell<i32>` cannot
//      |                   |                      be shared...
//      |                   required by a bound ...
//      |
//      = help: within `[closure@...]`, the trait `Sync` is not
//              implemented for `RefCell<i32>`
//      = note: if you want to do aliasing and mutation between
//              multiple threads, use `std::sync::RwLock` or
//              `std::sync::Mutex` instead
//
// 【为什么会这样】
//   RefCell 使用 Cell 内部的非原子计数器追踪 borrow/borrow_mut。
//   多线程并发访问会破坏计数器 → 数据竞争。
//
// 【如何修复】
//   用 Mutex 或 RwLock 替代 RefCell 实现线程安全的内部可变性

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // ❌ RefCell 不能跨线程共享
    // let data = RefCell::new(0);
    // let handle = thread::spawn(move || {
    //     *data.borrow_mut() += 1;
    // });

    // ✅ 修复: 用 Mutex 替代 RefCell
    {
        let data = Arc::new(Mutex::new(0));
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut guard = data_clone.lock().unwrap();
            *guard += 1;
            println!("子线程: {}", *guard);
        });

        handle.join().unwrap();
        println!("主线程: {}", *data.lock().unwrap());
    }

    println!();
    println!("核心: RefCell 不实现 Sync，不能跨线程共享");
    println!("      跨线程内部可变性 → Mutex/RwLock");
    println!("对比: C++ 没有 RefCell 概念");
    println!("      Go 的并发安全靠 -race flag 运行时检测");
}
