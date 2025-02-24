# tempus.ahk

Tempus is a DateTime library for AutoHotkey. It is, essentially, a wrapper to expose the API of the 
Rust [jiff crate](https://crates.io/crates/jiff) in AHK. 

So, to know what _tempus.ahk_ is about, is to know what _jiff_ is about:

> Jiff is a datetime library for Rust that encourages you to jump into the pit of success. The focus of this library is providing high level datetime primitives that are difficult to misuse and have reasonable performance. Jiff supports automatic and seamless integration with the Time Zone Database, DST aware arithmetic and rounding, formatting and parsing zone aware datetimes losslessly, \[...\] and a whole lot more.  
> Jiff takes enormous inspiration from [Temporal](https://tc39.es/proposal-temporal/docs/index.html), which is a [TC39](https://tc39.es/) proposal to improve datetime handling in JavaScript.


Right now, only a portion of the API is implemented, but development towards completion is rapidly underway.


# Installation

This project has two components: the compiled `tempus_ahk.dll` and the `tempus.ahk` script, which is intended to be used 
via [`#Include`](https://www.autohotkey.com/docs/v2/lib/_Include.htm). This script is for AHK v2 only. In principle, 
the DLL can also be used with AHK v1, but no such script is provided.

From the [releases page](https://github.com/spyoungtech/tempus.ahk/releases) you can download the compiled
`tempus_ahk.dll` file and `tempus.ahk` file (or the `tempus_ahk.zip` containing these). To ensure Dll loading works correctly, you should ensure that 
`tempus_ahk.dll` is somewhere on the Dll Library load search path, such as in the working directory, or 
a directory on `PATH`. Alternatively, you may provide your own `DllLoad` directive before `#Include tempus.ahk` to load the DLL. See 
[DllLoad](https://www.autohotkey.com/docs/v2/lib/_DllLoad.htm) for more information.

See also: [Binary security](#binary-security).

# Usage

The exposed AHK API aims to mirror, as much as is reasonable, the API of `jiff`. Most of the usage is a straightforward 
translation from [the rust API for jiff](https://docs.rs/jiff/latest/jiff/).

For example, in Rust with `jiff`:

```rust
extern crate jiff;
use jiff::{Timestamp, ToSpan};
let time: Timestamp = "2024-07-11T01:14:00Z".parse().unwrap();
assert_eq!(time.as_second(), 1720660440);
```

Looks like this with `tempus.ahk`:

```AutoHotkey
#Include "tempus.ahk"

time := Timestamp.parse("2024-07-11T01:14:00Z")
MsgBox(time.as_second())
```

## Examples

`Timestamp.strptime` / `Timestamp.as_second`

```AutoHotkey
ts := Timestamp.strptime("%F %H:%M %:z", "2024-07-14 21:14 -04:00")
MsgBox(ts.as_second()) ; 1721006040
```

`Timestamp.parse` / `Timestamp.to_string`

```AutoHotkey
ts := Timestamp.parse("2024-01-01T00:00:00Z")
MsgBox(ts.to_string()) ; 2024-01-01T00:00:00Z
```

`Timestamp.strftime` / `Timestamp.from_second`

```AutoHotkey
ts := Timestamp.from_second(86400)
MsgBox(ts.strftime("%a %b %e %I:%M:%S %p UTC %Y")) ; Fri Jan  2 12:00:00 AM UTC 1970
```

`Timestamp.round`

`round` takes three arguments: the rounding unit, the increment, and the rounding mode.


```AutoHotkey
ts := Timestamp.parse("2024-06-20 03:25:01Z")
rounded := ts.round(Unit.Minute, 1, RoundMode.Ceil)
MsgBox(rounded.to_string()) ; 2024-06-20T03:26:00Z
```

Convenience objects are available for specifying the unit and mode:

```AutoHotkey
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
```


## Binary Security

This project is distributed, in part, as a DLL file. DLL files are software compiled in binary form.
Because these files are not human-readable, it is important that you can trust the authors that produce them and
that you can verify the authenticity and integrity of the file you downloaded.

For this project, the DLL binaries in the [releases page](https://github.com/spyoungtech/tempus.ahk/releases) are
digitally signed as part of the GitHub Action where they are built. This digital signature can be used to verify that
you are receiving an authentic copy of `tempus_ahk.dll` that has not been tampered with.

You can view the digital signature by right-clicking the `tempus_ahk.dll` file, selecting "**properties**", clicking the "**Digital Signatures**" tab
and locating the digital signature of "Young Enterprise Solutions LLC" whose signing certificate is issued by Microsoft.
If you do not see the "Digital Signatures" tab or the signature shows as invalid or is signed by any other entity,
that means you do not have an authentic signed copy of the `tempus_ahk.dll` binary.

Moreover, the releases page also contains the hashes of all release files for each release. These can be used to verify 
their integrity. We also proactively submit our DLLs to VirusTotal to ensure our files are free of unexpected detections. 
You can find the links in the releases page.

Alternatively, you may build this binary yourself from source using Rust. See the _Building_ notes below.

# Building

Building this project is fairly simple. But I will try to explain the steps for those who may not have prior experience 
building Rust projects.

It's expected you already have [Rust installed](https://www.rust-lang.org/tools/install) (e.g., you can run `rustup`, `cargo`, etc.).

Prerequisites:

This project uses the GNU toolchain by default, so you will need this installed and ensure you add the target with `rustup`:

- Add the `x86_64-pc-windows-gnu` target: `rustup target add x86_64-pc-windows-gnu`
- Ensure you have the compatible linker on PATH (e.g. you can run `x86_64-w64-mingw32-gcc --version` to verify this). 
  For example, you can install [`MSYS2`](https://www.msys2.org/) and have the toolchain bin directory (`C:\msys64\mingw64\bin`) on `PATH`. If you don't
  see files in this directory, you must install the toolchain by opening the mingw bash shell (`C:\msys64\Mingw64.exe`) and running 
  the command `pacman -S --needed base-devel mingw-w64-x86_64-toolchain`

Build:

- run `cargo build --release` which should produce the DLL  located at `target/x86_64-pc-windows-gnu/release/tempus_ahk.dll`


If you struggle with building on the GNU toolchain with MSYS2, you can build against the default Windows target
by running `cargo build --target x86_64-pc-windows-msvc`. Though note that the produced DLL will have a dependency 
on `vcruntime140.dll`, so target machines you run this on will need the [VC redistributable package](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) 
installed (which, in all likelihood, many users already have due to this being a fairly ubiquitous dependency).


# API progress

This is mostly for loose reference. Not all methods will be implemented. Not all methods are listed here (especially 
things like trait impls, arithmetic, comparisons and more). But may give you an idea of what will be available.

## Timestamp

- [x] [now](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.now)
- [x] `parse` (equivalent of `let ts: Timestamp = "2024-07-11T01:14:00Z".parse()`) -- `Timestamp.parse(mystring)` in AHK
- [ ] [new](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.new)
- [x] [UNIX_EPOCH](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#associatedconstant.UNIX_EPOCH)
- [x] [from_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_second)
- [x] [from_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_millisecond)
- [x] [from_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_microsecond)
- [ ] ~~[from_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_nanosecond)~~ Not supported
- [ ] [from_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_duration)
- [x] [as_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_second)
- [x] [as_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_millisecond)
- [x] [as_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_microsecond)
- [ ] ~~[as_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_nanosecond)~~ Not supported
- [ ] [subsec_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_millisecond)
- [ ] [subsec_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_microsecond)
- [ ] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_nanosecond)
- [ ] [as_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_duration)
- [ ] [signum](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.signum)
- [x] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.is_zero)
- [x] [in_tz](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.in_tz)
- [ ] [to_zoned](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.to_zoned)
- [ ] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.checked_add)
- [ ] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.checked_sub)
- [ ] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.saturating_add)
- [ ] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.saturating_sub)
- [ ] [until](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.until)
- [ ] [since](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.since)
- [ ] [duration_until](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.duration_until)
- [ ] [duration_since](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.duration_since)
- [x] [round](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.round)
- [ ] [series](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.series)
- [x] [strptime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strptime)
- [x] [strftime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strftime)
- [ ] [display_with_offset](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.display_with_offset)
- [x] `to_string` (`Display` trait impl)
- [ ] Others (trait impls, arithmetic, comparisons)

## Zoned

- [x] [now](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.now)
- [x] `parse` (equivalent of `let ts: Zoned = "2024-07-11T01:14:00Z".parse()`) -- `Zoned.parse(mystring)` in AHK
- [ ] [new](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.new)
- [ ] [with](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.with)
- [ ] [with_time_zone](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.with_time_zone)
- [ ] [in_tz](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.in_tz)
- [ ] [time_zone](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.time_zone)
- [ ] [year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.year)
- [ ] [era_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.era_year)
- [ ] [month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.month)
- [ ] [day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day)
- [ ] [hour](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.hour)
- [ ] [minute](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.minute)
- [ ] [second](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.second)
- [ ] [millisecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.millisecond)
- [ ] [microsecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.microsecond)
- [ ] [nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nanosecond)
- [ ] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.subsec_nanosecond)
- [ ] [weekday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.weekday)
- [ ] [day_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day_of_year)
- [ ] [day_of_year_no_leap](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day_of_year_no_leap)
- [ ] [start_of_day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.start_of_day)
- [ ] [end_of_day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.end_of_day)
- [ ] [first_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.first_of_month)
- [ ] [last_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.last_of_month)
- [ ] [days_in_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.days_in_month)
- [ ] [first_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.first_of_year)
- [ ] [last_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.last_of_year)
- [ ] [days_in_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.days_in_year)
- [ ] [in_leap_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.in_leap_year)
- [ ] [tomorrow](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.tomorrow)
- [ ] [yesterday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.yesterday)
- [ ] [nth_weekday_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nth_weekday_of_month)
- [ ] [nth_weekday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nth_weekday)
- [ ] [timestamp](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.timestamp)
- [ ] [datetime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.datetime)
- [ ] [date](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.date)
- [ ] [time](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.time)
- [ ] [iso_week_date](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.iso_week_date)
- [ ] [offset](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.offset)
- [ ] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_add)
- [ ] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_sub)
- [ ] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_add)
- [ ] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_sub)
- [ ] [until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.until)
- [ ] [since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.since)
- [ ] [duration_until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_until)
- [ ] [duration_since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_since)
- [ ] [round](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.round)
- [ ] [strptime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strptime)
- [ ] [strftime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strftime)
- [ ] Others (trait impls (to_string, parse/from_str, etc.), arithmetic, comparisons)

