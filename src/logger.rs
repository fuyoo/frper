use std::fs::{File, DirBuilder};
pub use log::{info, warn, error, trace, debug};
use env_logger::{Env, Builder};
use anyhow::{Result};
use std::io::Write;
use env_logger::fmt::{Color, Formatter};
use log::{Level, Record};
use sciter::{Element, Value};
use crate::app::app_data_dir;

/// init logger
pub fn init() -> Result<()> {
    Builder::from_env(Env::default()
        .default_filter_or("info"))
        .format(format_log)
        .try_init()?;
    Ok(())
}

/// format log file
fn format_log(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    if !cfg!(debug_assertions) {
        if record.level() == Level::Error {
            match save_log_to_file(&record) {
                _ => {}
            }
        };
        return Ok(());
    }
    let color = match record.level() {
        Level::Error => {
            Color::Red
        }
        Level::Warn => {
            Color::Rgb(255, 125, 0)
        }
        Level::Info => {
            Color::Green
        }
        Level::Debug => {
            Color::Rgb(88, 88, 88)
        }
        Level::Trace => {
            Color::Rgb(88, 88, 88)
        }
    };
    let mut style = buf.style();
    style.set_color(color).set_bold(true);
    match writeln!(buf, "{} {}:{} {:>5} {}",
                   chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                   record.file().unwrap_or("unnamed"),
                   record.line().unwrap_or(0),
                   style.value(record.level()),
                   style.value(record.args())) {
        _ => {
            let ele = Element::create("html").map_err(|_| std::io::ErrorKind::Other)?;
            ele.broadcast_event(
                "log", true,
                Some(Value::from(format!("{} [S] {}",
                                         chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
                                         record.args()))))
                .map_err(|_| std::io::ErrorKind::Other)?;
            Ok(())
        }
    }
}

/// save app error log to log file
fn save_log_to_file(record: &Record) -> Result<()> {
    let mut file = open_log_file()?;
    let str = format!("{}:{} {:>5} {}\n",
                      record.file().unwrap_or("unnamed"),
                      record.line().unwrap_or(0),
                      record.level(),
                      record.args());
    file.write_all(str.as_bytes())?;
    Ok(())
}

pub fn open_log_file() -> std::io::Result<File> {
    let app_data_dir = app_data_dir(None);
    if !app_data_dir.exists() {
        DirBuilder::new().recursive(true).create(&app_data_dir)?;
    }
    let log_file = app_data_dir.join("app.log");
    Ok(File::options().create(true).append(true).open(log_file)?)
}