// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod common;
fn main() {
    // 自定义格式（包含时间、日志级别、目标模块）
    tracing_subscriber::fmt()
        .with_target(true)       // 显示模块路径
        .with_timer(tracing_subscriber::fmt::time::uptime())  // 显示时间
        .init();
    comicview_lib::run()
}
