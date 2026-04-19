use core::panic;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Local;

enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub struct LogHandle {
    file: File,
    file_path: PathBuf,
}

fn get_log_level(level: LogLevel) -> String {
    match level {
        LogLevel::INFO => return "INFO".to_string(),
        LogLevel::WARN => return "WARN".to_string(),
        LogLevel::ERROR => return "ERROR".to_string(),
        LogLevel::DEBUG => return "DEBUG".to_string(),
    }
}

fn get_time_now() -> String {
    let now = Local::now();
    now.format("%Y:%m:%d %H:%M:%S").to_string()
}

impl LogHandle {
    pub fn new(path: &str) -> Self {
        let result = OpenOptions::new().create(true).append(true).open(path);

        match result {
            Ok(f) => Self {
                file: f,
                file_path: PathBuf::from(path),
            },
            Err(e) => {
                eprintln!("无法创建log对象:{}", e);
                panic!()
            }
        }
    }

    fn log(&mut self, msg: String, level: LogLevel) {
        let message = format!("{}  [{}]: {}\n", get_time_now(), get_log_level(level), msg);
        let result = self.file.write_all(message.as_bytes());
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("无法写入Log{}", e);
                panic!()
            }
        }
    }

    pub fn clear(&mut self) {
        let result = self.file.set_len(0);
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("无法清空Log:{}", e);
                panic!()
            }
        }
    }

    pub fn info(&mut self, msg: String) {
        self.log(msg, LogLevel::INFO);
    }

    pub fn warn(&mut self, msg: String) {
        self.log(msg, LogLevel::WARN);
    }

    pub fn error(&mut self, msg: String) {
        self.log(msg, LogLevel::ERROR);
    }

    pub fn debug(&mut self, msg: String) {
        self.log(msg, LogLevel::DEBUG);
    }
}
