// ================================================================
// 自定义错误类型
// ================================================================
//
// WHAT: 定义项目特有的错误类型 SearchError
// WHY:
//   - 统一错误处理（所有函数返回 Result<_, SearchError>）
//   - 实现 From trait 自动转换标准库错误
//   - 实现 Display + Error trait 提供可读错误信息
//
// CONTRAST:
//   - Go:    自定义 error: type MyError struct{...} + Error() string
//   - Java:  class MyException extends Exception
//   - TS:    class MyError extends Error
//   - C++:   class MyError : public std::exception
//
//   关键差异: Rust 的 ? 运算符 + From trait 自动转换，
//           让你在代码中直接 ? 传播错误而不用手动类型转换

use std::fmt;
use std::io;

/// 搜索器的错误类型
///
/// 使用 enum 表达不同类型的错误——这是 Rust 惯用做法
/// CONTRAST: Go 中需要一个 error 接口值，携带错误描述字符串
///           Rust enum 可以携带结构化数据（文件路径等）
#[derive(Debug)]
pub enum SearchError {
    /// IO 错误（文件不存在、权限不足等）
    IoError {
        path: String,
        source: io::Error,
    },

    /// 正则表达式语法错误
    RegexError {
        pattern: String,
        source: regex::Error,
    },

    /// 目录不存在
    DirectoryNotFound(String),

    /// 搜索超时（未来可扩展）
    Timeout(String),
}

// ─── Display 实现（给用户看的错误信息） ───
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::IoError { path, source } => {
                write!(f, "IO 错误 [{}]: {}", path, source)
            }
            SearchError::RegexError { pattern, source } => {
                write!(f, "正则表达式错误 [{}]: {}", pattern, source)
            }
            SearchError::DirectoryNotFound(dir) => {
                write!(f, "目录不存在: {}", dir)
            }
            SearchError::Timeout(msg) => {
                write!(f, "搜索超时: {}", msg)
            }
        }
    }
}

// ─── Error trait 实现（标准错误接口） ───
impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SearchError::IoError { source, .. } => Some(source),
            SearchError::RegexError { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ─── From trait 实现：自动转换 ───
//
// WHAT: 实现 From 让 ? 运算符自动将 io::Error 转为 SearchError
// WHY:  这样在代码中可以直接: let f = File::open(path)?;
//       io::Error 自动通过 From 转换（但需要传入 path 信息）
//
// 这里展示的是从元组 (io::Error, &str) 转换的模式:
impl From<(io::Error, &str)> for SearchError {
    fn from((source, path): (io::Error, &str)) -> Self {
        SearchError::IoError {
            path: path.to_string(),
            source,
        }
    }
}

impl From<regex::Error> for SearchError {
    fn from(source: regex::Error) -> Self {
        SearchError::RegexError {
            pattern: String::from("unknown"),
            source,
        }
    }
}

// ─── 便捷构造方法 ───
impl SearchError {
    /// 从 io::Error 和路径创建
    pub fn io(error: io::Error, path: &str) -> Self {
        SearchError::IoError {
            path: path.to_string(),
            source: error,
        }
    }

    /// 目录不存在
    pub fn dir_not_found(path: &str) -> Self {
        SearchError::DirectoryNotFound(path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SearchError::DirectoryNotFound(String::from("/tmp/nope"));
        let msg = format!("{}", err);
        assert!(msg.contains("/tmp/nope"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "文件不存在");
        let err = SearchError::io(io_err, "/tmp/test");
        assert!(format!("{}", err).contains("/tmp/test"));
    }
}
