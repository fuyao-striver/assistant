import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

/**
 * 创建主窗口
 *
 * 该函数创建一个透明的、无装饰的Webview窗口作为应用程序的主窗口，
 * 设置了特定的尺寸和位置，并在窗口创建完成后显示它。
 */
export const createMainWindows = async () => {
  // 创建Webview窗口实例，配置窗口属性
  const webview = new WebviewWindow("mainWindow", {
    title: "Assistant",
    url: "src/main/index.html",
    width: 320,
    height: 720,
    visible: false,
    resizable: false,
    decorations: false,
    center: true,
    transparent: true,
  });

  // 监听窗口创建完成事件，在创建完成后显示窗口
  await webview.once("tauri://webview-created", async () => {
    await webview.show();
  });
};

/**
 * 创建查询战绩窗口
 *
 * 该函数创建一个用于显示用户战绩信息的独立窗口，配置了特定的窗口属性，
 * 包括尺寸、位置、外观等，并在窗口创建完成后显示出来
 *
 */
export const createQueryMatchWindow = async () => {
  // 创建战绩查询窗口实例，配置窗口标题、URL、尺寸等参数
  const webview = new WebviewWindow("queryMatchWindow", {
    title: "我的战绩",
    url: "src/queryMatch/index.html",
    width: 1174,
    height: 668,
    resizable: false,
    decorations: false,
    center: true,
    visible: false,
    transparent: true,
  });

  // 监听窗口创建完成事件，在事件触发后显示窗口
  await webview.once("tauri://webview-created", async () => {
    await webview.show();
  });
};
