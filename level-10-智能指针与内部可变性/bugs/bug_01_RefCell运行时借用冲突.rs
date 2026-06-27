// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: RefCell 运行时借用冲突 —— 同时 borrow 和      ║
// ║          borrow_mut 导致 panic                            ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   RefCell 的 borrow() 和 borrow_mut() 在运行时检查借用规则。
//   如果同时持有不可变借用和可变借用，运行时 panic。
//
// 【运行时会报什么错】
//   执行 `rustc bug_01_RefCell运行时借用冲突.rs && ./bug_01_xxx`:
//
//   thread 'main' panicked at .../src/cell.rs:XXX:
//   already borrowed: BorrowMutError
//
// 【为什么会这样】
//   RefCell 将借用检查从编译期推迟到运行时。
//   如果你在运行时不遵守借用规则，程序 panic。
//
// 【在 C++ 中对应的行为】
//   C++ 没有 RefCell 的概念 —— 你可以在有 const 引用的情况下
//   同时修改数据（using const_cast），编译器不阻止也不检查。
//   Rust RefCell 至少在运行时告诉你"你做错了"而不是 UB。
//
// 【如何修复】
//   确保 borrow() 在 borrow_mut() 之前结束

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    // ❌ 运行时 panic: already borrowed: BorrowMutError
    // let r1 = data.borrow();
    // let mut r2 = data.borrow_mut(); // PANIC!

    // ✅ 修复: 先让 borrow 结束
    {
        let r1 = data.borrow();
        println!("r1: {:?}", r1);
    } // r1 在这里释放

    let mut r2 = data.borrow_mut(); // ✅ 现在可以可变借用
    r2.push(4);
    println!("r2: {:?}", r2);

    println!();
    println!("核心: RefCell 运行时检查借用规则，违反则 panic");
    println!("对比: C++ 没有等价物——const_cast 可以随时绕过");
    println!("      编译期借用检查更安全(Level 03)，RefCell 是逃生舱");
}
