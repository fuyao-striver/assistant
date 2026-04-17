use serde::{Deserialize, Serialize};

/// LCU召唤师信息结构体
/// 包含召唤师的基本信息、游戏数据和个人设置等详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcuSummonerInfo {
    /// 账户ID，用于标识用户账户
    pub account_id: i64,
    /// 显示名称，用户在游戏中的显示名字
    pub display_name: String,
    /// 游戏名称，用户的游戏内名称
    pub game_name: String,
    /// 内部名称，系统内部使用的名称标识
    pub internal_name: String,
    /// 名称更改标志，指示用户是否最近更改了名称
    pub name_change_flag: bool,
    /// 下一等级完成百分比，表示当前经验值距离下一等级的进度
    pub percent_complete_for_next_level: i32,
    /// 隐私设置，控制个人资料的可见性
    pub privacy: String,
    /// 头像图标ID，标识用户选择的头像
    pub profile_icon_id: i32,
    /// PUUID（Player Universally Unique Identifier），玩家唯一标识符
    pub puuid: String,
    /// 重掷点数信息，包含当前可用的重掷点数相关数据
    pub reroll_points: IRerollPoint,
    /// 召唤师ID，游戏中召唤师的唯一标识
    pub summoner_id: i64,
    /// 召唤师等级，用户在游戏中的等级
    pub summoner_level: i32,
    /// 未命名标志，指示账户是否未被命名
    pub unnamed: bool,
    /// 自上次等级以来的经验值，当前等级已获得的经验
    pub xp_since_last_level: i64,
    /// 直到下一级所需的经验值，升级还需要多少经验
    pub xp_until_next_level: i64,
    /// 请求成功状态，可选项，用于标识请求是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 标签行，通常包含用户的后缀或标签信息
    pub tag_line: Option<String>,
}

/// 重掷点数信息结构体
/// 包含与重掷功能相关的点数和配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IRerollPoint {
    /// 当前点数，用户当前拥有的重掷点数
    pub current_points: i32,
    /// 最大重掷次数，用户可以进行的最大重掷次数
    pub max_rolls: i32,
    /// 已重掷次数，用户已经使用的重掷次数
    pub number_of_rolls: i32,
    /// 重掷花费点数，每次重掷需要消耗的点数
    pub points_cost_to_roll: i32,
    /// 重掷所需点数，可选项，根据实际API响应，这个字段可能是可选的
    pub points_to_reroll: Option<i32>,
}

/// 召唤师信息结构体，用于存储召唤师的基本信息和游戏数据
#[derive(Debug, Clone, Serialize)]
pub struct SummonerInfo {
    /// 召唤师名称 - 用户在游戏中的显示名称
    pub name: String,
    /// 隐私设置 - 控制个人资料的可见性状态
    pub privacy: String,
    /// 图片URL - 召唤师头像的链接地址
    pub img_url: String,
    /// 召唤师等级 - 表示当前等级
    pub lv: String,
    /// 经验值 - 当前累积的经验值数量
    pub xp: i32,
    /// PUUID - Player Universally Unique Identifier，玩家唯一标识符
    pub puuid: String,
    /// 当前ID - 账户当前的唯一标识ID
    pub current_id: i64,
    /// 标签行 - 游戏标签信息，可能为空
    pub tag_line: Option<String>,
}
