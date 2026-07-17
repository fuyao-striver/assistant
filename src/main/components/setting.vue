<template>
  <n-drawer-content body-style="padding:12px 0" body-content-style="padding: 0">
    <n-list>
      <n-scrollbar class="max-h-145!" content-style="padding:0 12px">
        <!--    鼓励开发-->
        <n-list-item class="pt-0!">
          <div class="gap-x-5 flex justify-between items-center">
            <n-tag :bordered="false">鼓励开发</n-tag>
            <n-button
              @click.prevent="showSponsor = true"
              class="w-51.5!"
              size="small"
              secondary
              :bordered="false"
              type="warning"
            >
              赞助英雄联盟助手
            </n-button>
          </div>
        </n-list-item>
        <!--        切换主题-->
        <n-list-item>
          <div class="flex gap-x-5 justify-between items-center">
            <n-tag :bordered="false">主题样式</n-tag>
            <div class="flex grow justify-between">
              <n-radio
                :checked="theme === Theme.LIGHT"
                value="light"
                name="theme"
                @click.prevent="handleThemeChange(Theme.LIGHT)"
              >
                白羽清风
              </n-radio>
              <n-radio
                :checked="theme === Theme.DARK"
                value="dark"
                name="theme"
                @click.prevent="handleThemeChange(Theme.DARK)"
              >
                幽黑星空
              </n-radio>
            </div>
          </div>
        </n-list-item>
        <!--        窗口吸附-->
        <n-list-item>
          <div class="gap-x-5 flex justify-between items-center">
            <n-tag :bordered="false">窗口吸附</n-tag>
            <div class="flex grow justify-between">
              <n-radio
                :checked="config.lolTracker === LolTrackerMode.CLOSE"
                @click.prevent="changeAutoAdhere(LolTrackerMode.CLOSE)"
                >关闭</n-radio
              >
              <n-radio
                :checked="config.lolTracker === LolTrackerMode.LEFT"
                @click.prevent="changeAutoAdhere(LolTrackerMode.LEFT)"
                >左侧</n-radio
              >
              <n-radio
                :checked="config.lolTracker === LolTrackerMode.RIGHT"
                @click.prevent="changeAutoAdhere(LolTrackerMode.RIGHT)"
                >右侧</n-radio
              >
            </div>
          </div>
        </n-list-item>
        <!--        秒选英雄-->
        <n-list-item>
          <div class="gap-x-5 flex justify-between">
            <n-tag :bordered="false">秒选英雄</n-tag>
            <div class="flex grow items-center justify-between">
              <n-select
                v-model:value="config.autoPickChampion.championId"
                filterable
                spellcheck="false"
                :filter="searchChamp"
                :options="optionsChampion"
                :disabled="!config.autoPickChampion.isAuto"
                @update:value="saveConfig"
                placeholder="选择英雄"
                class="w-31.5!"
              />
              <n-switch v-model:value="config.autoPickChampion.isAuto" @click.prevent="saveConfig()" />
            </div>
          </div>
        </n-list-item>
        <!--        秒禁英雄-->
        <n-list-item>
          <div class="flex gap-x-5 justify-between">
            <n-tag :bordered="false">秒禁英雄</n-tag>
            <div class="flex grow items-center justify-between">
              <n-select
                v-model:value="config.autoBanChampion.championId"
                filterable
                spellcheck="false"
                size="small"
                :filter="searchChamp"
                :options="optionsChampion"
                :disabled="!config.autoBanChampion.isAuto"
                @update:value="saveConfig"
                placeholder="选择英雄"
                class="w-31.5!"
              />
              <n-switch v-model:value="config.autoBanChampion.isAuto" @click.prevent="saveConfig()" />
            </div>
          </div>
        </n-list-item>
        <!--        秒选/秒禁英雄 是否使用一次关闭-->
        <n-list-item>
          <div class="gap-x-5 flex justify-between">
            <n-tag :bordered="false">昙花一现</n-tag>
            <div class="flex grow items-center justify-between">
              <n-tag :disabled="!config.autoIsOne" :type="config.autoIsOne ? 'success' : 'default'">
                使用一次后会禁用</n-tag
              >
              <n-switch v-model:value="config.autoIsOne" @click.prevent="saveConfig" class="mt-0!" />
            </div>
          </div>
          <n-tag class="mt-1.5 w-full justify-center" :disabled="true" :bordered="false" size="small">
            秒选/秒禁英雄 功能使用一次后关闭</n-tag
          >
        </n-list-item>
        <!--        游戏窗口-->
        <n-list-item>
          <div class="gap-x-5 flex justify-between">
            <n-tag :bordered="false">游戏窗口</n-tag>
            <div class="flex grow items-center justify-between">
              <n-tag :type="config.isGameInWindow ? 'success' : 'default'" :disabled="!config.isGameInWindow">
                自动打开游戏窗口</n-tag
              >
              <n-switch v-model:value="config.isGameInWindow" @click.prevent="saveConfig" />
            </div>
          </div>
          <n-tag class="mt-1.5! w-full! justify-center!" :disabled="true" :bordered="false" size="small">
            游戏内显示战绩窗口，显示|隐藏 SHIFT+TAB</n-tag
          >
          <n-tag
            class="mt-1.5! w-full! justify-center!"
            :disabled="config.isGameInWindow"
            :bordered="false"
            size="small"
          >
            关闭自动打开后，进入游戏需点击右下角图标</n-tag
          >
        </n-list-item>
        <!--        秒接对局-->
        <n-list-item>
          <div class="gap-x-5 flex justify-between items-center">
            <n-tag :bordered="false">秒接对局</n-tag>
            <n-slider v-model:value="config.autoAccept" :step="10" @update:value="saveConfig" />
          </div>
          <n-tag class="mt-1.5! w-full! justify-center!" :disabled="true" :bordered="false" size="small"
            >数值: [ {{ "<" }}50 关闭 ] [ =50 开启 ] [ {{ "=" }}60 延迟两秒 ]</n-tag
          >
        </n-list-item>
        <!--        软件信息-->
        <n-list-item class="pb-0!">
          <div class="flex justify-between items-center">
            <n-button size="small" secondary type="tertiary" @click.prevent="openWeb(false)">
              版本 {{ version }}
            </n-button>
            <n-button size="small" secondary type="tertiary" @click.prevent="openWeb(true)"> By Java_S </n-button>
            <n-button size="small" secondary type="tertiary" @click.prevent="restart"> 重启 </n-button>
          </div>
        </n-list-item>
      </n-scrollbar>
    </n-list>
    <n-modal class="m-2! max-w-83.5!" v-model:show="showSponsor">
      <Sponsor :is-completed="true" />
    </n-modal>
  </n-drawer-content>
