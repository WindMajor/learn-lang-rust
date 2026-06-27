// ╔══════════════════════════════════════════════════════════════╗
// ║  BUG-02: 迭代器惰性求值陷阱 —— 忘记 collect()          ║
// ╚══════════════════════════════════════════════════════════════╝
//
// 【这是什么错误】
//   编写了 map/filter 适配器链，但忘记调用 collect() 等消费者。
//   编译器警告"unused must be used"，但代码不会产生任何副作用。
//
// 【编译后会报什么错】
//   执行 `rustc bug_02_迭代器惰性求值陷阱.rs`:
//
//   warning: unused `std::iter::Map` that must be used
//     --> bug_02_迭代器惰性求值陷阱.rs:XX:XX
//      |
//   XX |     numbers.iter().map(|x| x * 2);
//      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//      |
//      = note: iterators are lazy and do nothing unless consumed
//      = note: `#[warn(unused_must_use)]` on by default
//
//   error: this expression is unreachable (或编译通过但无效果)
//
// 【为什么会这样】
//   Rust 迭代器是惰性的——map/filter 等只是构建"计算描述"，
//   不真正执行。只有消费者（collect、sum、for_each 等）才触发执行。
//
// 【在 C++/TS 中对应的行为】
//   - TS:    arr.map(x => x * 2)  // 立即执行！返回新数组
//   - C++:   ranges::transform(v, back_inserter(out), [](int x){ ... })
//            // ranges 版本惰性，但 transform 版本立即执行
//   - Python: map(lambda x: x*2, arr) // 惰性！需要 list() 触发
//   - Java:   stream.map(...) // 惰性，collect() 触发
//
//   关键差异:
//   TS 的 Array.map 立即执行（因为是同步的），
//   Rust 的 Iterator::map 惰性——性能更好但容易被忽略
//
// 【如何修复】
//   添加消费者: collect(), sum(), for_each(), count() 等

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // ❌ 错误：没有消费者——什么都不发生
    // numbers.iter().map(|x| x * 2); // 只是描述"我要把每个元素乘2"

    // ✅ 修复1: collect() 收集结果
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("collect: {:?}", doubled);

    // ✅ 修复2: for_each() 执行副作用
    print!("for_each: ");
    numbers.iter().map(|x| x * 2).for_each(|x| print!("{x} "));
    println!();

    // ✅ 修复3: for 循环消费
    print!("for: ");
    for x in numbers.iter().map(|x| x * 2) {
        print!("{x} ");
    }
    println!();

    println!();
    println!("核心: Rust 迭代器是惰性的，必须显式消费");
    println!("对比: TS Array.map() 立即执行并分配新数组");
    println!("      Rust 的惰性 + 零成本 = 无中间数组分配");
}
