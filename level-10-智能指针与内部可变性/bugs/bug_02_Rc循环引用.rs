// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: Rc 循环引用导致内存泄漏                        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   两个 Rc 互相引用形成循环，引用计数永远不会归零 → 内存泄漏。
//   这是纯引用计数方案的天生缺陷。
//
// 【运行时会报什么错】
//   不会报错！代码正常编译运行，但内存泄漏。
//   使用 valgrind 或 macOS leaks 工具才能检测到。
//   这是 Rust 安全保证中最容易被忽略的"合法内存泄漏"。
//
// 【为什么会这样】
//   Rc 只做引用计数，不处理循环引用。Go 的 GC 通过可达性分析
//   自动处理循环引用，C++ shared_ptr 同样有此问题（需要 weak_ptr）。
//
// 【在 C++/Go 中对应的行为】
//   - C++:    shared_ptr<A> a(new A); shared_ptr<B> b(new B);
//             a->b = b; b->a = a; // 循环引用 → 内存泄漏
//             解决: weak_ptr<A>
//
//   - Go:     GC 自动检测并回收循环引用（三色标记算法）
//
//   - Swift:  ARC + weak reference（与 Rust Weak 类似）
//
// 【如何修复】
//   使用 Weak<T> 打破循环（不增加强引用计数）

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak 不增加引用计数
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    println!("═══ Rc 循环引用与 Weak 解决方案 ═══");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 建立反向引用（用 Weak）
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf 强引用 = {}, 弱引用 = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("branch 强引用 = {}, 弱引用 = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

    println!();
    println!("核心: Rc 循环引用 → 内存泄漏");
    println!("      Weak<T> → 打破循环 → 访问时需 upgrade()");
    println!("对比: C++ shared_ptr 同样需要 weak_ptr 解决循环引用");
    println!("      Go GC 自动处理循环引用（但有 GC 暂停）");
    println!("      Swift ARC + weak 与 Rust 方案最接近");
}
