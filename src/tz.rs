use std::ffi::c_longlong;
use jiff::tz::TimeZone;
use crate::utils::{ahk_str_to_string, set_last_error_message, AHKWstr};

struct TempusTimeZone {
    tz: TimeZone
}

impl TempusTimeZone {
    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusTimeZone) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}

#[no_mangle]
pub extern "C" fn timezone_system() -> Box<TempusTimeZone> {
    let tz = TimeZone::try_system().unwrap_or(TimeZone::UTC);
    Box::new(TempusTimeZone { tz })
}

#[no_mangle]
pub extern "C" fn timezone_get(ahk_time_string: AHKWstr, out_tz: *mut *mut TempusTimeZone) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            set_last_error_message("string argument parsing error".to_string());
            -1
        }
        Ok(time_string) => {
            match TimeZone::get(&time_string) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(tz) => {
                    let ttz = TempusTimeZone { tz };
                    ttz.stuff_into(out_tz);
                    0
                }
            }
        }
    }

}

#[no_mangle]
pub extern "C" fn timezone_utc() -> Box<TempusTimeZone> {
    Box::new(TempusTimeZone { tz: TimeZone::UTC })
}

#[no_mangle]
pub extern "C" fn timezone_unknown() -> Box<TempusTimeZone> {
    Box::new(TempusTimeZone{tz: TimeZone::unknown()})
}

#[no_mangle]
pub extern "C" fn free_timezone(ts: Box<TempusTimeZone>) -> c_longlong {
    let raw = Box::into_raw(ts);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}

