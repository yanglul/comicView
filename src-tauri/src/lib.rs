// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod config;
use config::{*};
mod trans;
use crate::trans::transport::{Transport,TransMode,Msg};
mod common;

pub struct Comic{
    id:u64,
    auth:String,
    title:String,
    size:u32,
}







#[tauri::command]
fn greet(name: &str) -> String {
    let msg = Msg{
        msg:"".to_string(),
        model: TransMode::QUIC,
    };
    format!("Hello, {}! You've been greeted from Rust!", msg.send_msg())
}
 
#[tauri::command]
fn download_img(name: &str) -> String {
    let msg = Msg{
        msg: "1.jpg".to_string(),
        model: TransMode::QUIC,
    };
    
    let config = setting::load_config();
    msg.download("/1.jpg".to_string()).unwrap();
    format!("{}\\{}", config.user.download_path.clone(),"1.jpg" )
}



#[tauri::command]
fn get_downloadpath() -> String {
    let config = setting::load_config();
    config.user.download_path.clone()
}
 



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,get_downloadpath,download_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
