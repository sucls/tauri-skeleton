use rusqlite::Connection;
use crate::mail::{EmailMessage, Email, EmailServer};

pub fn find_email_by_source(conn: &Connection, source: &str) -> Option<Email>{
    //
    let mut stmt = conn.prepare("select id, address, password, kind, status from email where address = ?1").unwrap();
    let mut rows = stmt.query( [source.to_string()] ).unwrap();

    let row = rows.next().unwrap();

    if row.is_some() {
        return Some(Email {
            id: row.unwrap().get_unwrap(0),
            address: row.unwrap().get_unwrap(1),
            password: row.unwrap().get_unwrap(2),
            kind: None,
            status: None,
        });
    }
    None
}

pub fn find_email_server(conn: &Connection, address: &str, in_out: u8) -> Option<EmailServer>{
    if in_out == 0 { // 发送
        Some(EmailServer{
            protocol: "SMTP".to_string(),
            host: "smtp.qq.com".to_string(),
            port: 587,
        })
    }else{ // 接收
        Some(EmailServer{
            protocol: "IMAP".to_string(),
            host: "imap.qq.com".to_string(),
            port: 993,
        })
    }

}

pub fn save_email(conn: &Connection, email: EmailMessage){
    let mut stmt = conn.prepare("INSERT INTO email_message\
    (subject, source, target, cc, bcc, reply_to, category, folder, receive_date, send_date, text, html)\
    VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)").unwrap();
    let _res = stmt.insert(rusqlite::params![email.subject, email.source, email.target, email.cc, email.bcc, email.reply_to, email.category, email.folder, email.receive_date, email.send_date, email.subject, email.html]);

}
