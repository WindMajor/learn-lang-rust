// ================================================================
// Level 05: 结构体、枚举与模式匹配
// 目标: struct/enum/match/if let/模式解构
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ TS           │ Kotlin       │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 结构体      │ struct(值类型)│ interface    │ data class   │
// │ 代数类型    │ enum + data  │ Disc. Union  │ sealed class │
// │ 模式匹配    │ match(穷尽)  │ switch(+never)│ when(穷尽)  │
// │ 空安全      │ Option<T>    │ T | null/undef│ T?          │
// │ 方法定义    │ impl 块(分离) │ class 内     │ class 内     │
// └────────────┴──────────────┴──────────────┴──────────────┘

// ─── 结构体定义 ───
/// WHAT: 三种结构体形式
/// CONTRAST:
///   - C/C++: struct 是 C 兼容的类型（默认 public）
///   - Go:    struct 字段大写 public
///   - TS:    用 interface/type 定义形状（结构匹配而非名义匹配）
///   - Swift: struct 是值类型（与 Rust 最接近，但 Swift 用 ARC）

#[derive(Debug, Clone)] // 自动派生 Debug 和 Clone trait
struct User {
    name: String,
    email: String,
    active: bool,
}

/// 元组结构体 —— 命名的元组
/// CONTRAST: C++ std::pair、Go 中无直接对应、TS tuple type
#[derive(Debug)]
struct Color(u8, u8, u8);

/// 单元结构体 —— 无字段，用作标记
/// CONTRAST: C++ 中用作 tag dispatch 的空 struct
///           Go 中用空 struct{} 做信号
#[derive(Debug)]
struct LoggedIn;

// ─── 枚举定义 ───
/// WHAT: Rust enum 是代数数据类型，每个变体可以携带数据
/// WHY: 这比 C enum 强大得多，是 Rust 类型系统的核心组件
/// CONTRAST:
///   - C:     enum { A, B, C } — 只是整数常量
///   - C++:   enum class Color { Red, Green, Blue } — 作用域枚举但无数据
///   - TS:    type Shape = { kind:'Circle', radius:number }
///            | { kind:'Rectangle', w:number, h:number } — discriminated union
///    关键差异: TS 是结构匹配（两个相同 shape 的类型自动兼容），
///            Rust 是名义匹配（必须名义相同，强制显式）
///   - Kotlin: sealed class Shape — 最接近 Rust enum，有穷尽检查
///   - Swift:  enum Shape { case circle(radius: Double) } — 也很接近
#[derive(Debug)]
enum Message {
    Quit,                        // 无数据
    Move { x: i32, y: i32 },     // 命名字段
    Write(String),               // 元组变体
    ChangeColor(u8, u8, u8),     // 多个未命名值
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 05: 结构体、枚举与模式匹配     ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 结构体创建与更新 ───
    println!("━━━ 1. 结构体：创建、更新、方法 ━━━");
    {
        let mut user = User {
            email: String::from("alice@example.com"),
            name: String::from("Alice"),
            active: true,
        };
        user.active = false;
        println!("  {:?}", user);

        // 结构体更新语法（类似 JS spread，但有所有权转移！）
        let user2 = User {
            name: String::from("Bob"),
            ..user // user 的 email 被移动到 user2！
        };
        // println!("{:?}", user); // ❌ user.email 已被移走（部分移动）
        println!("  user2: {:?}", user2);

        // CONTRAST:
        // TS:   const user2 = { ...user, name: 'Bob' }
        //       user 仍然可用！因为 TS 是引用/浅拷贝语义
        // Rust: ..user 是部分移动 —— email 转移到 user2
        //       这是所有权模型在结构体上的体现
    }
    println!();

