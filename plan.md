#### 需求
```
通过modbus协议去控制机械设备的启停,监控相关的数据:电流/转速/扭矩...
```

- Electron
```
纯js开发
底层通过chromium C++编写的
与系统交互 node.js 主进程
前端 通过vue.js  渲染层
渲染层与主进程之间  preload 预加载脚本 为了避免直接通过前端js代码与系统进行交互
preload层会通过ipc通信的方式调用主进程的native接口
```

- 核心功能
> 控制/监控
> 读写寄存器 modbus中涉及到哪些寄存器? Coil线圈 holding register 保持寄存器
> input register 输入寄存器
```
控制: 写入holding register
读取: 读取input regitser/holding register
线圈: 1/0 设备的启停
```

- Electron遇到的问题
```
modbus tcp 网口
modbus rtu 串口
node-serial模块,在用electron-builder去打包的时候,发现版本不兼容
```
>解决方案: 编写dll共享链接库
```
c c++不会
golang 编写modbus tcp/rtu 相关的代码 然后通过c函数形式导出给node.js来调用
```

- 另外的问题: 打包出来的exe文件太大!!!!
> 为什么electron打包这么大?
```
内置了chromium浏览器
node环境 js的运行时
100m +
前端的库 可以通过vite tree-shaking 按需导入 最终编译的代码会很少
在主进程main中 node.js运行环境中没法打包 node_modules里面的文件 意味着 第三方库
有多大 打包之后就有多大
```

### 最近在学rust    没东西练手
> github上面start比较多的客户端开发框架 tauri 
- 选择原因
> 为了学rust 可以提升原本只是学习高级语言的认知 堆栈
> 考虑到你的数据是存储在内存的哪一部分?

> 另外就是 打包之后 很小 很小    ,...

- 主要做的东西
> 实现modbus监控/控制设备的核心内容

> 软件内部控制通过读写modbus.toml  目前是明文 aes加密 -->modbus....txt
> 提供给用户: excel文件模板 下载 按照模板的格式去填写相关参数 包括modbus
> 连接的参数: modbus tcp: ip port modbus rtu: 波特率/数据位/检验位/停止位

- 实现了整个通信 获取数据的流程


- 目前只是modbus  fanuc cnc数控系统 focas函数库

- 结果 产出: 一个开源软件 学习了一门语言 rust 遇到问题 解决问题
  熟悉这门语言遇到问题 解决问题 熟悉这门语言





























