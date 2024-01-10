enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $text:expr) => {
	match $level {
	    LogLevel::INFO => println!("{}", $text),
	    LogLevel::WARN => println!("warning: {}", $text),
	    LogLevel::ERROR => println!("ERROR {}", $text),
	}}
}

fn main() {
    log!(LogLevel::INFO, "text");
    log!(LogLevel::ERROR, "msg");
}
