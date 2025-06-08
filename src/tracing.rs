#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        tracing::error!($($arg)*);
    };
}