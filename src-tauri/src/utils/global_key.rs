use rdev::{Event, EventType, Key, listen};
use tauri::{AppHandle, Manager};

/// 初始化全局键盘监听器
/// 
/// 该函数设置一个全局键盘事件监听器，用于捕获应用程序之外的键盘输入事件。
/// 内部维护一个shift键状态，用于跟踪shift键的按下/释放状态。
/// 
/// # 参数
/// * `app` - 应用程序句柄，用于在回调中与应用程序进行交互
/// 
/// # 注意
/// 如果监听失败，错误信息将通过log::error记录
pub fn init_global_keyboard(app: AppHandle) {
    let mut shift_state: bool = false;
    // 捕获全局键盘事件
    if let Err(error) = listen(move |event: Event| callback(event, &mut shift_state, &app)) {
        log::error!("Error: {:?}", error);
    }
}

/// 处理键盘事件的回调函数
/// 
/// # 参数
/// * `event` - 键盘事件对象，包含事件类型和按键信息
/// * `shift_state` - Shift键状态的可变引用，用于跟踪Shift键是否被按下
/// * `app` - 应用程序句柄，用于与应用程序进行交互
/// 
/// # 功能
/// - 监听Shift键（左/右）的按下和释放事件
/// - 监听Tab键事件以显示或隐藏窗口
/// - 维护Shift键的状态管理
fn callback(event: Event, shift_state: &mut bool, app: &AppHandle) {
    match event.event_type {
        // 处理按键按下事件
        EventType::KeyPress(key_event) => match key_event {
            Key::ShiftLeft | Key::ShiftRight => handle_shift_press(shift_state),
            Key::Tab => handle_show_hide_window(shift_state, app, "recentMatchWindow"),
            _ => (),
        },

        // 处理按键释放事件
        EventType::KeyRelease(key_event) => {
            if (key_event == Key::ShiftLeft || key_event == Key::ShiftRight) && *shift_state {
                // 重置 Shift 键状态
                *shift_state = false;
            }
        }
        _ => (),
    }
}

/// 处理 Shift 键按下的状态更新
/// 
/// 当 Shift 键被按下时，将 shift_state 设置为 true，
/// 如果当前状态已经是 true，则保持不变
/// 
/// # 参数
/// * `shift_state` - 可变引用，表示当前 Shift 键的状态，true 表示按下状态，false 表示释放状态
/// 
/// # 返回值
/// 无返回值
fn handle_shift_press(shift_state: &mut bool) {
    if !*shift_state {
        *shift_state = true;
    }
}

/// 处理 Shift + Tab 快捷键来显示或隐藏窗口
/// 
/// # 参数
/// * `shift_state` - 布尔引用，表示 Shift 键的状态，仅在按下时才执行操作
/// * `app` - 应用句柄引用，用于获取和控制 webview 窗口
/// * `win_name` - 字符串切片引用，指定要操作的窗口名称
/// 
/// # 功能
/// 当 shift_state 为 true 时，获取指定名称的窗口并切换其可见状态：
/// - 如果窗口当前可见，则将其隐藏
/// - 如果窗口当前不可见，则将其显示
/// - 如果检查窗口可见性时发生错误，则记录错误日志
fn handle_show_hide_window(shift_state: &mut bool, app: &AppHandle, win_name: &str) {
    if *shift_state {
        if let Some(win) = app.get_webview_window(win_name) {
            // 检查窗口当前是否可见
            match win.is_visible() {
                Ok(true) => {
                    // 如果可见，则隐藏
                    win.hide().expect("hide window failed");
                }
                Ok(false) => {
                    // 如果隐藏，则显示
                    win.show().expect("show window failed");
                }
                Err(e) => {
                    // 处理错误情况
                    log::error!("Error checking window visibility: {}", e);
                }
            }
        }
    }
}
