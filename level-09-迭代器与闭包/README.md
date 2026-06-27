# Level 09：迭代器与闭包

## 通关标准

> 本关结束后，你能使用 Iterator trait 和适配器链（map/filter/fold/collect），能区分 Fn/FnMut/FnOnce 三种闭包特征，能向 C++/Go/TS 开发者解释 Rust 迭代器的零成本抽象原理。

## 核心概念速查

| Rust 概念 | 对比 Python | 对比 TS | 对比 C++ | 对比 Java |
|-----------|------------|--------|---------|----------|
| `Iterator` trait | `__iter__` | `Iterable<T>` | 迭代器概念 | `Iterator<T>` |
| `map/filter/fold` | 推导式/list comp | `Array.map/filter` | `<algorithm>` | `Stream API` |
| `Fn` 闭包 | lambda | arrow function | lambda | lambda |
| `FnMut` | mutable lambda | mutable closure | mutable lambda | mutable lambda |
| `FnOnce` | consuming lambda | consuming closure | move-only lambda | 无直接对应 |
| 零成本迭代器链 | ❌ | ❌ | ✅ | ❌ |

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写一个 Iterator 实现（自定义 next 方法）
- [ ] 能区分 Fn/FnMut/FnOnce 的使用场景
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释 Rust 迭代器链"零成本"的原理（编译器将链式调用优化为单一循环）
- [ ] 能对比 Rust 闭包与 TS 箭头函数的捕获机制差异
