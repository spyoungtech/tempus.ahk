#![allow(dead_code)]

use std::cmp::Ordering;
use jiff::{Error, Timestamp, TimestampDifference, TimestampRound, TimestampSeries};

use std::ffi::{c_char, c_int, c_longlong};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use jiff::fmt::strtime::BrokenDownTime;
use crate::duration::TempusSignedDuration;
use crate::span::TempusSpan;
use crate::tz::TempusTimeZone;
use crate::utils::{AHKWstr, ahk_str_to_string, set_last_error_message, string_into_ahk_buff, AHKStringBuffer, unit_from_i8, round_mode_from_i8};
use crate::zoned::TempusZoned;


#[repr(C)]
pub struct TempusTimestampSeries {
    pub series: TimestampSeries
}

#[repr(C)]
pub struct TempusTimestamp {
    pub ts: Timestamp
}



impl FromStr for TempusTimestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ts: Timestamp = s.parse()?;
        Ok(TempusTimestamp {ts: ts})
    }
}

impl Display for TempusTimestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.ts.fmt(f)
    }
}

impl TempusTimestamp {
    fn now() -> Self {
        TempusTimestamp {ts: Timestamp::now()}
    }

    fn as_millisecond(&self) -> i64 {
        self.ts.as_millisecond()
    }

    fn as_second(&self) -> i64 {
        self.ts.as_second()
    }

    fn as_microsecond(&self) -> i64 {
        self.ts.as_microsecond()
    }


    fn in_tz(&self, tz: &str) -> Result<TempusZoned, Error> {
        let zoned = self.ts.in_tz(tz)?;
        Ok(TempusZoned{zoned})
    }

    fn from_second(second: i64) -> Result<Self, Error> {
        let ts = Timestamp::from_second(second)?;
        Ok(TempusTimestamp{ts})
    }

    fn from_millisecond(second: i64) -> Result<Self, Error> {
        let ts = Timestamp::from_millisecond(second)?;
        Ok(TempusTimestamp{ts})
    }

    fn from_microsecond(second: i64) -> Result<Self, Error> {
        let ts = Timestamp::from_microsecond(second)?;
        Ok(TempusTimestamp{ts})
    }

    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusTimestamp) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_as_millisecond(t: &TempusTimestamp) -> c_longlong {
    t.as_millisecond()
}

#[no_mangle]
pub extern "C" fn timestamp_as_second(t: &TempusTimestamp) -> c_longlong {
    t.as_second()
}

#[no_mangle]
pub extern "C" fn timestamp_as_microsecond(t: &TempusTimestamp) -> c_longlong {
    t.as_microsecond()
}

