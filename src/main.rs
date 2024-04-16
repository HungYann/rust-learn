#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        println!("[{}] {}", $level, format_args!($($arg)*))
    };
}

fn main() {
    log!("debug", "This is a debug message");
    log!("error", "Something went wrong: {}", 404);
}