// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod config;
mod trans;
use crate::trans::transport::{Transport,TransMode,Msg};
mod common;
#[tauri::command]
fn greet(name: &str) -> String {
    let msg = Msg{
        msg:"".to_string(),
        model: TransMode::QUIC,
    };
    format!("Hello, {}! You've been greeted from Rust!", msg.send_msg())
}
 

#[tauri::command]
fn get_downloadpath() -> String {
    let config = config::Settings::new().unwrap();
    config.user.download_path
}
 



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,get_downloadpath])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
