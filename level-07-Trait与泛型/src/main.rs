// ================================================================
// Level 07: Trait 与泛型
// 目标: Trait 定义/实现、泛型单态化、Trait bound、关联类型、孤儿规则
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ TS           │ Go           │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 接口        │ Trait        │ interface    │ interface    │
// │ 实现声明    │ 显式 impl    │ implements   │ 自动隐式     │
// │ 泛型实现    │ 单态化(零成本)│ 类型擦除     │ 无泛型      │
// │ 分发        │ 静态(默认)   │ 动态(Class)  │ 动态        │
// │ 空接口      │ dyn Any      │ unknown      │ interface{} │
// │ 扩展方法    │ impl Trait   │ declaration  │ 隐式        │
// └────────────┴──────────────┴──────────────┴──────────────┘

// ─── 1. Trait 定义与实现 ───
/// 摘要特征：任何可以被摘要的类型
/// CONTRAST:
///   - TS:   interface Summary { summary(): string }
///   - Go:   type Summary interface { Summary() string }
///   - C++:  class Summary { virtual string summary() = 0; }
///   - Java: interface Summary { String summary(); }
///   - Swift: protocol Summary { func summary() -> String }
trait Summary {
    // WHAT: 方法签名（可以有默认实现）
    fn summarize(&self) -> String;

    // 带默认实现的方法
    fn describe(&self) -> String {
        format!("(默认描述) {}", self.summarize())
    }
}

/// 一个简单的文章结构体
struct Article {
    title: String,
    content: String,
}

/// 为 Article 实现 Summary
/// CONTRAST:
///   - TS:   class Article implements Summary { ... }
///           TS 在 class 定义处声明实现，Rust 在 impl 块中分开声明
///   - Go:   只需要实现 Summary() 方法，自动满足接口（不需要显式声明）
///   - C++:  class Article : public Summary { ... }
///   关键差异: Rust 的 trait 实现是分离的（impl Trait for Type），
///           这意味着你可以在不修改类型定义的情况下为它添加行为
impl Summary for Article {
    fn summarize(&self) -> String {
        format!(
            "{}: {}...",
            self.title,
            self.content.chars().take(20).collect::<String>()
        )
    }
}

/// 推文结构体
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "@{}: {}",
            self.username,
            // 安全截取前 30 个字符（而非字节），避免 UTF-8 边界问题
            self.content.chars().take(30).collect::<String>()
        )
    }
    // describe 使用默认实现
}

// ─── 2. 泛型函数 ───
/// 泛型函数：接受任何实现了 Summary 的类型
/// WHAT: T: Summary 是 trait bound，要求 T 必须实现 Summary
/// WHY: 编译期单态化——为每个具体类型生成专门的代码，零运行时开销
/// CONTRAST:
///   - TS:    function notify<T extends Summary>(item: T) // 类型擦除
///   - C++:   template<typename T> void notify(const T& item) // 编译期展开（无约束）
///   - Java:  <T extends Summary> void notify(T item) // 类型擦除+运行时检查
///   - Go:    func notify(s Summary) // 动态分发（接口值 = type+ptr）
///   关键差异:
///     Rust 泛型 = C++ 模板的零成本 + Java 的 trait bound 约束
///     比 C++ 模板多了编译期检查（不会出现几百行的模板错误信息）
///     比 Java 泛型少了运行时开销（单态化，不是擦除）
fn notify<T: Summary>(item: &T) {
    println!("  📢 通知: {}", item.summarize());
}

// 语法糖: impl Trait 作为参数（等价于上面的 T: Summary）
fn notify_impl_trait(item: &impl Summary) {
    println!("  📢 通知(impl Trait): {}", item.summarize());
}

// ─── 3. 关联类型 ───
/// 集合特征
/// CONTRAST:
///   - TS:   没有直接对应（可用泛型接口 extends）
///   - C++:  template<typename T> class Container { using value_type = T; }
///   - Swift: protocol Container { associatedtype Item }
///   - Java:  interface Container<T> { T get(); }  // 泛型接口（不是关联类型）
trait Container {
    type Item; // 关联类型 —— 每个实现指定具体类型

    fn get(&self) -> Option<&Self::Item>;
    fn is_empty(&self) -> bool;
}

/// String 作为字符容器的包装
struct CharContainer {
    data: String,
}

impl Container for CharContainer {
    type Item = char; // 指定关联类型

