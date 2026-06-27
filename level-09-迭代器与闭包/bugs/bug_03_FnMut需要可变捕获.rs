// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-03: FnMut 闭包需要可变环境 —— 可变捕获限制        ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   闭包修改了捕获的变量（实现了 FnMut），但被当作 Fn 使用。
//
// 【编译后会报什么错】
//   执行 `rustc bug_03_FnMut需要可变捕获.rs`:
//
//   error[E0596]: cannot borrow `counter` as mutable, as it is
//                 not declared as mutable
//     ...
//   或者
//   error[E0525]: expected a closure that implements the `Fn` trait,
//                 but this closure only implements `FnMut`
//
// 【为什么会这样】
//   修改捕获变量需要闭包实现 FnMut。如果闭包被传给要求 Fn 的地方
//   （比如 Iterator::map），编译失败。
//   注意: Iterator::map 接受 FnMut，但某些 API 只接受 Fn。
//
// 【如何修复】
//   方案1: 将变量声明为 mut
//   方案2: 使用内部可变性(Cell/RefCell，见 Level 10)

fn main() {
    // ❌ 错误: counter 不是 mut
    // let counter = 0;
    // let mut increment = || counter += 1;

    // ✅ 修复: mut 声明
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    println!("{}", increment());
    println!("{}", increment());

    // ─── 更隐蔽的场景: Fn 要求 ───
    // 有些 API 只接受 Fn 闭包:
    #[allow(unused)]
    fn call_twice<F: Fn() -> i32>(f: &F) -> i32 {
        f() + f()
    }

    let get_val = || 42; // Fn 闭包（只读，无捕获）
    println!("call_twice(Fn): {}", call_twice(&get_val));

    // ❌ call_twice 不接受 FnMut:
    // let mut state = 0;
    // let mut next = || { state += 1; state };
    // call_twice(&next); // ❌ FnMut 不满足 Fn 约束

    println!();
    println!("核心: Fn/FnMut/FnOnce 是编译期的 trait 约束");
    println!("      编译器自动推断闭包类型");
    println!("对比: C++ lambda 手动指定捕获方式 [=]/[&]");
}
