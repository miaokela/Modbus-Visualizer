// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use log::info;
use utils::common::*;
use utils::file_handler::{watch_param_config, trigger_file_change, init_log, download_file};


fn main() {
    info!("Start up app!");
    init_log();

    trigger_file_change();

    set_into_read_task();

    watch_param_config(); // 监听文件变化

    task_thread();

    // 触发保存文件
    trigger_file_change();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_result, get_params, is_active, download_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
