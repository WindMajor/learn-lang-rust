# Level 10：智能指针与内部可变性

## 通关标准

> 本关结束后，你能区分和使用 Box/Rc/RefCell/Arc/Mutex，能理解"内部可变性"概念，能向 C++ 开发者解释 Rust 智能指针与 C++ 智能指针的异同。

## 核心概念速查

| Rust 概念 | 对比 C++ | 对比 Swift | 用途 |
|-----------|---------|-----------|------|
| `Box<T>` | `std::unique_ptr<T>` | 堆分配（ARC 管理） | 堆分配 + 单一所有权 |
| `Rc<T>` | `std::shared_ptr<T>` | 自动 ARC | 单线程引用计数 |
| `Arc<T>` | `std::shared_ptr<T>`（原子） | 自动 ARC（原子） | 多线程引用计数 |
| `RefCell<T>` | 无直接对应（运行时借用检查） | 无 | 运行时借用检查 |
| `Mutex<T>` | `std::mutex` + 数据 | `DispatchSemaphore`? | 线程互斥锁 |
| `RwLock<T>` | `std::shared_mutex` | 读写锁 | 读写锁 |

## 关键差异

```
Box<T>:    堆上分配，单一所有者。等价于 unique_ptr，但更安全
Rc<T>:     单线程共享所有权。C++ shared_ptr 是线程安全的（原子计数）
            Rust Rc 不是！多线程用 Arc<T>。
            C++ shared_ptr 统一了单/多线程（性能代价不透明）
RefCell<T>: 运行时借用检查。当你在编译期无法满足借用规则时使用
            等价于"让编译器把借用检查推迟到运行时"
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写出 Box/Rc/RefCell/Arc/Mutex 的典型使用场景
- [ ] 能解释为什么 Rc 不能跨线程而 Arc 可以
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释 RefCell 的设计哲学（"我向你保证借用规则，检查推迟到运行时"）
- [ ] 能对比 Rust Rc 与 C++ shared_ptr 的单/多线程设计差异
