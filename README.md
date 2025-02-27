# tempus.ahk

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/spyoungtech/tempus.ahk/build.yaml)](https://github.com/spyoungtech/tempus.ahk/actions/workflows/build.yaml)  [![GitHub Release](https://img.shields.io/github/v/release/spyoungtech/tempus.ahk?color=blue)](https://github.com/spyoungtech/tempus.ahk/releases)


Tempus is a DateTime library for AutoHotkey. 

Right now, the API is mostly complete, but some additional changes are expected, potentially including breaking changes. See [API progress](#api-progress) for detailed status.

## Acknowledgement

`tempus.ahk` is built on top of [jiff](https://github.com/BurntSushi/jiff), which is authored by [Andrew Gallant](https://github.com/BurntSushi/) and other [contributors](https://github.com/BurntSushi/jiff/graphs/contributors). 
We thank Andrew for his helpful nature and numerous contributions to the community. Moreover, we thank Andrew for 
authoring and publishing `jiff` under permissive licenses, allowing Tempus to exist.


About _jiff_:

> Jiff is a datetime library for Rust that encourages you to jump into the pit of success. The focus of this library is providing high level datetime primitives that are difficult to misuse and have reasonable performance. Jiff supports automatic and seamless integration with the Time Zone Database, DST aware arithmetic and rounding, formatting and parsing zone aware datetimes losslessly, \[...\] and a whole lot more.  
> Jiff takes enormous inspiration from [Temporal](https://tc39.es/proposal-temporal/docs/index.html), which is a [TC39](https://tc39.es/) proposal to improve datetime handling in JavaScript.

Tempus aims to provide to AutoHotkey users the same benefits that `jiff` provides for Rust users. 
In most cases, Tempus simply provides a 1:1 bridge to `jiff`'s APIs.

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
translation from [the rust API for jiff](https://docs.rs/jiff/latest/jiff/) (so be sure to check out this documentation!).

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
MsgBox(time.as_second()) ; 1720660440
```

## Examples

### Timestamp

Jiff [Timestamp](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html)

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

### Span

Jiff [Span](https://docs.rs/jiff/latest/jiff/struct.Span.html)

```AutoHotkey
span1 := Span.new().hours(2).minutes(59)
span2 := Span.new().minutes(2)
span3 := span1.checked_add(span2)
MsgBox(span3.to_string()) ; PT3H1M 
```

`span.parse` / `span.round`

```AutoHotkey
span1 := Span.parse("PT23h50m3.123s")
expected := Span.new().hours(24)
rounded := span1.round(Unit.Minute, 30)
expected.eq(rounded) ; true
```


`span.total`

```AutoHotkey
span1 := Span.new().hours(3).minutes(10)
MsgBox(span1.total(Unit.Second)) ; 11400.0
```


Comparisons

The methods `eq`, `gt`, `lt`, `gte`, and `lte` can be used to compare two span objects

```AutoHotkey
span1 := Span.new().hours(3)
span2 := Span.new().minutes(180)
if (span1.eq(span2)) { ; true
  MsgBox("They are equal in length")
}
```

By default, `jiff` takes into account various factors when comparing spans and does not assume all days are 24 hours. 
Therefore, when a span's smallest component is days or greater (that is, it includes a calendar component), 
you either need to associate a relative datetime (Because, for example, 1 month from March 1 is 31 days, but 1 month from April 1 is 30 days.)
or (to compare weeks) opt into an assumption/invariant of days being calculated as 24 hours.

for example:

```AutoHotkey
span1 := Span.new().weeks(4)
span2 := Span.new().days(30)

span1.eq(span2) ; error!
```
But opting into the 24-hour-days invariant (by passing `true` as the second argument to the compare method) allows this:
```AutoHotkey
span1 := Span.new().weeks(4)
span2 := Span.new().days(30)

; opt into 24-hour-days invariant to allow comparison of days/weeks
span1.gt(span2, true) ; OK!
```

You can also specify a relative timeframe in the form of a Date, DateTime, or Zoned object. This is neccessary 
when your span includes calendar components beyond weeks.

```AutoHotkey
span1 := Span.new().weeks(4)
span2 := Span.new().days(30)

; use relative time to resolve the ambiguity
relative_to := Zoned.now()
span1.gt(span2, relative_to) ; OK!
```

### SignedDuration

Jiff [SignedDuration](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html)

```AutoHotkey
duration := SignedDuration.parse("2h 30m")
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
- [x] [new](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.new)
- [x] [UNIX_EPOCH](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#associatedconstant.UNIX_EPOCH)
- [x] [from_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_second)
- [x] [from_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_millisecond)
- [x] [from_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_microsecond)
- [ ] ~~[from_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_nanosecond)~~ Not supported
- [x] [from_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_duration)
- [x] [as_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_second)
- [x] [as_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_millisecond)
- [x] [as_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_microsecond)
- [ ] ~~[as_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_nanosecond)~~ Not supported
- [x] [subsec_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_millisecond)
- [x] [subsec_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_microsecond)
- [x] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_nanosecond)
- [x] [as_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_duration)
- [x] [signum](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.signum)
- [x] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.is_zero)
- [x] [in_tz](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.in_tz)
- [x] [to_zoned](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.to_zoned)
- [x] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.checked_add)
- [x] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.checked_sub)
- [x] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.saturating_add)
- [x] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.saturating_sub)
- [x] [until](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.until)
- [x] [since](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.since)
- [x] [duration_until](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.duration_until)
- [x] [duration_since](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.duration_since)
- [x] [round](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.round)
- [x] [series](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.series)
- [x] [strptime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strptime)
- [x] [strftime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strftime)
- [ ] [display_with_offset](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.display_with_offset)
- [x] `to_string` (`Display` trait impl)
- [x] cmp (`compare`, `lt`, `gt`, `gte`, `lte`, `eq`)

