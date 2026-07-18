use rdev::{Event, EventType, Key, listen};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

/// 全局快捷键状态，用于在线程间安全共享
struct KeyboardState {
    /// Shift 键是否处于按下状态
    shift_pressed: AtomicBool,
    /// 上次触发切换的时间，用于防抖
    last_toggle: std::sync::Mutex<Instant>,
}

/// 初始化全局键盘监听
///
/// 会在独立线程中启动 rdev 监听，检测 **Shift + Tab** 组合键
/// 来显示/隐藏名为 `recentMatch` 的窗口。
pub fn init_global_keyboard(app: AppHandle) {
    // rdev::listen 是阻塞调用，必须在独立线程中运行，否则会卡住主线程
    tauri::async_runtime::spawn_blocking(move || {
        // 使用 Arc + AtomicBool 在闭包与回调间安全共享状态
        let state = Arc::new(KeyboardState {
            shift_pressed: AtomicBool::new(false),
            last_toggle: std::sync::Mutex::new(Instant::now() - Duration::from_secs(1)),
        });

        if let Err(e) = listen(move |event| callback(event, &state, &app)) {
            log::error!("全局键盘监听启动失败: {:?}", e);
        }
    });
}

/// rdev 事件回调，处理每一次键盘按下/释放事件
fn callback(event: Event, state: &KeyboardState, app: &AppHandle) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            // 任意 Shift 键按下 → 标记状态
            Key::ShiftLeft | Key::ShiftRight => {
                state.shift_pressed.store(true, Ordering::Relaxed);
            }
            // Tab 按下时，若 Shift 已处于按下状态 → 切换窗口可见性
            Key::Tab => {
                if state.shift_pressed.load(Ordering::Relaxed) {
                    toggle_window(app, "recentMatch", state);
                }
            }
            _ => {}
        },
        EventType::KeyRelease(key) => match key {
            // 释放 Shift → 清除标记
            Key::ShiftLeft | Key::ShiftRight => {
                state.shift_pressed.store(false, Ordering::Relaxed);
            }
            _ => {}
        },
        _ => {}
    }
}

/// 切换指定窗口的显示/隐藏状态
///
/// 内置 300ms 防抖，避免连按导致窗口闪烁。
fn toggle_window(app: &AppHandle, window_label: &str, state: &KeyboardState) {
    // --- 防抖：300ms 内不重复触发 ---
    {
        let mut last = state.last_toggle.lock().unwrap();
        if last.elapsed() < Duration::from_millis(300) {
            return;
        }
        *last = Instant::now();
    }

    // 查找目标窗口（可能已被用户关闭或尚未创建）
    let Some(win) = app.get_webview_window(window_label) else {
        log::warn!("未找到窗口: {}", window_label);
        return;
    };

    // 根据当前可见性决定是隐藏还是显示
    match win.is_visible() {
        Ok(true) => {
            if let Err(e) = win.hide() {
                log::error!("隐藏窗口失败: {}", e);
            }
        }
        Ok(false) => {
            if let Err(e) = win.show() {
                log::error!("显示窗口失败: {}", e);
            }
            // 显示后将窗口提到最前面
            let _ = win.set_focus();
        }
        Err(e) => {
            log::error!("获取窗口可见性失败: {}", e);
        }
    }
}
