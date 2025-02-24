#![allow(dead_code)]

use std::cmp;
use std::ffi::c_longlong;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use jiff::{Error, Span, SpanCompare, SpanRelativeTo, SpanRound, SpanTotal};
use crate::utils::{ahk_str_to_string, round_mode_from_i8, set_last_error_message, string_into_ahk_buff, unit_from_i8, AHKStringBuffer, AHKWstr};

#[repr(C)]
pub struct TempusSpan {
    span: Span
}

impl Display for TempusSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.span.fmt(f)
    }
}

impl FromStr for TempusSpan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let span: Span = s.parse()?;
        Ok(TempusSpan{span})
    }
}

impl TempusSpan {
    pub(crate) fn stuff_into(self, pointer: *mut *mut TempusSpan) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }}

#[no_mangle]
pub extern "C" fn span_new() -> Box<TempusSpan> {
    Box::new(TempusSpan{span: Span::new()})
}


#[no_mangle]
pub extern "C" fn span_days(tspan: &TempusSpan, days: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_days(days) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_hours(tspan: &TempusSpan, hours: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_hours(hours) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn span_seconds(tspan: &TempusSpan, seconds: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_seconds(seconds) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_milliseconds(tspan: &TempusSpan, milliseconds: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_milliseconds(milliseconds) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_microseconds(tspan: &TempusSpan, microseconds: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_microseconds(microseconds) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_nanoseconds(tspan: &TempusSpan, nanoseconds: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_nanoseconds(nanoseconds) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_weeks(tspan: &TempusSpan, weeks: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_weeks(weeks) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}
#[no_mangle]
pub extern "C" fn span_months(tspan: &TempusSpan, months: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_months(months) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}
#[no_mangle]
pub extern "C" fn span_years(tspan: &TempusSpan, years: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_years(years) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}
#[no_mangle]
pub extern "C" fn span_minutes(tspan: &TempusSpan, minutes: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.try_minutes(minutes) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_get_years(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_years() as i64
}
#[no_mangle]
pub extern "C" fn span_get_months(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_months() as i64
}
#[no_mangle]
pub extern "C" fn span_get_weeks(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_weeks() as i64
}
#[no_mangle]
pub extern "C" fn span_get_days(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_days() as i64
}
#[no_mangle]
pub extern "C" fn span_get_hours(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_hours() as i64
}
#[no_mangle]
pub extern "C" fn span_get_minutes(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_minutes()
}
#[no_mangle]
pub extern "C" fn span_get_seconds(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_seconds()
}
#[no_mangle]
pub extern "C" fn span_get_milliseconds(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_milliseconds()
}
#[no_mangle]
pub extern "C" fn span_get_microseconds(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_microseconds()
}
#[no_mangle]
pub extern "C" fn span_get_nanoseconds(tspan: &TempusSpan) -> c_longlong {
    tspan.span.get_nanoseconds()
}

#[no_mangle]
pub extern "C" fn span_string_length(tspan: &TempusSpan) -> usize {
    tspan.to_string().len()
}

#[no_mangle]
pub extern "C" fn span_to_string(tspan: &TempusSpan, out_buff: AHKStringBuffer, buff_len: usize) -> c_longlong {
    let ret = tspan.to_string();
    string_into_ahk_buff(ret, out_buff, buff_len);
    0
}

#[no_mangle]
pub extern "C" fn span_abs(tspan: &TempusSpan) -> Box<TempusSpan> {
    Box::new(TempusSpan{span: tspan.span.abs()})
}

#[no_mangle]
pub extern "C" fn span_negate(tspan: &TempusSpan) -> Box<TempusSpan> {
    Box::new(TempusSpan{span: tspan.span.negate()})
}


#[no_mangle]
pub extern "C" fn span_is_negative(tspan: &TempusSpan) -> i8 {
    tspan.span.is_negative() as i8
}
#[no_mangle]
pub extern "C" fn span_is_positive(tspan: &TempusSpan) -> i8 {
    tspan.span.is_positive() as i8
}
#[no_mangle]
pub extern "C" fn span_is_zero(tspan: &TempusSpan) -> i8 {
    tspan.span.is_zero() as i8
}

#[no_mangle]
pub extern "C" fn span_signum(tspan: &TempusSpan) -> i8 {
    tspan.span.signum()
}

#[no_mangle]
pub extern "C" fn span_checked_mul(tspan: &TempusSpan, rhs: i64, out_span: *mut *mut TempusSpan) -> c_longlong {
    match tspan.span.checked_mul(rhs) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}
#[no_mangle]
pub extern "C" fn span_checked_add_span(tspan: &TempusSpan, other_span: &TempusSpan, days_are_24_hours_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let spanres: Result<Span, Error>;

    let days_are_24_hours = match days_are_24_hours_i {
        0 => false,
        1 => true,
        _ => {
            set_last_error_message("invalid options".to_string());
            return -1
        }
    };

    if days_are_24_hours {
        spanres = tspan.span.checked_add((other_span.span, SpanRelativeTo::days_are_24_hours()))
    } else {
        spanres = tspan.span.checked_add(other_span.span)
    }

    match spanres {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_checked_sub_span(tspan: &TempusSpan, other_span: &TempusSpan, days_are_24_hours_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let spanres: Result<Span, Error>;
    let days_are_24_hours = match days_are_24_hours_i {
        0 => false,
        1 => true,
        _ => {
            set_last_error_message("invalid options".to_string());
            return -1
        }
    };
    if days_are_24_hours {
        spanres = tspan.span.checked_sub((other_span.span, SpanRelativeTo::days_are_24_hours()))
    } else {
        spanres = tspan.span.checked_sub(other_span.span)
    }
    match spanres {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(new_span) => {
            let new_tspan = TempusSpan{span: new_span};
            new_tspan.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_compare(tspan: &TempusSpan, other_span: &TempusSpan, days_are_24_hours_i: i8) -> i8 {
    let days_are_24_hours = match days_are_24_hours_i {
        0 => false,
        1 => true,
        _ => {
            set_last_error_message("invalid options".to_string());
            return -2
        }
    };
    if days_are_24_hours {
        match tspan.span.compare(SpanCompare::from(other_span.span).days_are_24_hours()) {
            Err(e) => {
                set_last_error_message(e.to_string());
                -3
            }
            Ok(result) => {result as i8}
        }
    } else {
        match tspan.span.compare(other_span.span) {
            Err(e) => {
                set_last_error_message(e.to_string());
                -4
            }
            Ok(result) => {result as i8}
        }
    }
}

#[no_mangle]
pub extern "C" fn span_total(tspan: &TempusSpan, unit_i: i8, days_are_24_hours_i: i8, out_f64: *mut f64) -> c_longlong {
    let days_are_24_hours = match days_are_24_hours_i {
        0 => false,
        1 => true,
        _ => {
            set_last_error_message("invalid options".to_string());
            return -1
        }
    };
    let unit = match unit_from_i8(unit_i) {
        Ok(u) => u,
        Err(e) => {
            set_last_error_message(e.to_string());
            return -2
        }
    };
    if days_are_24_hours {
        match tspan.span.total(SpanTotal::from(unit).days_are_24_hours()) {
            Err(e) => {
                set_last_error_message(e.to_string());
                -3
            }
            Ok(res) => {
                unsafe {
                    out_f64.replace(res);
                }
                0
            }
        }
    } else {
        match tspan.span.total(unit) {
            Err(e) => {
                set_last_error_message(e.to_string());
                -4
            }
            Ok(res) => {
                unsafe {
                    out_f64.replace(res);
                }
                0
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn span_round(tspan: &TempusSpan, smallest_i: i8, increment: i64, largest_i: i8, round_mode_i: i8, out_span: *mut *mut TempusSpan) -> c_longlong {
    let round_mode = match round_mode_from_i8(round_mode_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(mode) => mode
    };
    let mut rounder = SpanRound::new().mode(round_mode).increment(increment);
    if smallest_i >= 0 {
        match unit_from_i8(smallest_i) {
            Err(e) => {
                set_last_error_message(e.to_string());
                return -2
            }
            Ok(smallest) => {
                rounder = rounder.smallest(smallest)
            }
        };
    }
    if largest_i >= 0 {
        match unit_from_i8(largest_i) {
            Err(e) => {
                set_last_error_message(e.to_string());
                return -3
            }
            Ok(largest) => {
                rounder = rounder.largest(largest);
            }
        }
    }

    match tspan.span.round(rounder) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -4
        }
        Ok(rounded) => {
            let new_tts = TempusSpan{span: rounded};
            new_tts.stuff_into(out_span);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn span_parse(ahk_time_string: AHKWstr, out_ts: *mut *mut TempusSpan) -> c_longlong {
    match ahk_str_to_string(ahk_time_string) {
        Err(_) => {
            -1
        }
        Ok(time_string) => {
            let maybe_ts= time_string.as_str().parse::<TempusSpan>();
            match maybe_ts {
                Err(e) => {
                    set_last_error_message(e.to_string());
                    -2
                }
                Ok(tspan) => {
                    tspan.stuff_into(out_ts);
                    0
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_span(tspan: Box<TempusSpan>) -> c_longlong {
    let raw = Box::into_raw(tspan);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}
