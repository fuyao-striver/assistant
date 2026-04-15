use tokio::process::Command;

#[tauri::command]
/// 启动英雄联盟游戏
///
/// 该函数通过指定路径启动英雄联盟游戏进程
///
/// # 参数
/// * path英雄联盟可执行文件的完整路径
///
/// # 返回值
/// * `Result<(), String>` - 启动成功返回Ok(())，失败返回包含错误信息的Err
pub fn launch_lol(path: &str) -> Result<(), String> {
    // 创建并启动英雄联盟进程
    Command::new(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("启动失败: {}", e))
}