## Zoned

- [x] [now](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.now)
- [x] `parse` (equivalent of `let ts: Zoned = "2024-07-11T01:14:00Z".parse()`) -- `Zoned.parse(mystring)` in AHK
- [x] [new](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.new)
- [ ] [with](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.with)
- [x] [with_time_zone](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.with_time_zone)
- [x] [in_tz](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.in_tz)
- [x] [time_zone](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.time_zone)
- [x] [year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.year)
- [x] [era_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.era_year)
- [x] [month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.month)
- [x] [day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day)
- [x] [hour](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.hour)
- [x] [minute](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.minute)
- [x] [second](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.second)
- [x] [millisecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.millisecond)
- [x] [microsecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.microsecond)
- [x] [nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nanosecond)
- [x] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.subsec_nanosecond)
- [x] [weekday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.weekday)
- [x] [day_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day_of_year)
- [x] [day_of_year_no_leap](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.day_of_year_no_leap)
- [x] [start_of_day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.start_of_day)
- [x] [end_of_day](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.end_of_day)
- [x] [first_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.first_of_month)
- [x] [last_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.last_of_month)
- [x] [days_in_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.days_in_month)
- [x] [first_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.first_of_year)
- [x] [last_of_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.last_of_year)
- [x] [days_in_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.days_in_year)
- [x] [in_leap_year](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.in_leap_year)
- [x] [tomorrow](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.tomorrow)
- [x] [yesterday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.yesterday)
- [x] [nth_weekday_of_month](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nth_weekday_of_month)
- [x] [nth_weekday](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.nth_weekday)
- [x] [timestamp](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.timestamp)
- [x] [datetime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.datetime)
- [x] [date](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.date)
- [x] [time](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.time)
- [x] [iso_week_date](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.iso_week_date)
- [x] [offset](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.offset)
- [x] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_add)
- [x] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_sub)
- [x] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_add)
- [x] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_sub)
- [x] [until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.until)
- [x] [since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.since)
- [x] [duration_until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_until)
- [x] [duration_since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_since)
- [x] [round](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.round)
- [x] [strptime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strptime)
- [x] [strftime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strftime)
- [x] cmp (`compare`, `lt`, `gt`, `gte`, `lte`, `eq`)


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
- [x] [checked_mul](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_mul) (for `Span` only so far)
- [x] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_add) (for `Span` only so far)
- [x] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.checked_sub)
- [x] [compare](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.compare)
- [x] [total](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.total)
- [x] [round](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.round)
- [ ] [to_duration](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.to_duration)

