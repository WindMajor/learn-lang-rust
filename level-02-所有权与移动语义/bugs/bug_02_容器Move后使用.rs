// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 将 Move 语义的类型放入容器后再次使用              ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   将一个 String 放入 Vec 后，所有权转移到 Vec，再次访问原变量出错。
//   这与 bug_01 本质相同，但发生在容器操作中，更容易被忽略。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_容器Move后使用.rs`:
//
//   error[E0382]: borrow of moved value: `name`
//     --> bug_02_容器Move后使用.rs:XX:XX
//      |
//   XX |     let name = String::from("Alice");
//      |         ---- move occurs because `name` has type `String`,
//      |              which does not implement the `Copy` trait
//   XX |     names.push(name);
//      |                ---- value moved here
//   XX |     println!("name = {}", name);
//      |                            ^^^^ value borrowed here after move
//
// 【为什么会这样】
//   Vec::push 接收 T（不是 &T），因此所有权会从 name 转移到 Vec。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    vector<string> names;
//             string name = "Alice";
//             names.push_back(name);    // 拷贝！name 仍然可用
//             names.push_back(std::move(name)); // 移动！name 状态未定义
//             cout << name; // 可能崩溃、可能输出空串 —— 编译器不阻止
//
//   - Go:     names := []string{}
//             name := "Alice"
//             names = append(names, name)  // 值复制，name 仍可用
//
//   关键差异:
//   C++ 的 vector::push_back 有重载版本——左值引用版（拷贝）和右值引用版（移动）。
//   你每次调用都在无意中选择语义。
//   Rust 只有一个版本（Move），你如果想保留所有权必须显式 clone。
//
// 【如何修复】
//   方案1: clone 后 push: names.push(name.clone());
//   方案2: push 后不再使用 name
//   方案3: 使用借用: names.push(&name); 但 Vec<&str> 涉及生命周期

fn main() {
    // ─── 错误演示 ───
    // let name = String::from("Alice");
    // let mut names = Vec::new();
    // names.push(name);              // 所有权移入 Vec
    // println!("name = {}", name);   // ❌ name 已被移走
    // println!("names = {:?}", names);

    // ─── 修复演示 ───
    {
        println!("═══ 修复方案1: clone ───");
        let name = String::from("Alice");
        let mut names = Vec::new();
        names.push(name.clone()); // 显式深拷贝——开销明确
        println!("name = {name}"); // ✅ 仍然可用
        println!("names = {:?}", names);
    }

    {
        println!();
        println!("═══ 修复方案2: 转移所有权后不再使用 ───");
        let name = String::from("Bob");
        let mut names = Vec::new();
        names.push(name); // 所有权转移
        // 之后不再访问 name
        println!("names = {:?}", names);
    }

    println!();
    println!("对比: C++ vector.push_back 左值=拷贝, 右值=移动（重载决定）");
    println!("      Rust 只有一个 push，语义统一，强制显式选择");
}
