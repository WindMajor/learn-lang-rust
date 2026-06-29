// ============================================
// 12 - Trait（特征）
// ============================================
// Trait 定义了类型共享的行为，类似于其他语言的接口

pub fn run() {
    println!("===== 12 Trait =====\n");

    // ==================== 定义 Trait ====================
    pub trait Summary {
        // 必需方法
        fn summarize_author(&self) -> String;

        // 默认实现
        fn summarize(&self) -> String {
            format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
        }
    }

    // ==================== 为类型实现 Trait ====================
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }

        // 可以选择覆盖默认实现
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // 使用
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 条新推文: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("大新闻！"),
        location: String::from("中国"),
        author: String::from("张三"),
        content: String::from("今天发生了大事..."),
    };

    println!("新闻摘要: {}", article.summarize());

    // ==================== Trait 作为参数 ====================
    pub fn notify(item: &impl Summary) {
        println!("突发新闻！{}", item.summarize());
    }

    notify(&tweet);

    // ==================== Trait Bound 语法 ====================
    // 上面的语法是下面语法的语法糖
    pub fn notify2<T: Summary>(item: &T) {
        println!("突发新闻！{}", item.summarize());
    }

    notify2(&article);

    // 多个参数
    pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
        // item1 和 item2 可以是不同类型
        println!("{} 和 {}", item1.summarize(), item2.summarize());
    }

    pub fn notify4<T: Summary>(item1: &T, item2: &T) {
        // item1 和 item2 必须是相同类型
        println!("{} 和 {}", item1.summarize(), item2.summarize());
    }

    // ==================== 多个 Trait Bound ====================
    use std::fmt::Display;

    pub fn notify5(item: &(impl Summary + Display)) {
        println!("显示: {}", item);
    }

    pub fn notify6<T: Summary + Display>(item: &T) {
        println!("显示: {}", item);
    }

    // ==================== where 子句 ====================
    // 简化复杂的 trait bound
    pub fn some_function<T, U>(_t: &T, _u: &U)
    where
        T: Summary + Clone,
        U: Display + Clone,
    {
        println!("使用 where 子句");
    }

    // ==================== 返回实现了 Trait 的类型 ====================
    pub fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("rust_lang"),
            content: String::from("Rust 1.70 发布了！"),
            reply: false,
            retweet: false,
        }
    }

    println!("返回的摘要: {}", returns_summarizable().summarize());

    // 注意：只能返回单一类型
    // 以下代码会编译错误：
    // pub fn returns_summarizable2(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle { ... }
    //     } else {
    //         Tweet { ... } // 错误！返回了不同类型
    //     }
    // }

    // 如果需要返回不同类型，使用 trait 对象
    pub fn returns_summarizable_boxed(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(NewsArticle {
                headline: String::from("新闻"),
                location: String::from("北京"),
                author: String::from("记者"),
                content: String::from("内容"),
            })
        } else {
            Box::new(Tweet {
                username: String::from("user"),
                content: String::from("推文"),
                reply: false,
                retweet: false,
            })
        }
    }

    println!(
        "Box<dyn Summary>: {}",
        returns_summarizable_boxed(true).summarize()
    );

    // ==================== 使用 Trait Bound 有条件地实现方法 ====================
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Pair { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大的成员是 x = {}", self.x);
            } else {
                println!("最大的成员是 y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(5, 3);
    pair.cmp_display();

    // ==================== 为实现了某 Trait 的类型自动实现 Trait ====================
    // 这称为 blanket implementations
    // impl<T: Display> ToString for T {
    //     // ...
    // }
    // 这意味着任何实现了 Display 的类型自动实现了 ToString

    let s = 3.to_string();
    println!("自动实现: {}", s);

    // ==================== 常用标准库 Trait ====================

    // 1. Display - 用于用户友好的格式化输出
    //    使用 {} 格式说明符

    // 2. Debug - 用于调试输出
    //    使用 {:?} 或 {:#?} 格式说明符
    //    可以使用 #[derive(Debug)] 自动生成

    // 3. PartialEq / Eq - 相等性比较
    //    PartialEq: 允许部分相等比较（浮点数）
    //    Eq: 完全相等（自反性）

    // 4. PartialOrd / Ord - 排序比较

    // 5. Clone - 显式深拷贝
    //    需要调用 .clone() 方法

    // 6. Copy - 隐式按位复制（栈上数据）
    //    不能和 Drop 同时实现

    // 7. Default - 默认值
    //    可以使用 #[derive(Default)]

    // 8. Hash - 哈希计算

    // 9. Iterator - 迭代器

    // 10. Drop - 析构时调用

    // ==================== 实现常用 Trait 示例 ====================
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("Display: {}", p1);
    println!("Debug: {:?}", p1);
    println!("相等: {}", p1 == p2);
    println!("默认: {:?}", Point::default());

    // ==================== Trait 对象 vs 泛型 ====================
    // 泛型 - 静态分发（单态化），编译时确定类型，零运行时开销
    fn do_something_generic<T: Summary>(item: &T) {
        println!("泛型: {}", item.summarize());
    }

    // Trait 对象 - 动态分发，运行时使用虚表（vtable），有运行时开销
    fn do_something_dyn(item: &dyn Summary) {
        println!("动态分发: {}", item.summarize());
    }

    do_something_generic(&tweet);
    do_something_dyn(&tweet);

    // ==================== Self 和 Sized ====================
    // trait MyTrait {
    //     fn create() -> Self;        // 返回自身类型
    //     fn compare(&self, other: &Self); // 比较相同类型
    // }

    // trait MyTrait2 {
    //     fn method(&self) where Self: Sized; // 只在类型大小已知时可用
    // }

    // ==================== 关联类型 ====================
    trait Iterator2 {
        type Item; // 关联类型
        fn next(&mut self) -> Option<Self::Item>;
    }

    // 与泛型 trait 的区别：
    // trait IteratorGeneric<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
    // 一个类型可以实现 IteratorGeneric 多次（不同 T）
    // 但只能实现 Iterator2 一次（只有一个 Item）

    // ==================== 完全限定语法 ====================
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // 用于消除方法名冲突

    println!();
}

// ==================== 自定义 Trait 示例 ====================

// 可绘制 trait
pub trait Drawable {
    fn draw(&self);
    fn draw_at(&self, x: i32, y: i32);
}

// 可调整大小 trait
pub trait Resizable {
    fn resize(&mut self, width: u32, height: u32);
    fn scale(&mut self, factor: f64);
}

// 可序列化 trait
pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Self
    where
        Self: Sized;
}

// 组合 trait
pub trait Widget: Drawable + Resizable {
    fn bounds(&self) -> (u32, u32, u32, u32);
}
