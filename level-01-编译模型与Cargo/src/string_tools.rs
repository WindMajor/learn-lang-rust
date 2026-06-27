// WHAT: 字符串工具模块
// WHY: 演示多模块项目的组织结构

/// 打印问候语
// CONTRAST: Go 的包内函数首字母大写即为公开
//           Rust 必须显式 pub，默认私有更安全
pub fn greet() {
    let name = "闯关者";
    let level = "Level 01";
    // WHAT: format!() 宏 —— 返回 String 而非打印
    // CONTRAST: C sprintf()（不安全，缓冲区溢出风险）
    //           C++ std::format()（C++20，格式安全检查）
    //           Go fmt.Sprintf()
    //           TS `${}` 模板字符串（语法级）
    let msg = format!("{name}，欢迎来到 {level}：编译模型与 Cargo 生态！");
    println!("  {}", msg);
}

// 私有工具函数（外部不可见）
fn _trim_whitespace(s: &str) -> String {
    s.trim().to_string()
}
