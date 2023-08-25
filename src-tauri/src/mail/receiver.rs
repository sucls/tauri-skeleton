extern crate imap;

use std::net::TcpStream;
use imap::Session;
use rustls_connector::TlsStream;
use crate::mail::EmailServer;

use super::EmailMessage;
// extern crate native_tls;

pub struct Receiver{
    address: String,
    password: String,
    server: EmailServer,
}

/// 收件箱
const EMAIL_INBOX: &str = "INBOX";

impl Receiver{

    pub fn from(address: String, password: String, server: EmailServer) -> Self{
        Receiver{
            address,
            password,
            server
        }
    }

    pub fn login(&self) -> Result<Session<TlsStream<TcpStream>>, String> {
        // build Client
        let client = imap::ClientBuilder::new(self.server.host.as_str(), self.server.port)
            .rustls().expect(format!("can't build email client by{}:{}!", self.server.host, self.server.port).as_str());
        // open session
        let session = client.login(self.address.as_str(), self.password.as_str())
        .map_err(|e| e.0.to_string())?;
        // .expect(format!("用户{}登录失败", self.address).as_str());
        
        Ok(session)
    }

    pub fn logout(&self, mut session: Session<TlsStream<TcpStream>>)->Result<(),String>{
        session.logout().expect("logout failure!");

        Ok(())
    }


    /// 获取邮箱内邮件
    // pub fn receive_mailbox_emails<'a>(self, mut session: &'a Session<TlsStream<TcpStream>>, mailbox_name: String, index: i32) -> Option<&'a Fetch<'a>>{
        // we want to fetch the first email in the INBOX mailbox
        // session.select(EMAIL_INBOX.to_string()).expect("INBOX not exist");

        // let messages = session.fetch(index.to_string(), "RFC822").unwrap();
        // // fetch message number 1 in this mailbox, along with its RFC822 field.
        // // RFC 822 dictates the format of the body of e-mails
        // if let Some(m) = &messages.iter().next() { 
        //     Some(m)
        // } else {
        //     None
        // }
    // }

    // pub fn parse_message() -> Email {
    //     Email { id: (), subject: (), source: (), target: (), cc: (), bcc: (), reply_to: (), category: (), folder: (), receive_date: (), send_date: (), text: (), content: () }
    // }

    pub fn receive_email(&self, latest_number: u32 ) -> Result<Vec<EmailMessage>, String>{
        
        let mut session = self.login()?;

        // we want to fetch the first email in the INBOX mailbox
        session.select(EMAIL_INBOX.to_string()).expect(format!("email mailbox name：{} dose not exist", EMAIL_INBOX).as_str());

        // count
        let total = session.select(EMAIL_INBOX.to_string()).unwrap().exists;

        let seq: String;
        if total > latest_number {
            seq = format!("{}:{}", total - latest_number,total);
        }else{
            seq = format!("{}:{}", 0,total); //
        }

        let messages = session.fetch(seq, "RFC822").expect("fetch message failure!");

        let mut emails: Vec<EmailMessage> = Vec::new();

        messages.iter().for_each(|message|{

            let body = message.body().expect("message body dose not exist!");
            let body = std::str::from_utf8(body).expect("message body is invalid utf-8 encoding!").to_string();

            let message = mail_parser::Message::parse(body.as_bytes()).expect("parse message body failure!");

            // print!("------------------- {:?}", message.clone());

            emails.push( EmailMessage::from(message));
        });

        // be nice to the server and log out
        self.login()?;

        Ok(emails)
    }
}

#[cfg(test)]
mod tests {
    use crate::mail::EmailServer;
    use crate::mail::receiver::Receiver;

    #[test]
    fn login(){
        let server = EmailServer{ protocol: "IMAP".to_string(), host: "imap.qq.com".to_string(), port: 993, };
        let address = "409835152@qq.com";
        let password = "*******"; //需要生成应用专用密码
        let receiver = Receiver::from(address.to_string(), password.to_string(), server);

        receiver.login().expect("登录失败");
    }

    #[test]
    fn receive(){
        let server = EmailServer{ protocol: "IMAP".to_string(), host: "imap.qq.com".to_string(), port: 993, };
        let address = "409835152@qq.com";
        let password = "*******"; //需要生成应用专用密码
        let receiver = Receiver::from(address.to_string(), password.to_string(), server);

        receiver.receive_email(10).expect("获取邮件失败");
    }
}