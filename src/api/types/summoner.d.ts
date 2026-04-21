/**
 * 召唤师信息接口
 */
export interface SummonerInfo {
  /** 召唤师名称 */
  name: string;
  /** 隐私设置状态 */
  privacy: string;
  /** 头像图片URL */
  imgUrl: string;
  /** 召唤师等级，可能是字符串或数字类型 */
  lv: string | number;
  /** 当前经验值 */
  xp: number;
  /** 召唤师唯一PUUID标识符 */
  puuid: string;
  /** 当前ID编号 */
  currentId: number;
  /** 标签后缀，可能未定义 */
  tagLine: string | undefined;
}

/**
 * 召唤师数据接口
 * 定义了召唤师信息、排位列表和英雄等级等数据结构
 */
export interface SummonerData {
  summonerInfo: SummonerInfo | null;
  rankList: string[] | null;
  champLevel: string[] | null;
}