## SignedDuration

(some variants will likely just be implemented once with 64bit precision)

- [x] `parse`
- [x] `ZERO`
- [x] `MIN`
- [x] `MAX`
- [x] cmp (`compare`, `lt`, `gt`, `gte`, `lte`, `eq`)
- [x] [new](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.new)
- [x] [from_secs](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_secs)
- [x] [from_millis](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_millis)
- [x] [from_micros](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_micros)
- [x] [from_nanos](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_nanos)
- [x] [from_hours](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_hours)
- [x] [from_mins](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_mins)
- [x] [is_zero](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.is_zero)
- [x] [as_secs](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_secs)
- [ ] [subsec_millis](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.subsec_millis)
- [ ] [subsec_micros](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.subsec_micros)
- [ ] [subsec_nanos](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.subsec_nanos)
- [x] [as_millis](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_millis)
- [x] [as_micros](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_micros)
- [ ] ~~[as_nanos](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_nanos)~~ not supported
- [x] [checked_add](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.checked_add)
- [ ] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.saturating_add)
- [x] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.checked_sub)
- [ ] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.saturating_sub)
- [x] [checked_mul](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.checked_mul)
- [ ] [saturating_mul](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.saturating_mul)
- [x] [checked_div](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.checked_div)
- [ ] ~~[as_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_secs_f64)~~ use `as_secs`
- [ ] ~~[as_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_secs_f32)~~
- [ ] ~~[as_millis_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_millis_f64)~~ use `as_millis`
- [ ] ~~[as_millis_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_millis_f32)~~
- [ ] ~~[from_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_secs_f64)~~ use `from_secs`
- [ ] ~~[from_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.from_secs_f32)~~
- [ ] ~~[try_from_secs_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.try_from_secs_f64)~~
- [ ] ~~[try_from_secs_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.try_from_secs_f32)~~
- [ ] [mul_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.mul_f64)
- [ ] [mul_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.mul_f32)
- [ ] [div_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_f64)
- [ ] [div_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_f32)
- [x] [div_duration_f64](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_duration_f64)
- [ ] ~~[div_duration_f32](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.div_duration_f32)~~
- [x] [as_hours](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_hours)
- [x] [as_mins](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.as_mins)
- [x] [abs](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.abs)
- [ ] ~~[unsigned_abs](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.unsigned_abs)~~ (`std::time::Duration` support not planned for now)
- [x] [checked_neg](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.checked_neg)
- [x] [signum](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.signum)
- [x] [is_positive](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.is_positive)
- [x] [is_negative](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.is_negative)
- [ ] [system_until](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.system_until)
- [x] [round](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html#method.round)


## TimeZone

- [x] [system](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.system) (uses try_system and fallsback to UTC on failure)
- [x] [get](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.get)
- [ ] [fixed](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.fixed)
- [x] [posix](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.posix)
- [ ] [tzif](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.tzif)
- [x] [unknown](https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.unknown)
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

Jiff [Civil Date](https://docs.rs/jiff/latest/jiff/civil/struct.Date.html)

- [x] `parse`
- [x] `string_length`
- [x] `to_string`
- [x] `compare` (`gt`, `lt`, `eq`, `lte`, `gte`)
- [x] `min`
- [x] `max`
- [x] `zero`
- [x] `new`
- [x] `year`
- [x] `month`
- [x] `day`
- [x] `era_year` (returns just the year component without era)
- [x] `era` ("BCE" or "CE" as a string)
- [x] `strftime_length`
- [x] `strftime`
- [x] `strptime`
- [x] `from_isoweekdate`
- [x] `weekday`
- [x] `day_of_year`
- [x] `day_of_year_no_leap`
- [x] `first_of_month`
- [x] `last_of_month`
- [x] `first_of_year`
- [x] `last_of_year`
- [x] `days_in_month`
- [x] `days_in_year`
- [x] `in_leap_year`
- [x] `tomorrow`
- [x] `yesterday`
- [x] `nth_weekday_of_month`
- [x] `nth_weekday`
- [x] `to_isoweekdate`
- [x] `in_tz`
- [x] `to_zoned`
- [x] `to_datetime`
- [x] `checked_add`
- [x] `checked_sub`
- [x] `saturating_add`
- [x] `saturating_sub`
- [x] `since`
- [x] `until` (as `until_date` because `until` is a reserved keyword)
- [x] `duration_until`
- [x] `duration_since`
- [x] `series`
- [ ] `with`

## Time

Jiff [Civil Time](https://docs.rs/jiff/latest/jiff/civil/struct.Time.html)

- [x] `string_length`
- [x] `to_string`
- [x] `parse`
- [x] `compare` (`gt`, `lt`, `eq`, `lte`, `gte`)
- [x] `MAX`
- [x] `MIN`
- [x] `new`
- [x] `checked_add`
- [x] `checked_sub`
- [x] `wrapping_add` (alias `add`)
- [x] `wrapping_sub` (alias `sub`)
- [x] `since`
- [x] `until` (as `until_time` because `until` is a reserved keyword)
- [x] `duration_until`
- [x] `duration_since`
- [x] `midnight`
- [x] `hour`
- [x] `minute`
- [x] `second`
- [x] `millisecond`
- [x] `microsecond`
- [x] `nanosecond`
- [x] `subsec_nanosecond`
- [x] `round`
- [x] `saturating_add`
- [x] `saturating_sub`
- [x] `series`
- [ ] `with`


## DateTime

Jiff [Civil DateTime](https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html)

- [x] `parse`
- [x] `string_length`
- [x] `to_string`
- [x] `compare` (`gt`, `lt`, `eq`, `lte`, `gte`)
- [x] `MIN`
- [x] `MAX`
- [x] `ZERO`
- [x] `new`
- [x] `hour`
- [x] `minute`
- [x] `second`
- [x] `millisecond`
- [x] `microsecond`
- [x] `nanosecond`
- [x] `subsec_nanosecond`
- [x] `year`
- [x] `month`
- [x] `day`
- [x] `era_year` (only returns the year component without era)
- [x] `era` ("CE" or "BCE" as a string)
- [x] `strftime_length`
- [x] `strftime`
- [x] `strptime`
- [x] `from_parts`
- [x] `start_of_day`
- [x] `end_of_day`
- [x] `weekday`
- [x] `day_of_year`
- [x] `day_of_year_no_leap`
- [x] `first_of_month`
- [x] `last_of_month`
- [x] `first_of_year`
- [x] `last_of_year`
- [x] `days_in_month`
- [x] `days_in_year`
- [x] `in_leap_year`
- [x] `tomorrow`
- [x] `yesterday`
- [x] `nth_weekday_of_month`
- [x] `nth_weekday`
- [x] `to_isoweekdate`
- [x] `to_date`
- [x] `to_time`
- [x] `to_zoned`
- [x] `checked_add`
- [x] `checked_sub`
- [x] `saturating_add`
- [x] `saturating_sub`
- [x] `until_datetime`
- [x] `until_date`
- [x] `since_datetime`
- [x] `since_date`
- [x] `duration_until`
- [x] `duration_since`
- [x] `series`
- [x] `round`
- [ ] `with`

## ISOWeekDate

Jiff [Civil ISOWeekDate](https://docs.rs/jiff/latest/jiff/civil/struct.ISOWeekDate.html)

- [x] `MIN`
- [x] `MAX`
- [x] `ZERO`
- [x] `new`
- [x] `from_date`
- [x] `year`
- [x] `week`
- [x] `weekday`
- [x] `first_of_week`
- [x] `last_of_week`
- [x] `first_of_year`
- [x] `last_of_year`
- [x] `tomorrow`
- [x] `yesterday`
- [x] `to_date`
- [x] `days_in_year`
- [x] `weeks_in_year`
- [x] `in_long_year`
