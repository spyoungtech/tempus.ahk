use std::ffi::c_longlong;
use std::str::FromStr;
use jiff::{Error, Zoned};
use crate::timestamp::TempusTimestamp;
use crate::utils::{ahk_str_to_string, AHKWstr, set_last_error_message};

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
    }
}