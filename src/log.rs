use tokio::{fs::File, io::AsyncWriteExt};

#[derive(PartialEq)]
pub enum LogStatus {
    Ok,
    Warning,
    Error
}

pub async fn log(message: &str, status: LogStatus, mut outfile: File) -> () {
    let status_str = match status {
        LogStatus::Ok => r"[ \e[0;32mOK\e[0m ]",
        LogStatus::Warning => r"[ \e[0;33WARNING\e[0m ]",
        LogStatus::Error => r"[ \e[0;31mERROR\e[0m ]",
    };

    let log_message = format!("{status_str} {message}");

    let _ = outfile.write_all(log_message.as_bytes()).await;
    
    if status == LogStatus::Ok || status == LogStatus::Warning {
        println!("{}", log_message);
    } else {
        eprintln!("{}", log_message);
    }
}
