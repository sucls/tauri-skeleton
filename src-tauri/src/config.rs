use tauri_plugin_sql::{Migration, MigrationKind};

pub const DB_URL: &str = "tauri-skeleton.db";
const DB_VERSION: i64 = 0;

///
/// get db url
///
pub fn db() -> String {
    let db = format!("sqlite:{}", DB_URL);
    // println!("获取db地址：{}", db);
    db
}

///
/// db migrations
///
pub fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: DB_VERSION,
        description: "init sql",
        sql: include_str!("../migrations/init.sql"),
        kind: MigrationKind::Up,
    }]
}
