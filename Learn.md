## Rust 新知识

#### lazy_static
```text
在 rust 中 lazy_static 表示程序运行期间只初始化一次的静态变量，
初始化的动作可以延迟到第一次使用时，而且初始化是线程安全的。
主要的应用场景：全局配置，环境变量，连接池等
如果要修改创建的全局变量，可以使用Mutex保护数据，多线程中配合Arc智能指针来完成这个需求
```

```rust
use lazy_static::lazy_static;

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

#[derive(Debug, Clone, Deserialize)]
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

lazy_static! {
    static ref TASKS: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
    static ref PARAMS: Arc<Mutex<HashMap<u16, Param>>> = Arc::new(Mutex::new(HashMap::new()));
}
```

> 使用定义的静态变量
```rust
// lock()返回一个MutexGuard智能指针，允许你去修改数据，离开作用域时会自动drop释放
let params = PARAMS.lock().unwrap();
```

> 简单示例
```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
} // `num` 离开其作用域，锁被释放

println!("m = {:?}", m);
```

#### match、unwrap、? 的使用场景

- match
```text
match 是 Rust 中的一个控制流结构，它允许你根据一个值的模式进行分支处理。
当你需要对 Option 或 Result 的所有可能的值进行详细处理时，你可以使用 match
```

> 示例
```rust
let optional = Some(5);
match optional {
    Some(i) => println!("Value is: {}", i),
    None => println!("No value"),
}
```

- unwrap
```text
unwrap 是 Option 和 Result 类型的一个方法，它会返回包装在 Some 或 Ok 中的值，
如果值是 None 或 Err，它会引发 panic。当你确定 Option 或 Result 一定不是 None 或 Err，
或者在它是 None 或 Err 时引发 panic 是可接受的，你可以使用 unwrap。
```

> 示例
```rust
let optional = Some(5);
let value = optional.unwrap(); // value is 5
```

- ?
```text
? 是 Rust 中的一个错误处理运算符，它用于简化错误处理。
如果 Result 是 Ok，它会返回包装在 Ok 中的值，如果 Result 是 Err，
它会立即从当前函数返回 Err。? 只能在返回 Result 或 Option 的函数中使用。
当你希望在遇到错误时立即返回错误，而不是处理错误，你可以使用 ?。
```

> 示例
```rust
fn foo() -> Result<(), &'static str> {
    let err_result: Result<(), &'static str> = Err("An error occurred");
    err_result?; // If `err_result` is `Err`, return it from `foo`
    Ok(())
}
```

- 总结
```text
当你调用 unwrap 并且 Option 或 Result 是 None 或 Err 时，Rust 程序会 panic 并开始崩溃过程。
这通常是不被推荐的，因为它会导致程序立即终止，而不是优雅地处理错误。

然而，有些情况下，你可能会选择让程序在遇到不可恢复的错误时 panic。
例如，如果你正在编写一个原型或者快速脚本，你可能会选择使用 unwrap 来快速处理错误，
而不是编写完整的错误处理代码。或者，如果你知道在某个特定的情况下，Option 或 Result 一定不会是 None 或 Err，
你也可以使用 unwrap。

但是，在生产环境的代码中，你应该尽量避免使用 unwrap，而是使用 match 或 ? 来处理错误。
这样，你可以优雅地处理错误，而不是让程序崩溃。
```

#### Serialize、Deserialize两个trait的作用
```text
在 Rust 中，Deserialize 和 Serialize 是两个来自 serde crate 的 trait，它们用于支持数据的序列化和反序列化。

1. Serialize：这个 trait 表示一个类型可以被序列化，也就是可以被转换为一种可以存储或传输的格式，如 JSON、YAML、Bincode 等。
如果你的类型实现了 Serialize trait，那么你可以使用 serde 提供的函数将你的类型序列化为这些格式。

2. Deserialize：这个 trait 表示一个类型可以被反序列化，也就是可以从一种可以存储或传输的格式中恢复。
如果你的类型实现了 Deserialize trait，那么你可以使用 serde 提供的函数从这些格式中恢复你的类型。
```
> 示例
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Serialize it to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize it back to a `Point`.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

#### as_mut 在rust中的作用与应用场景
> as_mut 是 Rust 中处理 Option 和 Result 的一种重要方法，它提供了一种在不消耗原始值的情况下修改值的方式
```text
在 Rust 中，as_mut 方法主要用于获取 Option 或 Result 中值的可变引用。
如果 Option 是 Some(value)，那么 as_mut 会返回 Some(&mut value)；
如果 Option 是 None，那么 as_mut 会返回 None。

这个方法在你需要修改 Option 中的值，但又不想取出这个值的时候非常有用。
因为 as_mut 返回的是一个引用，所以原始的 Option 不会被移动或者消耗。
```

> 应用场景
```text
1. 当你需要修改 Option 或 Result 中的值，但又不想消耗它们时。
2. 当你需要持有一个可变引用，以便稍后进行修改，但又不想立即进行修改时。
```

> 示例
```rust
let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
```

#### Rust 中监听文件的变化示例

```rust
use crate::utils::common::{update_connection, update_param, Config};
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs, thread};

use super::modbus_lib::get_modbus_conn;

// 监听文件变化线程
pub fn watch_param_config() {
    thread::spawn(move || {
        let (tx, rx) = channel();

        // 创建一个 watcher，当文件或目录变化时，事件会被发送到通道
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        // 添加要监听的目录，RecursiveMode::Recursive 表示递归监听所有子目录
        watcher
            .watch("modbus.toml", RecursiveMode::Recursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(_event) => {
                    // 读取modbus.toml文件内容，并更新
                    let content = fs::read_to_string("modbus.toml").unwrap();
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
                        modbus_conn.reconnect(&new_conn);
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
```

