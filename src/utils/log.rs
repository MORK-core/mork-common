#[macro_export]
macro_rules! mork_kernel_log {
    ($level:ident, $($arg:tt)*) => {
        log::$level!(target: "mork_logger", "[KERNEL] [{}:{}] {}", file!(), line!(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! mork_user_log {
    ($level:ident, $($arg:tt)*) => {
        log::$level!(target: "mork_logger", "[USER] [{}:{}] {}", file!(), line!(), format_args!($($arg)*));
    };
}