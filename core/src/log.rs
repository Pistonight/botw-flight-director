use std::ffi::CString;
use std::os::raw;

#[link(name = "obs")]
extern "C" {
    fn blog(level: u32, data: *const raw::c_char);
}

/// Log a message to OBS
#[inline]
pub fn obs_log(level: u32, message: &str) {
    let c_message = CString::new(format!("[botwfd][core] {message}")).unwrap();
    unsafe {
        blog(level, c_message.as_ptr());
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::obs_log(400, &format!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::obs_log(300, &format!($($arg)*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::obs_log(200, &format!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::obs_log(100, &format!($($arg)*));
    }
}