#[no_mangle]
pub extern "C" fn timestamp_from_second(s: i64, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    let maybe_ts = TempusTimestamp::from_second(s);
    match maybe_ts {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(ts) => {
            ts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_from_millisecond(s: i64, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    let maybe_ts = TempusTimestamp::from_millisecond(s);
    match maybe_ts {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(ts) => {
            ts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_from_microsecond(s: i64, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    let maybe_ts = TempusTimestamp::from_microsecond(s);
    match maybe_ts {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(ts) => {
            ts.stuff_into(out_ts);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_now() -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp::now())
}


#[no_mangle]
pub extern "C" fn timestamp_string_length(tts: &TempusTimestamp) -> usize {
    tts.to_string().len()
}

#[no_mangle]
pub extern "C" fn timestamp_to_string(tts: &TempusTimestamp, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = tts.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
}

#[no_mangle]
pub extern "C" fn timestamp_strftime_length(tts: &TempusTimestamp, ahk_format_str: AHKWstr) -> isize {
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(tts.ts);
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
pub extern "C" fn timestamp_strftime(tts: &TempusTimestamp, ahk_format_str: AHKWstr, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    if buff_len == 0 {
        return -1
    }
    match ahk_str_to_string(ahk_format_str) {
        Err(_) => {
            -1
        }
        Ok(format_str) => {
            let bdt = BrokenDownTime::from(tts.ts);
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
pub extern "C" fn timestamp_strptime(ahk_format_str: AHKWstr, ahk_time_str: AHKWstr, out_ts: *mut *mut TempusTimestamp) -> i64 {
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
                    match Timestamp::strptime(format_str, time_str) {
                        Err(e) => {
                            set_last_error_message(e.to_string());
                            -2
                        }
                        Ok(ts) => {
                            let tts = TempusTimestamp{ts};
                            tts.stuff_into(out_ts);
                            0
                        }
                    }
                }
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_in_tz(ahk_time_str: AHKWstr, tts: &TempusTimestamp, out_zoned: *mut *mut TempusZoned) -> c_longlong {
    match ahk_str_to_string(ahk_time_str) {
        Err(_) => {
            -1
        }
        Ok(zone_str) => {
            match tts.in_tz(zone_str.as_str()) {
                Ok(tzoned) => {
                    tzoned.stuff_into(out_zoned);
                    0
                }
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_is_zero(tts: &TempusTimestamp) -> i8 {
    tts.ts.is_zero().into()
}

#[no_mangle]
pub extern "C" fn timestamp_round(tts: &TempusTimestamp, unit: i8, increment: i64, round_mode: i8, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
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

    let ts_round = TimestampRound::new().smallest(round_unit).mode(mode).increment(increment);
    match tts.ts.round(ts_round) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -3
        }
        Ok(new_ts) => {
            let new_tts = TempusTimestamp{ts: new_ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_parse(ahk_time_string: AHKWstr, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            -1
        }
        Ok(time_string) => {
            let maybe_ts= time_string.as_str().parse::<TempusTimestamp>();
            match maybe_ts {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(ts) => {
                    ts.stuff_into(out_ts);
                    0
                }
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_min() -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp{ts: Timestamp::MIN})
}

#[no_mangle]
pub extern "C" fn timestamp_max() -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp{ts: Timestamp::MAX})
}

#[no_mangle]
pub extern "C" fn timestamp_unix_epoch() -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp{ts: Timestamp::UNIX_EPOCH})
}

#[no_mangle]
pub extern "C" fn timestamp_new(seconds: i64, nanoseconds: i32, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match Timestamp::new(seconds, nanoseconds) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let tts = TempusTimestamp{ts};
            tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_from_duration(tduration: &TempusSignedDuration, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match Timestamp::from_duration(tduration.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let tts = TempusTimestamp{ts};
            tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_subsec_millisecond(tts: &TempusTimestamp) -> c_int {
    tts.ts.subsec_millisecond()
}
#[no_mangle]
pub extern "C" fn timestamp_subsec_microsecond(tts: &TempusTimestamp) -> c_int {
    tts.ts.subsec_microsecond()
}
#[no_mangle]
pub extern "C" fn timestamp_subsec_nanosecond(tts: &TempusTimestamp) -> c_int {
    tts.ts.subsec_nanosecond()
}

#[no_mangle]
pub extern "C" fn timestamp_as_duration(tts: &TempusTimestamp) -> Box<TempusSignedDuration> {
    Box::new(TempusSignedDuration{duration: tts.ts.as_duration()})
}

#[no_mangle]
pub extern "C" fn timestamp_signum(tts: &TempusTimestamp) -> c_char {
    tts.ts.signum()
}

#[no_mangle]
pub extern "C" fn timestamp_to_zoned(tts: &TempusTimestamp, ttz: &TempusTimeZone) -> Box<TempusZoned> {
    Box::new(TempusZoned{zoned: tts.ts.to_zoned(ttz.tz.clone())})
}

#[no_mangle]
pub extern "C" fn timestamp_checked_add_span(tts: &TempusTimestamp, other: &TempusSpan, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.checked_add(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_checked_sub_span(tts: &TempusTimestamp, other: &TempusSpan, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.checked_sub(other.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}



#[no_mangle]
pub extern "C" fn timestamp_checked_add_signed_duration(tts: &TempusTimestamp, other: &TempusSignedDuration, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.checked_add(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_checked_sub_signed_duration(tts: &TempusTimestamp, other: &TempusSignedDuration, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.checked_sub(other.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_saturating_add_span(tts: &TempusTimestamp, rhs: &TempusSpan, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.saturating_add(rhs.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_saturating_sub_span(tts: &TempusTimestamp, rhs: &TempusSpan, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.saturating_sub(rhs.span) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_saturating_add_signed_duration(tts: &TempusTimestamp, rhs: &TempusSignedDuration, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.saturating_add(rhs.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn timestamp_saturating_sub_signed_duration(tts: &TempusTimestamp, rhs: &TempusSignedDuration, out_ts: *mut *mut TempusTimestamp) -> c_longlong {
    match tts.ts.saturating_sub(rhs.duration) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(ts) => {
            let new_tts = TempusTimestamp{ts};
            new_tts.stuff_into(out_ts);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn timestamp_until_timestamp(tts: &TempusTimestamp, other: &TempusTimestamp, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = TimestampDifference::from(other.ts).mode(round_mode).increment(increment);

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


    match tts.ts.until(dd) {
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
pub extern "C" fn timestamp_until_zoned(tts: &TempusTimestamp, other: &TempusZoned, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = TimestampDifference::from(other.zoned.clone()).mode(round_mode).increment(increment);

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


    match tts.ts.until(dd) {
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
pub extern "C" fn timestamp_since_timestamp(tts: &TempusTimestamp, other: &TempusTimestamp, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = TimestampDifference::from(other.ts).mode(round_mode).increment(increment);

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


    match tts.ts.since(dd) {
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
pub extern "C" fn timestamp_since_zoned(tts: &TempusTimestamp, other: &TempusZoned, largest_i: i8, smallest_i: i8, increment: i64, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e);
            return -2
        }
        Ok(round_mode) => round_mode,
    };
    let mut dd = TimestampDifference::from(other.zoned.clone()).mode(round_mode).increment(increment);

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


    match tts.ts.since(dd) {
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
pub extern "C" fn timestamp_duration_until(tts: &TempusTimestamp, other: &TempusTimestamp) -> Box<TempusSignedDuration> {
    let duration = tts.ts.duration_until(other.ts);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn timestamp_duration_since(tts: &TempusTimestamp, other: &TempusTimestamp) -> Box<TempusSignedDuration> {
    let duration = tts.ts.duration_since(other.ts);
    Box::new(TempusSignedDuration{duration})
}

#[no_mangle]
pub extern "C" fn timestamp_series(tts: &TempusTimestamp, tspan: &TempusSpan) -> Box<TempusTimestampSeries> {
    let series = tts.ts.series(tspan.span);
    Box::new(TempusTimestampSeries{series})
}


#[no_mangle]
pub extern "C" fn timestamp_compare(tts: &TempusTimestamp, other: &TempusTimestamp) -> c_char {
    match tts.ts.cmp(&other.ts) {
        Ordering::Less => {-1}
        Ordering::Equal => {0}
        Ordering::Greater => {1}
    }
}

#[no_mangle]
pub extern "C" fn free_timestamp(ts: Box<TempusTimestamp>) -> c_longlong {
    let raw = Box::into_raw(ts);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}

#[no_mangle]
pub extern "C" fn free_timestamp_series(ts: Box<TempusTimestampSeries>) -> c_longlong {
    let raw = Box::into_raw(ts);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}