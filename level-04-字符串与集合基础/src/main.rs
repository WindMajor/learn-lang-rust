// ================================================================
// Level 04: 字符串与集合基础
// 目标: 理解 String vs &str、Vec vs &[T]、切片、内存布局
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ C/C++        │ Go           │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 拥有字符串  │ String       │ std::string  │ strings.Builder│
// │ 字符串引用  │ &str         │ const char*  │ string       │
// │ 字符串视图  │ &str (即可)  │ string_view  │ string       │
// │ 动态数组    │ Vec<T>       │ vector<T>    │ []T (slice)  │
// │ 数组切片    │ &[T]         │ span<T>      │ []T          │
// │ 字符串切片  │ &str         │ string_view  │ string       │
// └────────────┴──────────────┴──────────────┴──────────────┘

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 04: 字符串与集合基础          ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. String vs &str 内存布局 ───
    println!("━━━ 1. String vs &str —— 内存布局对比 ━━━");
    {
        // WHAT: String = (ptr, len, capacity) → 堆上 UTF-8 数据
        // WHY: 拥有所有权，可以增长
        let owned = String::from("Hello, 世界"); // 12 字节（英文5+空格1+逗号1+中文3字节×2=6）

        // WHAT: &str = (ptr, len) → 只引用不拥有
        // WHY: 零拷贝视图，不持有数据
        let borrowed: &str = &owned; // 从 String 借用
        let literal: &str = "我是字面量"; // 编译期嵌入二进制，'static 生命周期

        // ─── 打印内存信息 ───
        // WHAT: as_ptr() 获取内部堆指针地址
        dbg!(owned.as_ptr());
        dbg!(borrowed.as_ptr()); // 与 owned 相同（借用同一数据）
        dbg!(literal.as_ptr()); // 不同的地址（字面量在 .rodata 段）

        println!("  owned.capacity() = {}", owned.capacity());
        println!("  owned.len() = {}", owned.len());
        println!("  borrowed.len() = {}", borrowed.len());
        println!("  literal.len() = {}", literal.len());

        // CONTRAST:
        // C:    char s[] = "hello";     // 栈上数组（固定大小）
        //       char* p = s;            // 指针（需要手动管理）
        // C++:  string s = "hello";     // 堆分配 + 拥有（类似 Rust String）
        //       string_view sv = s;     // C++17，类似 Rust &str
        // Go:   s := "hello"            // 不可变字符串，(ptr, len)
        //       b := strings.Builder{}  // 可变构建器
        // TS:   const s = "hello";      // 原始值，不需要区分**
    }
    println!();

    // ─── 2. 字符串切片 —— Rust 的杀手锏 ───
    println!("━━━ 2. 字符串切片 &str —— 零拷贝视图 ━━━");
    {
        let text = String::from("Rust 的内存安全哲学是编译期保证的");

        // WHAT: 切片操作 [..] 返回 &str，不分配新内存
        // WHY: 切片只是调整 (ptr, len)，ptr 指向原数据内部
        let first_word = &text[..4];          // "Rust"
        let chinese_part = &text[5..11];      // "的内"
        // WARNING: 切片必须在 UTF-8 字符边界上，否则 panic!
        // &text[0..1]  // "R" — 1字节，OK（ASCII）
        // &text[5..7]  // 中文"的"=3字节(5-7)，取2字节(5-6) → panic!

        println!("  text 地址: {:p}", text.as_ptr());
        println!("  first_word = \"{first_word}\", 地址: {:p}", first_word.as_ptr());
        println!("  chinese_part = \"{chinese_part}\", 地址: {:p}", chinese_part.as_ptr());
        println!("  (所有切片指向 text 堆内存的不同位置，零拷贝！)");

        // CONTRAST:
        // C++ 中 string_view 提供类似功能:
        //   string_view first = text.substr(0, 4); // 错误！substr 返回新 string
        //   string_view first(text.data(), 4);     // 正确：不分配
        // Go 中 string 本身就是不可变视图——没有区分

        // ─── 字符串迭代：chars() vs bytes() ───
        println!();
        println!("  字符迭代 chars():");
        // WHAT: .chars() 返回 Unicode 标量值（scalar value）迭代器
        //       每个 char 是 4 字节，代表一个 Unicode 码点
        // CONTRAST:
        //   C/C++: 没有内置 UTF-8 迭代，需要 ICU 或手动处理
        //   Go:    for _, r := range s {}  — rune 迭代（类似）
        //   TS:    for (const c of s) {}  — 自动按码点迭代
        for c in text.chars().take(8) {
            print!("{c} ");
        }
        println!();

        println!("  字节迭代 bytes():");
        for b in text.bytes().take(8) {
            print!("{b} ");
        }
        println!(" (UTF-8 编码后的字节值)");
    }
    println!();

    // ─── 3. Vec<T> 动态数组 ───
    println!("━━━ 3. Vec<T> —— 动态数组 ━━━");
    {
        // WHAT: Vec<T> = (ptr, len, capacity)，堆上连续内存
        // CONTRAST:
        //   C++:   vector<T> — 相同的内存布局
        //   Go:    []T — 也是 (ptr, len, cap)，但不拥有底层数组的所有权概念
        //   TS:    Array<T> — GC 管理，无容量概念
        let mut v = Vec::with_capacity(8); // 预分配容量
        v.extend_from_slice(&[1, 2, 3, 4, 5]);
        println!("  v = {:?}, len = {}, cap = {}", v, v.len(), v.capacity());

        // ─── Vec 常用操作 ───
        println!();
        println!("  ─── Vec 常用方法 ───");
        v.push(6);
        println!("  push(6) → {:?}", v);

        let popped = v.pop(); // 返回 Option<T>（安全）
        println!("  pop() → {:?}, v = {:?}", popped, v);

        v.insert(2, 99);
        println!("  insert(2, 99) → {:?}", v);

        v.remove(2);
        println!("  remove(2) → {:?}", v);

        // ─── Vec 切片 ───
        println!();
        println!("  ─── Vec 切片 &[T] ───");
        let slice: &[i32] = &v[1..3]; // 切片，不拷贝数据
        println!("  &v[1..3] = {:?}, 地址: {:p}", slice, slice);
        println!("  v 地址: {:p}", v.as_ptr());
    }
    println!();

    // ─── 4. 函数参数：&str vs String vs &[T] vs Vec<T> ───
    println!("━━━ 4. 函数参数选择指南 ━━━");
    {
        // 推荐: 函数参数用 &str（零拷贝，适用面最广）
        greet("Alice");
        greet(&String::from("Bob")); // String 自动解引用为 &str（Deref trait）
        greet(&"Charlie".to_string());

        // 推荐: 切片参数用 &[T]
        let v = vec![1, 2, 3, 4, 5];
        let sum = sum_slice(&v);
        let sum_slice = sum_slice(&v[1..3]); // 子切片也可以
        println!("  sum(&v) = {sum}, sum(&v[1..3]) = {sum_slice}");
    }
    println!();

    // ─── 5. 字符串构建：高效拼接 ───
    println!("━━━ 5. 字符串构建模式 ━━━");
    {
        // 方式1: format!() — 方便但分配新 String
        let a = format!("{} {}", "Hello", "World");

        // 方式2: push_str — 无额外分配（如果容量足够）
        let mut b = String::with_capacity(32);
        b.push_str("Hello");
        b.push(' '); // push 单个 char
        b.push_str("World");

        // 方式3: 迭代器 + collect — 最 Rust 惯用
        // CONTRAST:
        //   TS:  words.join(" ")
        //   Go:  strings.Join(words, " ")
        //   C++:  accumulate + ostringstream
        let words = ["零", "成本", "抽象"];
        let c: String = words.join(""); // 简洁但会分配中间字符串

        // 方式4: 收集迭代器（零分配中间结果？理论上可以，依赖优化）
        let words2 = vec!["编译", "期", "内存", "安全"];
        let d: String = words2.iter().fold(String::new(), |mut acc, w| {
            acc.push_str(w);
            acc
        });

        println!("  format!: {a}");
        println!("  push_str: {b}");
        println!("  join: {c}");
        println!("  fold: {d}");
    }
    println!();

    // ─── 6. 对比总结 ───
    println!("━━━ 6. 跨语言字符串/数组对比 ━━━");
    println!();
    println!("  ┌────────────────┬────────────┬────────────┬────────────┐");
    println!("  │ 操作            │ Rust       │ C++        │ Go         │");
    println!("  ├────────────────┼────────────┼────────────┼────────────┤");
    println!("  │ 创建字符串      │ String::from│ string(..) │ \"hello\"   │");
    println!("  │ 子串(零拷贝)    │ &s[..]     │ string_view│ s[idx:]    │");
    println!("  │ 子串(拷贝)      │ s[..].to_string()│ s.substr() │ string(s[idx:]) │");
    println!("  │ 字符串拼接      │ format!/+  │ +/format   │ +/Sprintf  │");
    println!("  │ 字符迭代        │ s.chars()  │ 需手动/ICU │ for range  │");
    println!("  │ 创建动态数组    │ Vec::new() │ vector     │ []T{{}}      │");
    println!("  │ 切片            │ &v[..]     │ span       │ v[:]       │");
    println!("  │ 预分配          │ with_capacity│ reserve │ make       │");
    println!("  └────────────────┴────────────┴────────────┴────────────┘");
    println!();
    println!("  Rust 的 &str 统一了 C++ 中 string_view 和 const char* 两种概念");
    println!("  不需要额外的类型，不需要额外的头文件——语言内建");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 04 通关！继续 Level 05        ║");
    println!("╚══════════════════════════════════════╝");
}

fn greet(name: &str) {
    println!("  Hello, {name}!");

    // CONTRAST: Go — func greet(name string) — 参数是 string 类型
    //           没有区分"我拥有"和"我借用"
    //           Rust &str 明确表达"我只是借用，不拥有"
}

fn sum_slice(xs: &[i32]) -> i32 {
    // CONTRAST:
    //   C++: int sum(const vector<int>& xs) 或 int sum(span<const int> xs)
    //   Go:  func sum(xs []int) int — Go 的 []int 既可以是拥有也可以是借用
    //   Rust &[i32] — 明确是借用，不拥有
    xs.iter().sum()
}
