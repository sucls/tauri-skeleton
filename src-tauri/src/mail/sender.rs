use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::MultiPart;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use crate::mail::EmailServer;

pub struct Sender{
    address: String,
    password: String,
    server: EmailServer
}

impl Sender{

    pub fn from(address: String, password: String, server: EmailServer) -> Self {
        Sender{
            address,
            password,
            server
        }
    }

    ///
    ///
    ///
    pub fn send_email(&self, to: String, subject: String, html: String)->Result<(),String>{
        let email = Message::builder()
            .from(self.address.to_string().parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(MultiPart::alternative_plain_html(
                "".to_string(), // 文本内容
                html,
            ))
            // .header(ContentType::TEXT_PLAIN).body(content)
            .unwrap();


        // 凭证
        let creds = Credentials::new(self.address.clone(), self.password.clone());
        // TLS
        // let tls = TlsParameters::new(self.server.host.as_str().into()).unwrap();
        // SMTP
        // let mailer = SmtpTransport::builder_dangerous(self.server.host.as_str())
        //     .port(self.server.port)
        //     .tls(Tls::Wrapper(tls))
        //     .credentials(creds)
        //     .build();
        
        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(self.server.host.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        // SEND
        match mailer.send(&email) {
            Ok(_) =>std::result::Result::Ok(()),
            Err(e) => Err(format!("Could not send email: {:?}", e)),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::mail::EmailServer;
    use crate::mail::sender::Sender;

    #[test]
    fn send(){
        let address = "409835152@qq.com";
        let password = "*******"; //需要生成应用专用密码

        let server = EmailServer{
            protocol: "SMTP".to_string(),
            host: "smtp.qq.com".to_string(),
            port: 587, // 25 tls 587
        };
        let sender = Sender::from(address.to_string(), password.to_string(), server);
        sender.send_email("su_chunlong@126.com".to_string(), "测试-0813".to_string(), "你好，每一天".to_string()).expect("邮件发送失败");
    }
}
