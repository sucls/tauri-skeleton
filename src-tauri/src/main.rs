#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::Rng;
use rusqlite::Connection;
use tauri::{Manager, WindowBuilder, WindowUrl};
use tauri_plugin_sql::{Migration, MigrationKind};
use crate::config::DB_URL;

mod config;
mod plugins;
mod mail;
mod sql;

struct Database {
    db: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(plugins::calculate_plugin::init())
        .plugin(plugins::email_plugin::init())
        .plugin(plugins::window_plugin::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&config::db(), config::migrations())
                .build(),
        ).setup(|app|{
            let app_dir = tauri::api::path::app_config_dir(&app.config()).unwrap();
            println!(">>> app dir: {}", app_dir.as_path().to_str().unwrap());
            let db_path: String = format!("{}/{}", app_dir.as_path().to_str().unwrap(), DB_URL);
            println!(">>> sqlite db path: {}", db_path);
            app.manage(Database { db: db_path });
            Ok(())

            // tauri::async_runtime::block_on(async move {
            //     let conn = Connection::open(db_path)?;
            //     app.manage(conn);
            //     Ok(())
            // })
        })
        .invoke_handler(tauri::generate_handler![dispatch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn dispatch(command: &str, args: &str) -> String {
    let mut rng = rand::thread_rng();
    format!(
        " command: {}, args:{}, result: {}",
        command,
        args,
        rng.gen::<u32>()
    )
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        println!("生成随机数：{}", rng.gen::<u32>());
    }
}
