import { type ConfigRank, type ConfigSettingTypes, Lane, LolTrackerMode, TierCode } from "@/background/types";

const configSetting: ConfigSettingTypes = {
  autoPickChampion: {
    championId: "157",
    isAuto: false,
  },
  autoBanChampion: {
    championId: "101",
    isAuto: false,
  },
  autoIsOne: true,
  autoAccept: 50,
  theme: "light",
  isGameInWindow: true,
  isGameInTips: false,
  autoWriteBlock: true,
  inWinOpacity: 100,
  warmTips: {
    autoRune: false,
    rankTips: false,
    teamTips: false,
  },
  lolTracker: LolTrackerMode.DISABLED,
  shouldCloseLOL: true,
};

const configRank: ConfigRank = {
  tier: TierCode.BRONZE, // 原值 200
  lane: Lane.MID, // 原值 "mid"
  is101: true, // 关联英雄 #101（泽拉斯）
};

// ─── 配置存储键名 ───
const CONFIG_KEYS = ["configSetting", "configRank"] as const;

type ConfigMap = {
  configSetting: ConfigSettingTypes;
  configRank: ConfigRank;
};

// ─── 核心逻辑 ───

/**
 * 将默认配置中新增的字段合并到已有配置
 *
 * 用于版本迭代后向已有用户的 localStorage 补充新字段，
 * 同时保留用户已有的个性化修改。
 *
 * @param storageKey - localStorage 中的键名
 * @param defaults   - 当前版本的默认配置对象（完整结构）
 */
const migrateConfig = <T extends object>(storageKey: string, defaults: T): void => {
  const raw = localStorage.getItem(storageKey);

  // localStorage 中不存在该配置，直接写入默认值
  if (raw === null) {
    localStorage.setItem(storageKey, JSON.stringify(defaults));
    return;
  }

  let stored: Record<string, unknown>;
  try {
    stored = JSON.parse(raw);
  } catch {
    // JSON 损坏，回退到默认配置
    console.warn(`[config] "${storageKey}" JSON 解析失败，已重置为默认值`);
    localStorage.setItem(storageKey, JSON.stringify(defaults));
    return;
  }

  // 快速检测：key 数量一致则无需逐项扫描
  if (Object.keys(stored).length === Object.keys(defaults).length) {
    return;
  }

  // 仅补充缺失字段，不覆盖用户已修改的值
  let mutated = false;
  for (const key of Object.keys(defaults)) {
    if (!(key in stored)) {
      stored[key] = (defaults as Record<string, unknown>)[key];
      mutated = true;
    }
  }

  if (mutated) {
    localStorage.setItem(storageKey, JSON.stringify(stored));
  }
};

/**
 * 初始化应用配置
 *
 * 首次启动时写入全部默认配置；
 * 版本升级后将新增字段合并到已有配置中，保留用户已有设置。
 */
export const configInit = (): void => {
  const defaults: Pick<ConfigMap, (typeof CONFIG_KEYS)[number]> = {
    configSetting,
    configRank,
  };

  for (const key of CONFIG_KEYS) {
    migrateConfig(key, defaults[key]);
  }
};
