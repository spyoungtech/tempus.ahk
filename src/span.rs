#![allow(dead_code)]

use jiff::Span;

#[repr(C)]
pub struct TempusSpan {
    span: Span
}