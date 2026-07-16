<template>
  <n-config-provider :locale="zhCN" :theme-overrides="themeOverrides" :theme="theme === Theme.DARK ? darkTheme : null">
    <n-dialog-provider>
      <n-message-provider :placement="'bottom'">
        <index />
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { darkTheme, NConfigProvider, NDialogProvider, NMessageProvider, zhCN } from "naive-ui";
import { themeOverrides } from "@/main/utils/theme.ts";
import { Theme } from "@/background/types";
import Index from "@/main/index.vue";

const theme = localStorage.getItem("theme") || Theme.LIGHT;

if (theme === Theme.DARK) {
  document.documentElement.classList.add(Theme.DARK);
}
</script>

<style scoped></style>
