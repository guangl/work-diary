use crate::utils::config::{get, SambaConfiguration};
use anyhow::Result;
use pavao::{SmbClient, SmbCredentials, SmbOpenOptions, SmbOptions};
use std::fs;
use std::io::Read;

pub fn get_file_content(filepath: &str) -> Result<String> {
    let content = fs::read_to_string(filepath)?;
    Ok(content)
}

pub fn get_samba_file_content(diary_path: &str) -> Result<String> {
    let samba_config = get::<SambaConfiguration>("features.samba");

    let server = format!("smb://{}", samba_config.server);
    let username = samba_config.username;
    let password = samba_config.password;
    let workgroup = samba_config.workgroup;
    let share = samba_config.share;

    let client = SmbClient::new(
        SmbCredentials::default()
            .server(server)
            .username(username)
            .password(password)
            .workgroup(workgroup)
            .share(share),
        SmbOptions::default().one_share_per_server(true),
    )?;

    let mut file = client.open_with(diary_path, SmbOpenOptions::default().read(true))?;

    let mut buffer = [0u8; 1024];
    let mut bytes = Vec::new();

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..n]);
    }

    let content = String::from_utf8(bytes).expect("文件内容不是有效的 UTF-8 格式");

    drop(file);
    drop(client);

    Ok(content)
}
