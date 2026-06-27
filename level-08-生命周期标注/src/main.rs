// ================================================================
// Level 08: 生命周期标注
// 目标: 理解生命周期标注语法、Elision规则、'static、结构体中的引用
// ================================================================
//
// CONTRAST: 生命周期是 Rust 独有的编译期概念，在其他语言中没有直接对应物
//   - C/C++:  有"对象的生命周期"概念，但没有编译期标注和强制检查
//              依赖程序员记忆和运行时工具（asan、valgrind）
//   - GC 语言: 所有对象在 GC 堆上，引用永远有效（直到 GC 决定回收）
//              没有"比引用者活得短"的问题
//   Rust 生命周期 = C 的 asan 在编译期运行，零开销、零漏报

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 08: 生命周期标注              ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ─── 1. 生命周期基本语法 ───
    println!("━━━ 1. 生命周期参数 'a ━━━");
    {
        // WHAT: 'a 是生命周期参数，读作 "生命周期 a"
        // WHY: 告诉编译器多个引用之间的关系（谁比谁活得久）
        // 关键理解: 生命周期标注不改变代码行为，只是描述约束
        //
        // CONTRAST:
        //   C++ 中同样的问题:
        //     const string& longer(const string& s1, const string& s2) {
        //       return s1.size() > s2.size() ? s1 : s2;
        //     }
        //     // 这段 C++ 代码通过编译（零警告）！
        //     // 但如果你调用 longer(temp(), another_temp())
        //     // 返回的引用指向已析构的临时对象 —— 悬垂引用！
        //     // 运行时可能崩溃也可能"正常" —— 未定义行为

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let s1 = String::from("hello");
        let s2 = String::from("world!");
        let result = longest(&s1, &s2);
        println!("  最长的字符串: \"{result}\"");

        // 生命周期演示：不同作用域
        let s_long = String::from("longer string");
        {
            let s_short = String::from("short");
            // 可以 —— result 的生命周期被限制为 s_short
            let r = longest(&s_long, &s_short);
            println!("  嵌套作用域: \"{r}\"");
        } // s_short 析构，r 也不再可用
    }
    println!();

    // ─── 2. 生命周期省略规则 (Elision) ───
    println!("━━━ 2. 生命周期省略规则 ━━━");
    {
        // 规则1: 每个引用参数获得独立的生命周期
        // 规则2: 如果只有一个输入生命周期，赋给所有输出
        // 规则3: 如果方法有 &self/&mut self，输出生命周期 = self 的生命周期
        //
        // 以下函数不需要生命周期标注（编译器自动推断）:
        fn first_word(s: &str) -> &str {
            // 规则2: 只有一个输入引用 → 输出 = 输入的生命周期
            s.split_whitespace().next().unwrap_or("")
        }

        let text = String::from("Hello World");
        let word = first_word(&text);
        println!("  第一个词: \"{word}\"");

        // 但以下需要手动标注:
        fn first_of_two<'a>(x: &'a str, _y: &str) -> &'a str {
            // 两个输入引用 → 编译器不知道输出关联哪一个 → 必须手动标注
            x
        }

        let a = String::from("Rust");
        let b = String::from("Go");
        let result = first_of_two(&a, &b);
        println!("  first_of_two: \"{result}\"");
    }
    println!();

    // ─── 3. 结构体中的引用字段 ───
    println!("━━━ 3. 结构体中的引用字段 ━━━");
    {
        // WHAT: 结构体中有引用字段时，必须标注生命周期
        // WHY: 编译器需要知道: "这个引用字段不能比结构体本身活得更短"
        //
        // CONTRAST:
        //   - C/C++: struct Excerpt { const char* part; };
        //            // 编译器完全不管 part 是否有效
        //            // 你可以通过指向局部变量的指针构造 Excerpt
        //   - Go:    type Excerpt struct { part string }
        //            // Go string 是值类型（(ptr, len)），自动管理
        //   - TS:    { part: string } —— GC 管理，无此问题

        #[derive(Debug)]
        struct Excerpt<'a> {
            // 生命周期标注：part 不能比 Excerpt 活得短
            part: &'a str,
        }

        let novel = String::from("很久很久以前。有一个 Rust 程序员。故事结束。");
        let first_sentence = novel.split('。').next().expect("应该有第一句话");
        let excerpt = Excerpt {
            part: first_sentence,
        };
        println!("  摘录: {:?}", excerpt);
        // novel 必须活得比 excerpt 长
    }
    println!();

    // ─── 4. 'static 生命周期 ───
    println!("━━━ 4. 'static 生命周期 ━━━");
    {
        // WHAT: 'static 表示"存活于整个程序运行期间"
        // 常见于: 字符串字面量、全局常量
        // WARNING: 'static 不等于"永远不回收"，
        //          它只是约束生命周期可以覆盖整个程序

        let literal: &'static str = "我是静态字符串"; // 编译期嵌入二进制
        println!("  {literal}");

        fn needs_static(s: &'static str) {
            println!("  'static 参数: {s}");
        }

        needs_static("编译期常量"); // ✅ 字面量是 'static
        // needs_static(&String::from("运行时")); // ❌ 不是 'static

        // CONTRAST:
        //   - C:    static const char* s = "hello"; // 静态存储期
        //   - C++:  const char* s = "hello"; // 字面量在 .rodata
        //   - Go:   const s = "hello" // 编译期常量
        //   - TS:   const s = "hello" // 原始值
    }
    println!();

    // ─── 5. 生命周期对比总结 ───
    println!("━━━ 5. 跨语言对比 ━━━");
    println!();
    println!("  场景: 返回两个字符串中较长者的引用");
    println!();
    println!("  Rust:");
    println!("    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {{");
    println!("      if x.len() > y.len() {{ x }} else {{ y }}");
    println!("    }}");
    println!("    // 编译期保证返回值的生命周期被约束");
    println!();
    println!("  C++:");
    println!("    const string& longest(const string& x, const string& y) {{");
    println!("      return x.size() > y.size() ? x : y;");
    println!("    }}");
    println!("    // 编译通过！但调用 longest(temp(), temp2()) 会悬垂");
    println!();
    println!("  C 语言:");
    println!("    const char* longest(const char* x, const char* y) {{");
    println!("      return strlen(x) > strlen(y) ? x : y;");
    println!("    }}");
    println!("    // 编译通过！谁被释放后调用谁 segfault");
    println!();
    println!("  关键差异:");
    println!("  Rust 生命周期 = 编译期 Address Sanitizer");
    println!("  零运行时开销，零漏报（C/C++ asan 可能有漏报）");
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║  Level 08 通关！继续 Level 09        ║");
    println!("╚══════════════════════════════════════╝");
}
