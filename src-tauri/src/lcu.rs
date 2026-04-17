use crate::{
    handler::get_client,
    lcu::summoner_types::{LcuSummonerInfo, SummonerInfo},
};

pub mod listener;
pub mod summoner_types;
pub mod types;
pub mod ws;

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
