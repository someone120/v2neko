use std::fs;

use crate::{error::{CoreConfigError, ProxySwitchError}, v2ray};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
    pub proxy_id: String,
    pub proxy_name: String,
    pub proxy_type: String,
    pub proxy_upload: i64,
    pub proxy_download: i64,
    pub proxy_config_path: String,
    pub proxy_delay: i32,
}

pub trait ProxyTrait {
    fn restart(&mut self);
    fn start(&mut self);
    fn stop(&mut self);
    fn check_version(&self) -> Result<String, CoreConfigError>;
    fn poll_output(&mut self) -> Option<String>;
}

pub fn use_proxy(proxy: &Proxy) -> Result<impl ProxyTrait,ProxySwitchError> {
    match fs::copy("~/.config/config.json", proxy.proxy_config_path.as_str()) {
        Ok(_) => match proxy.proxy_type.as_str() {
            "v2ray" => {
                let mut a = v2ray::core::init("/usr/bin/xray");
                a.restart();
                Ok(a)
            }
            _ => Err(ProxySwitchError{msg:format!("在切换代理时遇到了错误：不支持的类型：{}", proxy.proxy_type)}),
        },
        Err(i) => Err(ProxySwitchError{msg:format!("在切换代理时遇到了错误：{}", i.to_string())}),
    }
}
