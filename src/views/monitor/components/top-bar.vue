<template lang="">
  <div class="titlebar" data-tauri-drag-region>
    <div class="mac-window-controls">
      <div class="mac-window-control close" @click="appWindow.close()"></div>
      <div class="mac-window-control minimize" @click="appWindow.minimize()"></div>
      <div class="mac-window-control maximize" @click="appWindow.close()"></div>
    </div>
    <div class="title">Modbus Visualizer</div>
  </div>
</template>
<script setup>
import { appWindow } from "@tauri-apps/api/window";
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
  background: #2B2B2B;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  padding: 0 10px;
  border: 1px solid #3C3F41;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
}

.title {
  text-align: center;
  flex-grow: 1;
  pointer-events: none;
  color: #A9B7C6;
}

.mac-window-controls {
  display: flex;
  width: 54px;
  justify-content: space-between;
}

.mac-window-control {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

.mac-window-control.close {
  background-color: #FF5F57;
}

.mac-window-control.minimize {
  background-color: #FFBD2E;
}

.mac-window-control.maximize {
  background-color: #27C93F;
}

.mac-window-control.minimize::after {
  content: "－";
  color: black;
  font-size: 10px;
  position: relative;
}

.mac-window-control.maximize::after {
  content: "＋";
  color: black;
  font-size: 10px;
  position: relative;
}

.mac-window-control.close::after {
  content: "×";
  color: black;
  font-size: 10px;
  position: relative;
}
</style>
