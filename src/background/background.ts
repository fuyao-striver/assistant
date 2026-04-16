import { invoke } from "@tauri-apps/api/core";
import { configInit, getClientPath } from "./utils/config";
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
      listen<string>("client_status", async (event) => await this.handleClientStatus(event.payload));
    });
  };

  /**
   * 处理客户端状态变化
   * 根据传入的状态字符串执行相应的处理逻辑
   */
  private handleClientStatus = async (status: string) => {
    switch (status) {
      case "ClientStarted":
        await this.initAsistant();
        break;
    }
  };

  private initAsistant = async () => {
    const TIME_LIMIT = 30000;
		let elapsedTime = 0;
		const intervalTime = 3000;

		await invoke("init_keyboard");
		const lcuSuccess = setInterval(async () => {
      // TODO : 获取客户端路径失败
			const isGetPath = await getClientPath();
			if (isGetPath) {
				clearInterval(lcuSuccess);
				setTimeout(() => {
          // TODO:发送开始游戏事件
					this.gameFlow.sendStartEvent();
          // TODO: 启动监听器
					invoke("start_listener");
				}, 500);
			}

			elapsedTime += intervalTime;
			if (elapsedTime >= TIME_LIMIT) {
				clearInterval(lcuSuccess);
				console.log("超时，客户端未启动");
			}
		}, intervalTime);
  }
}

// 创建 Background 实例并初始化
await new Background().init();
