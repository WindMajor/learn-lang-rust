// ============================================
// 05 - 所有权 (Ownership)
// ============================================
// Rust 最核心的特性之一，无需垃圾回收器即可保证内存安全

pub fn run() {
    println!("===== 05 所有权 =====\n");

    // ==================== 所有权规则 ====================
    // 1. Rust 中的每个值都有一个所有者（owner）
    // 2. 值在任一时刻只能有一个所有者
    // 3. 当所有者离开作用域，值将被丢弃

    // ==================== 变量作用域 ====================
    {
        let s = "hello"; // s 在这里有效
        println!("作用域内: {}", s);
    } // s 在这里离开作用域，不再有效
    // println!("{}", s); // 错误！s 已经无效

    // ==================== String 类型的所有权 ====================

    // 字符串字面量是硬编码的，不可变
    let _s1 = "hello"; // &str 类型，存储在栈上（实际是引用）

    // String 类型存储在堆上，可变，有所有权
    let s2 = String::from("hello");
    println!("String: {}", s2);

    // ==================== 移动 (Move) ====================
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1); // 错误！s1 已经无效
    println!("移动后 s2: {}", s2);

    // ==================== 克隆 (Clone) ====================
    // 如果需要深拷贝（复制堆上的数据）
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    println!("克隆 s1: {}, s2: {}", s1, s2);

    // ==================== 复制 (Copy) trait ====================
    // 栈上的数据赋值时会自动复制
    let x = 5;
    let y = x; // x 被复制到 y，x 仍然有效
    println!("复制 x: {}, y: {}", x, y);
    // 这是因为整数实现了 Copy trait
    // 实现 Copy trait 的类型：所有整数类型、浮点类型、布尔类型、字符类型、
    // 以及仅包含 Copy 类型的元组和数组

    // ==================== 所有权与函数 ====================
    let s = String::from("hello");
    takes_ownership(s); // s 的所有权移动到函数中
    // println!("{}", s); // 错误！s 已经无效

    let x = 5;
    makes_copy(x); // x 被复制到函数中
    println!("x 仍然有效: {}", x);

    // ==================== 返回值与所有权 ====================
    let s1 = gives_ownership(); // 返回值的所有权移动到 s1
    println!("获得所有权: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 所有权移入，返回值所有权移给 s3
    println!("传入并返回: {}", s3);

    // ==================== 引用 (References) ====================
    // 使用引用可以避免所有权转移
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用 s1，不获取所有权
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然有效！

    // ==================== 可变引用 ====================
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后: {}", s);

    // 可变引用规则：
    // 1. 在特定作用域内，对某数据只能有一个可变引用
    // 2. 不能同时拥有可变引用和不可变引用

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // 多个不可变引用是允许的
    println!("{} {}", r1, r2);
    // r1 和 r2 在这里之后不再使用

    let r3 = &mut s; // 现在可以创建可变引用了
    println!("{}", r3);

    // ==================== 悬垂引用 (Dangling References) ====================
    // Rust 编译器保证不会出现悬垂引用
    // let reference_to_nothing = dangle(); // 编译错误！

    // ==================== 切片 (Slices) ====================
    let s = String::from("hello world");
    let hello = &s[0..5]; // 字符串切片
    let world = &s[6..11];
    println!("切片: '{}' 和 '{}'", hello, world);

    // 其他切片写法
    let slice1 = &s[0..2];   // 从开始到索引2（不含）
    let slice2 = &s[..2];    // 同上
    let slice3 = &s[3..];    // 从索引3到结束
    let slice4 = &s[..];     // 整个字符串
    println!("切片: {}, {}, {}, {}", slice1, slice2, slice3, slice4);

    // 字符串字面量就是切片
    let s: &str = "Hello, world!";
    println!("字符串切片: {}", s);

    println!();
}

fn takes_ownership(s: String) {
    println!("获得所有权: {}", s);
} // s 在这里离开作用域，内存被释放

fn makes_copy(i: i32) {
    println!("获得副本: {}", i);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s // 返回 s，所有权移出函数
}

fn takes_and_gives_back(s: String) -> String {
    s // 返回 s，所有权移出函数
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 在这里离开作用域，但不拥有所有权，所以不释放内存

fn change(s: &mut String) {
    s.push_str(", world");
}

// 这个函数会编译错误，因为返回了局部变量的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回 s 的引用，但 s 在函数结束时会被释放
// }
