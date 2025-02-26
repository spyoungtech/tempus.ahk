use std::cmp::Ordering;
use std::ffi::c_short;
use std::os::raw::{c_char, c_longlong};
use std::str::FromStr;
use jiff::civil::{Date, Era, Weekday};
use jiff::{Error, Zoned};
use jiff::fmt::strtime::BrokenDownTime;
use crate::isoweekdate::TempusISOWeekDate;
use crate::time::TempusTime;
use crate::tz::TempusTimeZone;
use crate::utils::{ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, AHKWstr};
use crate::zoned::TempusZoned;

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
pub extern "C" fn date_from_isoweekdate(tiwd: &TempusISOWeekDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: Date::from_iso_week_date(tiwd.weekdate)})
}

#[no_mangle]
pub extern "C" fn date_weekday(td: &TempusDate) -> c_char {
    td.date.weekday().to_sunday_one_offset()
}

#[no_mangle]
pub extern "C" fn date_day_of_year(td: &TempusDate) -> c_short {
    td.date.day_of_year()
}

#[no_mangle]
pub extern "C" fn date_day_of_year_no_leap(td: &TempusDate) -> c_short {
    match td.date.day_of_year_no_leap() {
        None => {
            -1
        }
        Some(d) => {
            d
        }
    }
}

#[no_mangle]
pub extern "C" fn date_first_of_month(td: &TempusDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.first_of_month()})
}

#[no_mangle]
pub extern "C" fn date_last_of_month(td: &TempusDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.last_of_month()})
}

#[no_mangle]
pub extern "C" fn date_first_of_year(td: &TempusDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.first_of_year()})
}

#[no_mangle]
pub extern "C" fn date_last_of_year(td: &TempusDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.last_of_year()})
}

#[no_mangle]
pub extern "C" fn date_days_in_month(td: &TempusDate) -> c_char {
    td.date.days_in_month()
}

#[no_mangle]
pub extern "C" fn date_days_in_year(td: &TempusDate) -> c_short {
    td.date.days_in_year()
}

#[no_mangle]
pub extern "C" fn date_in_leap_year(td: &TempusDate) -> c_char {
    td.date.in_leap_year() as i8
}

#[no_mangle]
pub extern "C" fn date_tomorrow(td: &TempusDate, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.tomorrow() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let new_td = TempusDate{date};
            new_td.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_yesterday(td: &TempusDate, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.yesterday() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let new_td = TempusDate{date};
            new_td.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_nth_weekday_of_month(td: &TempusDate, nth: i8, weekday_i: i8, out_date: *mut *mut TempusDate) -> i64 {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match td.date.nth_weekday_of_month(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(date) => {
            let new_td = TempusDate{date};
            new_td.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_nth_weekday(td: &TempusDate, nth: i32, weekday_i: i8, out_date: *mut *mut TempusDate) -> i64 {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match td.date.nth_weekday(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(date) => {
            let new_td = TempusDate{date};
            new_td.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_to_isoweekdate(td: &TempusDate) -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: td.date.iso_week_date()})
}

#[no_mangle]
pub extern "C" fn date_in_tz(td: &TempusDate, time_zone_name: AHKWstr, out_zoned: *mut *mut TempusZoned) -> i64 {
    match ahk_str_to_string(time_zone_name) {
        Err(_) => {
            set_last_error_message("failed to process time zone name as rust string".to_string());
            -1
        }
        Ok(time_zone_string) => {
            match td.date.in_tz(&time_zone_string) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(zoned) => {
                    let tzoned = TempusZoned{zoned};
                    tzoned.stuff_into(out_zoned);
                    0
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn date_to_zoned(td: &TempusDate, tz: &TempusTimeZone, out_zoned: *mut *mut TempusZoned) -> i64 {
    match td.date.to_zoned(*tz.tz) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let tzoned = TempusZoned{zoned};
            tzoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_to_datetime(td: &TempusDate, tt: &TempusTime, out_zoned: *mut *mut TempusZoned) -> i64 {
    match td.date.to_datetime(tt.time) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let tzoned = TempusZoned{zoned};
            tzoned.stuff_into(out_zoned);
            0
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