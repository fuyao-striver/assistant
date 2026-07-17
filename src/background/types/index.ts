/** 自动选择英雄的配置项 */
interface AutoChampionSetting {
  /** 目标英雄 ID，对应英雄联盟客户端中的 champion key */
  championId: string;
  /** 是否启用自动选择 */
  isAuto: boolean;
}

/** 温馨提示（游戏内辅助信息）配置 */
interface WarmTipsSetting {
  /** 是否自动应用推荐符文 */
  autoRune: boolean;
  /** 是否显示段位信息提示 */
  rankTips: boolean;
  /** 是否显示队友信息提示 */
  teamTips: boolean;
}

/** LOL 追踪器的运行模式 */
export enum LolTrackerMode {
  /** 关闭追踪 */
  CLOSE = 0,
  /** 吸附在客户端左侧 */
  LEFT = 1,
  /** 吸附在客户端右侧 */
  RIGHT = 2,
}

/**
 * 主题枚举
 */
export enum Theme {
  LIGHT = "light",
  DARK = "dark",
}

/**
 * 应用全局配置项的类型定义
 *
 * 涵盖自动 Ban/Pick、对局辅助、界面主题等用户可自定义的设置，
 * 所有配置通过本地持久化存储，随应用启动自动加载。
 */
export interface ConfigSettingTypes {
  // ────────────────────────────────────────
  //  英雄选择阶段
  // ────────────────────────────────────────

  /** 自动 Pick 英雄设置 */
  autoPickChampion: AutoChampionSetting;
  /** 自动 Ban 英雄设置 */
  autoBanChampion: AutoChampionSetting;
  /**
   * 自动选择是否锁定（true = 锁定，false = 仅亮出）
   *
   * 部分场景下亮出而非锁定可以留出沟通余地，避免与队友冲突。
   */
  autoIsOne: boolean;

  // ────────────────────────────────────────
  //  对局流程
  // ────────────────────────────────────────

  /**
   * 自动接受对局模式 延迟多少秒接受
   */
  autoAccept: number;
  /**
   * 是否在游戏结束后自动关闭英雄联盟客户端
   *
   * 启用后可减少后台资源占用，但会中断聊天等功能。
   */
  shouldCloseLOL: boolean;
  /**
   * LOL 追踪器运行模式
   *
   * 控制是否获取并展示对局相关数据（如对手战绩、段位等）。
   *
   * @see LolTrackerMode
   */
  lolTracker: LolTrackerMode;

  // ────────────────────────────────────────
  //  游戏内辅助
  // ────────────────────────────────────────

  /** 温馨提示子配置（符文、段位、队友信息）
   * @see WarmTipsSetting
   * */
  warmTips: WarmTipsSetting;
  /**
   * 是否在对局内弹出悬浮提示窗口
   *
   * 用于展示对手/队友的关键数据概览。
   */
  isGameInTips: boolean;
  /**
   * 是否自动写入装备推荐 Block（出装方案）
   *
   * 对局开始时根据推荐数据自动覆盖客户端内的出装配置文件。
   */
  autoWriteBlock: boolean;

  // ────────────────────────────────────────
  //  界面与外观
  // ────────────────────────────────────────

  /**
   * 应用主题标识
   *
   * @see Theme
   */
  theme: Theme;
  /**
   * 是否以窗口化模式启动游戏
   *
   * 对应客户端启动参数中的 `-window` 标志。
   */
  isGameInWindow: boolean;
  /**
   * 悬浮窗不透明度，取值范围 0 ~ 100（百分比）
   *
   * 数值越低越透明，便于在游戏过程中不遮挡关键画面。
   * 传入时除以 100 后赋值给 CSS `opacity` 属性。
   */
  inWinOpacity: number;
}

/** 段位等级编码 */
export enum TierCode {
  /** 无段位 / 未定级 */
  UNRANKED = 0,
  /** 黑铁 */
  IRON = 100,
  /** 青铜 */
  BRONZE = 200,
  /** 白银 */
  SILVER = 300,
  /** 黄金 */
  GOLD = 400,
  /** 铂金 */
  PLATINUM = 500,
  /** 翡翠 */
  EMERALD = 600,
  /** 钻石 */
  DIAMOND = 700,
  /** 大师 */
  MASTER = 800,
  /** 宗师 */
  GRANDMASTER = 900,
  /** 王者 */
  CHALLENGER = 1000,
}

/** 常用位置/路线标识 */
export enum Lane {
  TOP = "top",
  JUNGLE = "jungle",
  MID = "mid",
  ADC = "adc",
  SUPPORT = "support",
}

/**
 * 排位赛偏好配置
 *
 * 用于排位辅助功能，例如根据段位和位置筛选展示数据。
 */
export interface ConfigRank {
  /**
   * 段位等级编码
   *
   * 以百位数表示大段位，十位和个位预留用于小段位扩展。
   * 例如 `200` = 青铜，`401` = 黄金 II。
   *
   * @see TierCode
   */
  tier: TierCode;

  /**
   * 常用路线
   *
   * @see Lane
   */
  lane: Lane;

  /**
   * 是否关联 ID 为 101 的英雄（泽拉斯 / Xerath）
   *
   * 启用后可能在选人阶段加载该英雄专属的推荐数据或策略提示。
   * 此字段命名较隐晦，建议后续重构时改为更具描述性的名称。
   */
  is101: boolean;
}