## Span

- [x] [new](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.new)
- [x] `parse`
- [x] [years](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.years)
- [x] [months](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.months)
- [x] [weeks](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.weeks)
- [x] [days](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.days)
- [x] [hours](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.hours)
- [x] [minutes](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.minutes)
- [x] [seconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.seconds)
- [x] [milliseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.milliseconds)
- [x] [microseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.microseconds)
- [x] [nanoseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.nanoseconds)
- [x] [get_years](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_years)
- [x] [get_months](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_months)
- [x] [get_weeks](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_weeks)
- [x] [get_days](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_days)
- [x] [get_hours](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_hours)
- [x] [get_minutes](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_minutes)
- [x] [get_seconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_seconds)
- [x] [get_milliseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_milliseconds)
- [x] [get_microseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_microseconds)
- [x] [get_nanoseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_nanoseconds)
- [x] [abs](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.abs)
- [x] [negate](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.negate)
- [x] [signum](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.signum)
- [x] [is_positive](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_positive)
- [x] [is_negative](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_negative)
- [x] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_zero)
- [ ] ~~[fieldwise](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.fieldwise)~~
- [ ] [checked_mul](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_mul)
- [ ] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_add)
- [ ] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_sub)
- [ ] [compare](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.compare)
- [ ] [total](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.total)
- [ ] [round](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.round)
- [ ] [to_duration](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.to_duration)
- [ ] Others (trait impls (to_string, parse/from_str, etc.), arithmetic, comparisons)

