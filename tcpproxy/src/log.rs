macro_rules! log {
    ($str_tmpl: expr, $($args: expr ),* ) => {
        use colored::*;
        use chrono::Utc;
        let log_body = format!($str_tmpl, $($args)*);
        let date_time = format!("[{}]", Utc::now());
        println!("[{}] {} {}", date_time.cyan(), "[INFO]".green(), log_body)
    };
}

#[allow(unused_macros)]
macro_rules! err {
    ($str_tmpl: expr, $($args: expr ),* ) => {
        use colored::*;
        use chrono::Utc;
        let err_body = format!($str_tmpl, $($args)*);
        let date_time = format!("[{}]", Utc::now());
        println!("[{}] {} {}", date_time.cyan(), "[ERROR]".red(), log_body)
    };
}
