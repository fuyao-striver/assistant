use crate::{
    handler::get_client,
    lcu::summoner_types::{
        ChampionMastery, LcuSummonerInfo, LeagueRankedData, SummonerHonor, SummonerInfo,
    },
    utils::{champ_dict::ChampDict, tool::generate_rank_string},
};

pub mod listener;
pub mod summoner_types;
pub mod types;
pub mod ws;

/// 查询英雄熟练度数据
///
/// 该函数通过API端点获取用户的英雄熟练度信息，并将其转换为包含英雄图片、名称和熟练度详情的结构化数据
///
/// # 参数
/// * `endpoint` - API端点URL字符串引用，用于获取英雄熟练度原始数据
///
/// # 返回值
/// * `Ok(Vec<[String; 3]>)` - 成功时返回包含最多20个英雄数据的向量，每个元素为[String; 3]数组，
///   分别包含：英雄图片URL、英雄名称与标题、英雄等级和熟练度信息
/// * `Err(serde_json::Value)` - 失败时返回JSON值，通常为序列化错误或数据处理异常
///
/// # 错误处理
/// 当反序列化失败时记录错误日志并返回Null值；当无法获取客户端或初始化英雄字典失败时直接返回错误
#[tauri::command]
pub async fn query_champion_mastery(endpoint: &str) -> Result<Vec<[String; 3]>, serde_json::Value> {
    let client = get_client()?;
    ChampDict::init().await.expect("读取 champ_dict.json 失败");
    let result = client.get(endpoint).await?;
    let champion_mastery_list =
        serde_json::from_value::<Vec<ChampionMastery>>(result).map_err(|e| {
            log::error!("反序列化失败:{}", e);
            serde_json::Value::Null
        })?;

    // 构建英雄熟练度查询结果列表，限制最多20个英雄并过滤有效数据
    let query_mastery_champ_list: Vec<[String; 3]> = champion_mastery_list
        .iter()
        .take(20)
        .filter_map(|item| {
            // 安全地从全局字典取英雄信息
            let champ = ChampDict::get(&item.champion_id.to_string())?;
            Some([
                format!(
                    "https://game.gtimg.cn/images/lol/act/img/champion/{}.png",
                    champ.alias
                ),
                format!("{}•{}", champ.label, champ.title),
                format!(
                    "英雄等级 {} / 熟练度 {}",
                    item.champion_level, item.champion_points
                ),
            ])
        })
        .collect();
    Ok(query_mastery_champ_list)
}

/// 查询召唤师荣誉等级信息
///
/// 该函数通过LCU接口获取当前召唤师的荣誉等级和里程碑信息，
/// 并格式化为字符串返回
///
/// # Returns
/// * `Result<String, String>` - 成功时返回格式化的荣誉信息字符串，失败时返回错误信息
///   - 成功：格式为"荣誉等级X  里程Y"
///   - 失败：错误描述字符串
#[tauri::command]
pub async fn query_summoner_honor_level() -> Result<String, String> {
    const ERROR_VALUE: &str = "Error";

    // 获取HTTP客户端实例
    let client = get_client().map_err(|e| {
        log::error!("获取HTTP客户端失败: {}", e);
        ERROR_VALUE.to_string()
    })?;

    // 发起API请求获取荣誉信息
    let summoner_honor = client.get("/lol-honor-v2/v1/profile").await.map_err(|e| {
        log::error!("API请求失败: {}", e);
        ERROR_VALUE.to_string()
    })?;

    // 解析JSON响应数据为SummonerHonor结构体
    let lcu_summoner_info =
        serde_json::from_value::<SummonerHonor>(summoner_honor).map_err(|e| {
            log::error!("失败原因：{}", e);
            ERROR_VALUE.to_string()
        })?;

    return Ok(format!(
        "荣誉等级{}  里程{}",
        lcu_summoner_info.honor_level, lcu_summoner_info.checkpoint
    ));
}

