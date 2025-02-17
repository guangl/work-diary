use anyhow::Result;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::utils::config::Config;

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
        let email_config = Config::get_email_config(None).unwrap();
        let smtp_config = Config::get_smtp_config(None).unwrap();

        ExMail {
            send_email: email_config.send_email,
            receiver_email: email_config.receiver_email,
            cc_email: email_config.cc_email,
            password: email_config.password,

            smtp_server: smtp_config.server,
            smtp_port: smtp_config.port,
        }
    }

    pub async fn send_email(
        &self,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        let mailer = SmtpTransport::relay(&smtp_server)?
            .port(smtp_port)
            .credentials(credentials)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprintln!("Could not send email: {:?}", e),
        }

        Ok(())
    }
}