    fn get(&self) -> Option<&Self::Item> {
        // 简化实现：返回第一个字符的引用
        self.data.chars().next().as_ref()?;
        // 实际用 fold/collect 更方便，这里简化
        None // 简化
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ─── 4. Trait 作为参数：静态分发 vs 动态分发 ───
/// 静态分发：泛型单态化
fn static_dispatch<T: Summary>(item: &T) {
    println!("  [静态分发] {}", item.summarize());
    // CONTRAST:
    //   编译期为每个 T 生成独立的函数副本——类似于 C++ 模板
    //   零运行时开销，但增加编译时间和二进制大小
}

/// 动态分发：trait object
fn dynamic_dispatch(item: &dyn Summary) {
    println!("  [动态分发] {}", item.summarize());
    // CONTRAST:
    //   使用虚函数表(vtable)实现运行时多态 —— 类似于 C++ 虚函数、Java 接口
    //   有微小的运行时开销（虚表查找），但灵活
    //
    //   选择指南:
    //   - 静态分发: 类型已知且不会太多变体（大部分场景）
    //   - 动态分发: 需要在运行时存储不同类型（heterogeneous collection）
}

// ─── 5. 标准库 Trait 速览 ───
/// 演示常用的标准库 trait
/// CONTRAST: 这些 trait 相当于 C++ 的运算符重载 + Java 的 Cloneable/Comparable 等
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

// 手动实现 Display（类似 C++ operator<<、Go Stringer、Java toString）
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 07: Trait 与泛型              ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. Trait 基本使用 ───
    println!("━━━ 1. Trait 实现与方法调用 ━━━");
    {
        let article = Article {
            title: String::from("Rust 所有权详解"),
            content: String::from("Rust 的所有权系统是编程语言设计史上的重大创新..."),
        };
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Rust 1.75 发布！编译速度提升 20%"),
        };

        println!("  Article: {}", article.summarize());
        println!("  Tweet:   {}", tweet.summarize());
        println!();
        println!("  默认实现: {}", tweet.describe());
    }
    println!();

    // ─── 2. 泛型函数与 Trait Bound ───
    println!("━━━ 2. 泛型与 Trait Bound ━━━");
    {
        let article = Article {
            title: String::from("泛型"),
            content: String::from("单态化实现零成本"),
        };
        notify(&article);
        notify_impl_trait(&article);
    }
    println!();

    // ─── 3. 单态化 vs 擦除 ───
    println!("━━━ 3. 单态化 vs 类型擦除 ━━━");
    {
        let article = Article {
            title: String::from("对比"),
            content: String::from("Rust 单态化 vs Java 擦除"),
        };
        let tweet = Tweet {
            username: String::from("dev"),
            content: String::from("零成本抽象"),
        };

        // 静态分发：编译期为 Article 和 Tweet 各生成一份代码
        static_dispatch(&article);
        static_dispatch(&tweet);

        // 动态分发：基于虚表，可以存到 Vec 中
        let items: Vec<&dyn Summary> = vec![&article, &tweet];
        for item in &items {
            println!("  [Vec<dyn Summary>] {}", item.summarize());
        }

        // CONTRAST:
        // Rust: 默认静态分发（单个类型 T），需要时才用 dyn
        // Java: 默认动态分发（所有接口调用都走虚表）
        // Go:   默认动态分发（interface 值 = type+data ptr）
        // C++:  模板 = 静态，虚函数 = 动态（需要手动 virtual）
        // TS:   基于原型的动态分发
    }
    println!();

    // ─── 4. 标准库 Trait 实验 ───
    println!("━━━ 4. 标准库 Trait ━━━");
    {
        let p1 = Point { x: 10, y: 20 };
        let p2 = Point { x: 10, y: 20 };
        let p3 = Point { x: 5, y: 30 };

        println!("  p1: {p1}"); // Display
        println!("  p1 == p2: {}", p1 == p2); // PartialEq
        println!("  p1 < p3:  {}", p1 < p3); // PartialOrd (先比 x 再比 y)
        println!("  p1.clone(): {:?}", p1.clone()); // Clone + Debug

        // 使用 dbg! 查看 - 它使用 Debug trait
        dbg!(&p1);
    }
    println!();

    // ─── 5. 孤儿规则演示 ───
    println!("━━━ 5. 孤儿规则 (Orphan Rule) ━━━");

    // 创建本地包装类型以绕过孤儿规则
    struct MyString(String);

    // ✅ 可以为本地类型实现外部 trait
    impl std::fmt::Display for MyString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyString: {}", self.0)
        }
    }
    println!("  {} ", MyString(String::from("孤儿规则实验")));
    println!("  (我们为本地类型 MyString 实现了标准库 Display trait)");
    println!();
    println!("  孤儿规则:");
    println!("  你不能为外部类型实现外部 trait。例如:");
    println!("  impl Summary for Vec<String> {{ }}  // ❌ 两者都是外部的");
    println!("  impl Display for Vec<u8> {{ }}       // ❌ 同上");
    println!("  解决方案: newtype 模式（包装外部类型为本地类型）");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 07 通关！继续 Level 08        ║");
    println!("╚══════════════════════════════════════╝");
}
