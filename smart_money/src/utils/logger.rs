use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 日志文件路径
    let log_file = "logs/app.log";

    // 滚动策略：单个文件最大 10MB，最多保留 5 个备份文件
    let size_trigger = SizeTrigger::new(10_000_000); // 10MB
    let roller = FixedWindowRoller::builder()
        .build("logs/app.log.{}", 5)?; // 保留 5 个备份文件
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(roller));

    // 日志文件 Appender
    let file_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {M} - {m}{n}"))) // 日志格式
        .build(log_file, Box::new(policy))?;

    // 日志配置
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))?;

    // 初始化日志
    log4rs::init_config(config)?;

    Ok(())
}