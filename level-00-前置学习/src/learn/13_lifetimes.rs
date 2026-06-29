// ============================================
// 13 - 生命周期 (Lifetimes)
// ============================================
// 生命周期确保引用总是有效，防止悬垂引用

pub fn run() {
    println!("===== 13 生命周期 =====\n");

    // ==================== 为什么需要生命周期 ====================
    // Rust 编译器使用借用检查器来确保引用有效
    // 生命周期标注帮助编译器理解引用的有效范围

    // ==================== 生命周期标注语法 ====================
    // &i32        - 引用
    // &'a i32     - 带有显式生命周期的引用
    // &'a mut i32 - 带有显式生命周期的可变引用

    // ==================== 函数中的生命周期 ====================
    // 返回两个字符串切片中较长的一个
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是: {}", result);
    }

    // ==================== 生命周期标注规则 ====================
    // 1. 每个引用参数都有自己的生命周期
    // 2. 如果只有一个输入生命周期，它被赋予所有输出生命周期
    // 3. 如果有多个输入生命周期，但一个是 &self 或 &mut self，
    //    self 的生命周期被赋予所有输出生命周期

    // ==================== 省略生命周期 ====================
    // 编译器可以自动推断生命周期（生命周期省略规则）
    fn first_word(s: &str) -> &str {
        // 等价于 fn first_word<'a>(s: &'a str) -> &'a str
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    println!("第一个单词: '{}'", first_word("hello world"));

    // ==================== 结构体中的生命周期 ====================
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        // 生命周期省略规则适用
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("注意！{}", announcement);
            self.part
        }
    }

    let novel = String::from("很久以前...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("摘录: {}", excerpt.part);
    println!("级别: {}", excerpt.level());

    // ==================== 静态生命周期 ====================
    // 'static 生命周期持续整个程序运行期间
    // 字符串字面量具有 'static 生命周期
    let s: &'static str = "我有一个静态生命周期";
    println!("{}", s);

    // 谨慎使用 'static！大多数情况表示有内存泄漏或设计问题

    // ==================== 泛型、Trait Bound 和生命周期 ====================
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("公告！{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest_with_an_announcement(
        "hello",
        "world",
        "今天是个好天气！",
    );
    println!("结果: {}", result);

    // ==================== 生命周期子类型 ====================
    // 'a: 'b 表示 'a 至少和 'b 一样长
    fn some_function<'a, 'b>(x: &'a str, y: &'b str) -> &'b str
    where
        'a: 'b, // 'a 至少和 'b 一样长
    {
        if x.len() > y.len() { y } else { y }
    }

    // ==================== 高级生命周期 ====================

    // 生命周期约束 trait bound
    trait MyTrait<'a> {}
    fn my_function<T: MyTrait<'static>>(_t: T) {}

    // 在 trait 中使用生命周期
    trait Parser<'a> {
        type Output;
        fn parse(&self, input: &'a str) -> Option<Self::Output>;
    }

    // ==================== 生命周期在类型中的使用 ====================
    struct Borrowed<'a> {
        value: &'a i32,
    }

    let x = 5;
    let borrowed = Borrowed { value: &x };
    println!("借用的值: {}", borrowed.value);

    // ==================== 多重生命周期 ====================
    struct MultiBorrowed<'a, 'b> {
        x: &'a i32,
        y: &'b str,
    }

    // ==================== 生命周期与返回 self ====================
    // 方法链式调用时需要正确的生命周期
    struct Builder<'a> {
        data: &'a mut String,
    }

    impl<'a> Builder<'a> {
        fn append(&mut self, s: &str) -> &mut Self {
            self.data.push_str(s);
            self
        }

        fn get_data(&self) -> &str {
            self.data
        }
    }

    let mut data = String::from("Hello");
    let mut binding = Builder { data: &mut data };
    binding
        .append(", ")
        .append("World!");
    let result = binding.get_data();
    println!("构建结果: {}", result);

    // ==================== 常见生命周期模式 ====================

    // 1. 输入生命周期 -> 输出生命周期
    fn identity<'a>(x: &'a str) -> &'a str {
        x
    }

    // 2. 多个输入，返回其中一个
    fn choose<'a>(first: &'a str, _second: &'a str) -> &'a str {
        first
    }

    // 3. 返回新分配的内存（不需要输入生命周期）
    fn create_string() -> String {
        String::from("新字符串")
    }

    // ==================== 生命周期与闭包 ====================
    fn make_adder<'a>(x: &'a i32) -> impl Fn(i32) -> i32 + 'a {
        move |y| x + y
    }

    let x = 10;
    let adder = make_adder(&x);
    println!("闭包结果: {}", adder(5));

    // ==================== 生命周期与迭代器 ====================
    struct Words<'a> {
        text: &'a str,
        position: usize,
    }

    impl<'a> Words<'a> {
        fn new(text: &'a str) -> Self {
            Words { text, position: 0 }
        }
    }

    impl<'a> Iterator for Words<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            let remaining = &self.text[self.position..];
            let start = remaining.find(|c: char| !c.is_whitespace())?;
            let end = remaining[start..]
                .find(|c: char| c.is_whitespace())
                .unwrap_or(remaining.len() - start);

            self.position += start + end;
            Some(&remaining[start..start + end])
        }
    }

    let text = "hello world foo bar";
    let words: Vec<_> = Words::new(text).collect();
    println!("单词: {:?}", words);

    // ==================== 生命周期推断 ====================
    // 大多数情况下，编译器会自动推断生命周期
    // 只有在编译器无法推断时才需要显式标注

    // 需要显式标注的情况：
    // 1. 函数返回引用
    // 2. 结构体包含引用
    // 3. trait 中使用引用
    // 4. 复杂的多重引用关系

    println!();
}

// ==================== 生命周期进阶示例 ====================

// 自引用结构体是 Rust 中高级话题，需要使用 Pin 固定内存位置
// 这里展示一个简化版本：
use std::pin::Pin;

struct PinnedData {
    data: String,
}

impl PinnedData {
    fn new(data: String) -> Self {
        PinnedData { data }
    }

    fn get_data(self: Pin<&Self>) -> &str {
        &self.get_ref().data
    }
}

// ==================== Cow（Clone on Write） ====================
use std::borrow::Cow;

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果需要修改，才会克隆
            input.to_mut()[i] = -v;
        }
    }
}

pub fn demonstrate_cow() {
    // 不需要修改，不克隆
    let input = [1, 2, 3];
    let mut cow = Cow::from(&input[..]);
    abs_all(&mut cow);
    println!("Cow (不修改): {:?}", cow);

    // 需要修改，会克隆
    let input = [1, -2, 3];
    let mut cow = Cow::from(&input[..]);
    abs_all(&mut cow);
    println!("Cow (修改): {:?}", cow);
}
