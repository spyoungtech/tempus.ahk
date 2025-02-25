use std::cmp::Ordering;
use std::ffi::{c_char, c_int, c_long, c_longlong, c_short};
use std::str::FromStr;
use jiff::civil::{Time, TimeDifference};
use jiff::{Error};
use crate::datetime::TempusDateTime;
use crate::duration::TempusSignedDuration;
use crate::span::TempusSpan;
use crate::utils::{ahk_str_to_string, round_mode_from_i8, set_last_error_message, string_into_ahk_buff, unit_from_i8, AHKStringBuffer, AHKWstr};

#[repr(C)]
pub struct TempusTime {
    pub time: Time
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
pub extern "C" fn time_wrapping_add_span(tt: &TempusTime, other: &TempusSpan) -> Box<TempusTime> {
    let time = tt.time.wrapping_add(other.span);
    Box::new(TempusTime{time})
}

#[no_mangle]
pub extern "C" fn time_wrapping_add_signed_duration(tt: &TempusTime, other: &TempusSignedDuration) -> Box<TempusTime> {
    let time = tt.time.wrapping_add(other.duration);
    Box::new(TempusTime{time})
}


#[no_mangle]
pub extern "C" fn time_wrapping_sub_span(tt: &TempusTime, other: &TempusSpan) -> Box<TempusTime> {
    let time = tt.time.wrapping_sub(other.span);
    Box::new(TempusTime{time})
}

#[no_mangle]
pub extern "C" fn time_wrapping_sub_signed_duration(tt: &TempusTime, other: &TempusSignedDuration) -> Box<TempusTime> {
    let time = tt.time.wrapping_sub(other.duration);
    Box::new(TempusTime{time})
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
pub extern "C" fn time_until_time(tt: &TempusTime, other: &TempusTime, unit_i: i8, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let unit = match unit_from_i8(unit_i) {
        Err(e) => {
            set_last_error_message(e);
            return -1
        }
        Ok(unit) => unit,
    };
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };

    let td = TimeDifference::from(other.time).largest(unit).mode(round_mode);
    match tt.time.until(td) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(span) => {
            let new_span = TempusSpan{span};
            new_span.stuff_into(out_span);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn time_until_datetime(tt: &TempusTime, other: &TempusDateTime, unit_i: i8, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let unit = match unit_from_i8(unit_i) {
        Err(e) => {
            set_last_error_message(e);
            return -1
        }
        Ok(unit) => unit,
    };
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };

    let td = TimeDifference::from(other.datetime).largest(unit).mode(round_mode);
    match tt.time.until(td) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(span) => {
            let new_span = TempusSpan{span};
            new_span.stuff_into(out_span);
            0
        }
    }
}




#[no_mangle]
pub extern "C" fn time_since_time(tt: &TempusTime, other: &TempusTime, unit_i: i8, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let unit = match unit_from_i8(unit_i) {
        Err(e) => {
            set_last_error_message(e);
            return -1
        }
        Ok(unit) => unit,
    };
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };

    let td = TimeDifference::from(other.time).largest(unit).mode(round_mode);
    match tt.time.since(td) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(span) => {
            let new_span = TempusSpan{span};
            new_span.stuff_into(out_span);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn time_since_datetime(tt: &TempusTime, other: &TempusDateTime, unit_i: i8, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let unit = match unit_from_i8(unit_i) {
        Err(e) => {
            set_last_error_message(e);
            return -1
        }
        Ok(unit) => unit,
    };
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };

    let td = TimeDifference::from(other.datetime).largest(unit).mode(round_mode);
    match tt.time.since(td) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(span) => {
            let new_span = TempusSpan{span};
            new_span.stuff_into(out_span);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn time_duration_until(tt: &TempusTime, other: &TempusTime) -> Box<TempusSignedDuration> {
    let duration = tt.time.duration_until(other.time);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn time_duration_since(tt: &TempusTime, other: &TempusTime) -> Box<TempusSignedDuration> {
    let duration = tt.time.duration_since(other.time);
    Box::new(TempusSignedDuration{duration})
}


#[no_mangle]
pub extern "C" fn time_midnight() -> Box<TempusTime> {
    Box::new(TempusTime{time: Time::midnight()})
}

#[no_mangle]
pub extern "C" fn time_hour(tt: &TempusTime) -> c_char {
    tt.time.hour()
}
#[no_mangle]
pub extern "C" fn time_minute(tt: &TempusTime) -> c_char {
    tt.time.minute()
}
#[no_mangle]
pub extern "C" fn time_second(tt: &TempusTime) -> c_char {
    tt.time.second()
}

#[no_mangle]
pub extern "C" fn time_millisecond(tt: &TempusTime) -> c_short {
    tt.time.millisecond()
}
#[no_mangle]
pub extern "C" fn time_microsecond(tt: &TempusTime) -> c_short {
    tt.time.microsecond()
}
#[no_mangle]
pub extern "C" fn time_nanosecond(tt: &TempusTime) -> c_short {
    tt.time.nanosecond()
}
#[no_mangle]
pub extern "C" fn time_subsec_nanosecond(tt: &TempusTime) -> c_int {
    tt.time.subsec_nanosecond()
}

#[no_mangle]
pub extern "C" fn free_time(time: Box<TempusTime>) -> c_longlong {
    let raw = Box::into_raw(time);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}

