// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: MutexGuard 未释放导致死锁或编译错误             ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   在持有 MutexGuard 时不释放就再次尝试 lock 同一 Mutex。
//   Rust 标准库的 Mutex 不可重入——这会死锁。
//
//   【注意】Rust 标准库的 Mutex 不可重入：
//      lock() 返回 MutexGuard。如果在持有锁时再次 lock 同一个 Mutex，
//      tokio/parking_lot 的 Mutex 会死锁（标准库的也一样）。
//      这不像 Java 的 synchronized（可重入）。
//
// 【编译/运行时的行为】
//   代码编译通过！运行时死锁（线程永久阻塞）。
//   这是 Rust 无法在编译期检测的并发错误之一。
//
// 【为什么会这样】
//   Mutex::lock() 是阻塞调用，互斥锁不可重入。
//   如果在同一个线程中已经持有锁，再次 lock 会永远等待。
//
// 【在 C++/Go/Java 中对应的行为】
//   - C++:    std::mutex::lock() —— 同样不可重入，死锁
//             std::recursive_mutex —— 可重入选项
//
//   - Java:   synchronized 是可重入的（JVM 记录持有者线程）
//             ReentrantLock 也是可重入的
//
//   - Go:     sync.Mutex 不可重入（死锁）
//
// 【如何修复】
//   方案1: 确保在再次 lock 之前释放 MutexGuard（用作用域）
//   方案2: 重构代码避免嵌套锁

use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    // ✅ 正确: MutexGuard 在作用域结束时自动释放
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        // guard 在这里自动释放
    }

    // 可以再次加锁
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
    }

    // ❌ 错误的嵌套加锁（死锁！）
    // fn double_locked(counter: &Mutex<i32>) {
    //     let guard1 = counter.lock().unwrap();
    //     let guard2 = counter.lock().unwrap(); // 死锁！永远等不到
    //     *guard1 += *guard2;
    // }

    println!("最终值: {}", *counter.lock().unwrap());
    println!();
    println!("核心: Rust Mutex 不可重入，嵌套加锁会死锁");
    println!("      用作用域确保 MutexGuard 及时释放");
    println!("对比: Java synchronized 可重入（JVM 记录持有者）");
    println!("      Rust Mutex 设计更接近 C++/Go——简单但需谨慎");
}
