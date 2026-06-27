// ================================================================
// Level 11: 并发与线程安全
// 目标: thread、channel、Arc<Mutex<T>>、Send/Sync
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ Go           │ C++          │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 线程创建    │ thread::spawn│ go func()    │ std::thread  │
// │ 通信        │ channel      │ channel      │ 无内置       │
// │ 数据竞争    │ 编译期杜绝   │ 运行时检测   │ 无保护       │
// │ 共享可变    │ Arc+Mutex    │ sync.Mutex   │ shared+mutex │
// │ 并发安全    │ Send/Sync    │ -race flag   │ sanitizer    │
// └────────────┴──────────────┴──────────────┴──────────────┘

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 11: 并发与线程安全            ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 线程创建 ───
    println!("━━━ 1. std::thread::spawn ━━━");
    {
        // WHAT: thread::spawn 创建新线程，返回 JoinHandle<T>
        // CONTRAST:
        //   - Go:    go func() { ... }()  // 极简语法
        //   - C++:   std::thread t([]{ ... }); t.join();
        //   - Java:  new Thread(() -> { ... }).start();
        //   Rust 的 thread::spawn 返回 Result-based JoinHandle

        let handle = thread::spawn(|| {
            for i in 1..=3 {
                println!("  子线程: 第 {i} 步");
                thread::sleep(Duration::from_millis(10));
            }
            "子线程完成" // 返回值
        });

        println!("  主线程在做自己的事...");

        // join 等待线程结束并获取返回值
        match handle.join() {
            Ok(result) => println!("  join 结果: {result}"),
            Err(_) => println!("  线程 panic 了"),
        }
        // CONTRAST:
        //   Go:    用 sync.WaitGroup 或 channel 等待
        //   C++:   t.join() 返回 void
        //   Kotlin: join() 是挂起函数
    }
    println!();

    // ─── 2. Channel：消息传递 ───
    println!("━━━ 2. mpsc::channel：消息传递并发 ━━━");
    {
        // WHAT: mpsc = Multiple Producer, Single Consumer
        //       Go channel 哲学的 Rust 实现
        // CONTRAST:
        //   - Go:    ch := make(chan string, 2)
        //           ch <- "hello"
        //           msg := <-ch
        //   - Rust:  let (tx, rx) = mpsc::channel();
        //           tx.send(...).unwrap();
        //           let msg = rx.recv().unwrap();

        let (tx, rx) = mpsc::channel();

        // 生产者线程
        let producer = thread::spawn(move || {
            // WHAT: move 将 tx 的所有权移入闭包
            let messages = vec!["你好", "来自", "另一个线程"];
            for msg in messages {
                tx.send(msg.to_string()).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
            // tx 在这里 drop，rx 的迭代会终止
        });

        // 消费者（主线程）
        for received in rx {
            println!("  收到: {received}");
        }

        producer.join().unwrap();
    }
    println!();

    // ─── 3. Arc<Mutex<T>>：共享可变状态 ───
    println!("━━━ 3. Arc<Mutex<T>>：共享可变状态 ━━━");
    {
        // WHAT: Arc 提供线程安全的共享所有权
        //       Mutex 提供排他访问
        // CONTRAST:
        //   - C++:    auto p = make_shared<mutex_wrapper<Data>>();
        //             或 mutex m; Data d; // 锁和数据分离
        //   - Go:     var mu sync.Mutex; var data Data
        //   - Java:   synchronized(data) { ... }
        //   关键差异: Rust 的 Mutex 包裹数据——不解锁就无法访问

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // lock() 返回 MutexGuard，离开作用域自动解锁
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("  线程 {i}: counter = {num}");
                // MutexGuard 在这里 drop，自动 unlock
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("  最终计数: {}", *counter.lock().unwrap());
        // 预期: 5
    }
    println!();

    // ─── 4. Send/Sync trait ───
    println!("━━━ 4. Send 和 Sync —— 编译期并发安全 ━━━");
    {
        // WHAT: Send —— 类型可以安全地转移到另一个线程
        //       Sync —— 类型的不可变引用可以安全地在多个线程间共享
        //
        // 大多数类型自动实现 Send 和 Sync（编译期自动推导）
        // 不实现 Send 的例子: Rc<T>（非原子引用计数）
        // 不实现 Sync 的例子: RefCell<T>（运行时借用检查非线程安全）
        //
        // CONTRAST:
        //   没有语言有等价的编译期并发安全检查。
        //   - Go:    -race flag 运行时检测
        //   - C++:   sanitizer 运行时检测
        //   - TS:    单线程事件循环，不需要
        //   Rust 是唯一的"编译期并发安全"语言

        println!("  Send trait: 标记类型可以安全跨线程转移");
        println!("    ✅ i32, String, Vec<T>, Box<T>, Arc<T>, Mutex<T>");
        println!("    ❌ Rc<T> (非原子引用计数)");
        println!();
        println!("  Sync trait: 标记类型可以安全跨线程共享引用");
        println!("    ✅ i32, &T, Arc<T>, Mutex<T>");
        println!("    ❌ Rc<T>, RefCell<T>, Cell<T>");
        println!();
        println!("  编译器自动为你检查——不需要手动标注！");
    }
    println!();

    // ─── 5. 对比总结 ───
    println!("━━━ 5. 跨语言并发对比 ━━━");
    println!();
    println!("  场景: 100 个任务并发执行，累加计数器");
    println!();
    println!("  Rust:");
    println!("    编译器保证: 共享数据通过 Mutex 保护，无法绕过");
    println!("    Arc<Mutex<i32>> —— 不加锁根本拿不到数据");
    println!();
    println!("  C++:");
    println!("    mutex m; int counter = 0;");
    println!("    // 某处可能直接 counter++ 而不加锁");
    println!("    // 编译器不阻止，sanitizer 只有在那个路径被运行时才检测到");
    println!();
    println!("  Go:");
    println!("    var mu sync.Mutex; var counter int");
    println!("    counter++ // 不加锁也能编译——race detector 帮你找");
    println!();
    println!("  Rust 的 Send/Sync = 编译期 race detector");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 11 通关！进入毕业设计         ║");
    println!("╚══════════════════════════════════════╝");
}
