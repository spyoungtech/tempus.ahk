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


class SignedDuration {
    __New(pointer) {
        this.pointer := pointer
    }

    __Delete() {
        DllCall("tempus_ahk\free_signed_duration", "Ptr", this.pointer, "Int64")
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


    static parse(time_string) {
        ts_out := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_parse", "WStr", time_string, "Ptr", ts_out, "Int64")

        if (retcode = 0) {
            handle := NumGet(ts_out, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("error {}", message), -2)
        } else {
            throw "Unexpected error"
        }

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

    static from_second(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_second", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode = 0) {
            handle := NumGet(out_ts, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("Error: {}", message), -2)
        } else {
            throw "unexpected error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }

        return Timestamp(handle)

    }
    static from_millisecond(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_millisecond", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode = 0) {
            handle := NumGet(out_ts, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("Error: {}", message), -2)
        } else {
            throw "unexpected error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }

        return Timestamp(handle)

    }
    static from_microsecond(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_from_microsecond", "Int64", s, "Ptr", out_ts, "Int64")
        if (retcode = 0) {
            handle := NumGet(out_ts, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("Error: {}", message), -2)
        } else {
            throw "unexpected error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }

        return Timestamp(handle)

    }

    in_tz(timezone) {
        zoned_ptr := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\timestamp_parse", "WStr", timezone, "Ptr", this.pointer, "Ptr", zoned_ptr, "Int64")
        if (retcode = 0) {
            handle := NumGet(zoned_ptr, 0, "Ptr")
        } else if (retcode = -2) {
            message := _get_last_error()
            throw Error(Format("error {}", message), -2)
        } else {
            throw "Unexpected error"
        }

        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)

    }

    to_string() {
        buff_length := DllCall("tempus_ahk\timestamp_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := DllCall("tempus_ahk\timestamp_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }

    strftime(format_str) {
        buff_length := DllCall("tempus_ahk\timestamp_strftime_length", "Ptr", this.pointer, "WStr", format_str, "Int64")
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
        retcode := DllCall("tempus_ahk\timestamp_strftime", "Ptr", this.pointer, "WStr", format_str, "Ptr", buff, "UInt64", buff.Size, "Int64")
        if (retcode = 0) {
            ret := StrGet(buff, "UTF-8")
            return ret
        } else {
            message := _get_last_error()
            throw Error(Format("error: {}", message), -2)
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

    static UNIX_EPOCH() {
        return Timestamp.from_second(0)
    }

    is_zero() {
        ret := DllCall("tempus_ahk\timestamp_is_zero", "Ptr", this.pointer, "Char")
        if ret {
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

    compare(other_span, days_are_24_hours := false) {
        if other_span is Span {
            retcode := DllCall("tempus_ahk\span_compare", "Ptr", this.pointer, "Ptr", other_span.pointer, "Char", days_are_24_hours, "Char")
            if (retcode < -1) {
                message := _get_last_error()
                throw Error(Format("error({}): {}", retcode, message), -2)
            }
            return retcode
        } else {
            throw Error("Only spans can be compared with spans", -2)
        }
    }

    gt(other_span, days_are_24_hours := false) {
        result := this.compare(other_span, days_are_24_hours)
        if (result = _Ordering.GREATER) {
            return true
        }  else {
            return false
        }
    }

    lt(other_span, days_are_24_hours := false) {
        result := this.compare(other_span, days_are_24_hours)
        if (result = _Ordering.LESS) {
            return true
        } else {
            return false
        }
    }

    eq(other_span, days_are_24_hours := false) {
        result := this.compare(other_span, days_are_24_hours)
        if (result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    gte(other_span, days_are_24_hours := false) {
        result := this.compare(other_span, days_are_24_hours)
        if (result = _Ordering.GREATER || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    lte(other_span, days_are_24_hours := false) {
        result := this.compare(other_span, days_are_24_hours)
        if (result = _Ordering.LESS || result = _Ordering.EQUAL) {
            return true
        } else {
            return false
        }
    }

    total(unit, days_are_24_hours := false) {
        retcode := DllCall("tempus_ahk\span_total", "Ptr", this.pointer, "Char", unit, "Char", days_are_24_hours, "DoubleP", &out_buff:=0.0, "Int64")
        if (retcode != 0) {
            message := _get_last_error()
            throw Error(Format("error({}): {}", retcode, message), -2)
        } else {
            return out_buff
        }
    }

    round(smallest := -1, increment := 1, largest := -1, round_mode := RoundMode.HalfExpand) {
        out_span := Buffer(A_PtrSize)
        retcode := DllCall("tempus_ahk\span_round", "Ptr", this.pointer, "Char", smallest, "Int64", increment, "Char", largest, "Char", round_mode, "Ptr", out_span, "Int64")
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

