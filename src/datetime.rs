use std::cmp::Ordering;
use std::os::raw::{c_char, c_int, c_longlong, c_short};
use std::str::FromStr;
use jiff::civil::{DateTime, DateTimeDifference, DateTimeRound, DateTimeSeries, Era, Weekday};
use jiff::Error;
use jiff::fmt::strtime::BrokenDownTime;
use crate::date::TempusDate;
use crate::duration::TempusSignedDuration;
use crate::isoweekdate::TempusISOWeekDate;
use crate::span::TempusSpan;
use crate::time::TempusTime;
use crate::tz::TempusTimeZone;
use crate::utils::{ahk_str_to_string, round_mode_from_i8, set_last_error_message, string_into_ahk_buff, unit_from_i8, AHKStringBuffer, AHKWstr};
use crate::zoned::TempusZoned;

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


#[repr(C)]
pub struct TempusDateTimeSeries {
    series: DateTimeSeries
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
pub extern "C" fn datetime_from_parts(td: &TempusDate, tt: &TempusTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: DateTime::from_parts(td.date, tt.time)})
}

#[no_mangle]
pub extern "C" fn datetime_start_of_day(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.start_of_day()})
}

#[no_mangle]
pub extern "C" fn datetime_end_of_day(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.end_of_day()})
}

#[no_mangle]
pub extern "C" fn datetime_weekday(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.weekday().to_sunday_one_offset()
}



#[no_mangle]
pub extern "C" fn datetime_day_of_year(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.day_of_year()
}

#[no_mangle]
pub extern "C" fn datetime_day_of_year_no_leap(tdt: &TempusDateTime) -> c_short {
    match tdt.datetime.day_of_year_no_leap() {
        None => {
            -1
        }
        Some(d) => {
            d
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_first_of_month(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.first_of_month()})
}

#[no_mangle]
pub extern "C" fn datetime_last_of_month(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.last_of_month()})
}

#[no_mangle]
pub extern "C" fn datetime_first_of_year(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.first_of_year()})
}

#[no_mangle]
pub extern "C" fn datetime_last_of_year(tdt: &TempusDateTime) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.last_of_year()})
}

#[no_mangle]
pub extern "C" fn datetime_days_in_month(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.days_in_month()
}

#[no_mangle]
pub extern "C" fn datetime_days_in_year(tdt: &TempusDateTime) -> c_short {
    tdt.datetime.days_in_year()
}

#[no_mangle]
pub extern "C" fn datetime_in_leap_year(tdt: &TempusDateTime) -> c_char {
    tdt.datetime.in_leap_year() as i8
}