## SignedDuration

(typed variants will likely just be implemented once with 64bit precision)

- [x] `parse`
- [ ] [as_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_secs_f64)
- [ ] [as_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_secs_f32)
- [ ] [as_millis_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_millis_f64)
- [ ] [as_millis_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_millis_f32)
- [ ] [from_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_secs_f64)
- [ ] [from_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_secs_f32)
- [ ] [try_from_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.try_from_secs_f64)
- [ ] [try_from_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.try_from_secs_f32)
- [ ] [mul_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.mul_f64)
- [ ] [mul_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.mul_f32)
- [ ] [div_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_f64)
- [ ] [div_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_f32)
- [ ] [div_duration_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_duration_f64)
- [ ] [div_duration_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_duration_f32)
- [ ] [system_until](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.system_until)
- [ ] [round](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.round)


## TimeZone

- [ ] [system](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.system)
- [ ] [try_system](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.try_system)
- [ ] [get](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.get)
- [ ] [fixed](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.fixed)
- [ ] [posix](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.posix)
- [ ] [tzif](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.tzif)
- [ ] [unknown](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.unknown)
- [ ] [iana_name](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.iana_name)
- [ ] [is_unknown](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.is_unknown)
- [ ] [to_datetime](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_datetime)
- [ ] [to_offset](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_offset)
- [ ] [to_offset_info](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_offset_info)
- [ ] [to_fixed_offset](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_fixed_offset)
- [ ] [to_zoned](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_zoned)
- [ ] [to_ambiguous_zoned](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_ambiguous_zoned)
- [ ] [into_ambiguous_zoned](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.into_ambiguous_zoned)
- [ ] [to_timestamp](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_timestamp)
- [ ] [to_ambiguous_timestamp](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.to_ambiguous_timestamp)
- [ ] [preceding](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.preceding)
- [ ] [following](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.following)

## Date


## Time

## DateTime

