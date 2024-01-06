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
      style="width: 100%; height: 100%; overflow-y: auto"
      @contextmenu="onContextMenu($event)"
      flat
      dark
      @dblclick="store.dispatch('setGridState')"
    >
      <GridLayout
        v-model:layout="layout"
        :is-draggable="true"
        :is-resizable="true"
        :col-num="colNum"
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
          @resized="resizeChart"
        >
          <div class="l-item-slot" style="width: 100%; height: 100%;">
            <div class="operation" v-if="!store.getters.gridState">
              <span class="remove" @click="removeItem(item.i)">x</span>
            </div>
            <div v-if="store.getters.gridState" class="gap">
            </div>
            <div style="width: 100%; height: 100%;">
              <param-line :param-id="item.paramId" />
              <!-- <component
                :is="item.component"
                v-if="item.component"
                :param-id="item.paramId"
              ></component> -->
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
import { v4 as uuidv4 } from "uuid";
import paramLine from "@/views/monitor/components/param-line.vue";

const params = ref([]);

async function getResult(paramId) {
  return await invoke("get_result", { paramId: paramId });
}

async function getParams() {
  params.value = await invoke("get_params", {});
  // console.log(`params: ${JSON.stringify(params.value)}`);
}

onUnmounted(() => {
  clearInterval(interval);
});

const menus = ref("1");

/**
 * grid-layout
 */
const responsive = ref(true);
const colNum = ref(12);

/**
 * 获取uuid
 */
const generateUUID = () => {
  return uuidv4();
};

/**
 * context-menu
 */
const onContextMenu = async (e) => {
  e.preventDefault();
  if (store.getters.gridState) {
    return;
  }

  await getParams();
  //prevent the browser's default menu
  e.preventDefault();
  //show your menu
  ContextMenu.showContextMenu({
    x: e.x,
    y: e.y,
    theme: "flat dark",
    items: [
      {
        label: "添加曲线",
        children: paramChildren.value,
      },
    ],
  });
  e.preventDefault();
};

const paramChildren = computed(() => {
  const children = [];
  for (let [_, item] of Object.entries(params.value)) {
    // 已添加
    const index = layout.value.findIndex(
      (item) => item.paramId === item.param_id
    );
    if (index !== -1) {
      continue;
    }

    // 判断是否禁用
    // 判断 store.getters.layout中是否有paramId
    const layoutIndex = store.getters.layout.findIndex(
      (_item) => _item.paramId === item.param_id
    );
    let disabled = false;
    if (layoutIndex !== -1) {
      disabled = true;
    }

    children.push({
      label: item.name,
      value: item.param_id,
      disabled: disabled,
      onClick: () => {
        store.dispatch("addLayout", {
          x: (layout.value.length * 2) % (colNum.value || 12),
          y: layout.value.length + (colNum.value || 12),
          w: 4,
          h: 4,
          i: generateUUID(),
          component: "param-line",
          static: false,
          paramId: item.param_id,
          paramName: item.name,
        });
      },
    });
  }
  return children;
});

/**
 * 删除节点
 */
const removeItem = (idx) => {
  store.dispatch("removeLayout", idx);
};

const layout = computed(() => {
  return store.getters.layout;
});

/**
 * Resize Chart
 */
const resizeChart = (i, newX, newY, newHPx, newWPx) => {
  store.dispatch('setResizeTag');
};

onMounted(() => {
  nextTick(() => {
    // 获取所有参数列表
    getParams();

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

.gap {
  height: 12px;
}
</style>
