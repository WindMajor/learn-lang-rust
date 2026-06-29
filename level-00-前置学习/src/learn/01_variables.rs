// ============================================
// 01 - 变量与可变性 (Variables & Mutability)
// ============================================

pub fn run() {
    println!("===== 01 变量与可变性 =====\n");

    // 1. 不可变变量 (默认)
    // 使用 let 声明的变量默认是不可变的
    let x = 5;
    println!("不可变变量 x = {}", x);
    // x = 6; // 错误！不能对不可变变量重新赋值

    // 2. 可变变量
    // 使用 mut 关键字使变量可变
    let mut y = 5;
    println!("可变变量 y 初始值 = {}", y);
    y = 6;
    println!("修改后 y = {}", y);

    // 3. 常量 (Constants)
    // 常量总是不可变的，使用 const 声明，必须标注类型
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);
    // 常量可以在任何作用域中声明，包括全局作用域
    // 常量只能被设置为常量表达式，不能是运行时计算的值

    // 4. 变量遮蔽 (Shadowing)
    // 可以用相同的名字声明新变量，新变量会遮蔽旧变量
    let z = 5;
    let z = z + 1; // 这里创建了一个新变量 z
    let z = z * 2;
    println!("经过遮蔽后的 z = {}", z);

    // 5. 遮蔽与 mut 的区别
    // 遮蔽允许改变类型，而 mut 不行
    let spaces = "    "; // String 类型
    let spaces = spaces.len(); // 遮蔽后变为 usize 类型
    println!("空格数量 = {}", spaces);

    // let mut spaces2 = "    ";
    // spaces2 = spaces2.len(); // 错误！不能改变类型

    // 6. 下划线前缀 - 忽略未使用的变量
    let _unused = 42; // 编译器不会警告未使用

    println!();
}
