; the DLL is expected to be on PATH somewhere... Not sure if there's a better way to do this than to trust the user
; to put it in the right place.
#DllLoad "*i tempus_ahk"

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

