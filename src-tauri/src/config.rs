

use crate::files;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub dns: Vec<String>,
    pub socks_port: i32,
    pub socks_bind: String,
    pub http_port: i32,
    pub http_bind: String,
}

pub fn init_config() {
    let data = serde_json::to_string_pretty(&AppConfig {
        dns: vec![
            "1.1.1.1".to_string(),
            "8.8.8.8".to_string(),
            "8.8.4.4".to_string(),
        ],
        socks_port: 11451,
        socks_bind: "127.0.0.1".to_string(),
        http_port: 11452,
        http_bind: "127.0.0.1".to_string(),
    })
    .unwrap();

    files::write("config.json", data.as_str()).unwrap();
}

pub fn read() -> AppConfig {
    let raw_config = files::read("~/.config/v2neko/config.json");
    if raw_config.is_err() {
        init_config();
        return AppConfig {
            dns: vec![
                "1.1.1.1".to_string(),
                "8.8.8.8".to_string(),
                "8.8.4.4".to_string(),
            ],
            socks_port: 11451,
            socks_bind: "127.0.0.1".to_string(),
            http_port: 11452,
            http_bind: "127.0.0.1".to_string(),
        };
    }
    serde_json::from_str::<AppConfig>(&raw_config.ok().unwrap()).unwrap()
}
