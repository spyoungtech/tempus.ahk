use std::cmp::Ordering;
use std::ffi::c_longlong;
use std::ffi::{c_char, c_int, c_short};
use std::str::FromStr;
use jiff::{Error, Zoned, ZonedDifference, ZonedRound};
use jiff::civil::{Era, Weekday};
use jiff::fmt::strtime::BrokenDownTime;
use crate::date::TempusDate;
use crate::datetime::TempusDateTime;
use crate::duration::TempusSignedDuration;
use crate::isoweekdate::TempusISOWeekDate;
use crate::span::TempusSpan;
use crate::time::TempusTime;
use crate::timestamp::TempusTimestamp;
use crate::tz::TempusTimeZone;
use crate::utils::{ahk_str_to_string, AHKWstr, set_last_error_message, AHKStringBuffer, string_into_ahk_buff, unit_from_i8, round_mode_from_i8};

#[repr(C)]
pub struct TempusZoned {
    pub(crate) zoned: Zoned
}

impl TempusZoned {
    fn now() -> Self {
        TempusZoned{zoned: Zoned::now()}
    }

    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusZoned) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle)
        }
    }
}

impl FromStr for TempusZoned {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zoned: Zoned = s.parse()?;
        Ok(TempusZoned { zoned })
    }
}

#[no_mangle]
pub extern "C" fn zoned_now() -> Box<TempusZoned> {
    Box::new(TempusZoned::now())
}

#[no_mangle]
pub extern "C" fn zoned_new(tts: &TempusTimestamp, time_zone: &TempusTimeZone) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: Zoned::new(tts.ts, time_zone.tz.clone())})
}

#[no_mangle]
pub extern "C" fn zoned_with_time_zone(tzoned: &TempusZoned, time_zone: &TempusTimeZone) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tzoned.zoned.with_time_zone(time_zone.tz.clone())})
}

#[no_mangle]
pub extern "C" fn zoned_in_tz(tzoned: &TempusZoned, tzname: AHKWstr, out_zoned: *mut *mut TempusZoned) -> c_longlong  {
    match ahk_str_to_string(tzname) {
        Err(_) => {
            set_last_error_message("bad ahk string".to_string());
            -1
        }
        Ok(tz_string) => {
            match tzoned.zoned.in_tz(&tz_string) {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(zoned) => {
                    let new_zoned = TempusZoned{zoned: zoned};
                    new_zoned.stuff_into(out_zoned);
                    0
                }
            }
        }
    }
}



#[no_mangle]
pub extern "C" fn zoned_time_zone(tzoned: &TempusZoned) -> Box<TempusTimeZone> {
    Box::new(TempusTimeZone{tz: tzoned.zoned.time_zone().clone()})
}




