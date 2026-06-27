# Level 06：错误处理 —— Result 与 Option

## 通关标准

> 本关结束后，你能熟练使用 `Result<T, E>`、`Option<T>`、`?` 运算符、`panic!`，能理解 Rust 的"强迫显式错误处理"与 Go 的 error 返回值和 Java 的 Checked Exception 的哲学差异。

## 核心概念速查

| Rust 概念 | 对比 Go | 对比 Java | 对比 TS | 对比 C++ |
|-----------|--------|----------|--------|---------|
| `Result<T,E>` | `(T, error)` | Checked Exception | `T | Error` | `std::expected<T,E>`(C++23) |
| `Option<T>` | `*T` (nil) | `Optional<T>` | `T \| null` | `std::optional<T>` |
| `?` 运算符 | `if err != nil { return err }` | throw（隐式） | throw | 无 |
| `panic!` | `panic()` | RuntimeException | throw Error | `std::abort()` |
| `unwrap()` | 无（程序崩溃） | `.get()` | `!` (non-null) | `.value()` |

## 关键差异

```
Rust 错误处理哲学: "强迫你处理每个可能的错误，无一例外"
Go 错误处理哲学:   "返回 error，但你可以选择忽略"（_ = val）
Java Checked Exception: "声明可能抛出的异常，但可以不处理"
TS 错误处理:        "try-catch 是可选的，unhandled rejection 到运行时才发现"
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写出 `Option<T>` 和 `Result<T, E>` 的枚举定义
- [ ] 能解释 `?` 运算符的本质（`match + return Err` 的语法糖）
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释 Rust 错误处理与 Go error 模式的根本差异
- [ ] 能解释 `unwrap()` 和 `expect()` 何时该用、何时是代码坏味道