</template>

<script setup lang="ts">
import {
  NButton,
  NDrawerContent,
  NList,
  NListItem,
  NModal,
  NRadio,
  NScrollbar,
  NSelect,
  NSlider,
  NSwitch,
  NTag,
  type SelectOption,
  useDialog,
} from "naive-ui";
import { type Ref, ref } from "vue";
import Sponsor from "@/main/components/sponsor.vue";
import { type ConfigSettingTypes, LolTrackerMode, Theme } from "@/background/types";
import { relaunch } from "@tauri-apps/plugin-process";
import { keywordsList, optionsChampion } from "@/resources/champList.ts";
import { invoke } from "@tauri-apps/api/core";

const showSponsor = ref(false);
const dialog = useDialog();
const theme = localStorage.getItem("theme") || Theme.LIGHT;
const config: Ref<ConfigSettingTypes> = ref(JSON.parse(localStorage.getItem("configSetting") as string));
declare const __APP_VERSION__: string;
const version = __APP_VERSION__;

// 切换主题
const handleThemeChange = (value: string) => {
  if (theme !== value) {
    dialog.warning({
      title: "提示",
      content: "主题切换将重启应用, 是否执行操作",
      showIcon: true,
      positiveText: "确认",
      negativeText: "取消",
      maskClosable: true,
      closable: false,
      autoFocus: false,
      style: "margin:8px;max-width:334px",
      onPositiveClick: async () => {
        theme !== Theme.DARK ? localStorage.setItem("theme", Theme.DARK) : localStorage.setItem("theme", Theme.LIGHT);
        await relaunch();
      },
    });
  }
};

// 搜索英雄
const searchChamp = (pattern: string, option: SelectOption): boolean => {
  if (pattern === "" || pattern === null) {
    return false;
  }
  const keyword = pattern.toLowerCase();
  const renderList = keywordsList.filter((item) => item.keywords.toLowerCase().includes(keyword));

  if (renderList.length === 0) {
    return false;
  }

  for (const renderListElement of renderList) {
    if (renderListElement.name === option.label) {
      return true;
    }
  }

  return false;
};
// 设置自动吸附配置
const changeAutoAdhere = async (key: LolTrackerMode) => {
  config.value.lolTracker = key;
  saveConfig();

  const enabled = key !== LolTrackerMode.CLOSE;
  // 实时同步给后端，线程会在下一次循环（16ms内）自动调整位置
  await invoke("sync_tracker_config", { enabled, side: key });
};

const saveConfig = () => {
  localStorage.setItem("configSetting", JSON.stringify(config.value));
};
const restart = async () => {
  await relaunch();
};
const openWeb = (isSYJ: boolean) => {
  // todo
  // if (isSYJ) {
  //   open("https://syjun.vip");
  // } else {
  //   open("https://www.yuque.com/java-s/frank/introduction");
  // }
};
</script>

<style scoped></style>
