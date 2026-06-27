// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: 部分移动(Partial Move) —— 对结构体字段使用后    ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   结构体的部分字段被移动后，整个结构体不再可用。
//   即使其他字段还在，也不能再访问结构体整体。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_部分移动后访问结构体.rs`:
//
//   error[E0382]: borrow of partially moved value: `user`
//     --> bug_03_部分移动后访问结构体.rs:XX:XX
//      |
//   XX |     let user = User::new("Alice", "alice@example.com");
//      |         ---- move occurs because `user` has type `User`,
//      |              which does not implement the `Copy` trait
//   XX |     let email = user.email;
//      |                 ---------- `user.email` partially moved here
//   XX |     println!("用户: {:?}", user);
//      |                            ^^^^ value borrowed here after partial move
//      |
//   help: consider calling `.clone()` to prevent partial move
//      |
//   XX |     let email = user.email.clone();
//      |                           ++++++++
//
// 【为什么会这样】
//   Rust 不允许"部分有效"的结构体。如果 email 字段被移走，
//   结构体整体处于不一致状态（email 字段位置的数据不再有效），
//   因此编译器禁止访问整个结构体。但访问未被移动的字段是允许的。
//
//   CONTRAST:
//   - C++: 没有"Move 后不可访问"的概念，你甚至可以在 std::move 后
//          继续使用对象（未定义行为，编译器不报错）
//   - GC 语言: 没有 Move 概念，所以不会有此类问题
//
// 【如何修复】
//   方案1: 如果不需要所有权，使用借用: let email = &user.email;
//   方案2: 在所有字段被访问完毕后再移动字段
//   方案3: 先访问整个结构体，再移动字段

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

impl User {
    fn new(name: &str, email: &str) -> Self {
        User {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

fn main() {
    // ─── 错误演示 ───
    // let user = User::new("Alice", "alice@example.com");
    // let email = user.email;            // email 字段被移走！
    // println!("用户: {:?}", user);      // ❌ user 处于"部分移动"状态
    // println!("email: {}", email);      // ✅ email 单独可用

    // ─── 修复方案1: 借用 ───
    {
        println!("═══ 修复方案1: 借用字段 ───");
        let user = User::new("Alice", "alice@example.com");
        let email = &user.email; // 借用，不移动所有权
        println!("用户: {:?}", user); // ✅ 整个结构体仍可用
        println!("email: {email}");
    }

    // ─── 修复方案2: 先整体访问再部分移动 ───
    {
        println!();
        println!("═══ 修复方案2: 调整访问顺序 ───");
        let user = User::new("Bob", "bob@example.com");
        println!("用户: {:?}", user); // 先访问整体
        let email = user.email; // 再部分移动
        // user 之后不再访问即可
        println!("email: {email}");
    }

    // ─── 注意：可以访问未被移动的字段 ───
    {
        println!();
        println!("═══ 访问未被移动的字段是允许的 ───");
        let user = User::new("Charlie", "charlie@example.com");
        let email = user.email; // email 被移走
        // println!("{:?}", user); // ❌ 整体不行
        println!("name: {}  (未被移动的字段)", user.name); // ✅ 可以
        println!("email: {email}");
    }

    println!();
    println!("核心教训: 部分移动后，结构体整体不可用，但未移动字段可单独访问");
    println!("对比: C++ 允许 struct s; auto e = std::move(s.email); cout << s;");
    println!("      这在C++中是合法的但 s.email 处于未定义状态——编译器不警告!");
}
