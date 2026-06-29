// ============================================
// 10 - 错误处理 (Error Handling)
// ============================================
// Rust 没有异常，使用 Result<T, E> 和 panic!

pub fn run() {
    println!("===== 10 错误处理 =====\n");

    // ==================== panic! ====================
    // 不可恢复的错误，程序会终止
    // panic!("崩溃并燃烧！");

    // 可以使用 RUST_BACKTRACE=1 环境变量查看 backtrace

    // ==================== Result<T, E> ====================
    // 可恢复的错误
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    use std::fs::File;

    // 手动处理 Result
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => {
            println!("成功打开文件");
            file
        }
        Err(error) => {
            println!("打开文件失败（这是预期的）: {:?}", error);
            // 创建一个临时文件用于后续演示
            File::create("hello.txt").unwrap()
        }
    };

    // 写入测试内容
    std::fs::write("hello.txt", "rust learner\n").unwrap();

    // ==================== unwrap ====================
    // 成功返回 Ok 中的值，失败则 panic
    // let f = File::open("hello.txt").unwrap();

    // ==================== expect ====================
    // 类似 unwrap，但可以自定义 panic 消息
    // let f = File::open("hello.txt").expect("无法打开 hello.txt");

    // ==================== ? 运算符 ====================
    // 如果 Result 是 Err，提前返回错误
    // 如果 Result 是 Ok，解包值
    // 只能在返回 Result 的函数中使用

    // 传播错误
    fn read_username_from_file() -> Result<String, std::io::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("hello.txt")?; // 如果失败，直接返回 Err
        let mut s = String::new();
        f.read_to_string(&mut s)?; // 如果失败，直接返回 Err
        Ok(s)
    }

    // 更简洁的写法
    fn read_username_from_file2() -> Result<String, std::io::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // 使用 std::fs::read_to_string（最简洁）
    fn read_username_from_file3() -> Result<String, std::io::Error> {
        std::fs::read_to_string("hello.txt")
    }

    // 测试错误传播
    match read_username_from_file() {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取失败 (1): {}", e),
    }

    // ==================== Option 与 ? 运算符 ====================
    // ? 也可以用于 Option
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    println!(
        "最后一行最后一个字符: {:?}",
        last_char_of_first_line("Hello\nWorld")
    );
    println!(
        "空字符串: {:?}",
        last_char_of_first_line("")
    );

    // ==================== ? 运算符与 main 函数 ====================
    // main 函数也可以返回 Result
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("hello.txt")?;
    //     Ok(())
    // }

    // ==================== map 和 map_err ====================
    // 转换 Result 的值
    let text = "42";
    let number = text.parse::<i32>();
    match number {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    // 使用 map 转换 Ok 值
    let result = text.parse::<i32>().map(|n| n * 2);
    println!("map 结果: {:?}", result);

    // 使用 map_err 转换 Err 值
    let result = "not a number".parse::<i32>().map_err(|e| {
        format!("自定义错误: {}", e)
    });
    println!("map_err 结果: {:?}", result);

    // ==================== and_then / or_else ====================
    // 链式处理
    let result = text
        .parse::<i32>()
        .and_then(|n| Ok(n * 2))
        .and_then(|n| Ok(n + 1));
    println!("and_then 结果: {:?}", result);

    let result: Result<i32, std::num::ParseIntError> = "not a number"
        .parse::<i32>()
        .or_else(|_| Ok(0));
    println!("or_else 结果: {:?}", result);

    // ==================== unwrap_or / unwrap_or_else ====================
    let result = "not a number".parse::<i32>().unwrap_or(0);
    println!("unwrap_or 结果: {}", result);

    let result = "not a number"
        .parse::<i32>()
        .unwrap_or_else(|_| {
            println!("解析失败，使用默认值");
            0
        });
    println!("unwrap_or_else 结果: {}", result);

    // ==================== unwrap_or_default ====================
    let result: i32 = "not a number".parse().unwrap_or_default();
    println!("unwrap_or_default 结果: {}", result);

    // ==================== ok 和 err ====================
    // 将 Result 转换为 Option
    let result: Option<i32> = "42".parse().ok();
    println!("ok 结果: {:?}", result);

    let result: Option<std::num::ParseIntError> = "not a number".parse::<i32>().err();
    println!("err 结果: {:?}", result);

    // ==================== 自定义错误类型 ====================
    #[derive(Debug)]
    enum MyError {
        NotFound,
        InvalidInput(String),
        IoError(std::io::Error),
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                MyError::NotFound => write!(f, "未找到"),
                MyError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
                MyError::IoError(e) => write!(f, "IO 错误: {}", e),
            }
        }
    }

    impl std::error::Error for MyError {}

    // 从 std::io::Error 转换
    impl From<std::io::Error> for MyError {
        fn from(error: std::io::Error) -> Self {
            MyError::IoError(error)
        }
    }

    fn do_something() -> Result<(), MyError> {
        // 可以自动转换
        let _f = File::open("nonexistent.txt")?;
        Ok(())
    }

    match do_something() {
        Ok(()) => println!("成功"),
        Err(e) => println!("自定义错误: {}", e),
    }

    // ==================== anyhow / thiserror ====================
    // 生产环境中通常使用第三方库处理错误
    // anyhow: 简单易用的错误处理
    // thiserror: 方便定义自定义错误类型

    // 例如使用 anyhow:
    // use anyhow::Result;
    // fn main() -> Result<()> {
    //     let f = File::open("hello.txt")?;
    //     Ok(())
    // }

    println!();
}

// ==================== panic 的处理 ====================
// 设置 panic hook
pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        println!("自定义 panic 处理: {:?}", info);
    }));
}

// ==================== 断言 ====================
pub fn demonstrate_assertions() {
    // assert! - 条件必须为真
    assert!(true);

    // assert_eq! - 两个值必须相等
    assert_eq!(2 + 2, 4);

    // assert_ne! - 两个值必须不相等
    assert_ne!(2 + 2, 5);

    // 带自定义消息的断言
    assert!(true, "这应该为真");
    assert_eq!(1, 1, "1 应该等于 1");

    // debug_assert! - 只在 debug 模式下检查
    debug_assert!(true);
    debug_assert_eq!(1, 1);

    // unreachable! - 标记不可到达的代码
    // match some_value {
    //     A => ...,
    //     B => ...,
    //     _ => unreachable!("不应该到达这里"),
    // }

    // todo! - 标记未实现的功能
    // todo!("实现这个功能");

    // unimplemented! - 标记未实现的功能（已废弃，使用 todo!）
}
