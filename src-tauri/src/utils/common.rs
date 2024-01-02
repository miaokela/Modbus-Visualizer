use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::utils::modbus_lib::get_modbus_conn;
use tokio::runtime::Runtime;

// 定义数据结构
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Task {
    operation: u8,
    register_type: u8,
    start_address: u16,
    quantity: u16,
    param_id: u16,
    slave_id: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Param {
    pub param_id: u16,
    pub slave_id: u8,
    pub start_address: u16,
    pub data_type: u8,
    pub operation: u8,
    pub register_type: u8,
    pub name: String,
    pub unit: String,
    pub cycle_time: u16,
    pub curve_val: Vec<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct ModbusResult {
    param_id: u16,
    slave_id: u8,
    start_address: u16,
    data_type: u8,
    operation: u8,
    register_type: u8,
    name: String,
    state: u8,
    val: f64,
    unit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub protocol_type: u8,
    pub ip_address: String,
    pub port: u16,
    pub serial_port: String,
    pub baud_rate: u16,
    pub data_bit: u8,
    pub stop_bit: u8,
    pub parity_bit: u8,
    pub create_time: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: Connection,
    pub params: Vec<Param>,
}

// 创建全局变量
lazy_static! {
    static ref TASKS: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
    static ref PARAMS: Arc<Mutex<HashMap<u16, Param>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref RESULTS: Arc<Mutex<HashMap<u16, ModbusResult>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref CONNECTION: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection {
        protocol_type: 1,
        ip_address: "192.168.1.6".to_string(),
        port: 502,
        serial_port: "COM1".to_string(),
        baud_rate: 9600,
        data_bit: 1,
        stop_bit: 1,
        parity_bit: 8,
        create_time: "2020-10-10 10:10:10".to_string(),
    }));
}

pub fn get_config() -> Config {
    let connection = CONNECTION.lock().unwrap().clone();
    let params = PARAMS.lock().unwrap().values().cloned().collect();

    Config { connection, params }
}

/**
 * 更新参数
 */
pub fn update_param(param_id: u16, param: Param) {
    let mut params = PARAMS.lock().unwrap();
    params.insert(param_id, param);
}

/**
 * 更新连接配置
 */
pub fn update_connection(connection: &Connection) {
    let mut conn = CONNECTION.lock().unwrap();
    *conn = connection.clone();
}

/**
 * 供前端获取最终数据
 */
#[tauri::command]
pub fn get_result(param_id: u16) -> Option<ModbusResult> {
    let results = RESULTS.lock().unwrap();
    results.get(&param_id).cloned()
}

/**
 * 获取所有参数列表
 */
#[tauri::command]
pub fn get_params() -> Option<HashMap<u16, Param>> {
    let params = PARAMS.lock().unwrap();
    if params.is_empty() {
        None
    } else {
        Some(params.clone())
    }
}

/**
 * 执行任务的线程
 */
pub fn execute_task(task: &Task, rt: &mut Runtime) -> Vec<u16> {
    // 在这里根据你的具体需求来实现任务执行的逻辑
    // 这个函数应该根据Task和Param来生成一个ModbusResult
    // 这是一个简单的例子，你需要根据你的具体需求来修改它
    let data: Vec<u16> = vec![];

    // 读取任务获取值
    if task.operation == 2 {
        let config = get_config();
        // 根据 operation register_type start_address quantity slave_id 获取值
        // 判断是 register_type 为1 还是2 1 表示保持寄存器 2 表示输入寄存器
        let modbus_conn = get_modbus_conn();
        let mut conn = modbus_conn.lock().unwrap();

        // 判断是否连接成功 没有连接成功就返回空 并且重连
        if !conn.is_connected() {
            conn.reconnect(&format!(
                "{}:{}",
                config.connection.ip_address, config.connection.port
            ), rt);
            println!("重连成功");
        }

        if task.register_type == 1 {
            let result = conn.read_registers(task.start_address, task.quantity);
            match result {
                Ok(data) => {
                    println!("读取的数据: {:?}", data);
                    return data;
                },
                Err(e) => {
                    println!("读取错误: {:?}", e);
                }
            }
        }

        if task.register_type == 2 {}
    }

    // 返回 [1,2,3,4]这样的数据，但是长度不固定

    data
}

/**
 * 寄存器数据转化
 */
pub fn convert_result(param: &Param, task: &Task, result: &[u16], size: usize) -> f64 {
    let mut val: f64 = 0.0;
    if param.start_address >= task.start_address {
        let start = (param.start_address - task.start_address) as usize;
        let end = start + size;
        let val_list = &result[start..end];

        match param.data_type {
            1 => {
                // int16
                if let Some(&v) = val_list.first() {
                    val = v as i16 as f64;
                }
            }
            2 => {
                // int32
                if val_list.len() >= 2 {
                    let int_val = ((val_list[0] as i32) << 16) | (val_list[1] as i32);
                    val = int_val as f64;
                }
            }
            3 => {
                // float32
                if val_list.len() >= 2 {
                    let int_val = ((val_list[0] as i32) << 16) | (val_list[1] as i32);
                    let float_val = f32::from_bits(int_val as u32);
                    val = format!("{:.2}", float_val).parse::<f64>().unwrap();
                }
            }
            4 => {
                // float64
                if val_list.len() >= 4 {
                    let int_val = ((val_list[0] as i64) << 48)
                        | ((val_list[1] as i64) << 32)
                        | ((val_list[2] as i64) << 16)
                        | (val_list[3] as i64);
                    let float_val = f64::from_bits(int_val as u64);
                    val = format!("{:.2}", float_val).parse::<f64>().unwrap();
                }
            }
            _ => (),
        }
    }
    val
}

