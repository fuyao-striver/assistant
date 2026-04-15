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

#[tauri::command]
/// 关闭英雄联盟客户端进程
///
/// 该函数通过调用系统命令 taskkill 来强制终止 LeagueClient.exe 进程
///
/// # Returns
/// * `Ok(String)` - 操作结果描述，"successful" 表示成功关闭，"unsuccessful" 表示客户端未运行或关闭失败
/// * `Err(String)` - 执行命令失败时的错误信息
pub async fn close_lol_client() -> Result<String, String> {
    // CREATE_NO_WINDOW (0x08000000) 防止执行命令时弹出黑色 CMD 窗口
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("taskkill")
        .args(&["/F", "/IM", "LeagueClient.exe", "/T"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .await;

    match output {
        Ok(out) => {
            if out.status.success() {
                Ok("successful".into())
            } else {
                // 如果客户端没运行，taskkill 会报错，这里也视作"处理完成"
                Ok("unsuccessful".into())
            }
        }
        Err(e) => Err(format!("执行命令失败: {}", e)),
    }
}
