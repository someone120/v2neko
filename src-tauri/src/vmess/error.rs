use std::fmt;

#[derive(Debug)]
pub enum ParseLinkErrorCode {
    Base64Error,
    LinkError,
    JsonEror
}

/// 在解析连接时可能出现的错误。
/// code: 错误码
#[derive(Debug)]
pub struct ParseLinkError {
    pub msg: String,
    pub code: ParseLinkErrorCode,
}

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for ParseLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            ParseLinkErrorCode::LinkError => write!(f, "link error: could not parse link"),
            ParseLinkErrorCode::Base64Error => write!(f, "Base64 Decode error: {}", self.msg),
            ParseLinkErrorCode::JsonEror => write!(f, "Json parse error: {}", self.msg),
        }
    }
}

#[derive(Debug)]
pub struct GenerateLinkError {
    pub msg: String,
    pub code: i32,
}

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for GenerateLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            1 => write!(f, "link error: could not parse link"),
            2 => write!(f, "Base64 Decode error: {}", self.msg),
            3 => write!(f, "Json parse error: {}", self.msg),
            _ => write!(f, "Unknow error: {}", self.msg),
        }
    }
}

