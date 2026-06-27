// ================================================================
// Level 10: 智能指针与内部可变性
// 目标: Box/Rc/RefCell/Arc/Mutex/RwLock
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ C++          │ GC 语言      │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 堆分配      │ Box<T>       │ unique_ptr   │ 自动(GC 堆)  │
// │ 共享所有权  │ Rc<T>        │ shared_ptr   │ 自动(GC)     │
// │ 线程共享    │ Arc<T>       │ shared_ptr   │ 自动         │
// │ 运行时借用  │ RefCell<T>   │ 无           │ 无           │
// │ 排他锁      │ Mutex<T>     │ mutex+data   │ synchronized│
// └────────────┴──────────────┴──────────────┴──────────────┘

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 10: 智能指针与内部可变性      ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. Box<T>：堆分配 ───
    println!("━━━ 1. Box<T>：堆分配 ━━━");
    {
        // WHAT: Box 在堆上分配值，栈上存储指针
        // WHY: 1) 编译期大小未知的类型（递归类型） 2) 大值不想在栈上传递
        //       3) 所有权唯一且明确
        // CONTRAST:
        //   - C++:    auto p = make_unique<T>(...); // C++14
        //   - Go:     p := new(T)  // 返回 *T，但 GC 管理
        //   - TS/Java: 所有对象都在堆上（无选择）
        //   关键差异: Box 不能为空（不像 unique_ptr 可以 nullptr）

        let b = Box::new(42); // 在堆上分配 i32
        println!("  Box<i32>: {}, 地址: {:p}", *b, b);
        // b 离开作用域时自动释放堆内存（唯一所有者）

        // Box 解决递归类型问题
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>), // 需要 Box 因为 List 的大小在编译期未知
            Nil,
        }
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        println!("  递归列表: {:?}", list);
    }
    println!();

    // ─── 2. Rc<T>：单线程引用计数 ───
    println!("━━━ 2. Rc<T>：单线程引用计数 ━━━");
    {
        // WHAT: Rc 允许一个值有多个所有者，引用计数归零时自动释放
        // WHY: 单线程共享所有权 —— 比 C++ shared_ptr 更轻量（非原子计数）
        // CONTRAST:
        //   - C++:    shared_ptr<T> —— 原子引用计数（线程安全但有开销）
        //   - Swift:  自动 ARC —— 编译期插入 retain/release
        //   - Go:     GC —— 运行时追踪
        //   关键差异: Rust 让选择显式化——
        //           Rc (单线程非原子) vs Arc (多线程原子)

        let a = Rc::new(String::from("共享的数据"));
        println!("  初始引用计数: {}", Rc::strong_count(&a));

        let b = Rc::clone(&a); // WHAT: Rc::clone 只是增加引用计数（非深拷贝！）
        println!("  clone 后: {}", Rc::strong_count(&a));

        let c = Rc::clone(&a);
        println!("  再 clone: {}", Rc::strong_count(&a));

        // 所有引用指向同一数据
        println!("  a: {a}");
        println!("  b: {b}");
        println!("  c: {c}");

        // Rc 是不可变的——只能读取共享数据
        // 如果要修改，需要配合 RefCell
    }
    println!();

    // ─── 3. RefCell<T>：内部可变性 ───
    println!("━━━ 3. RefCell<T>：运行时借用检查 ━━━");
    {
        // WHAT: RefCell 将借用规则检查从编译期推迟到运行时
        // WHY: 当编译期无法证明借用规则但你知道运行时没问题时使用
        // CONTRAST: C++/Go/TS 中根本没有这个概念——它们的借用检查不在编译期

        let data = RefCell::new(vec![1, 2, 3]);

        // borrow() —— 不可变借用（运行时检查）
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("  不可变借用: {:?}, {:?}", r1, r2);
        drop(r1);
        drop(r2);

        // borrow_mut() —— 可变借用（运行时检查）
        {
            let mut r3 = data.borrow_mut();
            r3.push(4);
            println!("  可变借用后: {:?}", r3);
        } // r3 在这里释放借用

        // 再次不可变借用
        println!("  最终: {:?}", data.borrow());

        // WARNING: 如果同时在运行时持有 borrow 和 borrow_mut，会 panic:
        // let r1 = data.borrow();
        // let mut r2 = data.borrow_mut(); // PANIC! already borrowed
    }
    println!();

    // ─── 4. Rc + RefCell：可变的共享数据 ───
    println!("━━━ 4. Rc<RefCell<T>>：可变共享 ━━━");
    {
        // WHAT: Rc 提供共享所有权，RefCell 提供内部可变性
        // WHY: 这是单线程下最常见的"可变共享数据"模式
        // CONTRAST:
        //   - C++:    shared_ptr<vector<int>> v = ...;
        //             可以直接修改 v->push_back(4);
        //             但如果多线程需要锁保护，编译器不强制
        //   - Go:     共享 mutable slice 可能导致 data race
        //   - TS/JS:  单线程事件循环，共享可变是天然安全的

        let shared = Rc::new(RefCell::new(vec![10, 20, 30]));

        let owner1 = Rc::clone(&shared);
        let owner2 = Rc::clone(&shared);

        owner1.borrow_mut().push(40);
        owner2.borrow_mut().push(50);

        println!("  共享数据: {:?}", shared.borrow());
        println!("  引用计数: {}", Rc::strong_count(&shared));
    }
    println!();

    // ─── 5. Arc<T>、Mutex<T>、RwLock<T> 简介 ───
    println!("━━━ 5. Arc/Mutex/RwLock (多线程) ━━━");
    {
        println!("  Arc<T>:    原子引用计数（Atomic Reference Count）");
        println!("             等价于 Rc 的线程安全版本");
        println!("             C++ 中 shared_ptr 等价于 Arc（总是原子的）");
        println!();
        println!("  Mutex<T>:  互斥锁，包装数据，lock() 返回 MutexGuard<T>");
        println!("             等价于 C++ std::mutex + std::lock_guard");
        println!("             Rust 优势: 数据被锁'包裹'，不可能忘记加锁访问");
        println!();
        println!("  RwLock<T>: 读写锁，多个读者或一个写者");
        println!("             等价于 C++ std::shared_mutex");
        println!();
        println!("  C++ 对比:");
        println!("    C++:  mutex m; int data;  // 数据和锁分离！");
        println!("          可能不加锁就访问 data —— 编译器不阻止");
        println!("    Rust: Mutex<Data> —— 不加锁根本访问不到数据");
    }
    println!();

    println!("╔══════════════════════════════════════╗");
    println!("║  Level 10 通关！继续 Level 11        ║");
    println!("╚══════════════════════════════════════╝");
}
