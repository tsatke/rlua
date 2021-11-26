#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => {
        #[cfg(debug_assertions)]
        println!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($fmt, $($arg)*);
    };
}
