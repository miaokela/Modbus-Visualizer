<template>
  <div>
    <!-- <t-head-menu v-model="menus" theme="dark" height="120px">
      <template #logo>
        <img
          height="20"
          width="40"
          src="https://tdesign.gtimg.com/site/baseLogo-dark.png"
          alt="logo"
        />
      </template>
      <t-menu-item value="1"> Modbus </t-menu-item>
      <template #operations>
        <div class="t-demo-menu--dark">
          <t-button variant="text" shape="square">
            <template #icon><t-icon name="search" /></template>
          </t-button>
        </div>
      </template>
    </t-head-menu> -->
    <div
      style="width: 100%; height: 100%; overflow-y: scroll"
      @contextmenu="onContextMenu($event)"
      flat
      dark
    >
      <GridLayout
        v-model:layout="layout"
        :is-draggable="true"
        :is-resizable="true"
        :col-num="12"
        :row-height="30"
      >
        <GridItem
          v-for="item in layout"
          :key="item.i"
          v-slot="{ style }"
          class="l-item"
          :static="item.static"
          :x="item.x"
          :y="item.y"
          :w="item.w"
          :h="item.h"
          :i="item.i"
        >
          <div class="l-item-slot">
            <!-- <component
              :is="ComponentsLib[item.component]"
              v-if="item.component"
              :style-obj="style"
            ></component> -->
            <div class="operation">
              <span class="remove" @click="removeItem(item.i)">x</span>
            </div>
            <div>

            </div>
          </div>
        </GridItem>
      </GridLayout>
    </div>
  </div>
</template>

<script setup>
import { ref, onUnmounted, onMounted, nextTick, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { GridLayout, GridItem } from "vue3-grid-layout-next";
import ContextMenu from "@imengyu/vue3-context-menu";
import store from "@/store";

const result = ref("");

async function getResult() {
  result.value = await invoke("get_result", { paramId: 1 });
}

const interval = setInterval(getResult, 1000);

onUnmounted(() => {
  clearInterval(interval);
});

const menus = ref("1");

/**
 * grid-layout
 */
const responsive = ref(true);

/**
 * context-menu
 */
const onContextMenu = (e) => {
  //prevent the browser's default menu
  e.preventDefault();
  //show your menu
  ContextMenu.showContextMenu({
    x: e.x,
    y: e.y,
    theme: "flat dark",
    items: [
      {
        label: "A menu item",
        onClick: () => {
          alert("You click a menu item");
        },
      },
      {
        label: "A submenu",
        children: [{ label: "Item1" }, { label: "Item2" }, { label: "Item3" }],
      },
    ],
  });
};

/**
 * 删除节点
 */
const removeItem = (idx) => {
  store.dispatch("removeLayout", idx);
};

const layout = computed(() => {
  return store.getters.layout;
})

onMounted(() => {
  nextTick(() => {
    const divs = document.querySelectorAll(".l-item");
    divs.forEach(function (div) {
      div.addEventListener("contextmenu", function (event) {
        event.preventDefault();
      });
    });
  });
});
</script>

<style lang="less" scoped>
.t-menu__operations {
  .t-button {
    margin-left: 8px;
  }
}
.t-demo-menu--dark {
  .t-button {
    color: #fff;
    &:hover {
      background-color: #4b4b4b;
      border-color: transparent;
      --ripple-color: #383838;
    }
  }
}

.vue-grid-item {
  background-color: #4b4b4b;
}

.t-head-menu {
  background-color: #2b2b2b;
}

.operation {
  display: flex;
  justify-content: flex-end;

  .remove {
    cursor: pointer;
    padding-right: 10px;
  }
}
</style>
