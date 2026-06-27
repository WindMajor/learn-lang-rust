// ================================================================
// Level 02: 所有权与移动语义
// 目标: 理解 Rust 最核心的概念 —— Ownership、Move、Copy、Clone
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ C++          │ GC 语言      │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 默认语义    │ Move         │ Copy         │ 赋值引用     │
// │ 深拷贝      │ .clone()     │ 拷贝构造     │ 手动实现     │
// │ 浅拷贝      │ Copy trait   │ 编译器生成   │ 不适用       │
// │ 析构        │ Drop trait   │ ~析构函数    │ finalize()   │
// │ 释放时机    │ 编译期确定   │ 程序员确定   │ GC 不确定    │
// │ 所有权转移  │ 编译期检查   │ 无检查       │ 无此概念     │
// └────────────┴──────────────┴──────────────┴──────────────┘

// ─── 自定义类型：演示 Drop ───
/// 一个带有析构消息的自定义类型
/// CONTRAST: 类似 C++ 中带析构函数的 class，但 Rust 的 Drop 是 trait
#[derive(Debug)]
struct Resource {
    id: u32,
    name: String,
}

impl Resource {
    fn new(id: u32, name: &str) -> Self {
        println!("  [构造] Resource #{id} '{name}' 已分配");
        Resource {
            id,
            name: name.to_string(),
        }
    }
}

// WHAT: Drop trait —— 自定义析构逻辑
// WHY: Rust 编译期确定 drop 时机（离开作用域时），与 C++ RAII 类似
// CONTRAST:
//   - C++: ~Resource() 析构函数（语法不同，机制类似——离开作用域自动调用）
//   - Go:  defer 语句（手动，不是自动——关键差异！）
//   - TS:  无确定性析构，GC 的 finalizer 不可靠
//   - Java: finalize() 已被废弃，用 try-with-resources
//   - Kotlin: use {} （内联函数，等价于 try-with-resources）
//   - Swift: deinit（ARC 管理，引用计数归零时调用，与 Rust 的确定性不同）
// WARNING: 值被移动后，原来的变量不再可用，Drop 也不会在原变量上触发！
impl Drop for Resource {
    fn drop(&mut self) {
        println!("  [析构] Resource #{} '{}' 已释放", self.id, self.name);
    }
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 02: 所有权与移动语义          ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 所有权三规则 ───
    println!("━━━ 1. 所有权三规则 ━━━");
    println!("  规则1: Rust 中每一个值都有一个所有者");
    println!("  规则2: 值在任一时刻有且只有一个所有者");
    println!("  规则3: 当所有者离开作用域，值被丢弃（drop）");
    println!();

    // ─── 2. Move 语义（默认） ───
    println!("━━━ 2. Move 语义 —— Rust 的默认行为 ━━━");

    {
        let s1 = String::from("hello");
        // WHAT: s1 的值被移动到 s2，s1 此后不可用
        // WHY: String 不实现 Copy trait，默认行为是 Move
        // CONTRAST:
        //   - C++: auto s1 = std::string("hello");
        //          auto s2 = s1;          // 拷贝构造！s1 仍然可用
        //          auto s3 = std::move(s1); // 显式移动，s1 变成"有效但未定义状态"
        //   - Go:   s1 := "hello"
        //          s2 := s1              // 复制指针（Go string 不可变）
        //   - TS:   let s1 = "hello";
        //          let s2 = s1;          // 两个变量指向同一字符串
        //   - Swift: let s1 = "hello"
        //            let s2 = s1         // String 是值类型，自动 Copy
        let s2 = s1;
        println!("  s2 拥有 s1 的值: \"{s2}\"");
        // println!("{}", s1);  // ❌ error[E0382]: borrow of moved value: `s1`
        println!("  (s1 已被移动，不能再使用)");
        // WHAT: s2 在这里离开作用域，String 被 drop（释放堆内存）
        // WHY: 编译期确定——没有 GC、没有引用计数、没有运行时开销
    }
    println!();

    // ─── 3. Copy trait：隐式浅拷贝 ───
    println!("━━━ 3. Copy trait —— 位复制（bitwise copy） ━━━");

    {
        let x = 42;
        let y = x; // i32 实现了 Copy trait，所以 x 仍然可用
        println!("  x = {x}, y = {y}  ← 两个变量都可用");

        // WHAT: 哪些类型实现了 Copy？
        // - 所有整数类型: i8, i16, i32, i64, u8, u16, u32, u64, isize, usize
        // - 浮点类型: f32, f64
        // - 布尔类型: bool
        // - 字符类型: char
        // - 所有字段都是 Copy 的元组（如 (i32, i32)）
        // - 所有字段都是 Copy 的数组（如 [i32; 3]）
        // - 不可变引用: &T（注意: &mut T 不实现 Copy）
        //
        // CONTRAST:
        //   - C++: 默认 Copy，需 =delete 禁用；基本类型总是 Copy
        //   - Go:  基本类型值语义（自动复制），与 Rust Copy 类似
        //   - TS:  基本类型值语义，对象引用语义
        //   - Java: 基本类型值语义，对象引用语义

        // 地址观察实验
        let a = 100;
        let b = a;
        println!("  a 地址: {:p}, b 地址: {:p}", &a, &b);
        println!("  (Copy 类型: a 和 b 是栈上的两个独立副本)");
    }
    println!();

