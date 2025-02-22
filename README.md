# tempus.ahk

Tempus is a DateTime library for AutoHotkey. It is, essentially, a wrapper to expose the API of the 
Rust [jiff crate](https://crates.io/crates/jiff) in AHK. 

So, to know what _tempus_ is about, is to know what _jiff_ is about:

> Jiff is a datetime library for Rust that encourages you to jump into the pit of success. The focus of this library is providing high level datetime primitives that are difficult to misuse and have reasonable performance. Jiff supports automatic and seamless integration with the Time Zone Database, DST aware arithmetic and rounding, formatting and parsing zone aware datetimes losslessly, \[...\] and a whole lot more.
> Jiff takes enormous inspiration from [Temporal](https://tc39.es/proposal-temporal/docs/index.html), which is a [TC39](https://tc39.es/) proposal to improve datetime handling in JavaScript.


Right now, not much of the API is implemented.


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




# API progress

This is mostly for loose reference. Not all methods will be implemented. Not all methods are listed here (especially 
things like trait impls, arithmetic, comparisons and more). But may give you an idea of what will be available.

## Timestamp

- [x] [now](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.now)
- [x] `parse` (e.g. equivalent of `let ts: Timestamp = "2024-07-11T01:14:00Z".parse()`) -- `Timestamp.parse(mystring)` in AHK
- [ ] [new](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.new)
- [ ] [from_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_second)
- [ ] [from_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_millisecond)
- [ ] [from_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_microsecond)
- [ ] [from_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_nanosecond)
- [ ] [from_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.from_duration)
- [x] [as_second](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_second)
- [x] [as_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_millisecond)
- [ ] [as_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_microsecond)
- [ ] [as_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_nanosecond)
- [ ] [subsec_millisecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_millisecond)
- [ ] [subsec_microsecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_microsecond)
- [ ] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.subsec_nanosecond)
- [ ] [as_duration](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.as_duration)
- [ ] [signum](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.signum)
- [ ] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.is_zero)
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
- [ ] [round](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.round)
- [ ] [series](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.series)
- [ ] [strptime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strptime)
- [ ] [strftime](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strftime)
- [ ] [display_with_offset](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.display_with_offset)
- [x] `to_string` (`Display` trait impl)
- [ ] Others (trait impls, arithmetic, comparisons)

## Zoned

- [x] [now](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.now)
- [ ] [new](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.new)
- [ ] [from_second](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.from_second)
- [ ] [from_millisecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.from_millisecond)
- [ ] [from_microsecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.from_microsecond)
- [ ] [from_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.from_nanosecond)
- [ ] [from_duration](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.from_duration)
- [ ] [as_second](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.as_second)
- [ ] [as_millisecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.as_millisecond)
- [ ] [as_microsecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.as_microsecond)
- [ ] [as_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.as_nanosecond)
- [ ] [subsec_millisecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.subsec_millisecond)
- [ ] [subsec_microsecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.subsec_microsecond)
- [ ] [subsec_nanosecond](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.subsec_nanosecond)
- [ ] [as_duration](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.as_duration)
- [ ] [signum](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.signum)
- [ ] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.is_zero)
- [ ] [in_tz](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.in_tz)
- [ ] [to_zoned](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.to_zoned)
- [ ] [checked_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_add)
- [ ] [checked_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.checked_sub)
- [ ] [saturating_add](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_add)
- [ ] [saturating_sub](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.saturating_sub)
- [ ] [until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.until)
- [ ] [since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.since)
- [ ] [duration_until](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_until)
- [ ] [duration_since](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.duration_since)
- [ ] [round](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.round)
- [ ] [series](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.series)
- [ ] [strptime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strptime)
- [ ] [strftime](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strftime)
- [ ] [display_with_offset](https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.display_with_offset)
- [ ] Others (trait impls (to_string, parse/from_str, etc.), arithmetic, comparisons)

## Span

- [ ] [new](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.new)
- [ ] [years](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.years)
- [ ] [months](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.months)
- [ ] [weeks](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.weeks)
- [ ] [days](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.days)
- [ ] [hours](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.hours)
- [ ] [minutes](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.minutes)
- [ ] [seconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.seconds)
- [ ] [milliseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.milliseconds)
- [ ] [microseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.microseconds)
- [ ] [nanoseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.nanoseconds)
- [ ] [try_years](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_years)
- [ ] [try_months](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_months)
- [ ] [try_weeks](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_weeks)
- [ ] [try_days](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_days)
- [ ] [try_hours](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_hours)
- [ ] [try_minutes](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_minutes)
- [ ] [try_seconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_seconds)
- [ ] [try_milliseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_milliseconds)
- [ ] [try_microseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_microseconds)
- [ ] [try_nanoseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.try_nanoseconds)
- [ ] [get_years](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_years)
- [ ] [get_months](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_months)
- [ ] [get_weeks](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_weeks)
- [ ] [get_days](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_days)
- [ ] [get_hours](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_hours)
- [ ] [get_minutes](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_minutes)
- [ ] [get_seconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_seconds)
- [ ] [get_milliseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_milliseconds)
- [ ] [get_microseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_microseconds)
- [ ] [get_nanoseconds](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.get_nanoseconds)
- [ ] [abs](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.abs)
- [ ] [negate](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.negate)
- [ ] [signum](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.signum)
- [ ] [is_positive](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_positive)
- [ ] [is_negative](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_negative)
- [ ] [is_zero](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.is_zero)
- [ ] [fieldwise](https://docs.rs/jiff/latest/jiff/struct.Span.html#method.fieldwise)
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