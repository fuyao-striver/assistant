import { window } from "@tauri-apps/api";
import { emitTo } from "@tauri-apps/api/event";

export class GameFlow {
  /**
   * 发送开始事件到主窗口
   * 该方法异步获取主窗口实例，如果窗口存在则向其发送初始化主页的事件
   */
  public sendStartEvent = async () => {
    window.Window.getByLabel("mainWindow").then((win) => {
      if (win !== null) {
        emitTo("mainWindow", "initHome");
      }
    });
  };
}
