use std::fmt;

#[derive(Debug)]
pub struct CoreConfigError {
    pub msg: String,
    pub code: i64,
}

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for CoreConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Core config errorr:{} \n Code:{}",self.msg,self.code)
    }
}

#[derive(Debug)]
pub struct ProxySwitchError {
    pub msg: String,
}

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for ProxySwitchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while switching proxies
        msg:{}",self.msg)
    }
}
