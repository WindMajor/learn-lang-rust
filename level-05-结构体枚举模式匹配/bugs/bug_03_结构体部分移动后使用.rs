// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 结构体部分移动后使用 —— 解构时拿走 String 字段    ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   解构结构体时，String 字段被移动到新变量，之后访问原结构体报错。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_结构体部分移动后使用.rs`:
//
//   error[E0382]: borrow of partially moved value: `user`
//     --> bug_03_结构体部分移动后使用.rs:XX:XX
//      |
//   XX |     let User { name, email, .. } = user;
//      |                  ----  ----- `user.email` partially moved here
//      |                  |
//      |                  `user.name` partially moved here
//   XX |     println!("{:?}", user);
//      |                      ^^^^ value borrowed here after partial move
//
// 【为什么会这样】
//   String 字段不实现 Copy，解构时被移动。
//   Struct 的 `..` 展开语法也会触发部分移动。
//
// 【如何修复】
//   方案1: 使用引用解构 let User { name, email, .. } = &user;
//         此时 name 和 email 都是 &String 类型
//   方案2: 先访问结构体再解构

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    // ❌ 解构会移动 name 和 email 的所有权
    // let User { name, email, .. } = user;
    // println!("{:?}", user); // ❌ 部分移动后不能再使用

    // ✅ 方案1: 用引用解构
    let User { name, email, active } = &user;
    println!("引用解构: {name}, {email}, {active}"); // &String, &bool
    println!("user 仍可用: {:?}", user); // ✅

    // ✅ 方案2: 先访问整体，再解构
    let user2 = User {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        active: false,
    };
    println!("先整体访问: {:?}", user2);
    let User { name: n, email: e, .. } = user2; // 重命名绑定
    println!("解构后: {n}, {e}");
    // user2 不再使用

    println!();
    println!("核心: 解构结构体时注意 Copy 类型和非 Copy 类型的区别");
    println!("      bool/i32 等 Copy 类型不会被移动");
    println!("      String/Vec 等非 Copy 类型会被移动");
}
