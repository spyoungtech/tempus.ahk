use std::cmp::Ordering;
use std::os::raw::{c_char, c_int, c_longlong, c_short};
use std::str::FromStr;
use jiff::civil::{DateTime, Era};
use jiff::Error;
use jiff::fmt::strtime::BrokenDownTime;
use crate::utils::{ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, AHKWstr};

#[repr(C)]
pub struct TempusDateTime {
    pub datetime: DateTime,
}

impl FromStr for TempusDateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = DateTime::from_str(s)?;
        Ok(Self { datetime })
    }
}

impl TempusDateTime {
    pub fn stuff_into(self, pointer: *mut *mut TempusDateTime) {
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
pub extern "C" fn datetime_string_length(tdt: &TempusDateTime) -> usize {
    tdt.datetime.to_string().len()
}

#[no_mangle]
pub extern "C" fn datetime_to_string(tdt: &TempusDateTime, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = tdt.datetime.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
}

#[no_mangle]
pub extern "C" fn datetime_compare(tdt: &TempusDateTime, other_datetime: &TempusDateTime) -> c_char {
    match tdt.datetime.cmp(&other_datetime.datetime) {
        Ordering::Less => {-1}
        Ordering::Equal => {0}
        Ordering::Greater => {1}
    }
}

#[no_mangle]
pub extern "C" fn datetime_min() -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: DateTime::MIN})
}

#[no_mangle]
pub extern "C" fn datetime_max() -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: DateTime::MAX})
}

#[no_mangle]
pub extern "C" fn datetime_zero() -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: DateTime::ZERO})
}

#[no_mangle]
pub extern "C" fn datetime_new(year: i16,
                               month: i8,
                               day: i8,
                               hour: i8,
                               minute: i8,
                               second: i8,
                               subsec_nanosecond: i32, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match DateTime::new(year, month, day, hour, minute, second, subsec_nanosecond) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let tdt = TempusDateTime { datetime };
            tdt.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_hour(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.hour()
}
#[no_mangle]
pub extern "C" fn datetime_minute(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.minute()
}
#[no_mangle]
pub extern "C" fn datetime_second(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.second()
}

#[no_mangle]
pub extern "C" fn datetime_millisecond(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.millisecond()
}
#[no_mangle]
pub extern "C" fn datetime_microsecond(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.microsecond()
}
#[no_mangle]
pub extern "C" fn datetime_nanosecond(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.nanosecond()
}
#[no_mangle]
pub extern "C" fn datetime_subsec_nanosecond(tdt: &TempusDateTime) -> c_int {
    tdt.datetime.subsec_nanosecond()
}

#[no_mangle]
pub extern "C" fn datetime_year(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.year()
}
#[no_mangle]
pub extern "C" fn datetime_month(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.month()
}
#[no_mangle]
pub extern "C" fn datetime_day(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.day()
}


#[no_mangle]
pub extern "C" fn datetime_era_year(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.era_year().0
}

#[no_mangle]
pub extern "C" fn datetime_era(tdt: &TempusDateTime) -> c_char {
    match tdt.datetime.era_year().1 {
        Era::BCE => {-1}
        Era::CE => {1}
    }
}


#[no_mangle]
pub extern "C" fn datetime_strftime_length(tdt: &TempusDateTime, ahk_format_str: AHKWstr) -> isize {
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(tdt.datetime);
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
pub extern "C" fn datetime_strftime(tdt: &TempusDateTime, ahk_format_str: AHKWstr, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    if buff_len == 0 {
        return -1
    }
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(tdt.datetime);
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
pub extern "C" fn datetime_strptime(ahk_format_str: AHKWstr, ahk_time_str: AHKWstr, out_date: *mut *mut TempusDateTime) -> i64 {
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
                    match DateTime::strptime(format_str, time_str) {
                        Err(e) => {
                            set_last_error_message(e.to_string());
                            -2
                        }
                        Ok(datetime) => {
                            let tts = TempusDateTime{datetime};
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
pub extern "C" fn free_datetime(tdt: Box<TempusDateTime>) -> c_longlong {
    let raw = Box::into_raw(tdt);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}