#[no_mangle]
pub extern "C" fn datetime_tomorrow(tdt: &TempusDateTime, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.tomorrow() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let new_td = TempusDateTime{datetime};
            new_td.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_yesterday(tdt: &TempusDateTime, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.yesterday() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let new_td = TempusDateTime{datetime};
            new_td.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_nth_weekday_of_month(tdt: &TempusDateTime, nth: i8, weekday_i: i8, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match tdt.datetime.nth_weekday_of_month(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(datetime) => {
            let new_td = TempusDateTime{datetime};
            new_td.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_nth_weekday(tdt: &TempusDateTime, nth: i32, weekday_i: i8, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match tdt.datetime.nth_weekday(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(datetime) => {
            let new_td = TempusDateTime{datetime};
            new_td.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_to_isoweekdate(tdt: &TempusDateTime) -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: tdt.datetime.iso_week_date()})
}

#[no_mangle]
pub extern "C" fn datetime_to_date(tdt: &TempusDateTime) -> Box<TempusDate> {
    Box::new(TempusDate{date: tdt.datetime.date()})
}


#[no_mangle]
pub extern "C" fn datetime_to_time(tdt: &TempusDateTime) -> Box<TempusTime> {
    Box::new(TempusTime{time: tdt.datetime.time()})
}

#[no_mangle]
pub extern "C" fn datetime_to_zoned(tdt: &TempusDateTime, tz: &TempusTimeZone, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tdt.datetime.to_zoned(tz.tz.clone()) {
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
pub extern "C" fn datetime_checked_add_span(tdt: &TempusDateTime, other: &TempusSpan, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.checked_add(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let tdate = TempusDateTime{datetime};
            tdate.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_checked_sub_span(tdt: &TempusDateTime, other: &TempusSpan, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.checked_sub(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let tdate = TempusDateTime{datetime};
            tdate.stuff_into(out_datetime);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn datetime_checked_add_signed_duration(tdt: &TempusDateTime, other: &TempusSignedDuration, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.checked_add(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let tdate = TempusDateTime{datetime};
            tdate.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn datetime_checked_sub_signed_duration(tdt: &TempusDateTime, other: &TempusSignedDuration, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    match tdt.datetime.checked_sub(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(datetime) => {
            let tdate = TempusDateTime{datetime};
            tdate.stuff_into(out_datetime);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn datetime_saturating_add_span(tdt: &TempusDateTime, rhs: &TempusSpan) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.saturating_add(rhs.span)})
}

#[no_mangle]
pub extern "C" fn datetime_saturating_sub_span(tdt: &TempusDateTime, rhs: &TempusSpan) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.saturating_sub(rhs.span)})
}

#[no_mangle]
pub extern "C" fn datetime_saturating_add_signed_duration(tdt: &TempusDateTime, rhs: &TempusSignedDuration) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.saturating_add(rhs.duration)})
}

#[no_mangle]
pub extern "C" fn datetime_saturating_sub_signed_duration(tdt: &TempusDateTime, rhs: &TempusSignedDuration) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tdt.datetime.saturating_sub(rhs.duration)})
}

#[no_mangle]
pub extern "C" fn datetime_until_datetime(tdt: &TempusDateTime, other: &TempusDateTime, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateTimeDifference::from(other.datetime).mode(round_mode).increment(increment);

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


    match tdt.datetime.until(dd) {
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
pub extern "C" fn datetime_until_date(tdt: &TempusDateTime, other: &TempusDate, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateTimeDifference::from(other.date).mode(round_mode).increment(increment);

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


    match tdt.datetime.until(dd) {
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
pub extern "C" fn datetime_since_datetime(tdt: &TempusDateTime, other: &TempusDateTime, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateTimeDifference::from(other.datetime).mode(round_mode).increment(increment);

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


    match tdt.datetime.since(dd) {
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
pub extern "C" fn datetime_since_date(tdt: &TempusDateTime, other: &TempusDate, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = DateTimeDifference::from(other.date).mode(round_mode).increment(increment);

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


    match tdt.datetime.since(dd) {
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
pub extern "C" fn datetime_duration_until(tdt: &TempusDateTime, other: &TempusDateTime) -> Box<TempusSignedDuration> {
    let duration = tdt.datetime.duration_until(other.datetime);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn datetime_duration_since(tdt: &TempusDateTime, other: &TempusDateTime) -> Box<TempusSignedDuration> {
    let duration = tdt.datetime.duration_since(other.datetime);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn datetime_series(tdt: &TempusDateTime, tspan: &TempusSpan) -> Box<TempusDateTimeSeries> {
    Box::new(TempusDateTimeSeries{series: tdt.datetime.series(tspan.span)})
}


#[no_mangle]
pub extern "C" fn datetime_round(tdt: &TempusDateTime, smallest_i: i8, increment: i64, round_mode_i: i8, out_datetime: *mut *mut TempusDateTime) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -1
        }
        Ok(round_mode) => {round_mode}
    };
    let mut rounder = DateTimeRound::new().increment(increment).mode(round_mode);
    if smallest_i >= 0 {
        match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e);
                return -2
            }
            Ok(unit) => {
                rounder = rounder.smallest(unit);
            }
        }
    }
    match tdt.datetime.round(rounder) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -3
        }
        Ok(datetime) => {
            let new_dt = TempusDateTime{datetime};
            new_dt.stuff_into(out_datetime);
            0
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

#[no_mangle]
pub extern "C" fn free_datetime_series(tdt: Box<TempusDateTimeSeries>) -> c_longlong {
    let raw = Box::into_raw(tdt);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}
