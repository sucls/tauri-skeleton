use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// 插件名称
const PLUGIN_CALCULATE: &str = "calculate";

///
/// 定义插件方法
/// 加法
///
#[tauri::command]
pub async fn add<R: Runtime>(_app: tauri::AppHandle<R>, v1: f32, v2: f32) -> String {
    let val: f32 = v1 + v2;
    val.to_string()
}

///
/// 减法
///
#[tauri::command]
pub async fn subtract<R: Runtime>(_app: tauri::AppHandle<R>, v1: f32, v2: f32) -> String {
    let val: f32 = v1 - v2;
    val.to_string()
}

///
/// 乘法
///
#[tauri::command]
pub async fn multiply<R: Runtime>(_app: tauri::AppHandle<R>, v1: f32, v2: f32) -> String {
    let val: f32 = v1 * v2;
    val.to_string()
}

///
/// 触发
///
#[tauri::command]
pub async fn divide<R: Runtime>(_app: tauri::AppHandle<R>, v1: f32, v2: f32) -> String {
    if v2 == 0.0 {
        return String::from("0");
    }
    let val: f32 = v1 / v2;
    val.to_string()
}

///
/// 初始化插件
///
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_CALCULATE)
        .invoke_handler(tauri::generate_handler![add, subtract, multiply, divide])
        .build()
}
