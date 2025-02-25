use std::ffi::c_longlong;
use std::str::FromStr;
use jiff::civil::Time;
use jiff::Error;
use crate::utils::{ahk_str_to_string, set_last_error_message, AHKWstr};

#[repr(C)]
struct TempusTime {
    time: Time
}

impl FromStr for TempusTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time: Time = s.parse()?;
        Ok(Self { time })
    }
}

impl TempusTime {
    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusTime) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}


#[no_mangle]
pub extern "C" fn time_parse(ahk_time_string: AHKWstr, out_date: *mut *mut TempusTime) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            set_last_error_message("could not read ahk string".to_string());
            -1
        }
        Ok(time_string) => {
            match TempusTime::from_str(&time_string) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(td) => {
                    td.stuff_into(out_date);
                    0
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_time(time: Box<TempusTime>) -> c_longlong {
    let raw = Box::into_raw(time);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}