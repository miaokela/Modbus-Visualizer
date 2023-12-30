<script setup>
import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const result = ref("");

async function getResult() {
  result.value = await invoke("get_result", { paramId: 1 });
}

const interval = setInterval(getResult, 1000);

onUnmounted(() => {
  clearInterval(interval);
});
</script>

<template>
  <div class="param">
    <div class="param-item">{{ result.name }}</div>
    <div class="param-item">{{ result.val }}</div>
    <div class="param-item">{{ result.unit }}</div>
  </div>
</template>

<style lang="less" scoped>
.param {
  display: flex;
  .param-item {
    padding: 20px;
  }
}
</style>
