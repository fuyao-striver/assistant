<template>
  <header class="flex justify-between items-center h-8 mb-2 relative">
    <div data-tauri-drag-region class="dragDiv" />
    <div class="flex items-center">
      <img src="@/assets/icon/app-icon.png" alt="app-icon" class="h-8" draggable="false" />
      <img src="@/assets/icon/Frank.png" alt="frank" class="pl-1 h-6.25" draggable="false" />
    </div>
    <div class="flex mt-0.5 gap-x-2">
      <n-button :focusable="false" text @click.prevent="">
        <n-icon size="20" :color="'#f0a020'">
          <bulb />
        </n-icon>
      </n-button>
      <n-button :focusable="false" @click.prevent="handleMinimize()" text>
        <n-icon size="20">
          <remove-circle-outline />
        </n-icon>
      </n-button>
      <n-button :focusable="false" text circle @click.prevent="settingDrawer = true">
        <n-icon size="20">
          <settings-outline />
        </n-icon>
      </n-button>
      <n-button :focusable="false" text circle @click.prevent="handleClose()">
        <n-icon size="20">
          <close-circle-outline />
        </n-icon>
      </n-button>
    </div>
  </header>
  <n-drawer v-model:show="settingDrawer" :placement="'bottom'" :auto-focus="false" height="580" class="rounded-t-lg!">
    <setting />
  </n-drawer>
</template>

<script setup lang="ts">
import { NButton, NCheckbox, NDrawer, NIcon, NSpace, useDialog } from "naive-ui";
import { Bulb, RemoveCircleOutline, SettingsOutline, CloseCircleOutline } from "@vicons/ionicons5";
import { h, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { ConfigSettingTypes } from "@/background/types";
import { exit } from "@tauri-apps/plugin-process";
import Setting from "@/main/components/setting.vue";

const settingDrawer = ref(false);
const dialog = useDialog();
const { configSetting } = defineProps<{
  configSetting: ConfigSettingTypes;
}>();
const shouldCloseLOL = ref(configSetting.shouldCloseLOL);
// 最小化程序
const handleMinimize = async () => {
  await getCurrentWindow().minimize();
};
// 关闭程序
const handleClose = () => {
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
    onPositiveClick: () => {
      // todo
      // if (shouldCloseLOL.value) {
      //   invokeLcu("post", "/process-control/v1/process/quit");
      // }
      exit(1);
    },
    onNegativeClick: () => {},
  });
};
</script>

<style scoped></style>
