use std::time::{Duration, Instant};

use once_cell::sync::OnceCell;
use tauri::{AppHandle, Emitter};
use tokio::process::Command;

use crate::{rest::RESTClient, utils::process::get_auth_info};

// 定义全局的 REST 客户端
static REST_CLIENT: OnceCell<RESTClient> = OnceCell::new();
// 获取 REST_CLIENT 的函数
fn get_client() -> Result<&'static RESTClient, String> {
    REST_CLIENT
        .get()
        .ok_or_else(|| "REST_CLIENT未初始化".to_string())
}

/// 监听客户端启动状态，在后台异步检查客户端是否已启动并获取认证信息
///
/// 该函数会在后台启动一个异步任务，定期检查客户端是否存在，如果找到客户端
/// 则初始化REST客户端并发送启动完成信号，否则在超时后退出
///
/// # 参数
/// * `app` - Tauri应用句柄，用于发送事件通知
///
/// # 返回值
/// 无返回值，函数立即返回并启动后台任务
#[tauri::command]
pub async fn listen_for_client_start(app: AppHandle) {
    tokio::spawn({
        async move {
            let start_time = Instant::now();
            let timeout = Duration::from_secs(180); // 设置一个超时时间，例如 30 秒

            loop {
                // 获取客户端信息
                let is_exist = get_auth_info();

                if let Ok(value) = is_exist {
                    let _ = REST_CLIENT
                        .set(RESTClient::new(value.token, value.port).unwrap())
                        .map_err(|_| "REST_CLIENT已经初始化".to_string());
                    app.emit_to("background", "client_status", "ClientStarted")
                        .expect("发送信号失败！");
                    break; // 找到客户端信息后退出循环
                }

                // 超过指定的超时时间则退出
                if start_time.elapsed() > timeout {
                    log::error!("客户端启动超时，未能获取信息。");
                    break;
                }

                // 每隔一段时间重新检查
                tokio::time::sleep(Duration::from_secs(3)).await; // 每秒钟检查一次
            }
        }
    });
}

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
