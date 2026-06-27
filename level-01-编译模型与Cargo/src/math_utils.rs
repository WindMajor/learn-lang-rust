// WHAT: 数学工具模块 —— 演示 Rust 的模块组织结构
// WHY: Rust 的 pub 修饰符控制可见性，默认私有
// CONTRAST:
//   - C/C++: 默认 public，需 static 隐藏
//   - Go: 首字母大写=public，小写=private（包级）
//   - TS: export 显式导出，默认私有（文件级）

/// 两个 i32 相加
///
/// # 示例
/// ```
/// let result = math_utils::add(2, 3);
/// assert_eq!(result, 5);
/// ```
// WHAT: pub 使函数对外部可见
// WARNING: 不加 pub 的函数只能在本模块内使用
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 两个 i32 相乘
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// WHAT: 私有函数 —— 无 pub 修饰符，外部模块不可见
// WHY: Rust 默认私有是安全策略的一部分：最小化公开 API 表面积
// CONTRAST: Go 小写开头=私有，大写=公开
//           C/C++ 用匿名 namespace 或 static 限制可见性
//           TS 不 export 则不对外暴露
fn internal_helper() -> &'static str {
    "此函数仅 math_utils 模块内部可用"
}
