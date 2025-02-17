use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfiguration,
    pub email: EmailConfiguration,
    pub diary: DiaryConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct DiaryConfiguration {
    pub filepath: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfiguration {
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfiguration {
    pub send_email: String,
    pub receiver_email: String,
    pub cc_email: String,
    pub password: String,
}

impl Config {
    pub fn get_config(config_file_path: Option<&str>) -> Result<Config> {
        let config_file_path = config_file_path.unwrap_or("config.toml");

        let content = fs::read_to_string(config_file_path)?;
        Ok(toml::from_str::<Config>(&content)?)
    }

    pub fn get_email_config(config_file_path: Option<&str>) -> Result<EmailConfiguration> {
        let config = Config::get_config(config_file_path)?;
        Ok(config.email)
    }

    pub fn get_smtp_config(config_file_path: Option<&str>) -> Result<SmtpConfiguration> {
        let config = Config::get_config(config_file_path)?;
        Ok(config.smtp)
    }

    pub fn get_diary_config(config_file_path: Option<&str>) -> Result<DiaryConfiguration> {
        let config = Config::get_config(config_file_path)?;
        Ok(config.diary)
    }
}
