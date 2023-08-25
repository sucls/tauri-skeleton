use mail_parser::Message;
use mail_parser::HeaderValue::Address;
use serde::{Deserialize, Serialize};

pub mod sender;
pub mod receiver;
pub mod service;

/**
 * 邮箱
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub(crate) id: i32,
    pub address: String,
    pub password: String,
    pub(crate) kind: Option<String>,
    pub(crate) status: Option<String>,
}

/**
 * 邮件
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailMessage {
    id: i32, // ID
    pub subject: String, // 邮件标题
    pub(crate) source: String, //
    pub(crate) target: String, //
    cc: Option<String>, // 抄送
    bcc: Option<String>, // 密送
    reply_to: Option<String>,
    category: Option<String>, // 邮件分类
    folder: Option<String>, // 邮件文件夹
    receive_date: Option<i64>,
    send_date: Option<i64>,
    text: Option<String>, // 邮件片段
    pub html: String,
}

impl EmailMessage {

    fn from(message: Message) -> Self{
        println!( "{}", format!("消息时间：{}，主题：{}", message.date().unwrap(), message.subject().unwrap()) );
        let message_id = message.message_id().unwrap_or("");
        println!(">>>>> 解析消息：{:?}", message_id);

        let from = message.from().clone();
        let to = message.to().clone();

        let from_addr = if let Address(add) = from { add.address.unwrap().to_string() }else { "".to_string() } ;
        let to_addr = if let Address(add) = to { add.address.unwrap().to_string() }else { "".to_string() } ;

        EmailMessage {
            id: 0,
            subject: message.subject().unwrap().to_string(),
            source: from_addr,
            target: to_addr,
            cc: None,
            bcc: None,
            reply_to: None,
            category: None,
            folder: None,
            receive_date: Some(message.date().unwrap().to_timestamp()),
            send_date: None,
            text: Some(message.body_text(0).unwrap().to_string()),
            html: message.body_html(0).unwrap().to_string(),
        }
    }
}

/**
 * 附件
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment{
    id: i32,
    message_id: i32,
    name: String,
    kind: String, // text html file
    path: String,
    data: String
}

/**
 * 邮箱文件夹
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    id: i32,
    email_id: i32,
    name: String,
    icon: Option<String>,
    sort: Option<i32>
}

/**
 * 邮箱服务器
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailServer{
    protocol: String,
    host: String,
    port: u16,
}