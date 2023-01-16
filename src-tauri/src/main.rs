#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::Connection;
mod depositor;
mod proxy;
mod v2ray;
mod error;
mod ws;

pub static mut DATABSE: Option<Connection> = None;

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
fn push_v2ray_proxy() {}

#[tauri::command]
fn choice_proxy(proxy_id: i32)->String{
    let proxy;
    unsafe {
        proxy = depositor::get_proxy_by_id(&(&DATABSE).as_ref().unwrap(), proxy_id);
    }
    proxy::use_proxy(&proxy)
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
            choice_proxy
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
