# Level 04：字符串与集合基础

## 通关标准

> 本关结束后，你能区分 `String` 与 `&str`、`Vec<T>` 与 `&[T]`，能理解切片的内存布局，并能向 C/C++/Go 开发者解释 Rust 字符串类型设计的本质原因。

## 核心概念速查

| Rust 概念 | 对比 C/C++ | 对比 Go | 对比 Swift |
|-----------|-----------|--------|-----------|
| `String` | `std::string` | `strings.Builder` | `String`（值类型） |
| `&str` | `const char*`（无长度） | `string`（不可变） | `Substring`（切片） |
| `Vec<T>` | `std::vector<T>` | `[]T`（slice） | `Array<T>` |
| `&[T]` | `span<T>`（C++20） | `[]T`（切片本质） | `ArraySlice<T>` |
| 切片不拥有数据 | 裸指针危险 | 切片安全但有 GC | ARC 管理 |

## 关键差异

```
String : 拥有堆上 UTF-8 数据的所有权（可变、可增长）
&str   : 不可变引用（不拥有数据，只借用）。可以是字符串字面量，也可以是 String 的切片
Vec<T> : 拥有堆上数组的所有权（可变、可增长）
&[T]   : 不可变引用（不拥有数据）。切片 = (ptr, len) 两个机器字
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手画出 `String` vs `&str` 的内存布局图（栈+堆）
- [ ] 能解释为什么 `"hello"` 的类型是 `&str` 而不是 `String`
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释 Rust 的 `&[T]` 与 Go 的 `[]T` 的异同
- [ ] 能说明为什么 Rust 不需要 `string_view`（C++17 引入的概念）
