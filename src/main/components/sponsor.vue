<template>
  <n-card :bordered="false" size="small" role="dialog" aria-modal="true">
    <n-alert v-if="isCompleted" title="本月多次使用Frank完成对局" type="info">
      {{ sentence }}
      <br />[此提示每月只会弹出一次]
    </n-alert>

    <n-alert v-else title="感谢您的支持❤️" type="success">
      {{ sentence }}
    </n-alert>

    <n-collapse class="mt-4">
      <n-collapse-item title="微信" name="1">
        <div class="flex flex-col items-center gap-1 justify-center">
          <div class="w-37.5 h-37.5">
            <img class="w-full" :src="Wechat" alt="微信" />
          </div>
          <n-tag class="text-[13px]!" checkable disabled
            >若有任何疑问, 请在付款备注中 <br /><br />
            留下您的问题和邮箱, 与您联系</n-tag
          >
        </div>
      </n-collapse-item>
      <n-collapse-item title="支付宝" name="2">
        <div class="flex flex-col items-center gap-1 justify-center">
          <div class="w-37.5 h-37.5">
            <img class="w-full" :src="Alipay" alt="支付宝" />
          </div>
          <n-tag class="text-[13px]! mt-0.5!" checkable disabled
            >若有任何疑问, 请在付款备注中 <br /><br />
            留下您的问题和邮箱, 与您联系</n-tag
          >
        </div>
      </n-collapse-item>
    </n-collapse>
  </n-card>
</template>

<script setup lang="ts">
import { NAlert, NCard, NCollapse, NCollapseItem, NTag } from "naive-ui";

const Wechat = new URL("/src/assets/matchImage/Wechat.JPEG", import.meta.url).href;
const Alipay = new URL("/src/assets/matchImage/Alipay.JPEG", import.meta.url).href;
const { isCompleted } = defineProps<{
  isCompleted: boolean;
}>();
const sentence =
  "如果您觉得软件的内容有帮助，请考虑为Frank提供一点小小的赞助！您的支持，就是我追求完美代码的续航电源！！！ 😊";
</script>

<style scoped></style>
