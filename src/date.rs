use std::os::raw::c_longlong;
use std::str::FromStr;
use jiff::civil::Date;
use jiff::Error;
use crate::utils::{ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, AHKWstr};

#[repr(C)]
struct TempusDate {
    date: Date
}

impl TempusDate {
    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusDate) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}


impl FromStr for TempusDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date: Date = s.parse()?;
        Ok(TempusDate { date })
    }
}

#[no_mangle]
pub extern "C" fn date_parse(ahk_time_string: AHKWstr, out_date: *mut *mut TempusDate) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            set_last_error_message("could not read ahk string".to_string());
            -1
        }
        Ok(time_string) => {
            match TempusDate::from_str(&time_string) {
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
pub extern "C" fn date_string_length(td: &TempusDate) -> usize {
    td.date.to_string().len()
}

#[no_mangle]
pub extern "C" fn date_to_string(td: &TempusDate, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = td.date.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
}

#[no_mangle]
pub extern "C" fn free_timezone(tz: Box<TempusDate>) -> c_longlong {
    let raw = Box::into_raw(tz);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}