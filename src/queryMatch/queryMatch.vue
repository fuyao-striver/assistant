<template>
  <div class="queryMatchMain bg-neutral-100 dark:bg-neutral-900">
    <div data-tauri-drag-region class="dragDiv" />
    <!-- 查询窗口头部 -->
    <query-header class="h-10 mb-2" />

    <div class="flex">
      <summoner-info-view
        v-if="matchStore.summonerInfo"
        :key="matchStore.summonerId"
        :sum-info="matchStore.summonerInfo"
      />
      <div style="width: 254px" v-else></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import queryHeader from "./components/queryHeader.vue";
import useMatchStore from "@/queryMatch/store";
import summonerInfoView from "./components/summonerInfoView.vue";
import { onBeforeMount } from "vue";

const matchStore = useMatchStore();

onBeforeMount(async () => {
  // 判断是否从其它窗口启动的此窗口
  const isQueryRecord = localStorage.getItem("queSumMatch");

  if (isQueryRecord === null) {
    await matchStore.init();
  }
  // TODO
  // }else {
  //   handleBlackListMatch(isQueryRecord).then(() => {
  //     localStorage.removeItem('queSumMatch')
  //   })
  // }
});
</script>
