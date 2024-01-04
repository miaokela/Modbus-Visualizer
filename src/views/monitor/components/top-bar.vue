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
    <a style="cursor: pointer"><t-icon name="file-import"></t-icon></a>
  </span>
</template>
<script setup>
import { appWindow } from "@tauri-apps/api/window";
import { onMounted, onBeforeUnmount, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { writeBinaryFile } from '@tauri-apps/api/fs'
import { save } from '@tauri-apps/api/dialog'

/**
 * 下载模板
 */
const downloadTemplate = async () => {
  const data = await invoke("download_file", {});
  const path = await save({ defaultPath: '参数模板.xlsx' });
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
    }, 5000);
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
