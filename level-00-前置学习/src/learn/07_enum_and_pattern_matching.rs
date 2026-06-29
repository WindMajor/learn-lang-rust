// ============================================
// 07 - 枚举和模式匹配 (Enums and Pattern Matching)
// ============================================

pub fn run() {
    println!("===== 07 枚举和模式匹配 =====\n");

    // ==================== 基本枚举 ====================
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    match dir {
        Direction::North => println!("向北"),
        Direction::South => println!("向南"),
        Direction::East => println!("向东"),
        Direction::West => println!("向西"),
    }

    // ==================== 带关联值的枚举 ====================
    // 类似于其他语言中的"标签联合"或"变体"
    enum Message {
        Quit,                           // 没有关联数据
        Move { x: i32, y: i32 },        // 匿名结构体
        Write(String),                  // 单个值
        ChangeColor(u8, u8, u8),        // 元组
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // 处理带关联值的枚举
    fn process_message(msg: &Message) {
        match msg {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色: RGB({}, {}, {})", r, g, b),
        }
    }

    process_message(&msg1);
    process_message(&msg2);
    process_message(&msg3);
    process_message(&msg4);

    // ==================== Option 枚举 ====================
    // Rust 没有 null，使用 Option 表示可能不存在的值
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let _some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);

    // 使用 match 处理 Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    println!("plus_one(Some(5)) = {:?}", plus_one(Some(5)));
    println!("plus_one(None) = {:?}", plus_one(None));

    // ==================== Result 枚举 ====================
    // 用于错误处理
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(msg) => println!("错误: {}", msg),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("结果: {}", result),
        Err(msg) => println!("错误: {}", msg),
    }

    // ==================== match 的通配符 ====================
    let some_value = 3;
    match some_value {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他"), // 通配符，匹配所有其他情况
    }

    // ==================== match 守卫 ====================
    // 在模式后添加 if 条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于5: {}", x),
        Some(x) => println!("大于等于5: {}", x),
        None => (),
    }

    // ==================== @ 绑定 ====================
    // 在匹配时同时绑定值
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("id 在范围内: {}", id_variable),
        Message2::Hello { id: 10..=12 } => println!("id 在另一个范围内"),
        Message2::Hello { id } => println!("其他 id: {}", id),
    }

    // ==================== if let ====================
    // 只关心一个模式时的简化写法
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("值是 3！");
    }

    // 可以带 else
    if let Some(x) = some_value {
        println!("值是 {}", x);
    } else {
        println!("没有值");
    }

    // ==================== while let ====================
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // ==================== let else ====================
    // Rust 1.65+
    let some_value: Option<i32> = Some(42);
    let Some(value) = some_value else {
        panic!("没有找到值");
    };
    println!("let else: {}", value);

    // ==================== 模式匹配的其他用法 ====================

    // 解构元组
    let point = (3, 5);
    let (x, y) = point;
    println!("点坐标: ({}, {})", x, y);

    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("点: a={}, b={}", a, b);

    // 简写
    let Point { x, y } = p;
    println!("点: x={}, y={}", x, y);

    // 解构枚举和结构体嵌套
    enum Shape {
        Circle { center: Point, radius: f64 },
        Rectangle { top_left: Point, bottom_right: Point },
    }

    let shape = Shape::Circle {
        center: Point { x: 0, y: 0 },
        radius: 5.0,
    };

    match shape {
        Shape::Circle {
            center: Point { x, y },
            radius,
        } => println!("圆心: ({}, {}), 半径: {}", x, y, radius),
        Shape::Rectangle { .. } => println!("矩形"),
    }

    // 解构数组和切片
    let arr = [1, 2, 3];
    let [a, b, c] = arr;
    println!("数组: a={}, b={}, c={}", a, b, c);

    // 忽略值
    let some_tuple = (1, 2, 3);
    let (first, _, third) = some_tuple; // 忽略第二个值
    println!("{}, {}", first, third);

    // 使用 .. 忽略剩余值
    let (first, ..) = some_tuple;
    println!("第一个: {}", first);

    // 匹配字面值
    let x = 1;
    match x {
        1 | 2 => println!("一或二"),
        3..=5 => println!("三到五"),
        _ => println!("其他"),
    }

    // 匹配引用
    let s = String::from("hello");
    match &s {
        // 匹配字符串内容
        text => println!("引用匹配: {}", text),
    }

    // 使用 ref 关键字
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("机器人名字: {}", name),
        None => (),
    }
    println!("robot_name 仍然可用: {:?}", robot_name);

    // 使用 ref mut
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("修改后: {:?}", robot_name);

    println!();
}
