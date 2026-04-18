use crate::lcu::summoner_types::RankedEntry;

/// 将英文段位转换为中文段位
///
/// # 参数
/// * tier - 英文段位字符串，如 "CHALLENGER", "GRANDMASTER", "MASTER" 等
///
/// # 返回值
/// * 返回对应的中文段位字符串，如 "王者", "宗师", "大师" 等；如果输入无效则返回 "未定级"
pub fn english_to_chinese(tier: &str) -> &'static str {
    match tier {
        "CHALLENGER" => "王者",
        "GRANDMASTER" => "宗师",
        "MASTER" => "大师",
        "DIAMOND" => "钻石",
        "EMERALD" => "翡翠",
        "PLATINUM" => "铂金",
        "GOLD" => "黄金",
        "SILVER" => "白银",
        "BRONZE" => "青铜",
        "IRON" => "黑铁",
        _ => "未定级",
    }
}

/// 处理部门信息，将"NA"转换为空字符串，其他值保持不变
///
/// # 参数
/// * `divsion` - 部门名称字符串引用，如果为"NA"则表示未分配部门
///
/// # 返回值
/// 返回处理后的部门字符串，"NA"被转换为空字符串，其他值原样返回
pub fn deal_divsion(division: &str) -> String {
    if division == "NA" {
        "".to_string()
    } else {
        division.to_string()
    }
}

/// 生成排名字符串
///
/// 根据提供的排名信息生成对应的中文排名描述字符串。
/// 如果没有排名信息或段位为空，则返回"未定级"；否则返回格式化的排名信息。
///
/// # 参数
/// * `rank` - 可选的排名条目引用，包含段位、分区和联赛积分等信息
///
/// # 返回值
/// 返回表示排名信息的字符串，格式为"段位 分区 积分"或"未定级"
pub fn generate_rank_string(rank: Option<&RankedEntry>) -> String {
    match rank {
        None => "未定级".to_string(),
        Some(value) => {
            // 检查段位是否为空，为空则返回未定级
            if value.tier.is_empty() {
                "未定级".to_string()
            } else {
                // 格式化输出排名信息：中文段位 + 分区 + 联赛积分
                format!(
                    "{} {} {}",
                    english_to_chinese(&value.tier),
                    deal_divsion(&value.division),
                    value.league_points
                )
            }
        }
    }
}
