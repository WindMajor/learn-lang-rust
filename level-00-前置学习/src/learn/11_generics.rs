// ============================================
// 11 - 泛型 (Generics)
// ============================================
// 泛型允许定义灵活、可重用的代码

pub fn run() {
    println!("===== 11 泛型 =====\n");

    // ==================== 为什么需要泛型 ====================
    // 不用泛型，需要为每种类型写重复代码
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = [1, 2, 3, 4, 5];
    println!("最大 i32: {}", largest_i32(&numbers));

    let chars = ['a', 'b', 'c', 'd'];
    println!("最大 char: {}", largest_char(&chars));

    // ==================== 泛型函数 ====================
    // 使用泛型参数 T
    // T: PartialOrd 表示 T 必须实现 PartialOrd trait（可比较大小）
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("泛型 - 最大 i32: {}", largest(&numbers));
    println!("泛型 - 最大 char: {}", largest(&chars));

    // ==================== 泛型结构体 ====================
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Point<i32>: x={}, y={}", integer_point.x, integer_point.y);
    println!("Point<f64>: x={}, y={}", float_point.x, float_point.y);

    // 多个泛型参数
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let mixed = Point2 { x: 5, y: 4.0 };
    println!("Point2<i32, f64>: x={}, y={}", mixed.x, mixed.y);

    // ==================== 泛型枚举 ====================
    // Option<T> 和 Result<T, E> 都是泛型枚举
    enum MyOption<T> {
        Some(T),
        None,
    }

    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // ==================== 泛型方法 ====================
    impl<T> Point<T> {
        // 构造函数
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }

        // 获取引用
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    // 为特定类型实现方法
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point::new(3.0, 4.0);
    println!("距离原点: {}", p.distance_from_origin());

    // ==================== 泛型参数约束 ====================
    // 使用 where 子句使约束更清晰
    fn print_largest<T>(list: &[T])
    where
        T: PartialOrd + std::fmt::Display,
    {
        let largest = &list[0];
        for item in list {
            if item > largest {
                println!("更大的值: {}", item);
            }
        }
    }

    // ==================== 默认泛型参数 ====================
    // Rust 允许为泛型参数指定默认值
    trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // ==================== 泛型与生命周期 ====================
    // 泛型可以和生命周期一起使用
    fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: PartialOrd,
    {
        if x > y { x } else { y }
    }

    // ==================== 泛型代码的性能 ====================
    // Rust 使用单态化（Monomorphization）
    // 编译时会为每个具体类型生成专门的代码
    // 不会有运行时性能损失

    // ==================== const 泛型 ====================
    // Rust 1.51+ 支持 const 泛型
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    let arr = Array::<i32, 3> {
        data: [1, 2, 3],
    };
    println!("const 泛型数组长度: {}", arr.data.len());

    // const 泛型函数
    fn print_array<T: std::fmt::Display, const N: usize>(arr: [T; N]) {
        for item in arr {
            print!("{} ", item);
        }
        println!();
    }

    print_array([1, 2, 3, 4, 5]);
    print_array(['a', 'b', 'c']);

    // ==================== 泛型与动态分发 ====================
    // 使用 trait 对象可以实现动态分发
    fn print_anything(item: &dyn std::fmt::Display) {
        println!("dyn: {}", item);
    }

    print_anything(&42);
    print_anything(&"hello");

    println!();
}

// ==================== 泛型 trait 实现 ====================
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为所有实现了 Display 的类型实现 Summary
// impl<T: std::fmt::Display> Summary for T {
//     fn summarize(&self) -> String {
//         format!("(Read more: {})", self)
//     }
// }

// ==================== 关联类型 ====================
pub trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

// 与泛型的区别：
// 泛型：一个类型可以实现同一个 trait 多次（不同泛型参数）
// 关联类型：一个类型只能实现一次 trait

// ==================== 泛型与智能指针 ====================
use std::rc::Rc;
use std::sync::Arc;

pub fn demonstrate_generic_pointers() {
    // Box<T> - 堆分配
    let b = Box::new(5);
    println!("Box: {}", b);

    // Rc<T> - 引用计数（单线程）
    let rc = Rc::new(String::from("共享数据"));
    let rc2 = Rc::clone(&rc);
    println!("Rc 引用计数: {}", Rc::strong_count(&rc));
    println!("Rc 数据: {}", rc2);

    // Arc<T> - 原子引用计数（多线程）
    let arc = Arc::new(vec![1, 2, 3]);
    let arc2 = Arc::clone(&arc);
    println!("Arc 数据: {:?}", arc2);
}
