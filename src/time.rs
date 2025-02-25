use std::cmp::Ordering;
use std::ffi::{c_char, c_longlong};
use std::str::FromStr;
use jiff::civil::Time;
use jiff::Error;
use crate::duration::TempusSignedDuration;
use crate::span::TempusSpan;
use crate::utils::{ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, AHKWstr};

#[repr(C)]
pub struct TempusTime {
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
pub extern "C" fn time_string_length(tt: &TempusTime) -> usize {
    tt.time.to_string().len()
}

#[no_mangle]
pub extern "C" fn time_to_string(tt: &TempusTime, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = tt.time.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
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
pub extern "C" fn time_compare(tt: &TempusTime, other_time: &TempusTime) -> c_char {
    match tt.time.cmp(&other_time.time) {
        Ordering::Less => {-1}
        Ordering::Equal => {0}
        Ordering::Greater => {1}
    }
}

#[no_mangle]
pub extern "C" fn time_max() -> Box<TempusTime> {
    Box::new(TempusTime{time: Time::MAX})
}

#[no_mangle]
pub extern "C" fn time_min() -> Box<TempusTime> {
    Box::new(TempusTime{time: Time::MIN})
}

#[no_mangle]
pub extern "C" fn time_new(hour: i8, minute: i8, second: i8, subsec_nano: i32, out_time: *mut *mut TempusTime) -> c_longlong {
    match Time::new(hour, minute, second, subsec_nano) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(time) => {
            let ttime = TempusTime{time};
            ttime.stuff_into(out_time);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn time_checked_add_span(tt: &TempusTime, other: &TempusSpan, out_time: *mut *mut TempusTime) -> c_longlong {
    match tt.time.checked_add(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(time) => {
            let ttime = TempusTime{time};
            ttime.stuff_into(out_time);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn time_checked_add_signed_duration(tt: &TempusTime, other: &TempusSignedDuration, out_time: *mut *mut TempusTime) -> c_longlong {
    match tt.time.checked_add(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(time) => {
            let ttime = TempusTime{time};
            ttime.stuff_into(out_time);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn time_checked_sub_span(tt: &TempusTime, other: &TempusSpan, out_time: *mut *mut TempusTime) -> c_longlong {
    match tt.time.checked_sub(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(time) => {
            let ttime = TempusTime{time};
            ttime.stuff_into(out_time);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn time_checked_sub_signed_duration(tt: &TempusTime, other: &TempusSignedDuration, out_time: *mut *mut TempusTime) -> c_longlong {
    match tt.time.checked_sub(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(time) => {
            let ttime = TempusTime{time};
            ttime.stuff_into(out_time);
            0
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

