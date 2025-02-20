use config::{Config, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;

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

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init_config(config_file_path: Option<&str>) -> &'static Config {
    let config_file_path = config_file_path.unwrap_or("config.toml");

    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(File::with_name(config_file_path))
            .build()
            .unwrap()
    })
}

pub fn get<'a, T: Deserialize<'a>>(key: &str) -> T {
    CONFIG.get().unwrap().get::<T>(key).unwrap()
}
