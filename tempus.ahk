; try to load the tempus_ahk dll.
; we ignore this if it fails, allowing the user to provide their own `DllLoad` call before #Include of this script
#DllLoad "*i tempus_ahk"

; At runtime, check that the module was actually loaded successfully
if !DllCall("GetModuleHandle", "str", "tempus_ahk") {
    throw Error("Cannot load tempus_ahk.dll -- please ensure it is on PATH or use #DllLoad to load it in your script before your #Inlude of tempus.ahk")
}


Unit := {
    Nanosecond: 0, 
    Microsecond: 1, 
    Millisecond: 2, 
    Second: 3, 
    Minute: 4, 
    Hour: 5, 
    Day: 6, 
    Week: 7, 
    Month: 8, 
    Year: 9
}

WeekDay := {
    Sunday: 1,
    Monday: 2,
    Tuesday: 3,
    Wednesday: 4,
    Thursday: 5,
    Friday: 6,
    Saturday: 7
}

RoundMode := {
    Ceil: 1,
    Floor: 2,
    Expand: 3,
    Trunc: 4,
    HalfCeil: 5,
    HalfFloor: 6,
    HalfExpand: 7,
    HalfTrunc: 8,
    HalfEven: 9,
}

_Ordering := {
    LESS: -1,
    EQUAL: 0,
    GREATER: 1,
}

_get_last_error() {
    length := DllCall("tempus_ahk\get_last_error_length", "UInt")
    if (length > 0)
    {
        ; Allocate a buffer of length+1 for the null terminator
        buff := Buffer(length + 1, 0)

        success := DllCall("tempus_ahk\get_last_error"
                             , "Ptr", buff
                             , "UInt", buff.Size
                             , "UInt")

        errMsg := StrGet(buff, "UTF-8")
        return errMsg
    }
    else
    {
        return "Unknown Error"
    }
}


class ISOWeekDate {
    __New(pointer) {
        this.pointer := pointer
    }
    __Delete() {
        DllCall("tempus_ahk\free_isoweekdate", "Ptr", this.pointer, "Int64")
    }

    static new(year, month, weekday) {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_new", "Short", year, "Char", month, "Char", weekday, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    static MIN() {
        pointer := DllCall("tempus_ahk\isoweekdate_min", "Ptr")
        return ISOWeekDate(pointer)
    }
    static MAX() {
        pointer := DllCall("tempus_ahk\isoweekdate_max", "Ptr")
        return ISOWeekDate(pointer)
    }

    static ZERO() {
        pointer := DllCall("tempus_ahk\isoweekdate_zero", "Ptr")
        return ISOWeekDate(pointer)
    }
    static from_date(date_) {
        if !(date_ is Date) {
            throw Error("Unsupported type. Must be Date", -2)
        }
        pointer := DllCall("tempus_ahk\isoweekdate_from_date", "Ptr", date_.pointer, "Ptr")
        return ISOWeekDate(pointer)
    }

    year() {
        return DllCall("tempus_ahk\isoweekdate_year", "Ptr", this.pointer, "Short")
    }
    week() {
        return DllCall("tempus_ahk\isoweekdate_week", "Ptr", this.pointer, "Char")
    }

    weekday() {
        return DllCall("tempus_ahk\isoweekdate_weekday", "Ptr", this.pointer, "Char")
    }

    first_of_week() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_first_of_week", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    last_of_week() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_last_of_week", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    first_of_year() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_first_of_year", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    last_of_year() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_last_of_year", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    days_in_year() {
        return DllCall("tempus_ahk\isoweekdate_days_in_year", "Ptr", this.pointer, "Short")
    }

    weeks_in_year() {
        return DllCall("tempus_ahk\isoweekdate_weeks_in_year", "Ptr", this.pointer, "Short")
    }

