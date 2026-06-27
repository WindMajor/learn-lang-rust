// ================================================================
// Level 03: 借用与引用
// 目标: 理解 &T（不可变引用）、&mut T（可变引用）、借用规则、NLL
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ C/C++        │ GC 语言      │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 只读引用    │ &T           │ const T*     │ 变量别名     │
// │ 读写引用    │ &mut T       │ T*           │ 变量别名     │
// │ 空引用      │ Option<&T>   │ nullptr/NULL │ null/nil     │
// │ 悬垂引用    │ 编译期禁止   │ 运行时崩溃   │ GC 防止      │
// │ 别名限制    │ 编译期强制   │ 无限制       │ 无限制       │
// └────────────┴──────────────┴──────────────┴──────────────┘

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 03: 借用与引用                ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 不可变引用 &T ───
    println!("━━━ 1. &T：不可变引用（共享借用）━━━");
    {
        let s = String::from("hello");

        // WHAT: &s 创建一个不可变引用，不转移所有权
        // WHY: 借用允许"临时访问"数据，而不取得所有权
        // CONTRAST:
        //   - C:     const char* p = s;     // 类似，但没有所有权概念
        //   - C++:   const string& ref = s; // 非常接近！（const 引用）
        //   - Go:    函数参数 *T，但没有借用检查器
        //   - TS:    s 本身就是引用（GC 管理），没有 & 操作符的需求
        let len = calculate_length(&s);

        // s 仍然拥有数据，可以继续使用
        println!("  s = \"{}\" 的长度是 {len}", s);
    }
    println!();

    // ─── 2. 可变引用 &mut T ───
    println!("━━━ 2. &mut T：可变引用（排他借用）━━━");
    {
        let mut s = String::from("hello");

        // WHAT: &mut s 创建可变引用，可以修改被引用的值
        // CONTRAST:
        //   - C:     char* p = s; p[0] = 'H';            // 自由修改
        //   - C++:   string& ref = s; ref[0] = 'H';      // 非常量引用
        //   - Go:    func modify(s *string) { *s = "H" }  // 显式解引用
        //   - TS:    函数内直接修改，无任何限制
        //
        // 关键差异:
        //   C 给你完全的指针自由（也包括完全的 UB 风险）
        //   Rust 给你排他修改权，但同时保证没有其他引用正在读取
        append_world(&mut s);
        println!("  append_world 之后: \"{}\"", s);
    }
    println!();

    // ─── 3. 借用规则实验 ───
    println!("━━━ 3. 借用规则实验 ━━━");
    {
        let mut data = vec![1, 2, 3];

        // 规则1: 多个不可变引用可以共存
        let r1 = &data;
        let r2 = &data;
        let r3 = &data;
        println!("  r1: {:p}, r2: {:p}, r3: {:p}  ← 三个引用指向同一数据", r1, r2, r3);
        println!("  r1 = {:?}", r1);
        println!("  (多个不可变引用共存 —— 安全，因为谁也不能修改)");

        // r1, r2, r3 在这里不再被使用（NLL 会结束它们的借用）
        // data 可以重新被可变借用
        // 如果没有下面这行，上面的 x 仍在使用中，则下面会报错
    }

    {
        let mut data = vec![4, 5, 6];

        // 规则2: 可变引用不能与任何其他引用共存
        let r_mut = &mut data;
        r_mut.push(7);
        // let r_ref = &data;  // ❌ error[E0502]: cannot borrow as immutable
        println!("  修改后: {:?}", r_mut);

        // NLL: r_mut 在这里之后不再使用，因此借用结束
    }
    println!();

    // ─── 4. NLL（Non-Lexical Lifetimes）实验 ───
    println!("━━━ 4. NLL：非词法作用域生命周期 ━━━");

    {
        let mut x = 10;

        // WHAT: NLL 意味着借用从最后一次使用 point 结束，而非 } 处
        // 在 Rust 2015（词法生命周期）中，以下代码会报错
        // 在 Rust 2018+（NLL，edition 2021 默认启用）中，以下代码可以编译
        let r1 = &x;
        let r2 = &x; // 与前一个不可变引用共存
        println!("  r1 = {r1}, r2 = {r2}");
        // NLL: r1 和 r2 在这里之后都不再使用，借用结束

        let r3 = &mut x; // ✅ NLL 下，r1/r2 已不再使用
        *r3 += 1;
        println!("  r3 修改后 x = {x}");
    }
    println!();

    // ─── 5. 借用作为函数参数 ───
    println!("━━━ 5. 函数参数：& vs &mut vs 所有权 ━━━");

    let data = String::from("探索借用世界");

    // &T: 只读借用 —— 最常用
    print_string(&data);
    print_string(&data); // 可以多次借用

    // 所有权转移: 放弃所有权
    let data = consume_string(data);
    // data 的所有权已返回（如果函数返回的话）

    println!();

    // ─── 6. 对比总结 ───
    println!("━━━ 6. 借用跨语言对比 ━━━");
    println!();
    println!("  C/C++ 指针模型:");
    println!("    T* p = &obj;         // 你获得了一把\"万能钥匙\"");
    println!("    const_cast<T*>(p)     // 你可以强行去掉 const");
    println!("    delete p; p->foo();   // 编译通过，运行时爆炸");
    println!("    哲学: \"我相信你不会犯错\"（但人都会犯错）");
    println!();
    println!("  Rust 借用模型:");
    println!("    let r = &obj;         // 你获得了一个\"限定范围的观察窗\"");
    println!("    // 没有 const_cast    // 不可能绕过借用规则");
    println!("    // 没有悬垂引用      // 编译器保证 r 不会比 obj 活得久");
    println!("    哲学: \"我不相信你，但编译器帮你检查一切\"");
    println!();
    println!("  Go 指针:");
    println!("    p := &obj             // 无法做指针运算（安全）");
    println!("    // 没有借用检查器     // 可以同时存在读写者（数据竞争）");
    println!("    哲学: \"指针安全但不保证并发安全\"");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 03 通关！继续 Level 04        ║");
    println!("╚══════════════════════════════════════╝");
}

// ─── 辅助函数 ───

/// 通过不可变引用获取字符串长度
/// CONTRAST:
///   - C:    size_t len(const char* s) { return strlen(s); }
///           指针可以为 NULL，Rust 引用永远有效
///   - C++:  size_t len(const string& s) { return s.size(); }
///           引用可能悬挂（如果 s 是临时对象）
///   - Go:   func len(s *string) int { return len(*s) }
///           指针可以 nil
///   - TS:   function len(s: string): number { return s.length }
///           s 本身是运行时对象引用
fn calculate_length(s: &String) -> usize {
    // s.push_str("x");  // ❌ &String 不能修改
    s.len()
}

/// 通过可变引用修改数据
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

/// 只读借用函数
fn print_string(s: &String) {
    println!("  内容: \"{s}\", 长度: {}, 地址: {:p}", s.len(), s);
}

/// 所有权转移函数
fn consume_string(s: String) -> String {
    println!("  消费: \"{s}\"，然后归还所有权");
    s // 归还所有权
}
