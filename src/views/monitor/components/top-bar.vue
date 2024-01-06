<template lang="">
  <div class="titlebar" data-tauri-drag-region>
    <div class="window-controls">
      <div class="window-control close" @click="appWindow.close()"></div>
      <div class="window-control minimize" @click="appWindow.minimize()"></div>
    </div>
    <div class="title">
      <span>Modbus Visualizer</span>
      <span class="status"
        ><t-icon
          name="round"
          :style="{
            color: isActive ? 'green' : 'red',
          }"
      /></span>
    </div>
  </div>
  <span
    style="margin: 2px 10px; position: fixed; right: 0; top: 0"
    z-index="9999"
  >
    <span style="font-size: 11px; margin-right: 5px; color: #a9b7c6"
      >模板下载</span
    >
    <a @click="downloadTemplate" download style="cursor: pointer"
      ><t-icon name="download"></t-icon
    ></a>
    <span style="font-size: 11px; margin: 0 5px 0 10px; color: #a9b7c6"
      >导入文件</span
    >
    <a style="cursor: pointer" @click="selectAndParseFile"
      ><t-icon name="file-import"></t-icon
    ></a>
  </span>
</template>
<script setup>
import { appWindow } from "@tauri-apps/api/window";
import { onMounted, onBeforeUnmount, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { writeBinaryFile, readBinaryFile } from "@tauri-apps/api/fs";
import { save, open } from "@tauri-apps/api/dialog";
import * as XLSX from "xlsx";
import { dialog } from "@tauri-apps/api";

/**
 * 上传参数文件
 */
const selectAndParseFile = async () => {
  const path = await open({ multiple: false });
  if (path) {
    const data = await readBinaryFile(path);
    const workbook = XLSX.read(data, { type: "array" });
    const worksheet = workbook.Sheets[workbook.SheetNames[0]];

    const ip_address = worksheet["A3"].v;
    const port = worksheet["B3"].v;

    const params = XLSX.utils.sheet_to_json(worksheet, {
      range: "A6:G40",
      header: [
        "param_id",
        "name",
        "unit",
        "slave_id",
        "start_address",
        "data_type",
        "register_type",
      ],
    });
    console.log(`上传参数: ${JSON.stringify(params)}`);
    const _params = [];
    // 对params进行处理，参数名称 从站地址 起始地址 数据类型 寄存器类型 有任何一个没填写的，删除该条记录
    params.forEach((param) => {
      if (!param.name || !param.slave_id || !param.data_type) {
        params.splice(params.indexOf(param), 1);
      } else {
        // 参数起始值不存在，则默认为0
        if (!param.start_address) {
          param.start_address = 0;
        }

        // 所有参数都添加 operation=2
        param.operation = 2;
        // 将数据类型 按 1为int16 2为int32 3为float32 4为float64 转化成对应的数字
        param.data_type =
          param.data_type === "int16"
            ? 1
            : param.data_type === "int32"
            ? 2
            : param.data_type === "float32"
            ? 3
            : 4;
        // 将寄存器类型 按 1 表示保持寄存器 2 表示输入寄存器 转化成数字
        param.register_type = param.register_type === "保持寄存器" ? 1 : 2;
        _params.push(param);
      }
    });

    const d = {
      connection: {
        ip_address,
        port,
      },
      params: _params,
    };

    await invoke("convert_json_to_toml", { json: JSON.stringify(d) })
      .then((response) => {
        dialog.message("导入成功");
      })
      .catch((error) => {
        dialog.message(`导入失败: ${error}`);
      });
  }
};

/**
 * 下载模板
 */
const downloadTemplate = async () => {
  const data = await invoke("download_file", {}).catch(e => {
    console.log(`下载参数模板失败: ${e}`);
  });
  const path = await save({ defaultPath: "参数模板.xlsx" });
  if (path) {
    await writeBinaryFile(path, data);
  }
};

/**
 * 5s获取一次连接状态
 */
const timer = ref(null);
const isActive = ref(false);

const createCheckStatusTimer = async () => {
  isActive.value = await invoke("is_active", {});
  if (!timer.value) {
    timer.value = setInterval(async () => {
      isActive.value = await invoke("is_active", {});
    }, 2000);
  }
};

const removeTimer = () => {
  if (timer.value) {
    clearInterval(timer.value);
    timer.value = null;
  }
};

onMounted(() => {
  createCheckStatusTimer();
});

onBeforeUnmount(() => {
  removeTimer();
});
</script>
<style lang="less" scoped>
body {
  border: 1px solid #000;
  height: 100vh;
  width: 100vw;
  margin: 0;
  overflow: hidden;
  box-sizing: border-box;
}

.titlebar {
  height: 30px;
  background: #2b2b2b;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  padding: 0 10px;
  border: 1px solid #3c3f41;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
}

.title {
  text-align: center;
  flex-grow: 1;
  pointer-events: none;
  color: #a9b7c6;

  .status {
    margin-left: 10px;
  }
}

.window-controls {
  display: flex;
  width: 35px;
  justify-content: space-between;
}

.window-control {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

.window-control.close {
  background-color: #ff5f57;
}

.window-control.minimize {
  background-color: #ffbd2e;
}

.window-control.maximize {
  background-color: #27c93f;
}

.window-control.minimize::after {
  content: "－";
  color: black;
  font-size: 10px;
  position: relative;
}

.window-control.maximize::after {
  content: "↔";
  color: black;
  font-size: 10px;
  position: relative;
}

.window-control.close::after {
  content: "×";
  color: black;
  font-size: 10px;
  position: relative;
}
</style>
