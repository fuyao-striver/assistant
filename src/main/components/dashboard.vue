<template>
  <header class="flex justify-between items-center h-8 mb-2 relative">
    <div data-tauri-drag-region class="dragDiv" />
    <div class="flex items-center">
      <img src="@/assets/icon/app-icon.png" class="h-8" draggable="false" alt="app-icon" />
      <img src="@/assets/icon/Frank.png" draggable="false" class="pl-1 h-6.25" alt="Frank" />
    </div>
    <div class="flex mt-0.5 gap-x-2">
      <n-button v-if="isShowNoticeIcon" :focusable="false" @click.prevent="showDialog" text>
        <n-icon size="20" :color="'#f0a020'">
          <bulb />
        </n-icon>
      </n-button>
      <n-button :focusable="false" @click="handleMin" text>
        <n-icon size="20">
          <circle-minus />
        </n-icon>
      </n-button>
      <n-button :focusable="false" text circle @click.prevent="isShowDrawer = true">
        <n-icon size="20">
          <settings />
        </n-icon>
      </n-button>
      <n-button :focusable="false" @click.prevent="handleConfirm" text circle>
        <n-icon size="20">
          <circle-x />
        </n-icon>
      </n-button>
    </div>
  </header>
  <n-drawer
    style="border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem"
    v-model:show="isShowDrawer"
    :placement="'bottom'"
    :auto-focus="false"
    height="473"
  >
    <setting />
  </n-drawer>
</template>

<script setup lang="ts">
import type { ConfigSettingTypes } from "@/background/types";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { CircleMinus, Settings, CircleX, Bulb } from "@vicons/tabler";
import { NIcon, NButton, NCheckbox, NSpace, NDrawer, useDialog } from "naive-ui";
import { h, ref } from "vue";
import { exit } from "@tauri-apps/plugin-process";

const isShowDrawer = ref(false);
const isShowNoticeIcon = ref(false);
const dialog = useDialog();
const { configSetting } = defineProps<{
  configSetting: ConfigSettingTypes;
}>();
const shouldCloseLOL = ref(configSetting.shouldCloseLOL);

const showDialog = () => {
  //notice.showDialog();
};
const handleConfirm = () => {
  dialog.error({
    title: "退出",
    // 使用 render 函数自定义内容
    content: () =>
      h(
        NSpace,
        { vertical: true },
        {
          default: () => [
            h("div", { style: { lineHeight: "1.5", minHeight: "24px" } }, "是否退出英雄联盟助手?"),
            h(
              NCheckbox,
              {
                // 绑定值
                checked: shouldCloseLOL.value,
                // 更新值的回调
                "onUpdate:checked": (val) => {
                  shouldCloseLOL.value = val;

                  const config: ConfigSettingTypes = JSON.parse(localStorage.getItem("configSetting") as string);
                  config.shouldCloseLOL = val;

                  localStorage.setItem("configSetting", JSON.stringify(config));
                },
              },
              { default: () => "同时关闭 LOL 客户端" },
            ),
          ],
        },
      ),
    positiveText: "确定",
    negativeText: "取消",
    autoFocus: false,
    transformOrigin: "center",
    style: "margin:8px;max-width:334px;margin-bottom:78px; border-radius:12px;",
    closable: false,
    onPositiveClick: async () => {
      exit(1);
      if (shouldCloseLOL.value) {
        // 关闭 LOL 客户端的功能
        await invoke("close_lol_client");
      }
    },
    onNegativeClick: () => {},
  });
};
const handleMin = async () => {
  await getCurrentWindow().minimize();
};
</script>
