import { createMainWindows } from "./utils/createWindows";
/**
 * Background 类负责初始化应用程序的后台进程
 */
class Background {
  /**
   * 通过创建主窗口来初始化后台进程
   */
  async init() {
    await createMainWindows();
  }
}

// 创建 Background 实例并初始化
await new Background().init();
