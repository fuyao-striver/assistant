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

/**
 * 查询排位积分信息
 * @param puuid - 玩家的PUUID，可选参数。如果提供PUUID则查询指定玩家的排位统计，否则查询当前玩家的排位统计
 * @returns 返回排位积分数据数组的Promise
 */
export const queryRankPoint = async (puuid?: string): Promise<string[]> => {
  // 根据是否提供PUUID来构建不同的API端点
  const endpoint = puuid ? `/lol-ranked/v1/ranked-stats/${puuid}` : "/lol-ranked/v1/current-ranked-stats";
  return invoke<string[]>("query_rank_point", { endpoint });
};

/**
 * 查询召唤师荣誉等级
 *
 * @returns Promise<string> 返回召唤师的荣誉等级信息
 */
export const querySummonerHonorLevel = async (): Promise<string> => {
  return invoke<string>("query_summoner_honor_level");
};

/**
 * 查询英雄熟练度列表
 * 获取指定召唤师或本地玩家的英雄熟练度信息
 *
 * @param summonerPuuid - 召唤师PUUID，可选参数。如果提供则查询指定召唤师的英雄熟练度，否则查询本地玩家的英雄熟练度
 * @returns 返回一个Promise，解析为二维字符串数组，包含英雄熟练度数据
 */
export const queryMasteryChampList = async (summonerPuuid?: string) => {
  // 根据是否提供召唤师PUUID来构建不同的API端点
  const endpoint = summonerPuuid
    ? `/lol-champion-mastery/v1/${summonerPuuid}/champion-mastery`
    : "/lol-champion-mastery/v1/local-player/champion-mastery";
  return invoke<string[]>("query_champion_mastery", { endpoint });
};
