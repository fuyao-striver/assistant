import { invoke } from "@tauri-apps/api/core";
import type { SummonerInfo } from "./types/summoner";

/**
 * 查询召唤师信息
 * @param summonerId 召唤师ID，可选参数。如果不提供则查询当前登录的召唤师信息
 * @returns 返回SummonerInfo类型的数据或null
 */
export const querySummonerInfo = async (summonerId?: number | string): Promise<SummonerInfo | null> => {
  // 根据是否提供summonerId来确定请求端点
  const endpoint = summonerId ? `/lol-summoner/v1/summoners/${summonerId}` : "/lol-summoner/v1/current-summoner";
  return invoke<SummonerInfo | null>("query_summoner_info", { endpoint });
};
