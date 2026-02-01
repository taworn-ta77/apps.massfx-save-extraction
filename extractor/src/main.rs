mod extract;
mod setup;

#[tokio::main]
async fn main() {
    setup::fern::setup_logger(&setup::fern::LoggerOptions {
        output_console: true,
        output_file: true,
        log_path: "~log".to_string(),
        max_file_line_len: 60,
    })
    .expect("Initialize logger failed!");

    extract::save1::fetch_save1().await;
}
