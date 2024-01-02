use crate::utils::common::{update_connection, update_param, Config};
use notify::{watcher, RecursiveMode, Watcher};
use tokio::runtime::Runtime;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs, thread};
use std::fs::{read_to_string, write};
use super::modbus_lib::get_modbus_conn;
use std::env;
use std::path::PathBuf;
use log::{LevelFilter, info};
use simplelog::{WriteLogger, Config as LogConfig};
use std::fs::OpenOptions;

/**
 * 监听文件变化线程
 */
pub fn watch_param_config() {
    thread::spawn(move || {
        let mut rt: Runtime = Runtime::new().unwrap();

        let (tx, rx) = channel();

        // 创建一个 watcher，当文件或目录变化时，事件会被发送到通道
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        let file = get_modbus_toml_path();
        // 添加要监听的目录，RecursiveMode::Recursive 表示递归监听所有子目录
        watcher
            .watch(&file, RecursiveMode::Recursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(_event) => {
                    info!("File changed");

                    let content = fs::read_to_string(&file).unwrap();
                    let config: Config = toml::from_str(&content).unwrap();
                    update_connection(&config.connection);
                    for param in config.params {
                        update_param(param.param_id, param);
                    }
                    println!("modbus.toml changed");
                    {
                        // 重连
                        let modbus_conn = get_modbus_conn();
                        let mut modbus_conn = modbus_conn.lock().unwrap();
                        let new_conn = format!(
                            "{}:{}",
                            config.connection.ip_address, config.connection.port
                        );
                        modbus_conn.reconnect(&new_conn, &mut rt);
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}

pub fn is_dev_environment() -> bool {
    match env::var("TAURI_ENV") {
        Ok(val) => val == "dev",
        Err(_) => false,
    }
}

/**
 * 获取modbus.toml文件
 */
pub fn get_modbus_toml_path() -> PathBuf {
    let mut path = if is_dev_environment() {
        // 在开发环境中，我们假设 modbus.toml 文件位于项目的根目录下
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        let mut exe_path = env::current_exe().unwrap();
        if cfg!(target_os = "macos") {
            exe_path.pop();
            exe_path.pop();
            exe_path.push("Resources");
        }
        exe_path
    };
    path.push("modbus.toml");
    path
}

/**
 * 获取日志文件
 */
pub fn get_log_path() -> PathBuf {
    let mut path = if is_dev_environment() {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        let mut exe_path = env::current_exe().unwrap();
        if cfg!(target_os = "macos") {
            exe_path.pop();
            exe_path.pop();
            exe_path.push("Resources");
        }
        exe_path
    };
    path.push("log.txt");
    path
}

/**
 * 首次触发文件变化
 */
pub fn trigger_file_change() {
    info!("trigger change");

    let modbus_file_path = get_modbus_toml_path();
    let content = read_to_string(&modbus_file_path).unwrap();
    write(modbus_file_path, &content).unwrap();
}

/**
 * 初始化日志
 */
pub fn init_log() {
    let log_file_path = get_log_path();
    // println!("日志路径{:?}", log_file_path);
    let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(&log_file_path)
    .unwrap();

    WriteLogger::init(LevelFilter::Info, LogConfig::default(), file).unwrap();
}

