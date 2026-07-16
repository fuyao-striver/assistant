import { configInit } from "@/background/utils/config.ts";
import { createMainWindows } from "@/background/utils/windows.ts";

class Background {
  constructor() {
    configInit();
  }

  init = async () => {
    await createMainWindows();
  };
}

await new Background().init();
