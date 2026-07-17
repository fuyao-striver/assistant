import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { type ConfigSettingTypes, LolTrackerMode } from "@/background/types";
import { invoke } from "@tauri-apps/api/core";

// 创建主窗口
export const createMainWindows = async () => {
  const webview = new WebviewWindow("main", {
    title: "assistant",
    url: "src/main/index.html",
    width: 320,
    height: 720,
    visible: false,
    resizable: false,
    decorations: false,
    center: true,
    transparent: true,
  });
  await webview.once("tauri://webview-created", async () => {
    await webview.show();
    // 同步主窗口吸附
    const isTracker: ConfigSettingTypes = JSON.parse(localStorage.getItem("configSetting") as string);

    const enabled = isTracker.lolTracker !== LolTrackerMode.CLOSE;
    console.log(enabled, isTracker.lolTracker);
    // 同步配置到后端
    await invoke("sync_tracker_config", { enabled, side: isTracker.lolTracker });
    // 尝试启动循环
    await invoke("start_tracking_loop");
  });
};