    // ─── 4. Clone trait：显式深拷贝 ───
    println!("━━━ 4. Clone trait —— 显式深拷贝 ━━━");

    {
        let original = String::from("深拷贝测试");
        // WHAT: .clone() 执行深拷贝，创建新的堆分配
        // WHY: 与 Move 不同，clone 后两个变量各自独立拥有数据
        // CONTRAST:
        //   - C++: std::string copy = original;  // 拷贝构造——隐式深拷贝！
        //         这是 C++ 最常见的性能陷阱之一（无意中的深拷贝）
        //   - Go:  strings.Clone(s)  // Go 1.18+ 显式克隆
        //   - TS:  structuredClone(obj)  // 浏览器环境
        //   Rust 的哲学: 深拷贝必须显式写出来——你不会无意中复制 10MB 的数据
        let cloned = original.clone();
        println!("  original: \"{original}\"  — 地址: {:p}", &original);
        println!("  cloned:   \"{cloned}\"    — 地址: {:p}", &cloned);
    }
    println!();

    // ─── 5. 函数参数与返回值的所有权 ───
    println!("━━━ 5. 函数调用中的所有权转移 ━━━");

    {
        let msg = String::from("你好，所有权");
        // WHAT: msg 的所有权被移动到函数中
        //       函数内部使用后，msg 的所有权通过返回值传回来
        let msg = take_and_return(msg);
        println!("  取回的值: \"{msg}\"");
        // 如果函数不返回所有权，msg 在函数调用后就不可用了
    }
    println!();

    // ─── 6. Drop trait 实验 ───
    println!("━━━ 6. Drop trait —— 确定性析构 ━━━");

    {
        let r1 = Resource::new(1, "数据库连接");
        let r2 = Resource::new(2, "文件句柄");

        // WHAT: 观察析构顺序 —— 后创建的先析构（栈式 LIFO）
        // WHY: Rust 作用域结束时按声明逆序 drop
        // CONTRAST:
        //   - C++: 同样逆序析构（栈展开）
        //   - Go:  defer 是 LIFO（但需要手动写 defer，且不保证执行——panic 可能跳过）
        //   - Java: try-finally 或 try-with-resources（手动，finally 可能不执行）
        //   - Swift: defer 是 LIFO，但 ARC 释放时机不确定
        //   Rust 的 Drop 是编译期确定的——不需要手动写，不会忘记
        println!("  作用域即将结束，观察析构顺序（后创建先析构）:");
    } // r2.drop(), r1.drop() 在这里自动调用
    println!();

    // ─── 7. Move + Drop 结合 —— 值移动后不会在原位置 Drop ───
    println!("━━━ 7. Move 后原变量不触发 Drop ━━━");

    {
        let r3 = Resource::new(3, "临时资源");
        // WHAT: r3 的所有权移动到 r4
        // WHY: r3 不再拥有数据，因此 drop 不会在 r3 上触发
        //      只有 r4 离开作用域时触发一次 drop
        // CONTRAST:
        //   - C++: std::move 后原对象处于"有效但未定义状态"，
        //          其析构函数仍会被调用（可能访问未定义内存！）
        //   Rust 在编译期就杜绝了这种问题
        let _r4 = r3;
        println!("  _r4 现在拥有 Resource #3");
    } // 只触发一次 drop（r4 的）
    println!();

    // ─── 8. 对比总结 ───
    println!("━━━ 8. 所有权跨语言对比总结 ━━━");
    println!();
    println!("  场景: 将一个 100MB 的字符串赋值给另一个变量");
    println!();
    println!("  Rust:   默认 Move（零拷贝），需 .clone() 才深拷贝");
    println!("          编译期保证原变量不可用");
    println!();
    println!("  C++:    默认 Copy（深拷贝 100MB！），需 std::move 才移动");
    println!("          auto s2 = s1;  // 此时已无意识地创建了 100MB 副本");
    println!("          auto s3 = std::move(s1);  // 移动后 s1 状态未定义");
    println!();
    println!("  Go:     字符串不可变，赋值时复制 (ptr, len) 两个机器字");
    println!("          （相当于 Rust 的 &str 引用）");
    println!();
    println!("  TS/JS:  赋值引用 —— 两个变量指向同一 GC 管理的对象");
    println!("          没有深拷贝浅拷贝之分（所有对象都是引用语义）");
    println!();
    println!("  Swift:  值类型自动 Copy，引用类型用 ARC 管理引用计数");
    println!();
    println!("  Kotlin: 类似 Java，基本类型值语义，对象引用语义");
    println!();
    println!("  Rust 的核心创新:");
    println!("  将 C/C++ 中'需要程序员记忆'的规则变成了'编译器强制执行'的约束");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 02 通关！继续 Level 03        ║");
    println!("╚══════════════════════════════════════╝");
}

// ─── 辅助函数 ───

/// 接收所有权并归还
/// WHAT: 参数 s 的所有权移入函数，返回值将所有权移出
/// WHY: 这种模式在 Rust 中很常见——"借用"是更好的选择（见 Level 03）
/// CONTRAST:
///   - C++: void take_and_return(string& s); // 引用，无所有权转移
///          string take_and_return(string s); // 拷贝 + 移动
///   - Go:  func takeAndReturn(s string) string { return s }
///          Go 的字符串不可变，一直是值语义
fn take_and_return(s: String) -> String {
    println!("  函数内部: \"{s}\"");
    s // 所有权移出函数（返回给调用者）
}
