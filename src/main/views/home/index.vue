<template>
  <div class="mainContent">
    <start-game />
  </div>
</template>

<script lang="ts" setup>
import { querySummonerInfo } from "@/api/summoner";
import startGame from "./startGame.vue";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const curRegion = ref<string>("");
const init = async (isFristInit: boolean) => {
  const [summonerInfo] = await Promise.all([querySummonerInfo()]);
  console.log(summonerInfo);
};

onMounted(() => {
  invoke<string>("get_lol_region").then((region) => {
    curRegion.value = region;
    init(true);
  });
  // TODO: 错误处理
});
</script>
