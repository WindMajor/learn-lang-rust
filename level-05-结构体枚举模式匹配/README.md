# Level 05：结构体、枚举与模式匹配

## 通关标准

> 本关结束后，你能定义 struct/enum，能使用 match（穷尽检查）、if let、while let，能理解 Rust enum 作为"代数数据类型"与 C++ union/C/Java enum 的本质差异。

## 核心概念速查

| Rust 概念 | 对比 TS | 对比 Kotlin | 对比 Swift | 对比 C/C++ |
|-----------|--------|------------|-----------|-----------|
| struct（字段命名） | interface（结构匹配） | data class | struct（值类型） | struct（C 兼容） |
| 元组结构体 | tuple type | 无直接对应 | tuple | `std::tuple` |
| enum（带数据） | discriminated union | sealed class | enum（带关联值） | `std::variant`(C++17) |
| match（穷尽） | switch + never | when（穷尽 sealed） | switch（穷尽 enum） | switch（非穷尽） |
| Option<T> | T \| null | T? | Optional<T> | `std::optional` |

## 关键差异

```
C enum:      一组命名的整数常量（不携带数据，不安全）
Rust enum:  代数数据类型 — 每个变体可以携带不同类型和数量的数据
TS discriminated union: 运行时类型，结构匹配（非名义类型）
Kotlin sealed class: 编译期穷尽检查+关联数据（最接近 Rust enum）
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写一个带数据的 enum 和对应的 match 分支
- [ ] 能解释 Rust enum 与 C enum 的本质差异（数据携带+穷尽检查）
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释 match 穷尽检查与 TS exhaustiveness check 的异同
- [ ] 能写出 Option 和 Result 两个常用枚举的定义（它们就是普通 enum！）
