import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

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
    // todo 同步主窗口吸附
  });
};
