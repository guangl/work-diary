use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfiguration,
    pub email: EmailConfiguration,
    pub diary: DiaryConfiguration,
    pub features: FeaturesConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct FeaturesConfiguration {
    pub enable_notify: bool,
    pub enable_samba: bool,

    pub samba: SambaConfiguration,
    pub notify: NotifyConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct SambaConfiguration {
    pub server: String,
    pub username: String,
    pub password: String,
    pub workgroup: String,
    pub share: String,
}

#[derive(Debug, Deserialize)]
pub struct NotifyConfiguration {
    pub enable_wechatwork_bot: bool,
    pub wechatworkbot: WechatWorkBotConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct WechatWorkBotConfiguration {
    pub url: String,
    pub key: String,
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

// TODO use config crate to simplify the code
impl Config {
    pub fn get_config(config_file_path: &str) -> Result<Config> {
        let content = fs::read_to_string(config_file_path)?;
        Ok(toml::from_str::<Config>(&content)?)
    }

    pub fn get_email_config(config_file_path: Option<&str>) -> Result<EmailConfiguration> {
        let config_file_path = config_file_path.unwrap_or("config.toml");

        let config = Config::get_config(config_file_path)?;
        Ok(config.email)
    }

    pub fn get_smtp_config(config_file_path: Option<&str>) -> Result<SmtpConfiguration> {
        let config_file_path = config_file_path.unwrap_or("config.toml");

        let config = Config::get_config(config_file_path)?;
        Ok(config.smtp)
    }

    pub fn get_diary_config(config_file_path: Option<&str>) -> Result<DiaryConfiguration> {
        let config_file_path = config_file_path.unwrap_or("config.toml");

        let config = Config::get_config(config_file_path)?;
        Ok(config.diary)
    }

    pub fn get_features_config(config_file_path: Option<&str>) -> Result<FeaturesConfiguration> {
        let config_file_path = config_file_path.unwrap_or("config.toml");

        let config = Config::get_config(config_file_path)?;
        Ok(config.features)
    }

    pub fn get_samba_config(config_file_path: Option<&str>) -> Result<SambaConfiguration> {
        let config = Config::get_features_config(config_file_path)?;
        Ok(config.samba)
    }

    pub fn get_notify_config(config_file_path: Option<&str>) -> Result<NotifyConfiguration> {
        let config = Config::get_features_config(config_file_path)?;
        Ok(config.notify)
    }
}
