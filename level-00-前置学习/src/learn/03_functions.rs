// ============================================
// 03 - 函数 (Functions)
// ============================================

pub fn run() {
    println!("===== 03 函数 =====\n");

    // 调用函数
    say_hello();

    // 带参数的函数
    greet("Rust");

    // 多个参数
    print_sum(5, 3);

    // 有返回值的函数
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // 可以直接在表达式位置调用
    println!("5 + 3 = {}", add(5, 3));

    // 函数返回值可以直接赋值
    let x = five();
    println!("five() = {}", x);

    // 带有多个返回值的函数（使用元组）
    let (sum, diff) = calculate(10, 3);
    println!("calculate(10, 3): sum={}, diff={}", sum, diff);

    // 使用语句块作为表达式
    let y = {
        let a = 3;
        let b = 4;
        a + b // 注意：没有分号，这是表达式，会返回值
    };
    println!("语句块表达式结果: {}", y);

    // 带类型参数的泛型函数
    println!("最大值为: {}", max(5, 3));
    println!("最大值为: {}", max(3.14, 2.71));

    // 递归函数
    println!("阶乘 5! = {}", factorial(5));

    // 函数指针
    let f: fn(i32, i32) -> i32 = add;
    println!("通过函数指针调用: f(2, 3) = {}", f(2, 3));

    println!();
}

// 无参数、无返回值的函数
fn say_hello() {
    println!("Hello, Rust!");
}

// 带参数的函数
// 必须声明每个参数的类型
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 多个参数
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// 有返回值的函数
// 使用 -> 指定返回类型
// 返回值是函数体中最后一个表达式（无分号）
fn add(a: i32, b: i32) -> i32 {
    a + b // 没有分号，这是表达式，会被返回
    // 等价于: return a + b;
}

// 返回语句也可以显式使用 return
fn five() -> i32 {
    return 5;
}

// 返回元组（多个值）
fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// 泛型函数
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 递归函数
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// ==================== 函数相关概念 ====================

// 关联函数（静态方法）
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数 - 不使用 self，通常用作构造函数
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 方法 - 使用 &self
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可变方法 - 使用 &mut self
    pub fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// 闭包 (Closure) - 匿名函数
// 可以在函数内部定义
pub fn demonstrate_closure() {
    // 基本闭包
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("闭包: add_one(5) = {}", add_one(5));

    // 类型推断的闭包
    let multiply = |x, y| x * y;
    println!("闭包: multiply(4, 5) = {}", multiply(4, 5));

    // 捕获环境的闭包
    let offset = 10;
    let add_offset = |x| x + offset;
    println!("捕获环境: add_offset(5) = {}", add_offset(5));
}
