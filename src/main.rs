use anyhow::Result;
use chrono::Local;
use work_diary::exmail::ExMail;
use work_diary::utils::config::Config;
use work_diary::utils::file::get_file_content;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut config_file_path: Option<&str> = None;

    if args.len() == 2 {
        config_file_path = Some(&args[1]);
    }

    let diary_config = Config::get_diary_config(config_file_path)?;
    let today = Local::now().format("%Y-%m-%d");
    let subject = format!("{} 工作日志", today);
    let filepath = format!(
        "{}/{}.{}",
        diary_config.filepath, today, diary_config.suffix
    );

    let file_content = get_file_content(&filepath)?;

    let exmail = ExMail::new();
    exmail.send_email(&subject, &file_content).await?;

    Ok(())
}
