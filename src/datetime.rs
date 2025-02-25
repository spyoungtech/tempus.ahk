use std::os::raw::c_longlong;
use std::str::FromStr;
use jiff::civil::DateTime;
use jiff::Error;
use crate::utils::{ahk_str_to_string, set_last_error_message, AHKWstr};

#[repr(C)]
struct TempusDateTime {
    datetime: DateTime,
}

impl FromStr for TempusDateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = DateTime::from_str(s)?;
        Ok(Self { datetime })
    }
}

impl TempusDateTime {
    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusDateTime) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}


#[no_mangle]
pub extern "C" fn datetime_parse(ahk_time_string: AHKWstr, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            set_last_error_message("could not read ahk string".to_string());
            -1
        }
        Ok(time_string) => {
            match TempusDateTime::from_str(&time_string) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(td) => {
                    td.stuff_into(out_datetime);
                    0
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_datetime(tdt: Box<TempusDateTime>) -> c_longlong {
    let raw = Box::into_raw(tdt);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}

