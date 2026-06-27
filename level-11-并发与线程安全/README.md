# Level 11：并发与线程安全

## 通关标准

> 本关结束后，你能使用 std::thread 创建线程、通过 channel 进行消息传递、使用 Arc<Mutex<T>> 共享状态，能理解 Send/Sync trait 的编译期并发安全保证，并能向 Go/C++/Java 开发者解释 Rust 并发模型的本质优势。

## 核心概念速查

| Rust 概念 | 对比 Go | 对比 C++ | 对比 Java |
|-----------|--------|---------|----------|
| `std::thread` | goroutine | `std::thread` | `Thread` |
| `mpsc::channel` | channel | 无内置 | `BlockingQueue` |
| `Arc<Mutex<T>>` | `sync.Mutex` | `shared_ptr<mutex>` | `synchronized` |
| `Send` trait | 无（运行时检查） | 无 | 无 |
| `Sync` trait | 无 | 无 | 无 |
| 数据竞争 | 编译期杜绝 | 运行时（race detector） | 运行时 |

## 关键差异

```
Go:  "不要通过共享内存来通信，而要通过通信来共享内存"
Rust: "编译器保证你不能不合规地共享内存"
       Send/Sync 在编译期验证并发安全性

C++: 编译器给你全部的并发自由（和全部的数据竞争风险）
Rust: 编译器告诉你能并发做什么（和不能做什么）

C++ mutex:    mutex m; Data d;   // 锁和数据分离——可能忘记加锁
Rust Mutex:   Mutex<Data>         // 数据被锁包裹——不加锁拿不到
```

## 编译/运行命令

```bash
cargo run
```

## 自检清单

- [ ] 能手写一个使用 channel 的生产者-消费者模式
- [ ] 能解释 Send/Sync trait 是编译期自动派生的
- [ ] 能独立修复 `bugs/` 目录下的 3 个错误
- [ ] 能解释为什么 Rc 不能跨线程但 Arc 可以
- [ ] 能对比 Rust 的 "编译期并发安全" 与 Go 的 "运行时 goroutine 调度"
