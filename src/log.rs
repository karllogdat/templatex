use colorize::*;

pub enum LogType {
    Debug,
    Info,
    Success,
    Warning,
    Error,
}

pub fn log<T: ToString>(log_type: LogType, message: T) {
    let message = message.to_string();
    match log_type {
        LogType::Info => println!("{}", format!("[INFO] {}", message).blue()),
        LogType::Debug => println!("{}", format!("[DEBUG] {}", message).grey()),
        LogType::Success => println!("{}", format!("[SUCCESS] {}", message).green()),
        LogType::Warning => println!("{}", format!("[WARNING] {}", message).yellow()),
        LogType::Error => println!("{}", format!("[ERROR] {}", message).red()),
    }
}
