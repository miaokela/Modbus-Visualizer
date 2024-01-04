<template lang="">
  <div :id="'chart_' + paramId" style="width: 100%; height: 100%"></div>
</template>
<script setup>
import { ref, onMounted, onBeforeMount, nextTick, watch } from "vue";
import * as echarts from "echarts";
import { invoke } from "@tauri-apps/api/tauri";
import moment from "moment";
import store from "@/store";

const props = defineProps({
  paramId: {
    type: Number,
    default: 0,
  },
});

const chart = ref(null);

const dataX = ref([]); // X 轴的数据
const dataY = ref([]); // Y 轴的数据

const fontColor = "gray";

const timerBreak = ref(false);

async function getResult(paramId) {
  return await invoke("get_result", { paramId: paramId });
}

const initChart = () => {
  if (chart.value) {
    chart.value.dispose();
  }

  timerBreak.value = false;
  chart.value = echarts.init(document.getElementById(`chart_${props.paramId}`));

  // 更新数据事件
  async function updateData() {
    if (timerBreak.value) {
      return;
    }

    if (store.getters.gridState) {
      const param = await getResult(props.paramId);
      const val = param.val;

      console.log(`Get data: ${val}`);
      dataX.value.push(moment().format("HH:mm:ss"));
      dataY.value.push(val);

      chart.value.setOption({
        xAxis: {
          data: dataX.value,
        },
        series: [
          {
            data: dataY.value,
          },
        ],
      });
      // 修改name
      chart.value.setOption({
        yAxis: {
          name: `${param.name}(${param.unit})`,
        },
      });

      // 如果数据超过一定的长度，则移除最开始的数据，保持一定的显示范围
      if (dataX.value.length > 50) {
        dataX.value.shift();
        dataY.value.shift();
      }
    }

    const _during = 1000;
    // 控制更新的频率
    setTimeout(updateData, _during); // 每隔1秒更新一次数据
  }
  chart.value.setOption({
    smooth: true,
    grid: {
      left: "20px",
      right: "27px",
      top: "27px",
      bottom: "27px",
      containLabel: true,
    },
    tooltip: {
      show: true,
      trigger: "item",
    },
    legend: {
      show: true,
      x: "center",
      y: "20",
      icon: "stack",
      itemWidth: 10,
      itemHeight: 10,
      textStyle: {
        color: "fontColor",
      },
      data: [""],
    },
    tooltip: {
      trigger: "axis",
      axisPointer: {
        show: true,
        label: {
          show: false,
        },
      },
    },
    xAxis: [
      {
        type: "category",
        boundaryGap: false,
        axisLabel: {
          color: fontColor,
        },

        axisLine: {
          show: true,
          lineStyle: {
            color: fontColor,
          },
        },
        axisTick: {
          show: false,
        },
        splitLine: {
          show: false,
          lineStyle: {
            color: "#77787b",
          },
        },
        data: dataX.value,
      },
    ],

    yAxis: [
      {
        type: "value",
        name: "mm/s",
        axisLabel: {
          formatter: "{value}",
          textStyle: {
            color: fontColor,
          },
        },
        axisLine: {
          show: false,
          lineStyle: {
            color: fontColor,
          },
        },
        axisTick: {
          show: false,
        },
        splitLine: {
          show: true,
          lineStyle: {
            color: "#77787b",
          },
        },
        axisPointer: {
          show: false,
        },
      },
    ],
    series: [
      {
        name: "",
        type: "line",
        stack: "",
        smooth: true,
        symbol: "circle",
        symbolSize: 0,
        itemStyle: {
          normal: {
            color: "#3B86FF",
            lineStyle: {
              color: "#3B86FF",
              width: 2,
            },
            areaStyle: {
              //color: '#94C9EC'
              color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                {
                  offset: 0,
                  color: "rgba(59,134,255,0.1)",
                },
                {
                  offset: 1,
                  color: "rgba(59,134,255,0.6)",
                },
              ]),
            },
          },
        },
        markPoint: {
          itemStyle: {
            normal: {
              color: "red",
            },
          },
        },
        data: dataY.value,
      },
    ],
  });
  updateData();
};

/**
 * 监听是否重载图表大小
 */
watch(
  () => store.getters.resizeTag,
  () => {
    if (chart.value) {
      chart.value.resize();
    }
  }
)

/**
 * 监听是否重建立图表
 */
watch(
  () => store.getters.gridState,
  (n, o) => {
    if (n) {
      initChart();
    }
  }
)

onMounted(() => {
  // console.log(123);
  nextTick(() => {
    initChart();
  });
});

onBeforeMount(() => {
  if (chart.value) {
    chart.value.dispose();
  }
  timerBreak.value = true;
});
</script>
<style lang=""></style>
