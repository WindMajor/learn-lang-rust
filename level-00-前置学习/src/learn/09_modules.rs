// ============================================
// 09 - 包和模块 (Packages and Modules)
// ============================================
// Rust 的模块系统：包 -> crate -> 模块

// 模块的可见性
// pub: 公共，外部可访问
// pub(crate): 在当前 crate 内可见
// pub(super): 在父模块中可见
// 默认: 私有，只在当前模块可见

// ==================== 模块声明 ====================

// 内联模块
mod front_of_house {
    // 子模块
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }

        pub fn seat_at_table() {
            println!("安排座位");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("接受订单");
        }

        pub fn serve_order() {
            println!("上菜");
        }

        pub fn take_payment() {
            println!("收款");
        }

        // 私有函数 - 只能在 serving 模块内部使用
        fn private_helper() {
            println!("私有辅助函数");
        }
    }
}

// 使用 use 引入路径
use front_of_house::hosting;

// 使用 as 关键字重命名
use front_of_house::serving as svc;

// 使用 pub use 重新导出
pub use front_of_house::hosting::add_to_waitlist;

// ==================== 嵌套路径 ====================
// 使用嵌套路径清理 use 列表
// use std::collections::{HashMap, HashSet};
// use std::io::Write;

// 全局引入（谨慎使用）
// use std::collections::*;

pub fn run() {
    println!("===== 09 包和模块 =====\n");

    // 使用绝对路径调用
    crate::modules::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::seat_at_table();

    // 使用 use 引入后的调用
    hosting::add_to_waitlist();

    // 使用重命名的模块
    svc::take_order();
    svc::serve_order();
    svc::take_payment();

    // 使用重新导出的函数
    add_to_waitlist();

    // ==================== 模块文件组织 ====================
    // 模块可以放在单独的文件中
    // 例如：mod my_module; 会查找 my_module.rs 或 my_module/mod.rs

    // ==================== super 关键字 ====================
    // super 表示父模块，类似于文件系统中的 ..

    // ==================== self 关键字 ====================
    // self 表示当前模块

    println!();
}

// ==================== 结构体和枚举的可见性 ====================
mod back_of_house {
    // 结构体默认私有，字段也默认私有
    pub struct Breakfast {
        pub toast: String,      // 公共字段
        seasonal_fruit: String, // 私有字段
    }

    impl Breakfast {
        // 公共关联函数（构造函数）
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }

        pub fn get_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // 枚举只要标记 pub，所有变体都可见
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn demonstrate_visibility() {
    // 创建公共结构体
    let mut meal = back_of_house::Breakfast::summer("全麦");
    meal.toast = String::from("黑麦"); // 可以修改公共字段
    println!("我想吃 {} 吐司", meal.toast);
    println!("季节性水果: {}", meal.get_fruit());
    // meal.seasonal_fruit = String::from("蓝莓"); // 错误！私有字段

    // 使用公共枚举
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

// ==================== 模块路径总结 ====================
// crate::a::b::c      - 绝对路径，从 crate 根开始
// self::a::b::c       - 相对路径，从当前模块开始
// super::a::b::c      - 相对路径，从父模块开始
// a::b::c             - 相对路径，从当前模块开始

// ==================== 预lude ====================
// Rust 自动导入 std::prelude 中的内容
// 包括：Copy, Send, Sized, Sync, Unpin, Drop, Fn, FnMut, FnOnce,
//       Box, ToOwned, Clone, PartialEq, PartialOrd, Eq, Ord,
//       AsRef, AsMut, Into, From, Default, Iterator, Extend, IntoIterator,
//       DoubleEndedIterator, ExactSizeIterator, Option, Some, None,
//       Result, Ok, Err, String, ToString, Vec, Drop 等
