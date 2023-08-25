
use rusqlite::Connection;
use serde_json;
use tauri::{
    Runtime,
    plugin::{Builder, TauriPlugin},
};

use crate::Database;
use crate::mail::{Email, EmailMessage, EmailServer, service};
use crate::mail::receiver::Receiver;
use crate::mail::sender::Sender;
use crate::plugins::response::{PluginError, PluginResponse, PluginResult};

/**
 * 邮件插件
 */

/// 插件名称
const PLUGIN_EMAIL: &str = "email";

#[tauri::command]
async fn check_user<R: Runtime>(_app: tauri::Window<R>, address: String, password: String,  database: tauri::State<'_, Database>) -> Result<PluginResponse, String> {
    println!("检查邮箱 {}", address);

    let conn: Connection = Connection::open(database.db.as_str()).unwrap();

    // 获取服务器信息
    let server: Option<EmailServer> = service::find_email_server(&conn, address.as_str(), 1);
    if server.is_none() {
        return Err(format!("获取邮箱【{}】服务器配置信息失败", address.as_str()));
    }

    let receiver = Receiver::from(address.to_string(), password.to_string(), server.unwrap());

    receiver.login();

    Ok(PluginResponse{
        message: String::from("true")
    })
}

///
/// 定义插件方法（默认参数在js中使用驼峰，在rust自动转换成下划线）
/// （可以通过tauri::command(rename_all = "snake_case")更改）
///
#[tauri::command]
async fn send<R: Runtime>(_app: tauri::AppHandle<R>, email_json: String, database: tauri::State<'_, Database>) -> Result<PluginResponse, String>{
    // 反序列化json为消息
    let email_message: EmailMessage = serde_json::from_str(email_json.as_str()).expect("parse email json failure");

    let conn: Connection = Connection::open(database.db.as_str()).map_err(|e|e.to_string())?;

    // 获取邮箱信息
    let email: Option<Email> = service::find_email_by_source(&conn, email_message.source.as_ref());
    
    if email.is_none(){
        return Err(format!("获取邮箱【{}】信息失败", email_message.source));
    }
    // 获取服务器信息
    let server: Option<EmailServer> = service::find_email_server(&conn, email.as_ref().unwrap().address.as_str(), 0);
    if server.is_none() {
        return Err(format!("获取邮箱【{}】服务器配置信息失败", email_message.source));
    }

    // 发送邮件
    let sender = Sender::from(email.as_ref().unwrap().address.to_string(), email.as_ref().unwrap().password.to_string(), server.unwrap());
    sender.send_email(email_message.target.to_string(), email_message.subject.to_string(), email_message.html.to_string()).expect("send failure");

    Ok( PluginResponse{ message: "OK".to_string() } )
}

///
///
///
#[tauri::command]
async fn receive<R: Runtime>(_app: tauri::AppHandle<R>) {
    println!("receive email");
}

///
///
///
#[tauri::command]
async fn batch_receive<R: Runtime>(_app: tauri::AppHandle<R>, source: String, database: tauri::State<'_, Database>)  -> Result<PluginResponse, String>{
    println!("batch receive email");
    let conn: Connection = Connection::open(database.db.as_str()).map_err(|e|e.to_string())?;

    // 获取邮箱信息
    let email: Option<Email> = service::find_email_by_source(&conn, source.as_ref());
    if email.is_none(){
        return Err(format!("获取邮箱【{}】信息失败", source.as_str()));
    }

    // 获取服务器信息
    let server: Option<EmailServer> = service::find_email_server(&conn, source.as_str(), 1);
    if server.is_none() {
        return Err(format!("获取邮箱【{}】服务器配置信息失败", source.as_str()));
    }

    let receiver = Receiver::from(email.as_ref().unwrap().address.to_string(), email.as_ref().unwrap().password.to_string(), server.unwrap());
    let email_messages = receiver.receive_email(30)?;
    // 保存
    for email in email_messages {
        service::save_email(&conn, email );
    }

    Ok( PluginResponse{ message: "OK".to_string() } )
}

///
/// 初始化插件
///
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_EMAIL)
        .invoke_handler(tauri::generate_handler![ check_user, send, receive, batch_receive])
        .build()
}
