// ================================================================
// Level 09: 迭代器与闭包
// 目标: Iterator trait、适配器链、Fn/FnMut/FnOnce、闭包捕获
// ================================================================
//
// CONTRAST 核心差异地图:
// ┌────────────┬──────────────┬──────────────┬──────────────┐
// │ 概念        │ Rust         │ TS           │ C++          │
// ├────────────┼──────────────┼──────────────┼──────────────┤
// │ 迭代器链    │ 零成本       │ 运行时链     │ 零成本(模板) │
// │ 闭包类型    │ 三种(Fn系)   │ 统一(lambda) │ 统一(lambda) │
// │ 捕获方式    │ 显式(move)   │ 自动捕获     │ [=]/[&]     │
// │ 惰性求值    │ 默认         │ 默认（Array）│ 范围惰性     │
// └────────────┴──────────────┴──────────────┴──────────────┘

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 09: 迭代器与闭包              ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 迭代器基础 ───
    println!("━━━ 1. Iterator trait 基础 ━━━");
    {
        let numbers = vec![1, 2, 3, 4, 5];

        // WHAT: .iter() 创建不可变引用迭代器 &T
        //       .iter_mut() 创建可变引用迭代器 &mut T
        //       .into_iter() 创建所有权迭代器 T（消耗容器）
        // CONTRAST:
        //   - C++:   begin(v) / end(v) —— 返回迭代器对象
        //   - Go:    for _, v := range s —— 语言内建，不能链式调用
        //   - TS:    for (const v of arr) 或 arr.forEach()
        //   - Python: for v in list: 或 iter(list)
        //   关键差异: Rust 的迭代器是 trait，你可以为任何类型实现

        // 三种迭代器对比
        let sum_ref: i32 = numbers.iter().sum();
        let sum_copy: i32 = numbers.into_iter().sum();
        println!("  sum_ref: {sum_ref}");
        println!("  sum_copy: {sum_copy}");
        // numbers 已经被 into_iter 消耗！
    }
    println!();

    // ─── 2. 迭代器适配器链 ───
    println!("━━━ 2. 迭代器适配器链 ━━━");
    {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // WHAT: 链式调用，惰性求值——collect() 才是触发点
        // WHY: 编译期将整个链优化为等效的单一 for 循环（零成本抽象）
        //
        // CONTRAST:
        //   - TS:   arr.filter().map().reduce() —— 每次创建新数组！
        //   - C++:  ranges::filter | ranges::transform —— 惰性，接近 Rust
        //   - Java: stream().filter().map().reduce() —— 惰性但有对象开销
        //   - Go:   手动写 for 循环（没有链式调用）
        let result: Vec<i32> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)       // 只保留偶数
            .map(|&x| x * x)                // 平方
            .take(3)                         // 只取前3个
            .collect();                      // 触发执行（消费者）

        println!("  偶数平方（前3个）: {:?}", result);
        // 预期: 2²=4, 4²=16, 6²=36 → [4, 16, 36]

        // ─── 其他常用适配器 ───
        let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
        // enumerate: 带索引
        print!("  enumerate: ");
        for (i, v) in nums.iter().enumerate().take(4) {
            print!("({i}:{v}) ");
        }
        println!();

        // fold: 归约（类似 TS reduce）
        let sum = nums.iter().fold(0, |acc, x| acc + x);
        println!("  fold sum: {sum}");

        // zip + 其他迭代器
        let letters = ['a', 'b', 'c', 'd'];
        print!("  zip: ");
        for (num, letter) in nums.iter().zip(letters.iter()) {
            print!("{letter}{num} ");
        }
        println!();
    }
    println!();

    // ─── 3. 闭包：Fn/FnMut/FnOnce ───
    println!("━━━ 3. 闭包与三种 Fn trait ━━━");
    {
        // WHAT: Fn —— 不可变借用捕获（只读）
        //       FnMut —— 可变借用捕获（可修改捕获变量）
        //       FnOnce —— 所有权捕获（只能调用一次）
        //
        // CONTRAST:
        //   - TS:   所有闭包都是引用捕获（自动决定引用/复制）
        //          const fn = () => captured; // 自动捕获
        //
        //   - C++:  [=](int x){ return x + captured; }     // 复制捕获
        //          [&](int x){ return x + captured; }      // 引用捕获
        //          [captured](int x){ return x + captured; } // 移动捕获
        //          // 需要手动指定捕获方式（容易出错）
        //
        //   - Kotlin: { x -> x + captured }  // 自动捕获 final 变量
        //
        //   关键差异:
        //   Rust 通过 trait 系统自动推断闭包类型，
        //   编译器决定用 Fn/FnMut/FnOnce —— 你不需要手动指定

        // Fn: 不可变借用
        let prefix = String::from("[");
        let add_prefix = |s: &str| {
            // WHAT: 闭包借用了 prefix（不可变引用）
            //       实现了 Fn trait，可以多次调用
            format!("{prefix}{s}")
        };
        println!("  Fn: {}", add_prefix("hello"));
        println!("  Fn: {}", add_prefix("world")); // 可以多次调用
        println!("  prefix 仍可用: {prefix}");

        // FnMut: 可变借用
        let mut counter = 0;
        let mut increment = || {
            // WHAT: 闭包可变借用了 counter
            //       实现了 FnMut trait
            counter += 1;
            counter
        };
        println!("  FnMut: {}", increment());
        println!("  FnMut: {}", increment());
        println!("  counter = {counter}");

        // FnOnce: 所有权转移
        let owned = String::from("消耗品");
        let consume = || {
            // WHAT: 闭包获取了 owned 的所有权
            //       实现了 FnOnce trait（只能调用一次）
            let _captured = owned;
            "已消耗"
        };
        println!("  FnOnce: {}", consume());
        // println!("{}", consume()); // ❌ 不能再次调用
    }
    println!();

    // ─── 4. 自定义 Iterator ───
    println!("━━━ 4. 自定义 Iterator 实现 ━━━");
    {
        /// 斐波那契数列迭代器
        /// CONTRAST:
        ///   - C++:   class FibonacciIterator { ... }
        ///   - Go:    用闭包实现: func fib() func() int { ... }
        ///   - TS:    function* fib() { ... } 生成器
        ///   - Python: def fib(): ... yield ...
        struct Fibonacci {
            curr: u64,
            next: u64,
        }

        impl Fibonacci {
            fn new() -> Self {
                Fibonacci { curr: 0, next: 1 }
            }
        }

        impl Iterator for Fibonacci {
            type Item = u64;

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.curr;
                self.curr = self.next;
                self.next = current + self.next;
                Some(current)
            }
            // 没有 size_hint —— 这是无限迭代器
        }

        let fib = Fibonacci::new();
        let first_10: Vec<u64> = fib.take(10).collect();
        println!("  斐波那契前10项: {:?}", first_10);
    }
    println!();

    // ─── 5. 常见的迭代器消费者 ───
    println!("━━━ 5. 迭代器消费者 ━━━");
    {
        let v = vec![1, 2, 3, 4, 5];
        println!("  sum:     {}", v.iter().sum::<i32>());
        println!("  product: {}", v.iter().product::<i32>());
        println!("  min:     {:?}", v.iter().min());
        println!("  max:     {:?}", v.iter().max());
        println!("  count:   {}", v.iter().count());
        println!("  any > 3: {}", v.iter().any(|&x| x > 3));
        println!("  all > 0: {}", v.iter().all(|&x| x > 0));
        // find: 找到第一个满足条件的
        println!("  find > 3: {:?}", v.iter().find(|&&x| x > 3));
    }
    println!();

    println!("╔══════════════════════════════════════╗");
    println!("║  Level 09 通关！继续 Level 10        ║");
    println!("╚══════════════════════════════════════╝");
}
