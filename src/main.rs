use anyhow::Result;
use chrono::Local;
use tracing::{error, info};
use work_diary::utils::config::{get, init_config, DiaryConfiguration, FeaturesConfiguration};
use work_diary::utils::exmail::ExMail;
use work_diary::utils::file::{get_file_content, get_samba_file_content};
use work_diary::utils::log::init_logger;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_file_path = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };

    init_logger();
    init_config(config_file_path);

    let diary_config = get::<DiaryConfiguration>("diary");
    let features_config = get::<FeaturesConfiguration>("features");

    let today = Local::now().format("%Y-%m-%d");
    let subject = format!("{} 工作日志", today);
    let filepath = format!(
        "{}/{}.{}",
        diary_config.filepath, today, diary_config.suffix
    );

    let file_content = if features_config.enable_samba {
        get_samba_file_content(&filepath)?
    } else {
        get_file_content(&filepath)?
    };

    if !file_content.is_empty() {
        let exmail = ExMail::new();
        exmail.send_email(&subject, &file_content).await?;

        info!("邮件发送成功");
    } else {
        error!("文件内容为空，不发送邮件");
    }

    Ok(())
}
