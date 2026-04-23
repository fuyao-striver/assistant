<template>
  <div class="queryMatchMain bg-neutral-100 dark:bg-neutral-900">
    <div data-tauri-drag-region class="dragDiv" />
    <!-- 查询窗口头部 -->
    <query-header class="h-10 mb-2" />
  </div>
</template>

<script setup lang="ts">
import queryHeader from "./components/queryHeader.vue";
</script>
