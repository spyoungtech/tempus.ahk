use std::cmp::Ordering;
use std::ffi::c_short;
use std::os::raw::{c_char, c_longlong};
use std::str::FromStr;
use jiff::civil::{Date, Era};
use jiff::Error;
use jiff::fmt::strtime::BrokenDownTime;
use crate::utils::{ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, AHKWstr};

#[repr(C)]
pub struct TempusDate {
    pub date: Date
}

impl TempusDate {
    pub fn stuff_into(self, pointer: *mut *mut TempusDate) {
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
pub extern "C" fn date_compare(td: &TempusDate, other_date: &TempusDate) -> c_char {
    match td.date.cmp(&other_date.date) {
        Ordering::Less => {-1}
        Ordering::Equal => {0}
        Ordering::Greater => {1}
    }
}

#[no_mangle]
pub extern "C" fn date_min() -> Box<TempusDate> {
    Box::new(TempusDate{date: Date::MIN})
}

#[no_mangle]
pub extern "C" fn date_max() -> Box<TempusDate> {
    Box::new(TempusDate{date: Date::MAX})
}

#[no_mangle]
pub extern "C" fn date_zero() -> Box<TempusDate> {
    Box::new(TempusDate{date: Date::ZERO})
}

#[no_mangle]
pub extern "C" fn date_new(year: i16, month: i8, day: i8, out_date: *mut *mut TempusDate) -> c_longlong {
    match Date::new(year, month, day) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let td = TempusDate{date};
            td.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_year(td: &TempusDate) -> c_short {
    td.date.year()
}
#[no_mangle]
pub extern "C" fn date_month(td: &TempusDate) -> c_char {
    td.date.month()
}
#[no_mangle]
pub extern "C" fn date_day(td: &TempusDate) -> c_char {
    td.date.day()
}

#[no_mangle]
pub extern "C" fn date_era_year(td: &TempusDate) -> c_short {
    td.date.era_year().0
}

#[no_mangle]
pub extern "C" fn date_era(td: &TempusDate) -> c_char {
    match td.date.era_year().1 {
        Era::BCE => {-1}
        Era::CE => {1}
    }
}

#[no_mangle]
pub extern "C" fn date_strftime_length(td: &TempusDate, ahk_format_str: AHKWstr) -> isize {
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(td.date);
            let mut buf = String::new();
            match bdt.format(format_str, &mut buf) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(_) => {
                    match isize::try_from(buf.len()) {
                        Err(e) => {
                            set_last_error_message(e.to_string());
                            -3
                        }
                        Ok(ret) => {
                            ret
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn date_strftime(td: &TempusDate, ahk_format_str: AHKWstr, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    if buff_len == 0 {
        return -1
    }
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(td.date);
            let mut buf = String::new();
            match bdt.format(format_str, &mut buf) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(_) => {
                    string_into_ahk_buff(buf, out_buff, buff_len);
                    0
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn date_strptime(ahk_format_str: AHKWstr, ahk_time_str: AHKWstr, out_date: *mut *mut TempusDate) -> i64 {
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            set_last_error_message("failed to read format string".to_string());
            -1
        }
        Ok(format_str) => {
            match ahk_str_to_string(ahk_time_str) {
                Err(_) => {
                    set_last_error_message("failed to read time string".to_string());
                    -1
                }
                Ok(time_str) => {
                    match Date::strptime(format_str, time_str) {
                        Err(e) => {
                            set_last_error_message(e.to_string());
                            -2
                        }
                        Ok(date) => {
                            let tts = TempusDate{date};
                            tts.stuff_into(out_date);
                            0
                        }
                    }
                }
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn free_date(tz: Box<TempusDate>) -> c_longlong {
    let raw = Box::into_raw(tz);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}