<template>
  <header class="flex">
    <div class="flex gap-x-2 items-center mr-3">
      <img src="@/assets/icon/app-icon.png" class="h-10" draggable="false" />
      <img src="@/assets/icon/Frank.png" draggable="false" />
      <n-button
        v-if="false"
        @click.prevent="openWeb"
        size="small"
        style="margin-left: 30px; color: #666666; width: 90.41px"
        secondary
        type="tertiary"
      >
        lolfrank.cn
      </n-button>
      <n-button
        v-else
        @click.prevent="backSelf"
        size="small"
        style="margin-left: 30px; width: 90.41px"
        secondary
        type="info"
      >
        我的战绩
      </n-button>
    </div>
    <div class="grow flex items-center gap-x-3">
      <n-button
        size="small"
        secondary
        type="tertiary"
        :bordered="false"
        style="width: 141px; color: #666666; font-size: 13.5px"
      >
        仅显示玩家战绩数据
      </n-button>
      <n-button
        size="small"
        :bordered="false"
        @click.prevent="refreshPage"
        type="success"
        style="width: 46px; padding: 0 9px"
      >
        刷新
      </n-button>
      <n-select
        size="small"
        v-model:value="selectVal"
        :disabled="inputVal !== ''"
        @update:value="changeMatchMode"
        :options="options"
        style="width: 100px; margin-left: 28px"
      />
      <n-pagination v-model:page="pageVal" @update-page="pageChange" :page-slot="10" :page-count="12" />
    </div>
    <n-space style="padding-top: 10px" :size="[8, 0]">
      <n-button @click.prevent="handleMin" text>
        <n-icon size="20">
          <circle-minus />
        </n-icon>
      </n-button>
      <n-button text circle @click.prevent="handleSet">
        <n-icon size="20">
          <settings />
        </n-icon>
      </n-button>
      <n-button text circle @click.prevent="handleClose">
        <n-icon size="20">
          <circle-x />
        </n-icon>
      </n-button>
    </n-space>
  </header>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { CircleMinus, CircleX, Settings } from "@vicons/tabler";
import {
  NButton,
  NSelect,
  NPagination,
  NAlert,
  NModal,
  NCard,
  useMessage,
  NIcon,
  NSpace,
  MessageReactive,
} from "naive-ui";
import { ref } from "vue";

const message = useMessage();
const inputVal = ref("");
const selectVal = ref(0);
const pageVal = ref(1);
const options = [
  {
    label: "全部模式",
    value: 0,
  },
  {
    label: "单双排位",
    value: 420,
  },
  {
    label: "灵活排位",
    value: 440,
  },
  {
    label: "匹配模式",
    value: 430,
  },
  {
    label: "极地乱斗",
    value: 450,
  },
  {
    label: "斗魂竞技",
    value: 1700,
  },
];
const handleMin = async () => {
  await getCurrentWindow().minimize();
};

const handleClose = async () => {
  await getCurrentWindow().close();
};

const handleSet = () => {
  message.info("无效按钮，或许起到了造型上的作用");
};

const changeMatchMode = async (queueId: number) => {
  // TODO: 获取数据
};

const refreshPage = () => {
  // TODO: 刷新页面
};

const pageChange = (page: number) => {
  // TODO: 获取数据
};
</script>
