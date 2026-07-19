use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

// ── 订阅类型前缀常量 ──────────────────────────────────────────────

/// JSON API 事件基础前缀
const JSON_API_PREFIX: &str = "OnJsonApiEvent";
/// JSON API 事件带分隔符前缀（用于拼接具体路径）
const JSON_API_EVENT_SEP: &str = "OnJsonApiEvent_";
/// LCDS 事件基础前缀
const LCDS_PREFIX: &str = "OnLcdsEvent";
/// LCDS 事件带分隔符前缀（用于拼接具体路径）
const LCDS_EVENT_SEP: &str = "OnLcdsEvent_";

// ── LCU 事件主体 ─────────────────────────────────────────────────

/// WebSocket 连接返回的 LCU 事件。
///
/// LCU WebSocket 返回的 JSON 结构不能直接映射为目标类型，
/// 因此先反序列化为中间结构 [`DeEvent`]，再转换为 [`LcuEvent`]。
#[derive(Debug, Clone)]
pub struct LcuEvent {
    /// 事件订阅类型
    pub subscription_type: LcuSubscriptionType,
    /// 事件携带的 JSON 数据
    pub data: Value,
    /// 事件类型标识（如 `"Create"`, `"Update"`, `"Delete"`）
    pub event_type: String,
}

/// 反序列化用的中间结构，对应 LCU WebSocket 返回的原始 JSON 布局。
///
/// ```json
/// [8, "OnJsonApiEvent", { "data": { ... }, "eventType": "Update" }]
/// ```
#[derive(Deserialize, Debug)]
struct DeEvent {
    /// WebSocket 操作码（协议保留字段，当前未使用）
    #[allow(dead_code)]
    _opcode: i64,
    /// 事件订阅类型
    subscription_type: LcuSubscriptionType,
    /// 嵌套的事件数据体
    data: DeEventData,
}

/// LCU 事件的嵌套数据体。
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeEventData {
    /// 事件携带的 JSON 数据
    data: Value,
    /// 事件类型标识
    event_type: String,
}

impl<'de> Deserialize<'de> for LcuEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_event = DeEvent::deserialize(deserializer)?;
        Ok(Self {
            subscription_type: de_event.subscription_type,
            data: de_event.data.data,
            event_type: de_event.data.event_type,
        })
    }
}

// ── 订阅类型 ─────────────────────────────────────────────────────

/// LCU WebSocket 事件的订阅类型。
///
/// 用于指定要监听的事件范围，具体事件字符串参见
/// [LCU Schema 文档](https://www.mingweisamuel.com/lcu-schema/tool/#/)。
///
/// # 示例
///
/// ```ignore
/// // 监听所有 JSON API 事件
/// LcuSubscriptionType::AllJsonApiEvents
///
/// // 监听游戏流程状态变化
/// LcuSubscriptionType::JsonApiEvent("/lol-gameflow/v1/gameflow-phase".to_string())
/// ```
#[derive(Debug, Clone)]
pub enum LcuSubscriptionType {
    /// 监听所有 JSON API 事件
    /// （订阅频道：`OnJsonApiEvent`）
    AllJsonApiEvents,
    /// 监听所有 LCDS 事件
    /// （订阅频道：`OnLcdsEvent`）
    AllLcdsEvents,
    /// 监听指定路径的 JSON API 事件
    JsonApiEvent(String),
    /// 监听指定路径的 LCDS 事件
    LcdsEvent(String),
}

impl Display for LcuSubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllJsonApiEvents => f.write_str(JSON_API_PREFIX),
            Self::AllLcdsEvents => f.write_str(LCDS_PREFIX),
            // 将路径中的 `/` 替换为 `_`，去掉前导 `/`，与 LCU 事件频道命名一致
            // 例: "/lol-gameflow/v1/gameflow-phase" → "lol-gameflow_v1_gameflow-phase"
            Self::JsonApiEvent(path) => {
                write!(f, "{JSON_API_EVENT_SEP}{}", normalize_path(path))
            }
            Self::LcdsEvent(path) => {
                write!(f, "{LCDS_EVENT_SEP}{}", normalize_path(path))
            }
        }
    }
}

/// 将 API 路径规范化为 LCU 事件频道名称。
///
/// 去掉前导 `/`，并将剩余 `/` 替换为 `_`。
///
/// `/lol-gameflow/v1/gameflow-phase` → `lol-gameflow_v1_gameflow-phase`
fn normalize_path(path: &str) -> String {
    path.trim_start_matches('/').replace('/', "_")
}

/// 自定义反序列化：从字符串解析订阅类型。
///
/// 匹配规则（长前缀优先，避免将带路径的事件误判为基础事件）：
/// - `"OnJsonApiEvent_<path>"` → [`JsonApiEvent`]
/// - `"OnJsonApiEvent"`        → [`AllJsonApiEvents`]
/// - `"OnLcdsEvent_<path>"`    → [`LcdsEvent`]
/// - `"OnLcdsEvent"`           → [`AllLcdsEvents`]
impl<'de> Deserialize<'de> for LcuSubscriptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // 带路径前缀必须先于基础前缀匹配，否则 "OnJsonApiEvent_xxx"
        // 会被误匹配为 AllJsonApiEvents + 剩余字符串
        if let Some(path) = s.strip_prefix(JSON_API_EVENT_SEP) {
            Ok(Self::JsonApiEvent(path.to_string()))
        } else if s == JSON_API_PREFIX {
            Ok(Self::AllJsonApiEvents)
        } else if let Some(path) = s.strip_prefix(LCDS_EVENT_SEP) {
            Ok(Self::LcdsEvent(path.to_string()))
        } else if s == LCDS_PREFIX {
            Ok(Self::AllLcdsEvents)
        } else {
            Err(de::Error::custom(format!("未知的订阅类型: {s}")))
        }
    }
}
