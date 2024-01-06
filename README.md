# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Visualization display of Modbus protocol
[![Modbus Visualizer](https://github.com/miaokela/Modbus-Visualizer/blob/dev/modbus-visualizer.gif "Modbus Visualizer")](https://www.google.com)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Project purpose
```
Load data by importing files and provide the ability to export the entire configuration as a backup file for restoration after import. Monitor Modbus data in the form of charts and text lists, allowing for flexible drag-and-drop layout, 
and the freedom to choose components and their parameters.
```

## Use the component.
```
TDesign
Vue-router
Vuex
Echarts
Less
```

> yarn add tdesign-vue-next
> yarn add vue-router
> yarn add vuex
> yarn add echarts
> yarn add less less-loader --dev

#### Import TDesign on demand.
> yarn add unplugin-vue-components unplugin-auto-import --dev

- Vite config
```javascript
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { TDesignResolver } from 'unplugin-vue-components/resolvers';
export default {
  plugins: [
    // ...
    AutoImport({
      resolvers: [TDesignResolver({
        library: 'vue-next'
      })],
    }),
    Components({
      resolvers: [TDesignResolver({
        library: 'vue-next'
      })],
    }),
  ],
};
```

#### Windows下打包遇到的问题解决方案
> https://zhuanlan.zhihu.com/p/623152313

- 添加aarch64
> rustup target add aarch64-pc-windows-msvc

- 添加i686
> rustup target add i686-pc-windows-msvc
