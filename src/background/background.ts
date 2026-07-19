import { configInit, getClientPath } from "@/background/utils/config.ts";
import { createMainWindows } from "@/background/utils/windows.ts";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { GameFlow } from "@/background/gameFlow.ts";

class Background {
  private gameFlow: GameFlow;

  constructor() {
    this.gameFlow = new GameFlow();
    configInit();
    this.initializeListeners();
  }

  init = async () => {
    await createMainWindows();
  };

  private initializeListeners = () => {
    invoke("listen_for_client_start").then(async () => {
      await listen<string>("client_status", async (event) => await this.handleClientStatus(event.payload));
    });
  };

  private handleClientStatus = async (state: string) => {
    switch (state) {
      case "ClientStarted":
        await this.initMain();
        break;
    }
  };

  private initMain = async () => {
    const TIME_LIMIT = 30000;
    let elapsedTime = 0;
    const intervalTime = 3000;

    await invoke("init_keyboard");
    const lcuSuccess = setInterval(async () => {
      const isGetPath = await getClientPath();
      if (isGetPath) {
        clearInterval(lcuSuccess);
        setTimeout(async () => {
          await this.gameFlow.sendStartEvent();
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

await new Background().init();
