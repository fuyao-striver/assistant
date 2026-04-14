<template>
  <div class="main bg-neutral-100 dark:bg-neutral-900">
    <Dashboard :configSetting="configSetting"/>
  </div>
</template>

<script setup lang="ts">
import Dashboard from "./components/dashboard.vue";

const configSetting = JSON.parse(localStorage.getItem("configSetting") as string);
</script>
