import { queryRankPoint, querySummonerInfo } from "@/api/summoner";
import type { SummonerInfo } from "@/api/types/summoner";
import { defineStore } from "pinia";

const useMatchStore = defineStore("useMatchStore", {
  state: () => ({
    summonerId: -1,
    localSummonerId: -1,
    matchLoading: true,
    summonerInfo: null as { info: SummonerInfo; rank: string[] } | null,
  }),
  actions: {
    async init(summonerId?: number, localSummonerId?: number) {
      const sumResult = await this.getSummonerInfo(summonerId);
      if (sumResult === null) {
        return;
      }
      if (summonerId === undefined && localSummonerId === undefined) {
        this.localSummonerId = sumResult.summonerInfo.currentId;
      } else if (localSummonerId !== undefined) {
        this.localSummonerId = localSummonerId as number;
      }
      this.summonerInfo = { info: sumResult.summonerInfo, rank: sumResult.rankList };
      this.summonerId = sumResult.summonerInfo.currentId;
    },

    /**
     * 获取召唤师信息
     * @param summonerId - 召唤师ID，可选参数
     * @returns 返回包含召唤师基本信息和排名列表的对象，如果未找到召唤师信息则返回null
     */
    async getSummonerInfo(summonerId?: number) {
      // 查询召唤师基本信息
      const summonerInfo = await querySummonerInfo(summonerId);
      if (summonerInfo !== null) {
        // 根据puuid查询排名积分信息
        const rankList = await queryRankPoint(summonerInfo.puuid);
        return { summonerInfo, rankList };
      }
      return null;
    },
  },
});

export default useMatchStore;
