#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use proxy::{Proxy, ProxyTrait};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
mod config;
mod depositor;
mod error;
mod files;
mod proxy;
mod vmess;

pub static mut DATABSE: Option<Connection> = None;
static mut PROXY: Option<Box<dyn ProxyTrait>> = None;

#[derive(Debug, Deserialize, Serialize)]
struct Msg {
    code: i32,
    msg: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
/// 获取代理列表
fn get_proxies_list() -> Vec<proxy::Proxy> {
    unsafe {
        match &DATABSE {
            Some(i) => depositor::get_proxy_list(i),
            None => panic!("Haven't connect to database"),
        }
    }
}

#[tauri::command]
fn push_v2ray_proxy(name: String, proxy_type: String) {
    unsafe {
        match &DATABSE {
            Some(i) => {
                let id = uuid::Uuid::new_v4().to_string();
                let proxy = Proxy {
                    proxy_name: name,
                    proxy_type: proxy_type,
                    proxy_config_path: "~/.config/v2neko/connections/".to_owned(),
                    proxy_delay: -1,
                    proxy_download: 0,
                    proxy_upload: 0,
                    proxy_id: id,
                    proxy_group: "default".to_owned(),
                };
                depositor::push_proxy(i, &proxy)
            }
            None => panic!("Haven't connect to database"),
        }
    }
}

#[tauri::command]
fn choice_proxy(proxy_id: &str) -> Msg {
    let proxy;
    unsafe {
        proxy = depositor::get_proxy_by_id(&(&DATABSE).as_ref().unwrap(), proxy_id);
        let core = proxy::use_proxy(&proxy);
        if core.is_ok() {
            PROXY = Some(Box::new(core.ok().unwrap()));
            Msg {
                code: 0,
                msg: "success".to_owned(),
            }
        } else {
            Msg {
                code: -1,
                msg: core.err().unwrap().msg,
            }
        }
    }
}

#[tauri::command]
fn poll_output() -> Option<String> {
    unsafe {
        match &mut PROXY {
            Some(i) => i.poll_output(),
            None => None,
        }
    }
}

#[tokio::main]
async fn main() {
    unsafe {
        DATABSE = Some(depositor::init_database());
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_proxies_list,
            push_v2ray_proxy,
            choice_proxy,
            poll_output
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