/**
 * 执行任务线程
 */
pub fn task_thread() {
    // 在主线程中创建新的线程
    thread::spawn(move || {
        let mut rt: Runtime = Runtime::new().unwrap();

        loop {
            // println!("task_thread");
            let params;
            let task;

            // 获取任务
            {
                let params_data = PARAMS.lock().unwrap();
                params = params_data.clone();
            }
            // 获取参数
            {
                let mut tasks: std::sync::MutexGuard<'_, Vec<Task>> = TASKS.lock().unwrap();
                // println!("获取的参数: {:?}", params);
                // println!("获取的任务: {:?}", tasks);
    
                let c_task = tasks.pop();
                match c_task {
                    Some(item) => {
                        task = item.clone();
                    }
                    None => {
                        // 等待一段时间，以减少CPU使用率
                        thread::sleep(Duration::from_millis(100));
                        // println!("没有任务了");
                        continue;
                    }
                    
                }
            }

            println!("消耗任务: {:?}", task);
            // 执行任务
            let result = execute_task(&task, &mut rt);

            // 根据 task.register_type task.slave_id 两个条件匹配出符合条件的参数，遍历这些匹配出来的参数将结果写入ModbusResult
            for param in params.values() {
                if param.slave_id == task.slave_id && param.register_type == task.register_type {
                    // 根据param的 register_type 与data_type从result中匹配出结果
                    if result.len() == 0 {
                        continue;
                    }

                    // 根据 data_type 转换为对应的值 data_type 1为int16 2为int32 3为float32 4为float64
                    // int16 读取1个寄存器 int32 读取2个寄存器 float32 读取2个寄存器 float64 读取4个寄存器
                    // param.start_address - task.start_address 为获取的数据在result中的偏移量 再配合寄存器数量获取对应的数据 转化成最终数值根据大端序
                    let mut val: f64 = 0.0;

                    // [1,2,3,4,5,6,7,8,9,10]
                    if param.data_type == 1 {
                        val = convert_result(&param, &task, &result, 1);
                    } else if param.data_type == 2 || param.data_type == 3 {
                        val = convert_result(&param, &task, &result, 2);
                    } else if param.data_type == 4 {
                        val = convert_result(&param, &task, &result, 4);
                    }
                    // info!("val: {}", val);

                    let mut results = RESULTS.lock().unwrap();
                    results.insert(
                        param.param_id,
                        ModbusResult {
                            param_id: param.param_id,
                            slave_id: param.slave_id,
                            start_address: param.start_address,
                            data_type: param.data_type,
                            operation: param.operation,
                            register_type: param.register_type,
                            name: param.name.clone(),
                            state: 1,
                            val: val,
                            unit: param.unit.clone(),
                        },
                    );
                }
            }
        }
    });
}

/**
 * 抛出读取参数的线程
 */
pub fn set_into_read_task() {
    thread::spawn(move || {
        loop {
            // info!("set_into_read_task!");
            let params;

            // 获取任务
            {
                let params_data = PARAMS.lock().unwrap();
                params = params_data.clone();
            }
            {
                // 获取参数
                let mut tasks = TASKS.lock().unwrap();

                // 所有参数根据 slave_id、register_type 2个条件 分成多组
                // 从站地址!!!
                // 寄存器地址类型 线圈 输入寄存器 保持寄存器
                let mut groups: HashMap<(u8, u8), Vec<Param>> = HashMap::new();
                for param in params.values() {
                    let key: (u8, u8) = (param.slave_id, param.register_type);
                    groups
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(param.clone());
                }

                println!("组的数量: {}", groups.len());

                // 遍历所有组，生成任务
                for (slave_and_type, _params) in groups {
                    // 遍历所有参数 获取最小的start_address，与最大的start_address + 4
                    let mut min_start_address = 0;
                    let mut max_start_address = 0;

                    for param in _params {
                        if param.start_address <= min_start_address {
                            min_start_address = param.start_address;
                        }
                        if param.start_address >= max_start_address {
                            max_start_address = param.start_address;
                        }
                    }

                    // 生成读取任务
                    let task = Task {
                        operation: 2,
                        register_type: slave_and_type.1,
                        start_address: min_start_address,
                        quantity: max_start_address - min_start_address + 4,
                        param_id: 1,
                        slave_id: slave_and_type.0,
                    };
                    tasks.push(task);

                    println!("放入任务，任务数量: {}", tasks.len());
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
}
