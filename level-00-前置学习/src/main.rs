#![allow(dead_code)]
// ============================================
// Rust 学习项目 - 主入口
// ============================================
// 运行所有学习模块

#[path = "learn/01_variables.rs"]
mod variables;
#[path = "learn/02_data_types.rs"]
mod data_types;
#[path = "learn/03_functions.rs"]
mod functions;
#[path = "learn/04_control_flow.rs"]
mod control_flow;
#[path = "learn/05_ownership.rs"]
mod ownership;
#[path = "learn/06_structs.rs"]
mod structs;
#[path = "learn/07_enum_and_pattern_matching.rs"]
mod enum_and_pattern_matching;
#[path = "learn/08_collections.rs"]
mod collections;
#[path = "learn/09_modules.rs"]
mod modules;
#[path = "learn/10_error_handling.rs"]
mod error_handling;
#[path = "learn/11_generics.rs"]
mod generics;
#[path = "learn/12_traits.rs"]
mod traits;
#[path = "learn/13_lifetimes.rs"]
mod lifetimes;

fn main() {
    println!("Hello, world!");

    println!("╔══════════════════════════════════════════╗");
    println!("║       Rust 语法与功能学习项目             ║");
    println!("╚══════════════════════════════════════════╝\n");

    // 01 - 变量与可变性
    variables::run();

    // 02 - 数据类型
    data_types::run();

    // 03 - 函数
    functions::run();
    functions::demonstrate_closure();

    // 04 - 流程控制
    control_flow::run();

    // 05 - 所有权
    ownership::run();

    // 06 - 结构体
    structs::run();
    structs::demonstrate_derive();

    // 07 - 枚举和模式匹配
    enum_and_pattern_matching::run();

    // 08 - 常见集合及操作
    collections::run();

    // 09 - 包和模块
    modules::run();
    modules::demonstrate_visibility();

    // 10 - 错误处理
    error_handling::run();
    error_handling::demonstrate_assertions();

    // 11 - 泛型
    generics::run();
    generics::demonstrate_generic_pointers();

    // 12 - Trait
    traits::run();

    // 13 - 生命周期
    lifetimes::run();
    lifetimes::demonstrate_cow();

    println!("\n✅ 所有学习模块运行完毕！");
}
