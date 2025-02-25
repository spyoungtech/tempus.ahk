use std::ffi::c_longlong;
use std::str::FromStr;
use jiff::{SignedDuration, Error};
use crate::utils::{AHKWstr, ahk_str_to_string, set_last_error_message};
#[repr(C)]
pub struct TempusSignedDuration {
    duration: SignedDuration
}

impl TempusSignedDuration {
        pub(crate) fn stuff_into(self, pointer: *mut *mut TempusSignedDuration) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}

impl FromStr for TempusSignedDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let duration: SignedDuration = s.parse()?;
        Ok(TempusSignedDuration{duration})
    }
}


#[no_mangle]
pub extern "C" fn signed_duration_parse(ahk_duration_str: AHKWstr, duration_out: *mut *mut TempusSignedDuration) -> c_longlong {
    match ahk_str_to_string(ahk_duration_str) {
        Err(_) => {
            -1
        }
        Ok(duration_string) => {
            match duration_string.parse::<TempusSignedDuration>() {
                Ok(duration) => {
                    duration.stuff_into(duration_out);
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
pub extern "C" fn signed_duration_as_secs(tsd: &TempusSignedDuration) -> f64 {
    tsd.duration.as_secs_f64()
}
#[no_mangle]
pub extern "C" fn signed_duration_as_millis(tsd: &TempusSignedDuration) -> f64 {
    tsd.duration.as_millis_f64()
}

#[no_mangle]
pub extern "C" fn signed_duration_from_secs(secs: f64, out_sd: *mut *mut TempusSignedDuration) -> c_longlong {
    match SignedDuration::try_from_secs_f64(secs) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(duration) => {
            let tsd = TempusSignedDuration{duration};
            tsd.stuff_into(out_sd);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn signed_duration_from_millis(n: i64) -> Box<TempusSignedDuration> {
    let duration = SignedDuration::from_millis(n);
    Box::new(TempusSignedDuration{duration})
}
#[no_mangle]
pub extern "C" fn signed_duration_from_micros(n: i64) -> Box<TempusSignedDuration> {
    let duration = SignedDuration::from_micros(n);
    Box::new(TempusSignedDuration{duration})
}
#[no_mangle]
pub extern "C" fn signed_duration_from_nanos(n: i64) -> Box<TempusSignedDuration> {
    let duration = SignedDuration::from_nanos(n);
    Box::new(TempusSignedDuration{duration})
}



#[no_mangle]
pub extern "C" fn free_signed_duration(ts: Box<TempusSignedDuration>) -> c_longlong {
    let raw = Box::into_raw(ts);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}