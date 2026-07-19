// 查询本地召唤师信息
import type { LcuSummonerInfo, SummonerInfo } from "@/lcu/types/summoner.ts";
import { invokeLcu, Method } from "@/lcu/index.ts";

/**
 * 查询本地召唤师信息
 * @param summonerId 召唤师id
 */
export const querySummonerInfo = async (summonerId?: number | string): Promise<SummonerInfo | null> => {
  const endpoint = summonerId ? `/lol-summoner/v1/summoners/${summonerId}` : "/lol-summoner/v1/current-summoner";

  const summonerInfo: LcuSummonerInfo | null = await invokeLcu(Method.GET, endpoint);

  if (summonerInfo === null) {
    return null;
  }

  return {
    privacy: summonerInfo.privacy,
    puuid: summonerInfo.puuid,
    tagLine: summonerInfo.tagLine,
    name: summonerInfo.gameName || summonerInfo.displayName,
    currentId: summonerInfo.summonerId,
    lv: "Lv " + summonerInfo.summonerLevel,
    xp: parseInt(String((summonerInfo.xpSinceLastLevel / summonerInfo.xpUntilNextLevel) * 100)),
    imgUrl: `https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/${summonerInfo.profileIconId}.png`,
  };
};
