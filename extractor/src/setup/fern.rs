use std::fs;
use std::io::Write;
use std::path::PathBuf;

use log::Level;
use serde::{Deserialize, Serialize};

use colored::*;
use fern::Dispatch;
use file_rotate::{ContentLimit, FileRotate, compression::Compression, suffix::AppendCount};

///
/// Logger options
///
#[derive(Default, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoggerOptions {
    /// Enable log output to console.
    #[serde(default)]
    pub output_console: bool,

    /// Enable log output to file.
    #[serde(default)]
    pub output_file: bool,

    /// Folder path to store log files.
    /// Be sure the folder must exists.
    #[serde(default)]
    pub log_path: String,

    /// Max file:line length, for output file only
    #[serde(default)]
    pub max_file_line_len: usize,
}

///
/// Build dispatch for console.
///
fn create_log_to_console() -> Dispatch {
    Dispatch::new()
        .format(move |out, message, record| {
            let level_str = match record.level() {
                Level::Error => "ERR".red(),
                Level::Warn => "WAR".yellow(),
                Level::Info => "INF".white(),
                Level::Debug => "DEB".green(),
                Level::Trace => "TRA".cyan(),
            };

            out.finish(format_args!(
                "{} {} | {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%6f"),
                level_str,
                message,
            ));
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
}

///
/// Build dispatch for file.
///
fn create_log_to_file(options: &LoggerOptions) -> Dispatch {
    let log_path = PathBuf::from(&options.log_path);
    let _ = fs::create_dir_all(&log_path);

    let log_file_name = format!("extractor.log");
    let log_file = log_path.join(log_file_name);

    let rotator = FileRotate::new(
        log_file.clone(),
        AppendCount::new(2),
        ContentLimit::Bytes(1024 * 1024 * 10),
        Compression::None,
    );

    let mut max = options.max_file_line_len.clone();
    if max <= 0 {
        max = 20;
    }

    Dispatch::new()
        .format(move |out, message, record| {
            let level_str = match record.level() {
                Level::Error => "ERR",
                Level::Warn => "WAR",
                Level::Info => "INF",
                Level::Debug => "DEB",
                Level::Trace => "TRA",
            };

            let file_line = format!(
                "{}:{}",
                record.file().unwrap_or("?"),
                record.line().unwrap_or(0)
            );

            let truncated: String = file_line.chars().take(max).collect();
            let padding_needed = max.saturating_sub(truncated.len());
            let padding = (0..padding_needed).map(|_| ' ').collect::<String>();
            let adorn_file_line = format!("{}{}", truncated, padding);

            out.finish(format_args!(
                "{} {} | {} | {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%6f"),
                level_str,
                adorn_file_line,
                message,
            ));
        })
        .level(log::LevelFilter::Trace)
        //.chain(fern::log_file(log_file).expect("Open log file failed!"))
        .chain(Box::new(rotator) as Box<dyn Write + Send>)
}

///
/// Setup logger.
///
pub fn setup_logger(options: &LoggerOptions) -> Result<(), fern::InitError> {
    let mut dispatch = Dispatch::new();

    dispatch = dispatch
        .level(log::LevelFilter::Trace) // Global level (โปรแกรมของคุณ)
        .level_for("scraper", log::LevelFilter::Off)
        .level_for("html5ever", log::LevelFilter::Off)
        .level_for("selectors", log::LevelFilter::Off)
        .level_for("reqwest", log::LevelFilter::Off)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("rustls", log::LevelFilter::Off);

    if options.output_console {
        let console_dispatcher = create_log_to_console();
        dispatch = dispatch.chain(console_dispatcher);
    }

    if options.output_file {
        let file_dispatcher = create_log_to_file(options);
        dispatch = dispatch.chain(file_dispatcher);
    }

    dispatch.apply()?;
    Ok(())
}
