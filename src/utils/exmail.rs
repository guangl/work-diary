use anyhow::Result;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use tracing::{error, info};

use crate::utils::config::{
    get, EmailConfiguration, FeaturesConfiguration, NotifyConfiguration, SmtpConfiguration,
    WechatWorkBotConfiguration,
};

#[derive(Debug, Default)]
pub struct ExMail {
    pub send_email: String,
    pub receiver_email: String,
    pub cc_email: String,
    pub password: String,

    pub smtp_server: String,
    pub smtp_port: u16,
}

impl ExMail {
    pub fn new() -> Self {
        let email_config = get::<EmailConfiguration>("email");
        let smtp_config = get::<SmtpConfiguration>("smtp");

        ExMail {
            send_email: email_config.send_email,
            receiver_email: email_config.receiver_email,
            cc_email: email_config.cc_email,
            password: email_config.password,

            smtp_server: smtp_config.server,
            smtp_port: smtp_config.port,
        }
    }

    pub async fn send_email(&self, subject: &str, body: &str) -> Result<()> {
        let receiver_emails = self.receiver_email.split(',').map(|item| item.trim());
        let cc_emails = self.cc_email.split(',').map(|item| item.trim());
        let send_email = &self.send_email;
        let password = &self.password;

        let smtp_server = &self.smtp_server;
        let smtp_port = self.smtp_port;

        let mut email_builder = Message::builder();
        for email in receiver_emails {
            email_builder = email_builder.to(email.parse()?);
        }
        for email in cc_emails {
            email_builder = email_builder.cc(email.parse()?);
        }

        let email = email_builder
            .from(send_email.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        let credentials = Credentials::new(send_email.to_string(), password.to_string());

        let mailer = SmtpTransport::relay(smtp_server)?
            .port(smtp_port)
            .credentials(credentials)
            .build();

        let features_config = get::<FeaturesConfiguration>("features");
        match mailer.send(&email) {
            Ok(_) => {
                if features_config.enable_notify {
                    let notify_config = get::<NotifyConfiguration>("features.notify");

                    if notify_config.enable_wechatworkbot {
                        post_to_wechatworkbot(
                            notify_config.wechatworkbot,
                            "【guangluo】工作日志邮件发送成功!",
                        )
                        .await?;
                    }
                }

                info!("Email sent successfully!");
            }
            Err(e) => {
                if features_config.enable_notify {
                    let notify_config = get::<NotifyConfiguration>("features.notify");

                    if notify_config.enable_wechatworkbot {
                        post_to_wechatworkbot(
                            notify_config.wechatworkbot,
                            "【guangluo】工作日志邮件发送失败!",
                        )
                        .await?;
                    }
                }

                error!("Could not send email: {:?}", e)
            }
        }

        Ok(())
    }
}

async fn post_to_wechatworkbot(
    wechatworkbot_config: WechatWorkBotConfiguration,
    message: &str,
) -> Result<bool> {
    let url = format!(
        "{}?key={}",
        wechatworkbot_config.url, wechatworkbot_config.key
    );

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&serde_json::json!({
            "msgtype": "text",
            "text": {
                "content": message
            }
        }))
        .send()
        .await?;

    if res.status() != 200 {
        error!("Failed to post to WechatWorkBot: {:?}", res.text().await?);
        return Ok(false);
    }

    Ok(true)
}
