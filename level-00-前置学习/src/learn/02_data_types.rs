// ============================================
// 02 - 数据类型 (Data Types)
// ============================================
// Rust 是静态类型语言，编译时必须知道所有变量的类型

pub fn run() {
    println!("===== 02 数据类型 =====\n");

    // ==================== 标量类型 ====================

    // 1. 整数类型 (Integer Types)
    // 有符号: i8, i16, i32, i64, i128, isize
    // 无符号: u8, u16, u32, u64, u128, usize
    let a: i32 = -42;           // 有符号32位整数
    let b: u64 = 100;           // 无符号64位整数
    let c = 98_222;             // 数字字面量可使用下划线分隔增加可读性
    let d = 0xff;               // 十六进制
    let e = 0o77;               // 八进制
    let f = 0b1111_0000;        // 二进制
    let g: isize = 10;          // 取决于操作系统架构 (32或64位)
    println!("整数: a={}, b={}, c={}, d={}, e={}, f={}, g={}", a, b, c, d, e, f, g);

    // 2. 浮点类型 (Floating-Point Types)
    // f32: 单精度浮点数
    // f64: 双精度浮点数 (默认)
    let x = 2.0;                // f64 (默认)
    let y: f32 = 3.0;           // f32
    println!("浮点数: x={}, y={}", x, y);

    // 3. 布尔类型 (Boolean)
    let t = true;
    let f: bool = false;
    println!("布尔值: t={}, f={}", t, f);

    // 4. 字符类型 (Character)
    // 使用单引号，4字节 Unicode 标量值
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("字符: c={}, z={}, cat={}", c, z, heart_eyed_cat);

    // ==================== 复合类型 ====================

    // 5. 元组 (Tuple)
    // 固定长度，可以包含不同类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    // 使用点号索引访问
    println!("元组索引: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2);

    // 6. 数组 (Array)
    // 固定长度，元素类型必须相同
    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 类型;长度
    let b = [3; 5]; // 等价于 [3, 3, 3, 3, 3]
    println!("数组: arr[0]={}, months[0]={}, a[2]={}, b[4]={}", arr[0], months[0], a[2], b[4]);

    // 7. 切片 (Slice)
    // 对数组或 Vec 的引用，不拥有数据
    let s = &a[1..4]; // 包含索引 1, 2, 3
    println!("切片: {:?}", s);

    // 8. 字符串类型
    // &str: 字符串切片，不可变的 UTF-8 序列
    let string_literal: &str = "Hello, Rust!";
    // String: 可增长的、有所有权的 UTF-8 编码字符串
    let mut string_owned = String::from("Hello");
    string_owned.push_str(", World!");
    println!("字符串字面量: {}", string_literal);
    println!("String: {}", string_owned);

    println!();
}
