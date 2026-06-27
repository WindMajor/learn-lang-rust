// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: Deref 强制转换误用 —— Box/Rc 的自动解引用      ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   Box/Rc 通过 Deref trait 自动解引用，在某些场景下行为可能
//   与直觉不符——特别是涉及所有权的时候。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_Deref强制转换误用.rs`:
//
//   error[E0507]: cannot move out of dereference of `Rc<String>`
//     --> bug_03_Deref强制转换误用.rs:XX:XX
//      |
//   XX |     let s = *rc;
//      |             ^^^ move occurs because value has type `String`,
//      |                 which does not implement the `Copy` trait
//
// 【为什么会这样】
//   Rc<T> 实现 Deref<Target=T>，让你用 *rc 访问 T，但：
//   - &*rc 获取 &T 引用 ✅
//   - *rc 试图移动 T ❌ （Rc 共享所有权，不能唯一移动）
//
// 【如何修复】
//   使用 &*rc 或直接调用方法（方法调用自动解引用）

use std::rc::Rc;

fn main() {
    let rc = Rc::new(String::from("hello"));

    // ❌ 不能从 Rc 中"移出"值（共享所有权）
    // let s: String = *rc; // error[E0507]

    // ✅ 获取引用
    let s_ref: &String = &*rc;
    println!("引用: {s_ref}");

    // ✅ 方法调用自动解引用
    println!("长度: {}", rc.len()); // 等价于 (*rc).len()

    // ✅ clone 获取独立副本
    let s: String = (*rc).clone();
    println!("克隆: {s}");

    // ─── Box 可以移出 ───
    let b = Box::new(String::from("world"));
    let s: String = *b; // ✅ Box 单一所有者，可以移出
    println!("Box 移出: {s}");

    println!();
    println!("核心: Rc 不能移出值（共享所有权）");
    println!("      Box 可以（唯一所有权）");
    println!("      方法调用自动 Deref 解引用");
}
