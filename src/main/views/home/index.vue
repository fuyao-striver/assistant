<template>
  <div class="mainContent" v-if="summonerData.summonerInfo">
    <n-card size="small" class="shadow" content-style="padding-bottom: 0;">
      <!--    头像 昵称 等级-->
      <div class="h-14 flex gap-x-2">
        <n-avatar
          class="avatarEffect"
          round
          :bordered="false"
          :size="56"
          :src="summonerData.summonerInfo.imgUrl"
          fallback-src="https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/4027.png"
        />
        <n-space class="grow!" :size="[0, 0]" justify="space-between" vertical>
          <div class="flex justify-between">
            <!--昵称-->
            <n-tag type="success" class="w-32.5! justify-center!" :bordered="false" round>
              <n-ellipsis class="max-w-27.5!" :tooltip="false">
                {{ summonerData.summonerInfo.name }}
              </n-ellipsis>
            </n-tag>
            <n-button class="px-2!" :bordered="false" @click.prevent="openWin" type="success" size="small" round>
              我的战绩
            </n-button>
          </div>
          <div class="flex justify-between gap-x-3">
            <n-tag type="warning" size="small" round :bordered="false">
              {{ summonerData.summonerInfo.lv }}
            </n-tag>
            <div
              class="grow"
              style="
                background-color: rgba(240, 160, 32, 0.15);
                padding: 0 7px;
                color: #f0a020;
                font-size: 12px;
                border-radius: 12px;
              "
            >
              <div class="flex justify-between items-center">
                <n-progress
                  type="line"
                  :show-indicator="false"
                  :percentage="summonerData.summonerInfo.xp"
                  status="warning"
                  processing
                  class="w-25! mt-[1.2px]!"
                  :height="10"
                />
                <div class="pt-0.5">{{ summonerData.summonerInfo.xp }} %</div>
              </div>
            </div>
          </div>
        </n-space>
      </div>
      <!--    头像 昵称 等级-->

      <n-divider dashed style="margin: 14px 0 2px 0" />

      <!--段位 荣誉等级-->
      <n-list>
        <n-list-item>
          <n-space justify="space-between">
            <n-tag class="w-32! justify-center!" type="success" :bordered="false" :round="false">
              单双 {{ summonerData.rankList?.[0] }}
            </n-tag>
            <n-tag class="w-32! justify-center!" type="success" :bordered="false" :round="false">
              灵活 {{ summonerData.rankList?.[1] }}
            </n-tag>
          </n-space>
        </n-list-item>
        <n-list-item>
          <n-space justify="space-between">
            <n-tag class="w-32! justify-center!" type="warning" :bordered="false" :round="false">
              云顶 {{ summonerData.rankList?.[2] }}
            </n-tag>
            <n-tag class="w-32! justify-center!" type="warning" :bordered="false" :round="false">
              {{ summonerData.rankList?.[3] }}
            </n-tag>
          </n-space>
        </n-list-item>
      </n-list>
      <!--段位 荣誉等级-->
    </n-card>
  </div>
  <div class="mainContent" v-else>
    <start-game />
  </div>
</template>

<script lang="ts" setup>
import {
  NCard,
  NAvatar,
  NProgress,
  NSpace,
  NTag,
  NDivider,
  NList,
  NListItem,
  NButton,
  NEllipsis,
  NModal,
} from "naive-ui";
import { queryRankPoint, querySummonerHonorLevel, querySummonerInfo } from "@/api/summoner";
import startGame from "./startGame.vue";
import { onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SummonerData } from "@/api/types/summoner";

const curRegion = ref<string>("");
const summonerData = reactive<SummonerData>({
  summonerInfo: null,
  rankList: null,
  champLevel: null,
});
const init = async (isFristInit: boolean) => {
  const [summonerInfo,rankPoint,honorData] = await Promise.all([querySummonerInfo(),queryRankPoint(),querySummonerHonorLevel()]);
  rankPoint.push(honorData);
  summonerData.summonerInfo = summonerInfo;
  summonerData.rankList = rankPoint;
};

onMounted(() => {
  invoke<string>("get_lol_region").then((region) => {
    curRegion.value = region;
    init(true);
  });
  // TODO: 错误处理
});

const openWin = () => {
  // TODO: 打开战绩窗口
};
</script>
