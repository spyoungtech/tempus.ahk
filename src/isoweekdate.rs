use std::ffi::{c_char, c_short};
use std::ffi::c_longlong;
use jiff::civil::{ISOWeekDate, Weekday};
use crate::date::TempusDate;
use crate::utils::set_last_error_message;

#[repr(C)]
pub struct TempusISOWeekDate {
    pub weekdate: ISOWeekDate
}

impl TempusISOWeekDate {
    pub fn stuff_into(self, pointer: *mut *mut TempusISOWeekDate) {
        let handle = Box::new(self);
        unsafe {
            *pointer = Box::into_raw(handle);
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_min() -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: ISOWeekDate::MIN})
}

#[no_mangle]
pub extern "C" fn isoweekdate_max() -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: ISOWeekDate::MAX})
}

#[no_mangle]
pub extern "C" fn isoweekdate_zero() -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: ISOWeekDate::ZERO})
}

#[no_mangle]
pub extern "C" fn isoweekdate_new(year: i16, week: i8, weekday_i: i8, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    let weekday = match Weekday::from_sunday_one_offset(weekday_i) {
        Err(e) => {
            set_last_error_message(e.to_string());
            return -1
        }
        Ok(weekday) => weekday
    };
    match ISOWeekDate::new(year, week, weekday) {
        Err(e) => {
            set_last_error_message(e.to_string());
            -2
        }
        Ok(weekdate) => {
            let tiwd = TempusISOWeekDate{weekdate};
            tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_from_date(tdate: &TempusDate) -> Box<TempusISOWeekDate> {
    Box::new(TempusISOWeekDate{weekdate: ISOWeekDate::from_date(tdate.date)})
}

#[no_mangle]
pub extern "C" fn isoweekdate_year(tiwd: &TempusISOWeekDate) -> c_short {
    tiwd.weekdate.year()
}

#[no_mangle]
pub extern "C" fn isoweekdate_week(tiwd: &TempusISOWeekDate) -> c_char {
    tiwd.weekdate.week()
}

#[no_mangle]
pub extern "C" fn isoweekdate_weekday(tiwd: &TempusISOWeekDate) -> c_char {
    tiwd.weekdate.weekday().to_sunday_one_offset()
}

#[no_mangle]
pub extern "C" fn isoweekdate_first_of_week(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.first_of_week() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_last_of_week(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.last_of_week() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_first_of_year(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.first_of_year() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn isoweekdate_last_of_year(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.last_of_year() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_tomorrow(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.tomorrow() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_yesterday(tiwd: &TempusISOWeekDate, out_weekdate: *mut *mut TempusISOWeekDate) -> c_longlong {
    match tiwd.weekdate.yesterday() {
        Err(e) => {
            set_last_error_message(e.to_string());
            -1
        }
        Ok(weekdate) => {
            let new_tiwd = TempusISOWeekDate{weekdate};
            new_tiwd.stuff_into(out_weekdate);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn isoweekdate_to_date(tiwd: &TempusISOWeekDate) -> Box<TempusDate> {
    Box::new(TempusDate{date: tiwd.weekdate.date()})
}



#[no_mangle]
pub extern "C" fn isoweekdate_days_in_year(tiwd: &TempusISOWeekDate) -> c_short {
    tiwd.weekdate.days_in_year()
}

#[no_mangle]
pub extern "C" fn isoweekdate_weeks_in_year(tiwd: &TempusISOWeekDate) -> c_char {
    tiwd.weekdate.weeks_in_year()
}

#[no_mangle]
pub extern "C" fn isoweekdate_in_long_year(tiwd: &TempusISOWeekDate) -> c_char {
    tiwd.weekdate.in_long_year() as i8
}



#[no_mangle]
pub extern "C" fn free_isoweekdate(tiwd: Box<TempusISOWeekDate>) -> c_longlong {
    let raw = Box::into_raw(tiwd);
    unsafe {
        drop(Box::from_raw(raw))
    }
    0
}