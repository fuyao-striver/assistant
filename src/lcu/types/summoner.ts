export interface LcuSummonerInfo {
  accountId: number;
  displayName: string;
  gameName: string;
  internalName: string;
  nameChangeFlag: boolean;
  percentCompleteForNextLevel: number;
  privacy: string;
  profileIconId: number;
  puuid: string;
  rerollPoints: IRerollPoint;
  summonerId: number;
  summonerLevel: number;
  unnamed: boolean;
  xpSinceLastLevel: number;
  xpUntilNextLevel: number;
  success?: boolean;
  tagLine: string;
}

interface IRerollPoint {
  currentPoints: number;
  maxRolls: number;
  numberOfRolls: number;
  pointsCostToRoll: number;
  pointsToReroll: number;
}

export interface SummonerInfo {
  name: string;
  privacy: string;
  imgUrl: string;
  lv: string | number;
  xp: number;
  puuid: string;
  currentId: number;
  tagLine: string | undefined;
}
