use std::cmp::Ordering;
use std::ffi::c_short;
use std::ffi::{c_char, c_longlong};
use std::str::FromStr;
use jiff::civil::{Date, DateDifference, DateSeries, Era, Weekday};
use jiff::{Error};
use jiff::fmt::strtime::BrokenDownTime;
use crate::datetime::TempusDateTime;
use crate::duration::TempusSignedDuration;
use crate::isoweekdate::TempusISOWeekDate;
use crate::span::TempusSpan;
use crate::time::TempusTime;
use crate::tz::TempusTimeZone;
use crate::utils::{ahk_str_to_string, round_mode_from_i8, set_last_error_message, string_into_ahk_buff, unit_from_i8, AHKStringBuffer, AHKWstr};
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

#[repr(C)]
pub struct TempusDateSeries{
    series: DateSeries
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
pub extern "C" fn date_strptime(ahk_format_str: AHKWstr, ahk_time_str: AHKWstr, out_date: *mut *mut TempusDate) -> c_longlong {
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
pub extern "C" fn date_nth_weekday_of_month(td: &TempusDate, nth: i8, weekday_i: i8, out_date: *mut *mut TempusDate) -> c_longlong {
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
pub extern "C" fn date_nth_weekday(td: &TempusDate, nth: i32, weekday_i: i8, out_date: *mut *mut TempusDate) -> c_longlong {
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
pub extern "C" fn date_in_tz(td: &TempusDate, time_zone_name: AHKWstr, out_zoned: *mut *mut TempusZoned) -> c_longlong {
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
pub extern "C" fn date_to_zoned(td: &TempusDate, tz: &TempusTimeZone, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match td.date.to_zoned(tz.tz.clone()) {
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
pub extern "C" fn date_to_datetime(td: &TempusDate, tt: &TempusTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: td.date.to_datetime(tt.time)})
}


#[no_mangle]
pub extern "C" fn date_checked_add_span(td: &TempusDate, other: &TempusSpan, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.checked_add(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let tdate = TempusDate{date};
            tdate.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_checked_sub_span(td: &TempusDate, other: &TempusSpan, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.checked_sub(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let tdate = TempusDate{date};
            tdate.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_checked_add_signed_duration(td: &TempusDate, other: &TempusSignedDuration, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.checked_add(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let tdate = TempusDate{date};
            tdate.stuff_into(out_date);
            0
        }
    }
}



#[no_mangle]
pub extern "C" fn date_checked_sub_signed_duration(td: &TempusDate, other: &TempusSignedDuration, out_date: *mut *mut TempusDate) -> c_longlong {
    match td.date.checked_sub(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(date) => {
            let tdate = TempusDate{date};
            tdate.stuff_into(out_date);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn date_saturating_add_span(td: &TempusDate, rhs: &TempusSpan) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.saturating_add(rhs.span)})
}

#[no_mangle]
pub extern "C" fn date_saturating_sub_span(td: &TempusDate, rhs: &TempusSpan) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.saturating_sub(rhs.span)})
}

#[no_mangle]
pub extern "C" fn date_saturating_add_signed_duration(td: &TempusDate, rhs: &TempusSignedDuration) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.saturating_add(rhs.duration)})
}

#[no_mangle]
pub extern "C" fn date_saturating_sub_signed_duration(td: &TempusDate, rhs: &TempusSignedDuration) -> Box<TempusDate> {
    Box::new(TempusDate{date: td.date.saturating_sub(rhs.duration)})
}


#[no_mangle]
pub extern "C" fn date_since_datetime(td: &TempusDate, other: &TempusDateTime, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateDifference::from(other.datetime).mode(round_mode).increment(increment);

    if smallest_i >= 0 {
        let unit = match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.smallest(unit);
    }

    if largest_i >= 0 {
        let unit = match unit_from_i8(largest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.largest(unit);
    }


    match td.date.since(dd) {
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
pub extern "C" fn date_since_date(td: &TempusDate, other: &TempusDate, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateDifference::from(other.date).mode(round_mode).increment(increment);

    if smallest_i >= 0 {
        let unit = match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.smallest(unit);
    }

    if largest_i >= 0 {
        let unit = match unit_from_i8(largest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.largest(unit);
    }


    match td.date.since(dd) {
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
pub extern "C" fn date_until_datetime(td: &TempusDate, other: &TempusDateTime, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateDifference::from(other.datetime).mode(round_mode).increment(increment);

    if smallest_i >= 0 {
        let unit = match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.smallest(unit);
    }

    if largest_i >= 0 {
        let unit = match unit_from_i8(largest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.largest(unit);
    }


    match td.date.until(dd) {
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
pub extern "C" fn date_until_date(td: &TempusDate, other: &TempusDate, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateDifference::from(other.date).mode(round_mode).increment(increment);

    if smallest_i >= 0 {
        let unit = match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.smallest(unit);
    }

    if largest_i >= 0 {
        let unit = match unit_from_i8(largest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -1
            }
            Ok(unit) => unit,
        };
        dd = dd.largest(unit);
    }


    match td.date.until(dd) {
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
pub extern "C" fn date_duration_until(td: &TempusDate, other: &TempusDate) -> Box<TempusSignedDuration> {
    let duration = td.date.duration_until(other.date);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn date_duration_since(td: &TempusDate, other: &TempusDate) -> Box<TempusSignedDuration> {
    let duration = td.date.duration_since(other.date);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn date_series(td: &TempusDate, tspan: &TempusSpan) -> Box<TempusDateSeries> {
    let series = td.date.series(tspan.span);
    Box::new(TempusDateSeries{series})
}

#[no_mangle]
pub extern "C" fn date_series_next(tds: &mut TempusDateSeries, out_date: *mut *mut TempusDate) -> c_char {
    match tds.series.next() {
        None => {
            -1
        }
        Some(date) => {
            let tdate = TempusDate{date};
            tdate.stuff_into(out_date);
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

#[no_mangle]
pub extern "C" fn free_date_series(tz: Box<TempusDateSeries>) -> c_longlong {
    let raw = Box::into_raw(tz);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}