#[no_mangle]
pub extern "C" fn zoned_parse(ahk_zone_str: AHKWstr, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match ahk_str_to_string(ahk_zone_str) {
        Err(_) => {
            -1
        }
        Ok(zone_string) => {
            match zone_string.as_str().parse::<TempusZoned>() {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(tzoned) => {
                    tzoned.stuff_into(out_zoned);
                    0
                }
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn zoned_hour(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.hour()
}
#[no_mangle]
pub extern "C" fn zoned_minute(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.minute()
}
#[no_mangle]
pub extern "C" fn zoned_second(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.second()
}

#[no_mangle]
pub extern "C" fn zoned_millisecond(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.millisecond()
}
#[no_mangle]
pub extern "C" fn zoned_microsecond(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.microsecond()
}
#[no_mangle]
pub extern "C" fn zoned_nanosecond(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.nanosecond()
}
#[no_mangle]
pub extern "C" fn zoned_subsec_nanosecond(tzoned: &TempusZoned) -> c_int {
    tzoned.zoned.subsec_nanosecond()
}

#[no_mangle]
pub extern "C" fn zoned_year(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.year()
}
#[no_mangle]
pub extern "C" fn zoned_month(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.month()
}
#[no_mangle]
pub extern "C" fn zoned_day(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.day()
}


#[no_mangle]
pub extern "C" fn zoned_era_year(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.era_year().0
}

#[no_mangle]
pub extern "C" fn zoned_era(tzoned: &TempusZoned) -> c_char {
    match tzoned.zoned.era_year().1 {
        Era::BCE => {-1}
        Era::CE => {1}
    }
}


#[no_mangle]
pub extern "C" fn zoned_strftime_length(tzoned: &TempusZoned, ahk_format_str: AHKWstr) -> isize {
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(&tzoned.zoned);
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
pub extern "C" fn zoned_strftime(tzoned: &TempusZoned, ahk_format_str: AHKWstr, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    if buff_len == 0 {
        return -1
    }
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(&tzoned.zoned);
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
pub extern "C" fn zoned_strptime(ahk_format_str: AHKWstr, ahk_time_str: AHKWstr, out_date: *mut *mut TempusZoned) -> i64 {
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
                    match Zoned::strptime(format_str, time_str) {
                        Err(e) => {
                            set_last_error_message(e.to_string());
                            -2
                        }
                        Ok(zoned) => {
                            let tts = TempusZoned{zoned};
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
pub extern "C" fn zoned_start_of_day(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.start_of_day() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_end_of_day(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.end_of_day() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_weekday(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.weekday().to_sunday_one_offset()
}



#[no_mangle]
pub extern "C" fn zoned_day_of_year(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.day_of_year()
}

#[no_mangle]
pub extern "C" fn zoned_day_of_year_no_leap(tzoned: &TempusZoned) -> c_short {
    match tzoned.zoned.day_of_year_no_leap() {
        None => {
            -1
        }
        Some(d) => {
            d
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_first_of_month(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.first_of_month() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_last_of_month(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.last_of_month() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_first_of_year(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.first_of_year() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_last_of_year(tzoned: &TempusZoned, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.last_of_year() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_days_in_month(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.days_in_month()
}

#[no_mangle]
pub extern "C" fn zoned_days_in_year(tzoned: &TempusZoned) -> c_short {
    tzoned.zoned.days_in_year()
}

#[no_mangle]
pub extern "C" fn zoned_in_leap_year(tzoned: &TempusZoned) -> c_char {
    tzoned.zoned.in_leap_year() as i8
}

#[no_mangle]
pub extern "C" fn zoned_tomorrow(tzoned: &TempusZoned, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.tomorrow() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_yesterday(tzoned: &TempusZoned, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.yesterday() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_nth_weekday_of_month(tzoned: &TempusZoned, nth: i8, weekday_i: i8, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match tzoned.zoned.nth_weekday_of_month(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_nth_weekday(tzoned: &TempusZoned, nth: i32, weekday_i: i8, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match tzoned.zoned.nth_weekday(nth, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_to_timestamp(tzoned: &TempusZoned) -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp{ts: tzoned.zoned.timestamp()})
}

#[no_mangle]
pub extern "C" fn zoned_to_datetime(tzoned: &TempusZoned) -> Box<TempusDateTime> {
    Box::new(TempusDateTime{datetime: tzoned.zoned.datetime()})
}

#[no_mangle]
pub extern "C" fn zoned_to_date(tzoned: &TempusZoned) -> Box<TempusDate> {
    Box::new(TempusDate{date: tzoned.zoned.date()})
}

#[no_mangle]
pub extern "C" fn zoned_to_time(tzoned: &TempusZoned) -> Box<TempusTime> {
    Box::new(TempusTime{time: tzoned.zoned.time()})
}

#[no_mangle]
pub extern "C" fn zoned_to_isoweekdate(tzoned: &TempusZoned) -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: tzoned.zoned.clone().iso_week_date()})
}

#[no_mangle]
pub extern "C" fn zoned_checked_add_span(tzoned: &TempusZoned, other: &TempusSpan, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.checked_add(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_checked_add_signed_duration(tzoned: &TempusZoned, other: &TempusSignedDuration, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.checked_add(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_checked_sub_signed_duration(tzoned: &TempusZoned, other: &TempusSignedDuration, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.checked_sub(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}



#[no_mangle]
pub extern "C" fn zoned_checked_sub_span(tzoned: &TempusZoned, other: &TempusSpan, out_datetime: *mut *mut TempusZoned) -> c_longlong {
    match tzoned.zoned.checked_sub(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_datetime);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn zoned_saturating_add_span(tzoned: &TempusZoned, rhs: &TempusSpan) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tzoned.zoned.saturating_add(rhs.span)})
}

#[no_mangle]
pub extern "C" fn zoned_saturating_add_signed_duration(tzoned: &TempusZoned, rhs: &TempusSignedDuration) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tzoned.zoned.saturating_add(rhs.duration)})
}

#[no_mangle]
pub extern "C" fn zoned_saturating_sub_span(tzoned: &TempusZoned, rhs: &TempusSpan) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tzoned.zoned.saturating_sub(rhs.span)})
}

#[no_mangle]
pub extern "C" fn zoned_saturating_sub_signed_duration(tzoned: &TempusZoned, rhs: &TempusSignedDuration) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tzoned.zoned.saturating_sub(rhs.duration)})
}

#[no_mangle]
pub extern "C" fn zoned_until_zoned(tzoned: &TempusZoned, other: &TempusZoned, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = ZonedDifference::from(&other.zoned).mode(round_mode).increment(increment);

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


    match tzoned.zoned.until(dd) {
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
pub extern "C" fn zoned_since_zoned(tzoned: &TempusZoned, other: &TempusZoned, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = ZonedDifference::from(&other.zoned).mode(round_mode).increment(increment);

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


    match tzoned.zoned.since(dd) {
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
pub extern "C" fn zoned_duration_until(tzoned: &TempusZoned, other: &TempusZoned) -> Box<TempusSignedDuration> {
    let duration = tzoned.zoned.duration_until(&other.zoned);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn zoned_duration_since(tzoned: &TempusZoned, other: &TempusZoned) -> Box<TempusSignedDuration> {
    let duration = tzoned.zoned.duration_since(&other.zoned);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn zoned_string_length(tzoned: &TempusZoned) -> usize {
    tzoned.zoned.to_string().len()
}

#[no_mangle]
pub extern "C" fn zoned_to_string(tzoned: &TempusZoned, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = tzoned.zoned.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
}

#[no_mangle]
pub extern "C" fn zoned_round(tzoned: &TempusZoned, unit: i8, increment: i64, round_mode: i8, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    let round_unit = match unit_from_i8(unit) {
        Ok(unit) => unit,
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
    };
    let mode = match round_mode_from_i8(round_mode) {
        Ok(m) => m,
        Err(e) => {
            set_last_error_message(e.to_string());
            return -2
        }
    };

    let ts_round = ZonedRound::new().smallest(round_unit).mode(mode).increment(increment);
    match tzoned.zoned.round(ts_round) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -3
        }
        Ok(zoned) => {
            let new_zoned = TempusZoned{zoned};
            new_zoned.stuff_into(out_zoned);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn zoned_compare(tzoned: &TempusZoned, other: &TempusZoned) -> c_char {
    match tzoned.zoned.cmp(&other.zoned) {
        Ordering::Less => {-1}
        Ordering::Equal => {0}
        Ordering::Greater => {1}
    }
}

#[no_mangle]
pub extern "C" fn free_zoned(tzoned: Box<TempusZoned>) -> c_longlong {
    let raw = Box::into_raw(tzoned);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let maybe_zoned : Result<Zoned, Error> = "something invalid".parse();
        maybe_zoned.unwrap_err();
    }
}