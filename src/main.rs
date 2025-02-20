use anyhow::Result;
use chrono::Local;
use work_diary::utils::exmail::ExMail;
use work_diary::utils::config::{get, init_config, DiaryConfiguration, FeaturesConfiguration};
use work_diary::utils::file::{get_file_content, get_samba_file_content};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_file_path: Option<&str> = Some(&args[1]);

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

    let exmail = ExMail::new();
    exmail.send_email(&subject, &file_content).await?;

    Ok(())
}
