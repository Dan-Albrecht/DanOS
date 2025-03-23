#[macro_export]
macro_rules! loggerWrite {
    ($($args:tt)*) => {
        if let Some(formattedString) = core::format_args!($($args)*).as_str() {
            $crate::logging::logger::SYSTEM_LOGGER.Write(formattedString.as_bytes());
        } else {
            use core::fmt::Write;
            let _ = write!($crate::logging::logWriter::LogWriter::new(), $($args)*);
        }
    };
}

#[macro_export]
macro_rules! loggerWriteLine {
    ($($args:tt)*) => {
        $crate::loggerWrite!($($args)*);
        $crate::logging::logger::SYSTEM_LOGGER.Write(b"\r\n");
    };
}
