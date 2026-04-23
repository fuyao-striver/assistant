use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Champion {
    pub label: String,
    pub alias: String,
    pub title: String,
    pub roles: Vec<String>,
}

/// 全局单例存储
static CHAMP_DICT: OnceCell<HashMap<String, Champion>> = OnceCell::new();

/// 公共类 / 模块接口
pub struct ChampDict;

impl ChampDict {
    /// 初始化（在程序启动时调用一次）
    pub async fn init() -> Result<(), serde_json::Error> {
        let content = fs::read_to_string("champ_dict.json")
            .await
            .expect("读取 champ_dict.json 失败");
        let dict: HashMap<String, Champion> = serde_json::from_str(&content)?;
        CHAMP_DICT
            .set(dict)
            .map_err(|_| {
                // Already initialized - create a custom error or panic
                panic!("ChampDict already initialized!")
            })
            .unwrap();
        Ok(())
    }

    /// 根据键获取英雄（返回引用，零拷贝）
    pub fn get(key: &str) -> Option<&'static Champion> {
        CHAMP_DICT.get()?.get(key)
    }
}
