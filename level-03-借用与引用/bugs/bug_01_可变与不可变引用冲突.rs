// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-01: 可变引用与不可变引用冲突 —— 借用规则 101        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   同时持有可变引用和不可变引用，违反借用规则。
//   这是 Rust 借用检查器最常见的摘错场景。
//
// 【编译后会报什么错】
//   执行 `rustc bug_01_可变与不可变引用冲突.rs`:
//
//   error[E0502]: cannot borrow `data` as mutable because it is also
//                 borrowed as immutable
//     --> bug_01_可变与不可变引用冲突.rs:XX:XX
//      |
//   XX |     let r1 = &data;
//      |              ----- immutable borrow occurs here
//   XX |     let r2 = &data;
//      |              ----- immutable borrow later used here
//   XX |     let r3 = &mut data;
//      |              ^^^^^^^^^ mutable borrow occurs here
//
// 【为什么会这样】
//   借用规则: 同一作用域内，要么多个不可变引用，要么一个可变引用。
//   r1 和 r2 持有不可变引用，此时 data 被"冻结"——不能修改。
//   Rust 设计哲学: 如果有人正在读，别人就不能写（防止读到不一致的数据）。
//
// 【在 C++/Go/TS 中对应的行为】
//   - C++:    vector<int> data = {1,2,3};
//             const auto& r1 = data;  // 只读引用
//             auto& r2 = data;        // 可读写引用
//             r1[0];                  // 读——没问题，编译器不拦
//             r2[0] = 10;             // 写——可能导致 r1 看到被修改的值
//             // 无编译期限制，可能导致逻辑错误
//
//   - Go:     data := []int{1,2,3}
//             p := &data       // 指针
//             // Go 没有任何借用检查，并发访问可能导致数据竞争
//
//   - TS:     数组是引用语义，多个引用自然共存
//             但 JS 事件循环的单线程模型规避了大部分问题
//
// 【如何修复】
//   方案1: 让不可变引用在可变借用前结束使用（调整作用域/NLL）
//   方案2: 先可变借用做完修改，再做不可变借用
//   方案3: 使用 Clone 拷贝一份数据

fn main() {
    // ─── 错误演示 ───
    // let mut data = vec![1, 2, 3];
    // let r1 = &data;
    // let r2 = &data;        // 第二个不可变引用
    // let r3 = &mut data;    // ❌ r1 和 r2 还在使用中！
    // println!("{:?} {:?} {:?}", r1, r2, r3);

    // ─── 修复方案1: 调整作用域 ───
    {
        println!("═══ 修复方案1: 调整作用域 ───");
        let mut data = vec![1, 2, 3];

        // 不可变借用存在于这个子作用域中
        {
            let r1 = &data;
            let r2 = &data;
            println!("  读取: {:?}, {:?}", r1, r2);
        } // r1, r2 离开作用域——借用结束

        let r3 = &mut data; // ✅ 现在可以可变借用
        r3.push(4);
        println!("  修改后: {:?}", r3);
    }

    // ─── 修复方案2: NLL —— 依赖编译器的聪明 ───
    {
        println!();
        println!("═══ 修复方案2: NLL 自动结束借用 ───");
        let mut data = vec![5, 6, 7];
        let r1 = &data;
        let r2 = &data;
        println!("  r1, r2 读取完毕: {:?}, {:?}", r1, r2);
        // NLL: 编译器知道 r1, r2 之后不再使用，借用自动结束

        let r3 = &mut data; // ✅ NLL 让这变成合法代码
        r3.push(8);
        println!("  修改后: {:?}", r3);
    }

    println!();
    println!("核心教训: 借用规则不是教条，NLL 让它在实践中灵活得多");
    println!("对比: C++ 允许同时读写同一数据——给你自由，也给你陷阱");
}
