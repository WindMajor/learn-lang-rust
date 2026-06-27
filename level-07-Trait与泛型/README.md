# Level 07：Trait 与泛型

## 通关标准

> 本关结束后，你能定义 trait、为类型实现 trait、使用泛型和 trait bound，理解单态化（monomorphization）实现零成本抽象，并能向 C++/TS/Go 开发者解释 Rust trait 系统的本质优势。

## 核心概念速查

| Rust 概念 | 对比 TS | 对比 Go | 对比 C++ | 对比 Swift |
|-----------|--------|--------|---------|-----------|
| `trait` | interface（结构匹配） | interface（隐式实现） | 纯虚类/Concepts | Protocol |
| `impl Trait for Type` | `class X implements I` | 无需声明（自动匹配） | `class X : public I` | `class X: Protocol` |
| 泛型（单态化） | 类型擦除（运行时） | 无泛型 | 模板（编译期展开） | 泛型（部分动态） |
| 关联类型 | interface 字段 | 无直接对应 | `typename T` | `associatedtype` |
| 孤儿规则 | 无此概念 | 无此概念 | 无此概念 | 无此概念 |
| `dyn Trait` | 接口类型的变量 | `interface{}` | 虚函数表 | `any Protocol` |

## 关键差异

```
TS interface:    结构匹配——"你长得像鸭子，你就是鸭子"
Rust trait:      名义匹配——"你必须说自己是鸭子，才能当鸭子"
                  孤儿规则: 你只能在自己的 crate 中为你的类型实现 trait
Go interface:    隐式匹配——"你能叫出鸭子的声音，你就是鸭子"
C++ concepts:    编译期约束——"你的行为满足鸭子概念"（C++20 新特性）
Kotlin interface: 名义匹配+默认实现（与 Rust 最接近）
Swift Protocol:   名义匹配+扩展实现（与 Rust 部分相似）
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写一个 trait 定义和 impl 块
- [ ] 能解释泛型单态化 vs 类型擦除的性能差异
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误（特别是孤儿规则）
- [ ] 能解释 Rust 孤儿规则为什么存在（防止 crate 间冲突）
- [ ] 能对比 Rust trait 与 Go 隐式接口的优劣