    // ─── 2. 枚举创建 ───
    println!("━━━ 2. 枚举变体创建 ━━━");
    {
        let msgs = vec![
            Message::Quit,
            Message::Move { x: 10, y: 20 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(255, 0, 0),
        ];
        for m in &msgs {
            println!("  {:?}", m);
        }
    }
    println!();

    // ─── 3. match：穷尽模式匹配 ───
    println!("━━━ 3. match：穷尽模式匹配 ━━━");

    {
        // WHAT: match 强制覆盖所有变体 —— 编译器帮你检查
        // CONTRAST:
        //   - C/C++: switch 不穷尽也不会报错（默认 fallthrough）
        //   - TS:    switch + discriminated union + never 类型穷尽检查
        //   - Kotlin: when 表达式 + sealed class → 穷尽检查
        //   - Go:    switch 不穷尽（编译器不帮）

        fn describe_event(event: &WebEvent) -> &str {
            match event {
                WebEvent::PageLoad => "页面加载",
                WebEvent::KeyPress(c) => {
                    println!("    按下的键: '{c}'");
                    "按键事件"
                }
                WebEvent::Click { x, y } => {
                    println!("    点击位置: ({x}, {y})");
                    "点击事件"
                } // 如果删除此分支，编译错误: non-exhaustive patterns
            }
        }

        let events = vec![
            WebEvent::PageLoad,
            WebEvent::KeyPress('a'),
            WebEvent::Click { x: 100, y: 200 },
        ];
        for e in &events {
            println!("  {:?} → {}", e, describe_event(e));
        }
    }
    println!();

    // ─── 4. if let / while let —— 条件解构 ───
    println!("━━━ 4. if let / while let —— 条件模式匹配 ━━━");

    {
        // WHAT: if let 只匹配一种模式，忽略其他
        // CONTRAST:
        //   - TS:  没有直接等价物（需要手动 type guard）
        //   - Kotlin: 无直接等价物
        //   - Swift: if case .some(let x) = value {} ← 最接近

        let config = Some(8080);

        // match 写法（冗长）
        match config {
            Some(port) => println!("  [match]  端口: {port}"),
            None => {} // 无操作也要写！除非用 _
        }

        // if let 写法（简洁）
        if let Some(port) = config {
            println!("  [if let] 端口: {port}");
        }

        // 也可以带 else
        let maybe_name: Option<&str> = None;
        if let Some(name) = maybe_name {
            println!("  有名字: {name}");
        } else {
            println!("  没有名字");
        }

        // while let —— 循环匹配
        let mut stack = vec![1, 2, 3];
        print!("  弹出: ");
        while let Some(top) = stack.pop() {
            print!("{top} ");
        }
        println!();
    }
    println!();

    // ─── 5. 模式解构的威力 ───
    println!("━━━ 5. 模式解构深度演示 ━━━");

    {
        // 解构元组
        let (x, y, z) = (1, 2, 3);
        println!("  元组解构: {x}, {y}, {z}");

        // 解构结构体
        let user = User {
            name: String::from("Charlie"),
            email: String::from("charlie@example.com"),
            active: true,
        };
        let User { name, email, .. } = user; // .. 忽略其余字段
        println!("  结构体解构: {name}, {email}");
        // user 在此之后 email 已被移动，但可以访问未被移动的字段

        // 解构嵌套结构
        let msg = Message::Move { x: 10, y: 20 };
        match msg {
            Message::Move { x: 0, y: 0 } => println!("  原点"),
            Message::Move { x, y: 0 } => println!("  x轴: {x}"),
            Message::Move { x: 0, y } => println!("  y轴: {y}"),
            Message::Move { x, y } => println!("  坐标: ({x}, {y})"),
            _ => {}
        }

        // match guards（守卫条件）
        let number = Some(7);
        match number {
            Some(n) if n < 5 => println!("  小于 5: {n}"),
            Some(n) if n < 10 => println!("  5~9: {n}"),
            Some(n) => println!("  其他: {n}"),
            None => println!("  没有数字"),
        }

        // @ 绑定：同时匹配和绑定
        let value = (1, 100);
        match value {
            (x @ 1..=10, y) => println!("  x在1~10: {x}, y={y}"),
            _ => {}
        }
    }
    println!();

    // ─── 6. 模式匹配在函数参数中的使用 ───
    println!("━━━ 6. 函数参数模式解构 ━━━");

    fn print_coords(&(x, y): &(i32, i32)) {
        println!("  ({x}, {y})");
    }

    fn on_click(User { name, email, .. }: &User) {
        println!("  点击了 {name} ({email})");
    }

    let point = (3, 4);
    print_coords(&point);

    let user = User {
        name: String::from("Dave"),
        email: String::from("dave@example.com"),
        active: true,
    };
    on_click(&user); // 借用，user 仍然可用
    println!("  user 仍可用: {:?}", user);

    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 05 通关！继续 Level 06        ║");
    println!("╚══════════════════════════════════════╝");
}
