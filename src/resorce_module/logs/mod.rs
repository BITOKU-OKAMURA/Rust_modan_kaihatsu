use log::{debug, error, info, trace, warn, LevelFilter,SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/*
 * log4rsの初期化 汎用性を重視して作成する事。
 * 引数:rust_log_input(String型) ⇒RUST環境変数
 *     _log_path(String型)      ⇒出力するログのパス
 * 
 */
pub fn log4rs_init(rust_log_input: &str,_log_path: &str){
    let level = log::LevelFilter::Info;
    let file_path = _log_path.to_string();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%m)} [{l}]:{m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();
    //環境変数をここで定義 ※善悪は未定。postgresql的には悪
    std::env::set_var("RUST_LOG", rust_log_input.to_string());
    log4rs::init_config(config).unwrap();
}
