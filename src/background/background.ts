import { invoke } from "@tauri-apps/api/core";
import { configInit } from "./utils/config";
import { createMainWindows } from "./utils/createWindows";
import { listen } from "@tauri-apps/api/event";
/**
 * Background 类负责初始化应用程序的后台进程
 */
class Background {
  constructor() {
    configInit();
  }
  /**
   * 通过创建主窗口来初始化后台进程
   */
  async init() {
    await createMainWindows();
    await this.initializeListeners();
  }

  /**
   * 初始化监听器
   * 设置客户端状态监听，等待客户端启动信号后开始监听客户端状态变化
   */
  private initializeListeners = async () => {
    invoke("listen_for_client_start").then(async () => {
      listen<string>("client_status", (event) => this.handleClientStatus(event.payload));
    });
  };

  private handleClientStatus = (status: string) => {
    switch (status) {
      case "ClientStarted":
        break;
    }
  };
}

// 创建 Background 实例并初始化
await new Background().init();
