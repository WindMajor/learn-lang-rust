// ============================================
// 06 - 结构体 (Structs)
// ============================================
// 自定义数据类型，用于将相关数据组合在一起

pub fn run() {
    println!("===== 06 结构体 =====\n");

    // ==================== 定义结构体 ====================
    // 1. 命名结构体 (Named Struct)
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 2. 创建实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 访问字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);

    // 修改字段（需要实例是可变的）
    user1.email = String::from("another@example.com");
    println!("修改后的邮箱: {}", user1.email);

    // 3. 使用字段初始化简写语法
    let email = String::from("new@example.com");
    let username = String::from("newuser");
    let user2 = User {
        email,      // 等价于 email: email
        username,   // 等价于 username: username
        active: false,
        sign_in_count: 0,
    };
    println!("user2: {} <{}>", user2.username, user2.email);

    // 4. 从其他实例创建（结构体更新语法）
    let user3 = User {
        email: String::from("third@example.com"),
        ..user2 // 其余字段使用 user2 的值
    };
    println!("user3: {} <{}>, active={}", user3.username, user3.email, user3.active);

    // ==================== 元组结构体 ====================
    // 没有命名字段，只有类型
    struct Point(i32, i32, i32);
    struct Color(u8, u8, u8);

    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);

    // 通过索引访问
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("黑色: RGB({}, {}, {})", black.0, black.1, black.2);

    // ==================== 单元结构体 ====================
    // 没有字段，常用于实现 trait
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // ==================== 结构体方法 ====================
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // 实现块
    impl Rectangle {
        // 构造函数（关联函数）
        fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        // 正方形构造函数
        fn square(size: u32) -> Self {
            Rectangle {
                width: size,
                height: size,
            }
        }

        // 方法 - 第一个参数是 &self
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // 方法 - 带参数
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // 可变方法
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }

        // getter 方法
        fn width(&self) -> u32 {
            self.width
        }
    }

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::square(20);

    println!("rect1: {:?}", rect1);
    println!("rect1 面积: {}", rect1.area());
    println!("rect1 宽度: {}", rect1.width()); // 调用 getter
    println!("rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3: {}", rect1.can_hold(&rect3));

    let mut rect4 = Rectangle::new(10, 20);
    rect4.scale(2);
    println!("放大后: {:?}", rect4);

    // ==================== 多个 impl 块 ====================
    // 一个结构体可以有多个 impl 块
    impl Rectangle {
        fn height(&self) -> u32 {
            self.height
        }
    }

    // ==================== 结构体与所有权 ====================
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 使用解构获取字段
    let Person { name, age } = person;
    println!("{} 的年龄是 {}", name, age);
    // person 现在无效了，因为 name 字段是 String，发生了移动

    // 如果只想借用字段，使用引用
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    let Person { name: ref n, age } = person2;
    println!("引用解构: {} 的年龄是 {}", n, age);
    // person2 仍然有效，因为我们只借了 name

    println!();
}

// ==================== 常用派生 trait ====================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point2D {
    x: i32,
    y: i32,
}

pub fn demonstrate_derive() {
    let p1 = Point2D { x: 1, y: 2 };
    let p2 = p1; // Copy，因为实现了 Copy
    println!("p1: {:?}, p2: {:?}", p1, p2);

    let p3 = p1.clone();
    println!("clone: {:?}", p3);

    let p4 = Point2D::default();
    println!("default: {:?}", p4);

    println!("相等: {}", p1 == p2);
}