    in_long_year() {
        ret := DllCall("tempus_ahk\isoweekdate_in_long_year", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }

    tomorrow() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_tomorrow", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }
    yesterday() {
        out_weekdate := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\isoweekdate_yesterday", "Ptr", this.pointer, "Ptr", out_weekdate, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_weekdate, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return ISOWeekDate(handle)
    }

    to_date() {
        pointer := DllCall("tempus_ahk\isoweekdate_to_date", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }
}



class SignedDuration {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_signed_duration", "Ptr", this.pointer, "Int64")
    }
    to_string() {
        buff_length := DllCall("tempus_ahk\signed_duration_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\signed_duration_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    to_string_friendly() {
        buff_length := DllCall("tempus_ahk\signed_duration_string_length_friendly", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\signed_duration_to_string_friendly", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string_friendly()
    }

    static parse(duration_string) {
        duration_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_parse", "WStr", duration_string, "Ptr", duration_out, "Int64")

        if (retcode = 0) {
            handle := NumGet(duration_out, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("error {}", message), -2)
        } else {
            throw "Unexpected error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    static new(secs, nanosecs) {
        duration_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_new", "Int64", secs, "Int", nanosecs, "Ptr", duration_out, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(duration_out, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    static ZERO() {
        pointer := DllCall("tempus_ahk\signed_duration_zero", "Ptr")
        return SignedDuration(pointer)
    }

    static MIN() {
        pointer := DllCall("tempus_ahk\signed_duration_min", "Ptr")
        return SignedDuration(pointer)
    }
    static MAX() {
        pointer := DllCall("tempus_ahk\signed_duration_max", "Ptr")
        return SignedDuration(pointer)
    }


    static from_secs(secs) {
        duration_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_from_secs", "Double", secs, "Ptr", duration_out, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(duration_out, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    static from_hours(hours) {
        return this.from_secs(hours * 3600)
    }

    static from_mins(minutes) {
        return this.from_secs(minutes * 60)
    }


    static from_millis(n) {
        pointer := DllCall("tempus_ahk\signed_duration_from_millis", "Int64", n, "Ptr")
        return SignedDuration(pointer)
    }
    static from_micros(n) {
        pointer := DllCall("tempus_ahk\signed_duration_from_micros", "Int64", n, "Ptr")
        return SignedDuration(pointer)
    }
    static from_nanos(n) {
        pointer := DllCall("tempus_ahk\signed_duration_from_nanos", "Int64", n, "Ptr")
        return SignedDuration(pointer)
    }
    as_secs() {
        return DllCall("tempus_ahk\signed_duration_as_secs", "Ptr", this.pointer, "Double")
    }
    as_millis() {
        return DllCall("tempus_ahk\signed_duration_as_millis", "Ptr", this.pointer, "Double")
    }

    as_hours() {
        return DllCall("tempus_ahk\signed_duration_as_hours", "Ptr", this.pointer, "Int64")
    }

    as_mins() {
        return DllCall("tempus_ahk\signed_duration_as_mins", "Ptr", this.pointer, "Int64")
    }

    is_zero() {
        ret := DllCall("tempus_ahk\signed_duration_is_zero", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }

    is_positive() {
        ret := DllCall("tempus_ahk\signed_duration_is_positive", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }

    is_negative() {
        ret := DllCall("tempus_ahk\signed_duration_is_negative", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }

    signum() {
        return DllCall("tempus_ahk\signed_duration_signum", "Ptr", this.pointer, "Char")
    }

    checked_neg() {
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_checked_neg", "Ptr", this.pointer, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    checked_add(other_signed_duration) {
        if !(other_signed_duration is SignedDuration) {
            throw Error("add only supported for SignedDuration", -2)
        }
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_checked_add", "Ptr", this.pointer, "Ptr", other_signed_duration.pointer, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }
    checked_sub(other_signed_duration) {
        if !(other_signed_duration is SignedDuration) {
            throw Error("add only supported for SignedDuration", -2)
        }
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_checked_sub", "Ptr", this.pointer, "Ptr", other_signed_duration.pointer, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    checked_mul(i) {
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_checked_mul", "Ptr", this.pointer, "Int", i, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    checked_div(i) {
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_checked_div", "Ptr", this.pointer, "Int", i, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }

    div_duration(other_duration) {
        if !(other_duration is SignedDuration) {
            throw Error("argument must be a SignedDuration", -2)
        }
        return DllCall("tempus_ahk\signed_duration_div_duration", "Ptr", this.pointer, "Ptr", other_duration.pointer, "Double")
    }

    compare(other_duration) {
        if !(other_duration is SignedDuration) {
            throw Error("argument must be a SignedDuration", -2)
        }
        return DllCall("tempus_ahk\signed_duration_compare", "Ptr", this.pointer, "Ptr", other_duration.pointer, "Char")
    }

    gt(other_duration) {
        result := this.compare(other_duration)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_duration) {
        result := this.compare(other_duration)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_duration) {
        result := this.compare(other_duration)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_duration) {
        result := this.compare(other_duration)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_duration) {
        result := this.compare(other_duration)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    round(smallest := Unit.Nanosecond, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_round", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", round_mode, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return SignedDuration(handle)
    }



    abs() {
        out_duration := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\signed_duration_abs", "Ptr", this.pointer, "Ptr", out_duration, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_duration, 0, "Ptr")
        if (handle = 0) {
            throw "Unexpected error"
        }
        return SignedDuration(handle)
    }




}

class Zoned {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_zoned", "Ptr", this.pointer, "Int64")
    }

    static now() {
        ptr := DllCall("tempus_ahk\zoned_now", "Ptr")
        return Zoned(ptr)
    }

    static new(ts, tz) {
        if !(ts is Timestamp) {
            throw Error("Unsupported type for first argument. Must be Timestamp", -2)
        }
        if !(tz is Timezone) {
            throw Error("Unsupported type for second argument. Must be Timezone", -2)
        }
        pointer := DllCall("tempus_ahk\zoned_new", "Ptr", ts.pointer, "Ptr", tz.pointer, "Ptr")
        return Zoned(pointer)
    }

    with_time_zone(tz) {
        if !(tz is Timezone) {
            throw Error("argument must be Timezone", -2)
        }
        pointer := DllCall("tempus_ahk\zoned_with_time_zone", "Ptr", this.pointer, "Ptr", tz.pointer, "Int64")
        return Zoned(pointer)
    }

    in_tz(tz_name) {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_in_tz", "Ptr", this.pointer, "WStr", tz_name, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    year() {
        return DllCall("tempus_ahk\zoned_year", "Ptr", this.pointer, "Short")
    }
    month() {
        return DllCall("tempus_ahk\zoned_month", "Ptr", this.pointer, "Char")
    }
    day() {
        return DllCall("tempus_ahk\zoned_day", "Ptr", this.pointer, "Char")
    }
    hour() {
        return DllCall("tempus_ahk\zoned_hour", "Ptr", this.pointer, "Char")
    }
    minute() {
        return DllCall("tempus_ahk\zoned_minute", "Ptr", this.pointer, "Char")
    }
    second() {
        return DllCall("tempus_ahk\zoned_second", "Ptr", this.pointer, "Char")
    }
    millisecond() {
        return DllCall("tempus_ahk\zoned_millisecond", "Ptr", this.pointer, "Short")
    }
    microsecond() {
        return DllCall("tempus_ahk\zoned_microsecond", "Ptr", this.pointer, "Short")
    }
    nanosecond() {
        return DllCall("tempus_ahk\zoned_nanosecond", "Ptr", this.pointer, "Short")
    }
    subsec_nanosecond() {
        return DllCall("tempus_ahk\zoned_subsec_nanosecond", "Ptr", this.pointer, "Int")
    }

    era() {
        ret := DllCall("tempus_ahk\zoned_era", "Ptr", this.pointer, "Char")
        if (ret = -1) {
            return "BCE"
        } else if (ret = 1) {
            return "CE"
        } else {
            throw "unexpected error"
        }
    }

    era_year() {
        return DllCall("tempus_ahk\zoned_era_year", "Ptr", this.pointer, "Short")
    }

    weekday() {
        return DllCall("tempus_ahk\zoned_weekday", "Ptr", this.pointer, "Short")
    }

    day_of_year() {
        return DllCall("tempus_ahk\zoned_day_of_year", "Ptr", this.pointer, "Short")
    }

    day_of_year_no_leap() {
        ret := DllCall("tempus_ahk\zoned_day_of_year_no_leap", "Ptr", this.pointer, "Short")
        if (ret = -1) {
            return
        } else {
            return ret
        }
    }

    first_of_month() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_first_of_month", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    start_of_day() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_start_of_day", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }
    end_of_day() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_end_of_day", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }



    last_of_month() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_last_of_month", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    first_of_year() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_first_of_year", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    last_of_year() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_last_of_year", "Ptr", this.pointer, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    days_in_month() {
        return DllCall("tempus_ahk\zoned_days_in_month", "Ptr", this.pointer, "Char")
    }

    days_in_year() {
        return DllCall("tempus_ahk\zoned_days_in_year", "Ptr", this.pointer, "Short")
    }

    in_leap_year() {
        ret := DllCall("tempus_ahk\zoned_in_leap_year", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }
    tomorrow() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_tomorrow", "Ptr", this.pointer, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }
    yesterday() {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_yesterday", "Ptr", this.pointer, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    nth_weekday_of_month(nth, weekday_i) {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_nth_weekday_of_month", "Ptr", this.pointer, "Char", nth, "Char", weekday_i, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    nth_weekday(nth, weekday_i) {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_nth_weekday", "Ptr", this.pointer, "Int", nth, "Char", weekday_i, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    to_timestamp() {
        pointer := DllCall("tempus_ahk\zoned_to_timestamp", "Ptr", this.pointer, "Ptr")
        return Timestamp(pointer)
    }

    to_datetime() {
        pointer := DllCall("tempus_ahk\zoned_to_datetime", "Ptr", this.pointer, "Ptr")
        return DateTime(pointer)
    }

    to_date() {
        pointer := DllCall("tempus_ahk\zoned_to_date", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }

    to_time() {
        pointer := DllCall("tempus_ahk\zoned_to_time", "Ptr", this.pointer, "Ptr")
        return Time(pointer)
    }

    to_isoweekdate() {
        pointer := DllCall("tempus_ahk\zoned_to_isoweekdate", "Ptr", this.pointer, "Ptr")
        return ISOWeekDate(pointer)
    }


    checked_add(other) {
        out_zoned := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\zoned_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_zoned, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\zoned_checked_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_zoned, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    checked_sub(other) {
        out_zoned := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\zoned_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_zoned, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\zoned_checked_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_zoned, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    since_zoned(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Zoned) {
            retcode := DllCall("tempus_ahk\zoned_since_zoned", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Zoned", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    until_zoned(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Zoned) {
            retcode := DllCall("tempus_ahk\zoned_until_zoned", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Zoned", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    saturating_add(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\zoned_saturating_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\zoned_saturating_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Zoned(pointer)
    }

    saturating_sub(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\zoned_saturating_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\zoned_saturating_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Zoned(pointer)
    }

    strftime(format_str) {
        buff_length := DllCall("tempus_ahk\zoned_strftime_length", "Ptr", this.pointer, "WStr", format_str, "Int64")
        if buff_length < 0 {
            error_code := buff_length
            if (error_code = -2 || error_code = -3) {
                message := _get_last_error()
                throw Error(Format("error({}): {}", error_code, message), -2)
            }
            else {
                throw "unexpected error getting buff length"
            }
        }
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\zoned_strftime", "Ptr", this.pointer, "WStr", format_str, "Ptr", buff, "UInt64", buff.Size, "Int64")
        if (retcode = 0) {
            ret := StrGet(buff, "UTF-8")
            return ret
        } else {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
    }

    static strptime(format_str, time_str) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_strptime", "WStr", format_str, "WStr", time_str, "Ptr", out_ts)
        if (retcode < 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    duration_until(other_time) {
        if !(other_time is Zoned) {
            throw Error("Unsupported Type. Must be Zoned", -2)
        }
        pointer := DllCall("tempus_ahk\zoned_duration_until", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }
    duration_since(other_time) {
        if !(other_time is Zoned) {
            throw Error("Unsupported Type. Must be Zoned", -2)
        }
        pointer := DllCall("tempus_ahk\zoned_duration_since", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    to_string() {
        buff_length := DllCall("tempus_ahk\zoned_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\zoned_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string()
    }
    round(round_unit, increment := 1, round_mode := RoundMode.HalfExpand) {
        if round_unit < 0 {
            throw Error("Invalid round unit.", -2)
        }
        if round_unit > 5 {
            throw Error("Largest allowed unit is Unit.Hour", -2)
        }
        if (round_mode < 1 || round_mode > 9) {
            throw Error("Invalid round mode", -2)
        }
        out_zoned := Buffer(A_PtrSize)

        retcode := DllCall("tempus_ahk\zoned_round", "Ptr", this.pointer, "Char", round_unit, "Int64", increment, "Char", round_mode, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error ({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    compare(other_zoned) {
        if !(other_zoned is Zoned) {
            throw Error("unsupported type. Must be Zoned", -2)
        }
        return DllCall("tempus_ahk\zoned_compare", "Ptr", this.pointer, "Ptr", other_zoned.pointer, "Char")
    }

    gt(other_zoned) {
        result := this.compare(other_zoned)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_zoned) {
        result := this.compare(other_zoned)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_zoned) {
        result := this.compare(other_zoned)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_zoned) {
        result := this.compare(other_zoned)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_zoned) {
        result := this.compare(other_zoned)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }
    static parse(time_string) {
        ts_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\zoned_parse", "WStr", time_string, "Ptr", ts_out, "Int64")

        if (retcode = 0) {
            handle := NumGet(ts_out, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("error in parsing zoned: {}", message), -2)
        } else {
            throw "unknown error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }
}

class Timestamp {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_timestamp", "Ptr", this.pointer, "Int64")
    }

    static now() {
        ptr := DllCall("tempus_ahk\timestamp_now", "Ptr")
        return Timestamp(ptr)
    }

    static MIN() {
        ptr := DllCall("tempus_ahk\timestamp_min", "Ptr")
        return Timestamp(ptr)
    }

    static MAX() {
        ptr := DllCall("tempus_ahk\timestamp_max", "Ptr")
        return Timestamp(ptr)
    }
    static UNIX_EPOCH() {
        ptr := DllCall("tempus_ahk\timestamp_unix_epoch", "Ptr")
        return Timestamp(ptr)
    }


    static parse(time_string) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_parse", "WStr", time_string, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    static new(second, nanosecond) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_new", "Int64", second, "Int", nanosecond, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    as_millisecond() {
        return DllCall("tempus_ahk\timestamp_as_millisecond", "Ptr", this.pointer, "Int64")
    }

    as_microsecond() {
        return DllCall("tempus_ahk\timestamp_as_microsecond", "Ptr", this.pointer, "Int64")
    }

    as_second() {
        return DllCall("tempus_ahk\timestamp_as_second", "Ptr", this.pointer, "Int64")
    }

    subsec_millisecond() {
        return DllCall("tempus_ahk\timestamp_subsec_millisecond", "Ptr", this.pointer, "Int")
    }
    subsec_microsecond() {
        return DllCall("tempus_ahk\timestamp_subsec_microsecond", "Ptr", this.pointer, "Int")
    }
    subsec_nanosecond() {
        return DllCall("tempus_ahk\timestamp_subsec_nanosecond", "Ptr", this.pointer, "Int")
    }

    static from_second(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_second", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)

    }
    static from_millisecond(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_millisecond", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)

    }
    static from_microsecond(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_microsecond", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)

    }

    static from_duration(duration) {
        if !(duration is SignedDuration) {
            throw Error("Unsupported Type. Must be SignedDuration")
        }
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_duration", "Ptr", duration.pointer, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    as_duration() {
        pointer := DllCall("tempus_ahk\timestamp_as_duration", "Ptr", this.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    in_tz(timezone) {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_in_tz", "WStr", timezone, "Ptr", this.pointer, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    time_zone() {
        pointer := DllCall("tempus_ahk\zoned_time_zone")
        return Timezone(pointer)
    }

    to_zoned(tz) {
        if !(tz is Timezone) {
            throw Error("Unsupported type. Must be Timezone", -2)
        }
        pointer := DllCall("tempus_ahk\timestamp_to_zoned", "Ptr", this.pointer, "Ptr", tz.pointer, "Ptr")
        return Zoned(pointer)
    }

    signum() {
        return DllCall("tempus_ahk\timestamp_signum", "Ptr", this.pointer, "Char")
    }

    checked_add(other) {
        out_ts := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\timestamp_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\timestamp_checked_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    checked_sub(other) {
        out_ts := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\timestamp_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\timestamp_checked_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    saturating_add(other) {
        out_ts := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\timestamp_saturating_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\timestamp_saturating_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    saturating_sub(other) {
        out_ts := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\timestamp_saturating_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\timestamp_saturating_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_ts, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    until_time(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Timestamp) {
            retcode := DllCall("tempus_ahk\timestamp_until_timestamp", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is Zoned) {
            retcode := DllCall("tempus_ahk\timestamp_until_zoned", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Timestamp or Zoned", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    since(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Timestamp) {
            retcode := DllCall("tempus_ahk\timestamp_since_timestamp", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is Zoned) {
            retcode := DllCall("tempus_ahk\timestamp_since_zoned", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Timestamp or Zoned", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    duration_until(other_time) {
        if !(other_time is Timestamp) {
            throw Error("Unsupported Type. Must be Timestamp", -2)
        }
        pointer := DllCall("tempus_ahk\timestamp_duration_until", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }
    duration_since(other_time) {
        if !(other_time is Timestamp) {
            throw Error("Unsupported Type. Must be Timestamp", -2)
        }
        pointer := DllCall("tempus_ahk\timestamp_duration_since", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }
    to_string() {
        buff_length := DllCall("tempus_ahk\timestamp_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\timestamp_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string()
    }

    strftime(format_str) {
        buff_length := DllCall("tempus_ahk\timestamp_strftime_length", "Ptr", this.pointer, "WStr", format_str, "Int64")
        if buff_length < 0 {
            error_code := buff_length
            if (error_code = -2 || error_code = -3) {
                message := _get_last_error()
                throw Error(Format("error({}): {}", error_code, message), -2)
            }
            else {
                throw "unexpected error getting buff length"
            }
        }
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\timestamp_strftime", "Ptr", this.pointer, "WStr", format_str, "Ptr", buff, "UInt64", buff.Size, "Int64")
        if (retcode = 0) {
            ret := StrGet(buff, "UTF-8")
            return ret
        } else {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
    }

    static strptime(format_str, time_str) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_strptime", "WStr", format_str, "WStr", time_str, "Ptr", out_ts)
        if (retcode < 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    is_zero() {
        ret := DllCall("tempus_ahk\timestamp_is_zero", "Ptr", this.pointer, "Char")
        if ret {
            return true
        } else {
            return false
        }
    }

    compare(other_timestamp) {
        if !(other_timestamp is Timestamp) {
            throw Error("unsupported type. Must be Timestamp", -2)
        }
        return DllCall("tempus_ahk\timestamp_compare", "Ptr", this.pointer, "Ptr", other_timestamp.pointer, "Char")
    }

    gt(other_timestamp) {
        result := this.compare(other_timestamp)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_timestamp) {
        result := this.compare(other_timestamp)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_timestamp) {
        result := this.compare(other_timestamp)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_timestamp) {
        result := this.compare(other_timestamp)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_timestamp) {
        result := this.compare(other_timestamp)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }


    round(round_unit, increment := 1, round_mode := RoundMode.HalfExpand) {
        if round_unit < 0 {
            throw Error("Invalid round unit.", -2)
        }
        if round_unit > 5 {
            throw Error("Largest allowed unit is Unit.Hour", -2)
        }
        if (round_mode < 1 || round_mode > 9) {
            throw Error("Invalid round mode", -2)
        }
        out_ts := Buffer(A_PtrSize)
        
        retcode := DllCall("tempus_ahk\timestamp_round", "Ptr", this.pointer, "Char", round_unit, "Int64", increment, "Char", round_mode, "Ptr", out_ts, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error ({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_ts, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timestamp(handle)
    }

    series(span_interval) {
        ; TODO: support creating span from string arg?
        if !(span_interval is Span) {
            throw Error("Unsupported Type. Must be a Span type")
        }
        pointer := DllCall("tempus_ahk\timestamp_series", "Ptr", this.pointer, "Ptr", span_interval.pointer, "Ptr")
        return TimestampSeries(pointer)
    }

}

class Span {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_span", "Ptr", this.pointer, "Int64")
    }

    static parse(time_string) {
        span_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_parse", "WStr", time_string, "Ptr", span_out, "Int64")

        if (retcode = 0) {
            handle := NumGet(span_out, 0, "Ptr")
        } else {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }

        if (handle = 0) {
            throw "unexpected error"
        }

        return Span(handle)
    }
    static new() {
        pointer := DllCall("tempus_ahk\span_new", "Ptr")
        return Span(pointer)
    }
    days(days) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_days", "Ptr", this.pointer, "Int64", days, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    hours(hours) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_hours", "Ptr", this.pointer, "Int64", hours, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    seconds(seconds) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_seconds", "Ptr", this.pointer, "Int64", seconds, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    milliseconds(milliseconds) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_milliseconds", "Ptr", this.pointer, "Int64", milliseconds, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    microseconds(microseconds) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_microseconds", "Ptr", this.pointer, "Int64", microseconds, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    nanoseconds(nanoseconds) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_nanoseconds", "Ptr", this.pointer, "Int64", nanoseconds, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    weeks(weeks) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_weeks", "Ptr", this.pointer, "Int64", weeks, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message) -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    months(months) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_months", "Ptr", this.pointer, "Int64", months, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message) -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    years(years) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_years", "Ptr", this.pointer, "Int64", years, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message) -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    minutes(minutes) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_minutes", "Ptr", this.pointer, "Int64", minutes, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message) -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }
    get_years() {
        return DllCall("tempus_ahk\span_get_years", "Ptr", this.pointer, "Int64")
    }
    get_months() {
        return DllCall("tempus_ahk\span_get_months", "Ptr", this.pointer, "Int64")
    }
    get_weeks() {
        return DllCall("tempus_ahk\span_get_weeks", "Ptr", this.pointer, "Int64")
    }
    get_days() {
        return DllCall("tempus_ahk\span_get_days", "Ptr", this.pointer, "Int64")
    }
    get_hours() {
        return DllCall("tempus_ahk\span_get_hours", "Ptr", this.pointer, "Int64")
    }
    get_minutes() {
        return DllCall("tempus_ahk\span_get_minutes", "Ptr", this.pointer, "Int64")
    }
    get_seconds() {
        return DllCall("tempus_ahk\span_get_seconds", "Ptr", this.pointer, "Int64")
    }
    get_milliseconds() {
        return DllCall("tempus_ahk\span_get_milliseconds", "Ptr", this.pointer, "Int64")
    }
    get_microseconds() {
        return DllCall("tempus_ahk\span_get_microseconds", "Ptr", this.pointer, "Int64")
    }
    get_nanoseconds() {
        return DllCall("tempus_ahk\span_get_nanoseconds", "Ptr", this.pointer, "Int64")
    }

    to_string() {
        buff_length := DllCall("tempus_ahk\span_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\span_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    to_string_friendly() {
        buff_length := DllCall("tempus_ahk\span_string_length_friendly", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\span_to_string_friendly", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string_friendly()
    }

    abs() {
        pointer := DllCall("tempus_ahk\span_abs", "Ptr", this.pointer, "Ptr")
        return Span(pointer)
    }
    negate() {
        pointer := DllCall("tempus_ahk\span_negate", "Ptr", this.pointer, "Ptr")
        return Span(pointer)
    }
    is_negative() {
        return DllCall("tempus_ahk\span_is_negative", "Ptr", this.pointer, "Char")
    }
    is_positive() {
        return DllCall("tempus_ahk\span_is_positive", "Ptr", this.pointer, "Char")
    }
    is_zero() {
        return DllCall("tempus_ahk\span_is_zero", "Ptr", this.pointer, "Char")
    }
    signum() {
        return DllCall("tempus_ahk\span_signum", "Ptr", this.pointer, "Char")
    }

    checked_add(other, days_are_24_hours := false) {
        out_span := Buffer(A_PtrSize)
        if (other is Span) {
           retcode := DllCall("tempus_ahk\span_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Char", days_are_24_hours, "Ptr", out_span, "Int64")
        } else {
            throw Error("add is only currently supported with other Span types", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    checked_sub(other, days_are_24_hours := false) {
        out_span := Buffer(A_PtrSize)
        if (other is Span) {
           retcode := DllCall("tempus_ahk\span_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Char", days_are_24_hours, "Ptr", out_span, "Int64")
        } else {
            throw Error("sub is only currently supported with other Span types", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    checked_mul(i) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_checked_mul", "Ptr", this.pointer, "Int64", i, "Ptr", out_span, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    compare(other_span, relative_options := -1) {
        if !(other_span is Span) {
            throw Error("Unsupported type. Can only compare to another Span", -2)
        }

        if (relative_options == 0 || relative_options == 1) {
            days_are_24_hours := relative_options
        } else {
            days_are_24_hours := false
        }
        if (relative_options is Date) {
            retcode := DllCall("tempus_ahk\span_compare_relative_to_date", "Ptr", this.pointer, "Ptr", other_span.pointer, "Ptr", relative_options.pointer, "Char")
        } else if (relative_options is DateTime) {
            retcode := DllCall("tempus_ahk\span_compare_relative_to_datetime", "Ptr", this.pointer, "Ptr", other_span.pointer, "Ptr", relative_options.pointer, "Char")
        } else if (relative_options is Zoned) {
            retcode := DllCall("tempus_ahk\span_compare_relative_to_zoned", "Ptr", this.pointer, "Ptr", other_span.pointer, "Ptr", relative_options.pointer, "Char")
        } else {
            retcode := DllCall("tempus_ahk\span_compare", "Ptr", this.pointer, "Ptr", other_span.pointer, "Char", days_are_24_hours, "Char")
        }

        if (retcode < -1) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        return retcode

    }

    gt(other_span, relative_options := -1) {
        result := this.compare(other_span, relative_options)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_span, relative_options := -1) {
        result := this.compare(other_span, relative_options)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_span, relative_options := -1) {
        result := this.compare(other_span, relative_options)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_span, relative_options := -1) {
        result := this.compare(other_span, relative_options)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_span, relative_options := -1) {
        result := this.compare(other_span, relative_options)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    total(unit, relative_options := -1) {
        if (relative_options == 0 || relative_options == 1) {
            days_are_24_hours := relative_options
        } else {
            days_are_24_hours := false
        }
        if (relative_options is Date) {
            retcode := DllCall("tempus_ahk\span_total_relative_to_date", "Ptr", this.pointer, "Char", unit, "Ptr", relative_options.pointer, "DoubleP", &out_buff:=0.0, "Int64")
        } else if (relative_options is DateTime) {
            retcode := DllCall("tempus_ahk\span_total_relative_to_datetime", "Ptr", this.pointer, "Char", unit, "Ptr", relative_options.pointer, "DoubleP", &out_buff:=0.0, "Int64")
        } else if (relative_options is Zoned) {
            retcode := DllCall("tempus_ahk\span_total_relative_to_zoned", "Ptr", this.pointer, "Char", unit, "Ptr", relative_options.pointer, "DoubleP", &out_buff:=0.0, "Int64")
        } else {
            retcode := DllCall("tempus_ahk\span_total", "Ptr", this.pointer, "Char", unit, "Char", days_are_24_hours, "DoubleP", &out_buff:=0.0, "Int64")
        }

        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        } else {
            return out_buff
        }
    }

    round(smallest := -1, increment := 1, largest := -1, round_mode := RoundMode.HalfExpand, relative_options := -1) {
        if (relative_options == 0 || relative_options == 1) {
            days_are_24_hours := relative_options
        } else {
            days_are_24_hours := false
        }
        out_span := Buffer(A_PtrSize)
        if (relative_options is Date) {
            retcode := DllCall("tempus_ahk\span_round_relative_to_date", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", largest, "Char", round_mode, "Ptr", relative_options.pointer, "Ptr", out_span, "Int64")
        } else if (relative_options is DateTime) {
            retcode := DllCall("tempus_ahk\span_round_relative_to_datetime", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", largest, "Char", round_mode, "Ptr", relative_options.pointer, "Ptr", out_span, "Int64")
        } else if (relative_options is Zoned) {
            retcode := DllCall("tempus_ahk\span_round_relative_to_zoned", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", largest, "Char", round_mode, "Ptr", relative_options.pointer, "Ptr", out_span, "Int64")
        } else {
            retcode := DllCall("tempus_ahk\span_round", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", largest, "Char", round_mode, "Char", days_are_24_hours, "Ptr", out_span, "Int64")
        }

        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }

        return Span(handle)
    }
}

class Timezone {
    __New(pointer) {
        this.pointer := pointer
    }
    __Delete() {
        DllCall("tempus_ahk\free_timezone", "Ptr", this.pointer, "Int64")
    }

    static system() {
        pointer := DllCall("tempus_ahk\timezone_system", "Ptr")
        return Timezone(pointer)
    }

    static get(timezone_name) {
        out_tz := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timezone_get", "WStr", timezone_name, "Ptr", out_tz, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_tz, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Timezone(handle)
    }

    static posix(posix_tz_name) {
        out_tz := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timezone_posix", "WStr", posix_tz_name, "Ptr", out_tz, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_tz, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }

        return Timezone(handle)
    }

    static UTC() {
        pointer := DllCall("tempus_ahk\timezone_utc", "Ptr")
        return Timezone(pointer)
    }

    static unknown() {
        pointer := DllCall("tempus_ahk\timezone_unknown", "Ptr")
        return Timezone(pointer)
    }

}

class Date {
    __New(pointer) {
        this.pointer := pointer
    }
    __Delete() {
        DllCall("tempus_ahk\free_date", "Ptr", this.pointer, "Int64")
    }
    static MIN() {
        pointer := DllCall("tempus_ahk\date_min", "Ptr")
        return Date(pointer)
    }

    static MAX() {
        pointer := DllCall("tempus_ahk\date_max", "Ptr")
        return Date(pointer)
    }

    static ZERO() {
        pointer := DllCall("tempus_ahk\date_zero", "Ptr")
        return Date(pointer)
    }

    static new(year := 1970, month := 1, day := 1) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_new", "Short", year, "Char", month, "Char", day, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    static from_iso_weekdate(isoweek) {
        if !(isoweek is ISOWeekDate) {
            throw Error("Unsupported Type. Must be ISOWeekDate", -2)
        }
        pointer := DllCall("tempus_ahk\date_from_isoweekdate", "Ptr", isoweek.pointer, "Ptr")
        return Date(pointer)
    }


    static parse(date_string) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_parse", "WStr", date_string, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    at(hour := 0, minute := 0, second := 0, subsec_nanosecond := 0) {
        return DateTime.new(this.year(), this.month(), this.day(), hour, minute, second, subsec_nanosecond)
    }

    strftime(format_str) {
        buff_length := DllCall("tempus_ahk\date_strftime_length", "Ptr", this.pointer, "WStr", format_str, "Int64")
        if buff_length < 0 {
            error_code := buff_length
            if (error_code = -2 || error_code = -3) {
                message := _get_last_error()
                throw Error(Format("error {}", message), -2)
            }
            else {
                throw "unexpected error getting buff length"
            }
        }
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\date_strftime", "Ptr", this.pointer, "WStr", format_str, "Ptr", buff, "UInt64", buff.Size, "Int64")
        if (retcode = 0) {
            ret := StrGet(buff, "UTF-8")
            return ret
        } else {
            message := _get_last_error()
            throw Error(Format("error: {}", message), -2)
        }
    }

    static strptime(format_str, time_str) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_strptime", "WStr", format_str, "WStr", time_str, "Ptr", out_date)
        if (retcode < 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    to_string() {
        buff_length := DllCall("tempus_ahk\date_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\date_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size, "Int64")
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string()
    }

    compare(other_date) {
        if !(other_date is Date) {
            throw Error("argument must be a Date", -2)
        }
        return DllCall("tempus_ahk\date_compare", "Ptr", this.pointer, "Ptr", other_date.pointer, "Char")
    }

    gt(other_date) {
        result := this.compare(other_date)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_date) {
        result := this.compare(other_date)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_date) {
        result := this.compare(other_date)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_date) {
        result := this.compare(other_date)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_date) {
        result := this.compare(other_date)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    year() {
        return DllCall("tempus_ahk\date_year", "Ptr", this.pointer, "Short")
    }
    month() {
        return DllCall("tempus_ahk\date_month", "Ptr", this.pointer, "Char")
    }
    date() {
        return DllCall("tempus_ahk\date_day", "Ptr", this.pointer, "Char")
    }

    era() {
        ret := DllCall("tempus_ahk\date_era", "Ptr", this.pointer, "Char")
        if (ret = -1) {
            return "BCE"
        } else if (ret = 1) {
            return "CE"
        } else {
            throw "unexpected error"
        }
    }

    era_year() {
        return DllCall("tempus_ahk\date_era_year", "Ptr", this.pointer, "Short")
    }

    weekday() {
        return DllCall("tempus_ahk\date_weekday", "Ptr", this.pointer, "Char")
    }

    day_of_year() {
        return DllCall("tempus_ahk\date_day_of_year", "Ptr", this.pointer, "Short")
    }

    day_of_year_no_leap() {
        ret := DllCall("tempus_ahk\date_day_of_year_no_leap", "Ptr", this.pointer, "Short")
        if (ret = -1) {
            return
        } else {
            return ret
        }
    }

    first_of_month() {
        pointer := DllCall("tempus_ahk\date_first_of_month", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }
    last_of_month() {
        pointer := DllCall("tempus_ahk\date_last_of_month", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }
    first_of_year() {
        pointer := DllCall("tempus_ahk\date_first_of_year", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }
    last_of_year() {
        pointer := DllCall("tempus_ahk\date_last_of_year", "Ptr", this.pointer, "Ptr")
        return Date(pointer)
    }

    days_in_month() {
        return DllCall("tempus_ahk\date_days_in_month", "Ptr", this.pointer, "Char")
    }

    days_in_year() {
        return DllCall("tempus_ahk\date_days_in_year", "Ptr", this.pointer, "Short")
    }

    in_leap_year() {
        ret := DllCall("tempus_ahk\date_in_leap_year", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }
    tomorrow() {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_tomorrow", "Ptr", this.pointer, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }
    yesterday() {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_yesterday", "Ptr", this.pointer, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    nth_weekday_of_month(nth, weekday_i) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_nth_weekday_of_month", "Ptr", this.pointer, "Char", nth, "Char", weekday_i, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    nth_weekday(nth, weekday_i) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_nth_weekday", "Ptr", this.pointer, "Int", nth, "Char", weekday_i, "Ptr", out_date, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    to_isoweekdate() {
        pointer := DllCall("tempus_ahk\date_to_isoweekdate", "Ptr", this.pointer)
        return ISOWeekDate(pointer)
    }

    in_tz(time_zone_name) {
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_in_tz", "Ptr", this.pointer, "WStr", time_zone_name, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    to_zoned(tz) {
        if !(tz is Timezone) {
            throw Error("unsupported type. Must be Timezone", -2)
        }
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_to_zoned", "Ptr", this.pointer, "Ptr", tz.pointer, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    to_datetime(t) {
        if !(t is Time) {
            throw Error("unsupported type. Must be Time", -2)
        }
        pointer := DllCall("tempus_ahk\date_to_datetime", "Ptr", this,pointer, "Ptr", t.pointer, "Ptr")
        return DateTime(pointer)
    }

    checked_add(other) {
        out_date := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\date_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_date, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\date_checked_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_date, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    checked_sub(other) {
        out_date := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\date_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_date, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\date_checked_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_date, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Date(handle)
    }

    saturating_sub(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\date_saturating_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\date_saturating_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Date(pointer)
    }


    saturating_add(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\date_saturating_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\date_saturating_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Date(pointer)
    }
    until_date(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Date) {
            retcode := DllCall("tempus_ahk\date_until_date", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\date_until_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    since(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Date) {
            retcode := DllCall("tempus_ahk\date_since_date", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\date_since_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    duration_until(other_date) {
        if !(other_date is Date) {
            throw Error("Unsupported Type. Must be Date", -2)
        }
        pointer := DllCall("tempus_ahk\date_duration_until", "Ptr", this.pointer, "Ptr", other_date.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    duration_since(other_date) {
        if !(other_date is Date) {
            throw Error("Unsupported Type. Must be Date", -2)
        }
        pointer := DllCall("tempus_ahk\date_duration_since", "Ptr", this.pointer, "Ptr", other_date.pointer, "Ptr")
        return SignedDuration(pointer)
    }
    series(span_interval) {
        ; TODO: support creating span from string arg?
        if !(span_interval is Span) {
            throw Error("Unsupported Type. Must be a Span type")
        }
        pointer := DllCall("tempus_ahk\date_series", "Ptr", this.pointer, "Ptr", span_interval.pointer, "Ptr")
        return DateSeries(pointer)
    }
}


class DateTime {
    __New(pointer) {
        this.pointer := pointer
    }
    __Delete() {
        DllCall("tempus_ahk\free_datetime", "Ptr", this.pointer, "Int64")
    }
    static MIN() {
        pointer := DllCall("tempus_ahk\datetime_min", "Ptr")
        return DateTime(pointer)
    }

    static MAX() {
        pointer := DllCall("tempus_ahk\datetime_max", "Ptr")
        return DateTime(pointer)
    }

    static ZERO() {
        pointer := DllCall("tempus_ahk\datetime_zero", "Ptr")
        return DateTime(pointer)
    }

    static new(year := 1970, month := 1, day := 1, hour := 0, minute := 0, second := 0, subsec_nanosecond := 0) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_new", "Short", year, "Char", month, "Char", day, "Char", hour, "Char", minute, "Char", second, "Int", subsec_nanosecond, "Ptr", out_datetime, "Int64")
        if (retcode := 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    static parse(date_string) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_parse", "WStr", date_string, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    static from_parts(date_part, time_part) {
        if !(date_part is Date) {
            throw Error("Unsupported type for date_part. Must be Date")
        }
        if !(time_part is Time) {
            throw Error("Unsupported type for time_part. Must be Time")
        }
        pointer := DllCall("tempus_ahk\datetime_from_parts", "Ptr", date_part.pointer, "Ptr", time_part.pointer, "Ptr")
        return DateTime(pointer)
    }

    strftime(format_str) {
        buff_length := DllCall("tempus_ahk\datetime_strftime_length", "Ptr", this.pointer, "WStr", format_str, "Int64")
        if buff_length < 0 {
            error_code := buff_length
            if (error_code = -2 || error_code = -3) {
                message := _get_last_error()
                throw Error(Format("error {}", message), -2)
            }
            else {
                throw "unexpected error getting buff length"
            }
        }
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\datetime_strftime", "Ptr", this.pointer, "WStr", format_str, "Ptr", buff, "UInt64", buff.Size, "Int64")
        if (retcode = 0) {
            ret := StrGet(buff, "UTF-8")
            return ret
        } else {
            message := _get_last_error()
            throw Error(Format("error: {}", message), -2)
        }
    }

    static strptime(format_str, time_str) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_strptime", "WStr", format_str, "WStr", time_str, "Ptr", out_date)
        if (retcode < 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_date, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }
    to_string() {
        buff_length := DllCall("tempus_ahk\datetime_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\datetime_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size, "Int64")
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        this.to_string()
    }

    year() {
        return DllCall("tempus_ahk\datetime_year", "Ptr", this.pointer, "Short")
    }
    month() {
        return DllCall("tempus_ahk\datetime_month", "Ptr", this.pointer, "Char")
    }
    day() {
        return DllCall("tempus_ahk\datetime_day", "Ptr", this.pointer, "Char")
    }
    hour() {
        return DllCall("tempus_ahk\datetime_hour", "Ptr", this.pointer, "Char")
    }
    minute() {
        return DllCall("tempus_ahk\datetime_minute", "Ptr", this.pointer, "Char")
    }
    second() {
        return DllCall("tempus_ahk\datetime_second", "Ptr", this.pointer, "Char")
    }
    millisecond() {
        return DllCall("tempus_ahk\datetime_millisecond", "Ptr", this.pointer, "Short")
    }
    microsecond() {
        return DllCall("tempus_ahk\datetime_microsecond", "Ptr", this.pointer, "Short")
    }
    nanosecond() {
        return DllCall("tempus_ahk\datetime_nanosecond", "Ptr", this.pointer, "Short")
    }
    subsec_nanosecond() {
        return DllCall("tempus_ahk\datetime_subsec_nanosecond", "Ptr", this.pointer, "Int")
    }

    era() {
        ret := DllCall("tempus_ahk\datetime_era", "Ptr", this.pointer, "Char")
        if (ret = -1) {
            return "BCE"
        } else if (ret = 1) {
            return "CE"
        } else {
            throw "unexpected error"
        }
    }

    era_year() {
        return DllCall("tempus_ahk\datetime_era_year", "Ptr", this.pointer, "Short")
    }

    weekday() {
        return DllCall("tempus_ahk\datetime_weekday", "Ptr", this.pointer, "Short")
    }

    day_of_year() {
        return DllCall("tempus_ahk\datetime_day_of_year", "Ptr", this.pointer, "Short")
    }

    day_of_year_no_leap() {
        ret := DllCall("tempus_ahk\datetime_day_of_year_no_leap", "Ptr", this.pointer, "Short")
        if (ret = -1) {
            return
        } else {
            return ret
        }
    }

    first_of_month() {
        pointer := DllCall("tempus_ahk\datetime_first_of_month", "Ptr", this.pointer, "Ptr")
        return DateTime(pointer)
    }
    last_of_month() {
        pointer := DllCall("tempus_ahk\datetime_last_of_month", "Ptr", this.pointer, "Ptr")
        return DateTime(pointer)
    }
    first_of_year() {
        pointer := DllCall("tempus_ahk\datetime_first_of_year", "Ptr", this.pointer, "Ptr")
        return DateTime(pointer)
    }
    last_of_year() {
        pointer := DllCall("tempus_ahk\datetime_last_of_year", "Ptr", this.pointer, "Ptr")
        return DateTime(pointer)
    }

    days_in_month() {
        return DllCall("tempus_ahk\datetime_days_in_month", "Ptr", this.pointer, "Char")
    }

    days_in_year() {
        return DllCall("tempus_ahk\datetime_days_in_year", "Ptr", this.pointer, "Short")
    }

    in_leap_year() {
        ret := DllCall("tempus_ahk\datetime_in_leap_year", "Ptr", this.pointer, "Char")
        if (ret = 1) {
            return true
        } else if (ret = 0) {
            return false
        } else {
            throw "unexpected error"
        }
    }
    tomorrow() {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_tomorrow", "Ptr", this.pointer, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }
    yesterday() {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_yesterday", "Ptr", this.pointer, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    nth_weekday_of_month(nth, weekday_i) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_nth_weekday_of_month", "Ptr", this.pointer, "Char", nth, "Char", weekday_i, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    nth_weekday(nth, weekday_i) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_nth_weekday", "Ptr", this.pointer, "Int", nth, "Char", weekday_i, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    start_of_day() {
        pointer := DllCall("tempus_ahk\datetime_start_of_day")
        return DateTime(pointer)
    }
    end_of_day() {
        pointer := DllCall("tempus_ahk\datetime_end_of_day")
        return DateTime(pointer)
    }

    to_isoweekdate() {
        pointer := DllCall("tempus_ahk\datetime_to_isoweekdate", "Ptr", this.pointer)
        return ISOWeekDate(pointer)
    }

    to_date() {
        pointer := DllCall("tempus_ahk\datetime_to_date", "Ptr", this.pointer)
        return Date(pointer)
    }

    to_time() {
        pointer := DllCall("tempus_ahk\datetime_to_time", "Ptr", this.pointer)
        return Time(pointer)
    }

    to_zoned(tz) {
        if !(tz is Timezone) {
            throw Error("unsupported type. Must be Timezone", -2)
        }
        out_zoned := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_to_zoned", "Ptr", this.pointer, "Ptr", tz.pointer, "Ptr", out_zoned, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message))
        }
        handle := NumGet(out_zoned, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)
    }

    checked_add(other) {
        out_datetime := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\datetime_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_datetime, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\datetime_checked_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_datetime, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    checked_sub(other) {
        out_datetime := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\datetime_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_datetime, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\datetime_checked_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_datetime, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }
    saturating_sub(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\datetime_saturating_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\datetime_saturating_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return DateTime(pointer)
    }

    saturating_add(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\datetime_saturating_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\datetime_saturating_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return DateTime(pointer)
    }

    until_date(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Date) {
            retcode := DllCall("tempus_ahk\datetime_until_date", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\datetime_until_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    since(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Date) {
            retcode := DllCall("tempus_ahk\datetime_since_date", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\datetime_since_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    duration_until(other_date) {
        if !(other_date is DateTime) {
            throw Error("Unsupported Type. Must be Date", -2)
        }
        pointer := DllCall("tempus_ahk\datetime_duration_until", "Ptr", this.pointer, "Ptr", other_date.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    duration_since(other_date) {
        if !(other_date is DateTime) {
            throw Error("Unsupported Type. Must be Date", -2)
        }
        pointer := DllCall("tempus_ahk\datetime_duration_since", "Ptr", this.pointer, "Ptr", other_date.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    compare(other_time) {
        if !(other_time is DateTime) {
            throw Error("argument must be a DateTime", -2)
        }
        return DllCall("tempus_ahk\datetime_compare", "Ptr", this.pointer, "Ptr", other_time.pointer, "Char")
    }

    gt(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    round(smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_round", "Ptr", this.pointer, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_datetime, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_datetime, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return DateTime(handle)
    }

    series(span_interval) {
        ; TODO: support creating span from string arg?
        if !(span_interval is Span) {
            throw Error("Unsupported Type. Must be a Span type")
        }
        pointer := DllCall("tempus_ahk\datetime_series", "Ptr", this.pointer, "Ptr", span_interval.pointer, "Ptr")
        return DateTimeSeries(pointer)
    }

}

class TimeSeries {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_time_series", "Ptr", this.pointer, "Int64")
    }

    Call(&t) {
        out_time := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\time_series_next", "Ptr", this.pointer, "Ptr", out_time, "Char")
        if (retcode != 0) {
            return false
        } else {
            handle := NumGet(out_time, 0, "Ptr")
            if (handle = 0) {
                throw "unexpected error"
            }
            t := Time(handle)
            return true
        }
    }
}


class DateSeries {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_date_series", "Ptr", this.pointer, "Int64")
    }

    Call(&d) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\date_series_next", "Ptr", this.pointer, "Ptr", out_date, "Char")
        if (retcode != 0) {
            return false
        } else {
            handle := NumGet(out_date, 0, "Ptr")
            if (handle = 0) {
                throw "unexpected error"
            }
            d := Date(handle)
            return true
        }
    }
}

class DateTimeSeries {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_datetime_series", "Ptr", this.pointer, "Int64")
    }

    Call(&dt) {
        out_datetime := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\datetime_series_next", "Ptr", this.pointer, "Ptr", out_datetime, "Char")
        if (retcode != 0) {
            return false
        } else {
            handle := NumGet(out_datetime, 0, "Ptr")
            if (handle = 0) {
                throw "unexpected error"
            }
            dt := DateTime(handle)
            return true
        }
    }
}

class TimestampSeries {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_timestamp_series", "Ptr", this.pointer, "Int64")
    }

    Call(&ts) {
        out_date := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_series_next", "Ptr", this.pointer, "Ptr", out_date, "Char")
        if (retcode != 0) {
            return false
        } else {
            handle := NumGet(out_date, 0, "Ptr")
            if (handle = 0) {
                throw "unexpected error"
            }
            ts := DateTime(handle)
            return true
        }
    }
}


class Time {
    __New(pointer) {
        this.pointer := pointer
    }
    __Delete() {
        DllCall("tempus_ahk\free_time", "Ptr", this.pointer, "Int64")
    }

    static MIN() {
        pointer := DllCall("tempus_ahk\time_min", "Ptr")
        return Time(pointer)
    }

    static MAX() {
        pointer := DllCall("tempus_ahk\time_max", "Ptr")
        return Time(pointer)
    }

    static midnight() {
        pointer := DllCall("tempus_ahk\time_midnight", "Ptr")
        return Time(pointer)
    }

    static new(hour := 0, minute := 0, second := 0, subsec_nano := 0) {
        out_time := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\time_new", "Char", hour, "Char", minute, "Char", second, "Int", subsec_nano, "Ptr", out_time, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_time, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Time(handle)
    }

    static parse(time_string) {
        out_time := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\time_parse", "WStr", time_string, "Ptr", out_time, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_time, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Time(handle)
    }

    hour() {
        return DllCall("tempus_ahk\time_hour", "Ptr", this.pointer, "Char")
    }
    minute() {
        return DllCall("tempus_ahk\time_minute", "Ptr", this.pointer, "Char")
    }
    second() {
        return DllCall("tempus_ahk\time_second", "Ptr", this.pointer, "Char")
    }

    millisecond() {
        return DllCall("tempus_ahk\time_millisecond", "Ptr", this.pointer, "Short")
    }
    microsecond() {
        return DllCall("tempus_ahk\time_microsecond", "Ptr", this.pointer, "Short")
    }
    nanosecond() {
        return DllCall("tempus_ahk\time_nanosecond", "Ptr", this.pointer, "Short")
    }
    subsec_nanosecond() {
        return DllCall("tempus_ahk\time_subsec_nanosecond", "Ptr", this.pointer, "Int")
    }

    to_string() {
        buff_length := DllCall("tempus_ahk\time_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\time_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size, "Int64")
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    ToString() {
        return this.to_string()
    }


    compare(other_time) {
        if !(other_time is Time) {
            throw Error("argument must be a Time", -2)
        }
        return DllCall("tempus_ahk\time_compare", "Ptr", this.pointer, "Ptr", other_time.pointer, "Char")
    }

    gt(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_time) {
        result := this.compare(other_time)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    checked_add(other) {
        out_time := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\time_checked_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_time, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\time_checked_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_time, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_time, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Time(handle)
    }

    checked_sub(other) {
        out_time := Buffer(A_PtrSize)
        if (other is Span) {
            retcode := DllCall("tempus_ahk\time_checked_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_time, "Int64")
        } else if (other is SignedDuration) {
            retcode := DllCall("tempus_ahk\time_checked_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr", out_time, "Int64")
        } else {
            throw Error("Unsupported type. Must be Span or SignedDuration", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_time, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Time(handle)
    }

    saturating_sub(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\time_saturating_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\time_saturating_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Time(pointer)
    }


    saturating_add(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\time_saturating_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\time_saturating_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Time(pointer)
    }

    wrapping_add(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\time_wrapping_add_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\time_wrapping_add_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Time(pointer)
    }

    wrapping_sub(other) {
        if (other is Span) {
            pointer := DllCall("tempus_ahk\time_wrapping_sub_span", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else if (other is SignedDuration) {
            pointer := DllCall("tempus_ahk\time_wrapping_sub_signed_duration", "Ptr", this.pointer, "Ptr", other.pointer, "Ptr")
        } else {
            throw Error("Unsupported Type. Must be Span or SignedDuration")
        }
        return Time(pointer)
    }

    add(other) {
        return this.wrapping_add(other)
    }

    sub(other) {
        return this.wrapping_sub(other)
    }

    until_time(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Time) {
            retcode := DllCall("tempus_ahk\time_until_time", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\time_until_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    since(other, largest_unit := -1, smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        if (other is Time) {
            retcode := DllCall("tempus_ahk\time_since_time", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else if (other is DateTime) {
            retcode := DllCall("tempus_ahk\time_since_datetime", "Ptr", this.pointer, "Ptr", other.pointer, "Char", largest_unit, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_span, "Int64")
        } else {
            throw Error("Unsupported Type. Must be Time or DateTime", -2)
        }
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_span, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Span(handle)
    }

    duration_until(other_time) {
        if !(other_time is Time) {
            throw Error("Unsupported Type. Must be Time", -2)
        }
        pointer := DllCall("tempus_ahk\time_duration_until", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    duration_since(other_time) {
        if !(other_time is Time) {
            throw Error("Unsupported Type. Must be Time", -2)
        }
        pointer := DllCall("tempus_ahk\time_duration_since", "Ptr", this.pointer, "Ptr", other_time.pointer, "Ptr")
        return SignedDuration(pointer)
    }

    on(year, month, day) {
        return DateTime.new(year, month, day, this.hour(), this.minute(), this.second(), this.subsec_nanosecond())
    }

    to_datetime(to_date) {
        if !(to_date is Date) {
            throw Error("Unsupported Type. Must be Date", -2)
        }
        return DateTime.new(date.year(), date.month(), date.day(), this.hour(), this.minute(), this.second(), this.subsec_nanosecond())
    }

    round(smallest_unit := -1, increment := 1, round_mode := RoundMode.HalfExpand) {
        out_time := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\time_round", "Ptr", this.pointer, "Char", smallest_unit, "Int64", increment, "Char", round_mode, "Ptr", out_time, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        }
        handle := NumGet(out_time, 0, "Ptr")
        if (handle = 0) {
            throw "unexpected error"
        }
        return Time(handle)
    }

    series(span_interval) {
        ; TODO: support creating span from string arg?
        if !(span_interval is Span) {
            throw Error("Unsupported Type. Must be a Span type")
        }
        pointer := DllCall("tempus_ahk\time_series", "Ptr", this.pointer, "Ptr", span_interval.pointer, "Ptr")
        return TimeSeries(pointer)
    }
}

