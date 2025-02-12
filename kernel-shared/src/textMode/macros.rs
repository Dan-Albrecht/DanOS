#[macro_export]
macro_rules! vgaWrite {
    ($($args:tt)*) => {{
        if let Some(buildTimeFormatted) = core::format_args!($($args)*).as_str() {
            $crate::textMode::vga::writeString(buildTimeFormatted.as_bytes());
        }
        else {
            use core::fmt::Write;
            let mut writer = $crate::textMode::writer::Writer::new();
            let _ = write!(writer, $($args)*);
        }
    }};
}

#[macro_export]
macro_rules! vgaWriteLine {
    ($($args:tt)*) => {{
        $crate::vgaWrite!($($args)*);
        $crate::textMode::vga::writeString(b"\r\n");
    }};
}
