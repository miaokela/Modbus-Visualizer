<script setup>
import topBar from "@/views/monitor/components/top-bar.vue";

</script>

<template>
  <div id="app">
    <top-bar></top-bar>
    <router-view class="router" />
  </div>
</template>

<style lang="less">
.router {
  margin-top: 30px;
  height: calc(100vh - 30px);
}
body {
  background-color: gray;
}

.mx-menu-host {
  padding-top: 2px !important;
  padding-bottom: 2px !important;
  .mx-context-menu {
    top: -2px !important;
    padding-top: 2px !important;
    padding-bottom: 2px !important;
  }
}
</style>
