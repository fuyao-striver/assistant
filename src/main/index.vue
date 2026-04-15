<template>
  <div class="main bg-neutral-100 dark:bg-neutral-900">
    <Dashboard :configSetting="configSetting" />
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>
</template>

<script setup lang="ts">
import Dashboard from "./components/dashboard.vue";

const configSetting = JSON.parse(localStorage.getItem("configSetting") as string);
</script>
