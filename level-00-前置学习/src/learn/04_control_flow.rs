// ============================================
// 04 - 流程控制 (Control Flow)
// ============================================

pub fn run() {
    println!("===== 04 流程控制 =====\n");

    // ==================== if 表达式 ====================
    let number = 7;

    // 基本 if-else
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // if-else if-else
    if number % 4 == 0 {
        println!("可被 4 整除");
    } else if number % 3 == 0 {
        println!("可被 3 整除");
    } else if number % 2 == 0 {
        println!("可被 2 整除");
    } else {
        println!("不可被 4, 3, 2 整除");
    }

    // if 是表达式，可以返回值
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", value);

    // ==================== match 表达式 ====================
    // 强大的模式匹配（详见 07 枚举和模式匹配）
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"),
    }

    // ==================== 循环 ====================

    // 1. loop - 无限循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以带返回值
        }
    };
    println!("loop 结果: {}", result);

    // 带标签的循环（用于嵌套循环中指定跳出哪个循环）
    let mut count = 0;
    'outer: loop {
        println!("外层循环: count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  内层循环: remaining = {}", remaining);
            if remaining == 9 {
                break; // 跳出内层循环
            }
            if count == 2 {
                break 'outer; // 跳出外层循环
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("带标签循环结束");

    // 2. while 循环
    let mut number = 3;
    while number != 0 {
        println!("while: {}", number);
        number -= 1;
    }
    println!("发射!");

    // while 遍历数组
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < arr.len() {
        println!("while 遍历: arr[{}] = {}", index, arr[index]);
        index += 1;
    }

    // 3. for 循环（推荐使用）
    // 遍历集合
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("for 遍历: {}", element);
    }

    // 使用 Range
    for number in 1..4 { // 1, 2, 3（不包含4）
        println!("Range: {}", number);
    }

    // 包含末尾
    for number in 1..=4 { // 1, 2, 3, 4
        println!("包含Range: {}", number);
    }

    // 反向 Range
    for number in (1..=4).rev() {
        println!("反向: {}", number);
    }

    // 带步长 (需要 itertools 或使用标准库方法)
    // for number in (1..10).step_by(2) {
    //     println!("步长2: {}", number);
    // }

    // 4. for 配合 enumerate
    for (i, val) in arr.iter().enumerate() {
        println!("enumerate: 索引={}, 值={}", i, val);
    }

    // ==================== continue ====================
    for number in 1..=10 {
        if number % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("奇数: {}", number);
    }

    // ==================== if let ====================
    // 简化只关心一个模式的 match
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("值是 3");
    }

    // 等价于：
    // match some_value {
    //     Some(3) => println!("值是 3"),
    //     _ => (),
    // }

    // ==================== while let ====================
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("while let: 弹出 {}", top);
    }

    // ==================== let else ====================
    // Rust 1.65+ 特性
    let some_value: Option<i32> = Some(42);
    let Some(value) = some_value else {
        panic!("值不存在！");
    };
    println!("let else: value = {}", value);

    println!();
}
