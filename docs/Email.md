## 邮箱对接

　　126邮箱         

    POP3服务器: pop.126.com
    SMTP服务器: smtp.126.com
    IMAP服务器: imap.126.com

　　163邮箱 

|协议类型|协议功能|服务器地址|非SSL端口|SSL端口号|
|:-:|:-:|:-:|---|---|
|SMTP|发送邮件|smtp.126.com|25|465|
|POP|接收邮件|pop.126.com|110|995|
|IMAP|接收邮件|imap.126.com|143|993|

　　QQ邮箱 

|协议类型|协议功能|服务器地址|非SSL端口|SSL端口号|
|:-:|:-:|:-:|---|---|
|SMTP|发送邮件|smtp.qq.com|25|465、587|
|POP|接收邮件|pop.qq.com|110|995|
|IMAP|接收邮件|imap.qq.com|143|993|

　　QQ企业邮箱      SMTP服务器地址：smtp.exmail.qq.com（SSL启用 端口：587/465）
　　阿里云企业邮箱  SMTP服务器地址：smtp.mxhichina.com（端口：80）

### QQ 邮箱

+ SMTP
  
SMTP 的全称是“Simple Mail Transfer Protocol”，即简单邮件传输协议。它是一组用于从源地址到目的地址传输邮件的规范，通过它来控制邮件的中转方式。SMTP 协议属于 TCP/IP 协议簇，它帮助每台计算机在发送或中转信件时找到下一个目的地。SMTP 服务器就是遵循 SMTP 协议的发送邮件服务器。


+ IMAP

POP允许电子邮件客户端下载服务器上的邮件，但是您在电子邮件客户端的操作（如：移动邮件、标记已读等），这是不会反馈到服务器上的，比如：您通过电子邮件客户端收取了QQ邮箱中的3封邮件并移动到了其他文件夹，这些移动动作是不会反馈到服务器上的，也就是说，QQ邮箱服务器上的这些邮件是没有同时被移动的 。但是IMAP就不同了，电子邮件客户端的操作都会反馈到服务器上，您对邮件进行的操作（如：移动邮件、标记已读等），服务器上的邮件也会做相应的动作。也就是说，IMAP是“双向”的。

同时，IMAP可以只下载邮件的主题，只有当您真正需要的时候，才会下载邮件的所有内容。


### 126邮箱

### RFC headers:

https://www.iana.org/assignments/message-headers/message-headers.xhtml

https://github.com/SpamScope/mail-parser

  RFC   821   -   Simple   Mail   Transfer   Protocol    
  RFC   822   -   Standard   For   The   Format   Of   ARPA   Internet   Text   Messages    
  RFC   974   -   Mail   Routing   And   The   Domain   System    
  RFC   1123   -   Requirements   for   Internet   Hosts   --   Application   and   Support    
  RFC   1652   -   SMTP   Service   Extension   for   8bit-MIMEtransport    
  RFC   1842   -   ASCII   Printable   Characters-Based   Chinese   Character   Encoding    
  RFC   1869   -   SMTP   Service   Extensions    
  RFC   1892   -   The   Multipart/Report   Content   Type   for   the   Reporting   of   Mail   System    
  RFC   1893   -   Enhanced   Mail   System   Status   Codes    
  RFC   1894   -   An   Extensible   Message   Format   for   Delivery   Status   Notifications    
  RFC   1939   -   Post   Office   Protocol   -   Version   3    
  RFC   1957   -   Some   Observations   on   Implementations   of   the   Post   Office   Protocol   (POP3)    
  RFC   2197   -   SMTP   Service   Extension   for   Command   Pipelining     
   

Requests For Comments

http://www.rfc-editor.org

http://www.freesoft.org/CIE/RFC/index.htm