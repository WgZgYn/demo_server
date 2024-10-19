use lettre::message::{header, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

async fn send_email(
    receiver: String,
    subject: String,
    content: String,
) -> Result<Response, Box<dyn std::error::Error>> {
    // 配置邮件发送者、接收者及邮件内容
    let email = Message::builder()
        .from("sender@example.com".parse::<Mailbox>().unwrap()) // 发件人邮箱
        .to(receiver.parse::<Mailbox>().unwrap()) // 收件人邮箱
        .subject(subject)
        .header(header::ContentType::TEXT_PLAIN)
        .body(content)?;

    // 配置SMTP服务器和认证
    // TODO move it to config file
    let creds = Credentials::new(
        "a2049562063@outlook.com".to_string(),
        ";AccrmpmTh7Gf(,".to_string(),
    );

    // 使用lettre的异步SMTP传输
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .port(587)
        .credentials(creds)
        .build();

    // 发送邮件
    match mailer.send(email).await {
        Ok(response) => Ok(response),
        Err(err) => Err(Box::new(err)),
    }
}
