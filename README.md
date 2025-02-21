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


