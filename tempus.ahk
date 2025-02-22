#DllLoad target\Debug\tempus_ahk.dll


_TempusCall(func_name, args*) {
    return DllCall("target\Debug\tempus_ahk.dll\" . func_name, args*)
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
    length := _TempusCall("get_last_error_length", "UInt")
    MsgBox(length)
    if (length > 0)
    {
        ; Allocate a buffer of length+1 for the null terminator
        buff := Buffer(length + 1, 0)

        success := _TempusCall("get_last_error"
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
        _TempusCall("free_signed_duration", "Ptr", this.pointer, "Int64")
    }

    static parse(duration_string) {
        duration_out := Buffer(A_PtrSize)
        retcode := _TempusCall("signed_duration_parse", "WStr", duration_string, "Ptr", duration_out, "Int64")

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
        _TempusCall("free_zoned", "Ptr", this.pointer, "Int64")
    }

    static now() {
        ptr := _TempusCall("zoned_now", "Ptr")
        return Zoned(ptr)
    }

    static parse(time_string) {
        ts_out := Buffer(A_PtrSize)
        retcode := _TempusCall("zoned_parse", "WStr", time_string, "Ptr", ts_out, "Int64")

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
        _TempusCall("free_timestamp", "Ptr", this.pointer, "Int64")
    }

    static now() {
        ptr := _TempusCall("timestamp_now", "Ptr")
        return TimeStamp(ptr)
    }


    static parse(time_string) {
        ts_out := Buffer(A_PtrSize)
        retcode := _TempusCall("timestamp_parse", "WStr", time_string, "Ptr", ts_out, "Int64")

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
        return TimeStamp(handle)
    }

    as_millisecond() {
        return _TempusCall("timestamp_as_millisecond", "Ptr", this.pointer, "Int64")
    }

    as_microsecond() {
        return _TempusCall("timestamp_as_microsecond", "Ptr", this.pointer, "Int64")
    }

    as_second() {
        return _TempusCall("timestamp_as_second", "Ptr", this.pointer, "Int64")
    }

    static from_second(s) {
        out_ts := Buffer(A_PtrSize)
        retcode := _TempusCall("timestamp_from_second", "Int64", s, "Ptr", out_ts, "Int64")
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
        retcode := _TempusCall("timestamp_from_millisecond", "Int64", s, "Ptr", out_ts, "Int64")
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
        retcode := _TempusCall("timestamp_from_microsecond", "Int64", s, "Ptr", out_ts, "Int64")
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
        retcode := _TempusCall("timestamp_parse", "WStr", timezone, "Ptr", this.pointer, "Ptr", zoned_ptr, "Int64")
        if (retcode = 0) {
            handle := NumGet(zoned_ptr, 0, "Ptr")
        } else {
            throw Format("error {}", retcode)
        }

        if (handle = 0) {
            throw "unexpected error"
        }
        return Zoned(handle)

    }

    to_string() {
        buff_length := _TempusCall("timestamp_string_length", "Ptr", this.pointer, "UInt64")
        buff := Buffer(buff_length+1, 0)
        retcode := _TempusCall("timestamp_to_string", "Ptr", this.pointer, "Ptr", buff, "UInt64", buff.Size)
        ret := StrGet(buff, "UTF-8")
        return ret
    }
}

