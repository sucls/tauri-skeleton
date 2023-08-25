
use tauri::{Manager, Runtime};
use tauri::plugin::{Builder, TauriPlugin};

const PLUGIN_WINDOW: &str = "window";

const SUB_WIN: &str = "sub-win";

///
/// 打开窗口
///
#[tauri::command]
async fn open<R: Runtime>(app: tauri::AppHandle<R>, url: String){
    let sub_win = app.get_window(SUB_WIN);
    if let Some(win) = sub_win{
        println!("open sub window {}", SUB_WIN);
        win.show().unwrap();
    }else{
        println!("create sub window {}", SUB_WIN);
        tauri::WindowBuilder::new(&app, SUB_WIN, tauri::WindowUrl::External(url.parse().unwrap())).title(url).build().unwrap();
    }
}

///
/// 初始化插件
///
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_WINDOW)
        .invoke_handler(tauri::generate_handler![open])
        .build()
}