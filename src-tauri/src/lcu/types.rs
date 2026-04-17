use serde::{Deserialize, Deserializer, de};
use serde_json::Value;
use std::fmt::{self, Display};

/// The Websocket connection returns LcuEvents
#[derive(Debug, Clone)]
pub struct LcuEvent {
    pub subscription_type: LcuSubscriptionType,
    pub data: Value,
    pub event_type: String,
}

impl<'de> Deserialize<'de> for LcuEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DeEvent {
            _opcode: i64,
            subscription_type: LcuSubscriptionType,
            data: DeData,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DeData {
            data: Value,
            event_type: String,
        }

        let de = DeEvent::deserialize(deserializer)?;
        Ok(Self {
            subscription_type: de.subscription_type,
            data: de.data.data,
            event_type: de.data.event_type,
        })
    }
}

/// The Websocket events to subscribe to
#[derive(Debug, Clone)]
pub enum LcuSubscriptionType {
    AllJsonApiEvents,
    AllLcdsEvents,
    JsonApiEvent(String),
    LcdsEvent(String),
}

impl LcuSubscriptionType {
    const PREFIX_JSON: &'static str = "OnJsonApiEvent";
    const PREFIX_LCDS: &'static str = "OnLcdsEvent"; // 注意：修正为 OnLcdsEvent

    fn prefix(&self) -> &'static str {
        match self {
            Self::AllJsonApiEvents | Self::JsonApiEvent(_) => Self::PREFIX_JSON,
            Self::AllLcdsEvents | Self::LcdsEvent(_) => Self::PREFIX_LCDS,
        }
    }
}

impl Display for LcuSubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.prefix())?;
        match self {
            Self::JsonApiEvent(path) | Self::LcdsEvent(path) => {
                let normalized = path.trim_start_matches('/').replace('/', "_");
                write!(f, "_{}", normalized)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for LcuSubscriptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if let Some(rest) = s.strip_prefix(LcuSubscriptionType::PREFIX_JSON) {
            Ok(if rest.is_empty() {
                Self::AllJsonApiEvents
            } else {
                Self::JsonApiEvent(rest.trim_start_matches('_').to_string())
            })
        } else if let Some(rest) = s.strip_prefix(LcuSubscriptionType::PREFIX_LCDS) {
            Ok(if rest.is_empty() {
                Self::AllLcdsEvents
            } else {
                Self::LcdsEvent(rest.trim_start_matches('_').to_string())
            })
        } else {
            Err(de::Error::custom(format!(
                "Unknown SubscriptionType: {}",
                s
            )))
        }
    }
}
