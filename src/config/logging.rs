use std::str::FromStr;

use log::LevelFilter;
use log4rs::{
  append::{
    console::{ConsoleAppender, Target},
    rolling_file::policy::compound::{
      roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
      CompoundPolicy,
    },
  },
  config::{Appender, Root},
  encode::{json::JsonEncoder, pattern},
  Config,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LogSettings {
  path: String,
  log_level: Option<String>,
  max_size: Option<u64>,
  max_files: Option<u32>,
}

pub fn build_logger(settings: &LogSettings) {
  let max_level =
    LevelFilter::from_str(&settings.log_level.clone().unwrap_or("info".into()))
      .unwrap();

  let size_trigger = SizeTrigger::new(settings.max_size.unwrap_or(16 * 1024));
  let logfile_window = FixedWindowRoller::builder()
    .build(
      &format!("{}/splatternet.{{}}.log", &settings.path),
      settings.max_files.unwrap_or(3),
    )
    .unwrap();

  let logfile_policy =
    CompoundPolicy::new(Box::new(size_trigger), Box::new(logfile_window));

  let logfile_roller =
    log4rs::append::rolling_file::RollingFileAppender::builder()
      .encoder(Box::new(JsonEncoder::new()))
      .build(
        &format!("{}/splatternet.log", &settings.path),
        Box::new(logfile_policy),
      )
      .unwrap();

  let console = ConsoleAppender::builder()
    .target(Target::Stdout)
    .encoder(Box::new(pattern::PatternEncoder::new(
      "{h([{d(%Y-%m-%d %H:%M:%S)} {l} {M}])}:{n}  {m}{n}",
    )))
    .build();

  let config = Config::builder()
    .appender(Appender::builder().build("console", Box::new(console)))
    .appender(Appender::builder().build("logfile", Box::new(logfile_roller)))
    .build(
      Root::builder()
        .appender("logfile")
        .appender("console")
        .build(max_level),
    )
    .unwrap();

  match log4rs::init_config(config) {
    Ok(..) => {}
    Err(e) => panic!("Failed to initialize logger: {:?}", e),
  }
}
