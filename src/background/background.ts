import { invoke } from "@tauri-apps/api/core";
import { configInit, getClientPath } from "./utils/config";
import { createMainWindows } from "./utils/createWindows";
import { listen } from "@tauri-apps/api/event";
import { GameFlow } from "./gameFlow";
/**
 * Background 类负责初始化应用程序的后台进程
 */
class Background {
  private gameFlow: GameFlow;

  constructor() {
    configInit();
    this.gameFlow = new GameFlow();
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

  /**
   * 初始化助手功能
   * 该方法负责初始化键盘监听并等待客户端启动，设置超时限制和轮询机制
   */
  private initAsistant = async () => {
    const TIME_LIMIT = 30000;
    let elapsedTime = 0;
    const intervalTime = 3000;
  
    await invoke("init_keyboard");
    
    // 定时检查客户端是否启动成功
    const lcuSuccess = setInterval(async () => {
      // 获取客户端路径
      const isGetPath = await getClientPath();
      if (isGetPath) {
        clearInterval(lcuSuccess);
        setTimeout(async () => {
          // 发送开始游戏事件
          this.gameFlow.sendStartEvent();
          // 启动监听器
          await invoke("start_listener");
        }, 500);
      }
  
      elapsedTime += intervalTime;
      if (elapsedTime >= TIME_LIMIT) {
        clearInterval(lcuSuccess);
        console.log("超时，客户端未启动");
      }
    }, intervalTime);
  };
}

// 创建 Background 实例并初始化
await new Background().init();
