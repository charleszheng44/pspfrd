use colored::{self, *};
use chrono::Utc;
use std::process;

#[doc(hidden)]
pub fn _fatal(fatal_msg: String) {
    let date_time = format!("[{}]", Utc::now());
    println!("{} {} {}", date_time.cyan(), "[FATAL]".red(), fatal_msg);
    process::exit(1);
}

#[doc(hidden)]
pub fn _log(log_msg: String) {
    let date_time = format!("[{}]", Utc::now());
    println!("{} {} {}", date_time.cyan(), "[INFO]".green(), log_msg);
}

#[macro_export]
macro_rules! fatal {
    ($fatal_msg: expr) => {
        tcpproxy::macros::_fatal($fatal_msg.to_string());
    };
    ($str_tmpl: expr, $($args: expr ),* ) => {
        let fatal_msg = format!($str_tmpl, $($args)*);
        tcpproxy::macros::_fatal(fatal_msg);
    };
}

#[macro_export]
macro_rules! log {
    ($log_msg: expr) => {
        tcpproxy::macros::_log($log_msg.to_string());
    };
    ($str_tmpl: expr, $($args: expr ),* ) => {
        let log_msg = format!($str_tmpl, $($args)*);
        tcpproxy::macros::_log(log_msg);
    };
}