/// 查询排位积分信息
///
/// 该函数通过指定的端点查询LCU（League Client Update）的排位数据，
/// 并解析出三种主要游戏模式的排位信息：单双排、灵活组排和云顶之弈。
///
/// # 参数
/// * `endpoint` - API端点URL字符串引用
///
/// # 返回值
/// * `Ok(Vec<String>)` - 包含三种游戏模式排位信息的字符串向量，顺序为：[单双排, 灵活组排, 云顶之弈]
/// * `Err(Vec<String>)` - 错误时返回包含三个"未定级"字符串的向量
///
/// # 错误处理
/// 当网络请求失败、JSON反序列化失败或数据为空时，返回默认的错误值
#[tauri::command]
pub async fn query_rank_point(endpoint: &str) -> Result<Vec<String>, Vec<String>> {
    static ERROR_VALUE: &[&str] = &["未定级", "未定级", "未定级"];

    // 创建HTTP客户端
    let client = match get_client() {
        Ok(c) => c,
        Err(e) => {
            log::error!("失败原因：{}", e);
            return Err(ERROR_VALUE.iter().map(|s| s.to_string()).collect());
        }
    };

    // 发起GET请求获取响应
    let response = match client.get(endpoint).await {
        Ok(r) => r,
        Err(e) => {
            log::error!("失败原因：{}", e);
            return Err(ERROR_VALUE.iter().map(|s| s.to_string()).collect());
        }
    };

    // 反序列化JSON响应为排位数据结构
    let lcu_rank_point_queues = match serde_json::from_value::<LeagueRankedData>(response) {
        Ok(data) => data.queues,
        Err(e) => {
            log::error!("反序列化失败: {}", e);
            return Err(ERROR_VALUE.iter().map(|s| s.to_string()).collect());
        }
    };

    // 检查是否获取到有效的排位队列数据
    if lcu_rank_point_queues.is_empty() {
        return Err(ERROR_VALUE.iter().map(|s| s.to_string()).collect());
    }

    // 查找三种不同游戏模式的排位信息
    let rank_solo = lcu_rank_point_queues
        .iter()
        .find(|item| item.queue_type == "RANKED_SOLO_5x5");
    let rank_flex_result = lcu_rank_point_queues
        .iter()
        .find(|item| item.queue_type == "RANKED_FLEX_SR");
    let rank_tft_result = lcu_rank_point_queues
        .iter()
        .find(|item| item.queue_type == "RANKED_TFT");

    // 生成各游戏模式的排位字符串表示
    let rank_solo_str = generate_rank_string(rank_solo);
    let rank_flex_str = generate_rank_string(rank_flex_result);
    let rank_tft_str = generate_rank_string(rank_tft_result);

    Ok(vec![rank_solo_str, rank_flex_str, rank_tft_str])
}

/// 查询召唤师信息
///
/// 该函数通过给定的端点获取LCU（League Client Update）中的召唤师信息，
/// 并将其转换为应用所需的SummonerInfo格式。
///
/// # 参数
/// * `endpoint` - LCU API的端点URL字符串引用
///
/// # 返回值
/// * `Ok(SummonerInfo)` - 成功时返回包含召唤师详细信息的结构体
/// * `Err(serde_json::Value)` - 失败时返回JSON值类型的错误信息
///
/// # 错误处理
/// 当反序列化失败时，会在日志中记录错误并返回空的JSON值
#[tauri::command]
pub async fn query_summoner_info(endpoint: &str) -> Result<SummonerInfo, serde_json::Value> {
    // 创建HTTP客户端实例
    let client = get_client()?;
    // 发起GET请求获取原始数据
    let result = client.get(endpoint).await?;
    // 将响应结果反序列化为LCU召唤师信息结构体
    let lcu_summoner_info = serde_json::from_value::<LcuSummonerInfo>(result).map_err(|e| {
        log::error!("反序列化失败:{}", e);
        serde_json::Value::Null
    })?;
    // 构建并返回标准化的召唤师信息结构体
    Ok(SummonerInfo {
        name: lcu_summoner_info.game_name,
        privacy: lcu_summoner_info.privacy,
        img_url: format!(
            "https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/{}.png",
            lcu_summoner_info.profile_icon_id
        ),
        lv: format!("Lv {}", lcu_summoner_info.summoner_level),
        xp: (lcu_summoner_info.xp_since_last_level as f64
            / lcu_summoner_info.xp_until_next_level as f64
            * 100.0) as i32,
        puuid: lcu_summoner_info.puuid,
        current_id: lcu_summoner_info.summoner_id,
        tag_line: lcu_summoner_info.tag_line,
    })
}
