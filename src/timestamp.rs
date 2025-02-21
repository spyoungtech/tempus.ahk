use jiff::{Error, Span, Timestamp, Zoned};

use std::ffi::{c_longlong};
use std::str::FromStr;

use crate::utils::{AHKWstr, ahk_str_to_string, set_last_error_message};
use crate::zoned::TempusZoned;

#[repr(C)]
pub struct TempusTimestamp {
    ts: Timestamp
}



impl FromStr for TempusTimestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ts: Timestamp = s.parse()?;
        Ok(TempusTimestamp {ts: ts})
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

    fn in_tz(&self, tz: &str) -> Result<TempusZoned, Error> {
        let zoned = self.ts.in_tz(tz)?;
        Ok(TempusZoned{zoned})
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
pub extern "C" fn timestamp_now() -> Box<TempusTimestamp> {
    Box::new(TempusTimestamp::now())
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
pub extern "C" fn free_timestamp(ts: Box<TempusTimestamp>) -> c_longlong {
    let raw = Box::into_raw(ts);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}