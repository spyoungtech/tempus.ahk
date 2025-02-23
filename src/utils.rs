pub(crate) type AHKWstr = *const u16;
pub(crate) type AHKStringBuffer = *mut c_char;

use std::ffi::{c_char};
use std::borrow::BorrowMut;
use std::ptr;
use std::sync::{Mutex, Once};

static mut STD_ONCE_COUNTER: Option<Mutex<String>> = None;
static INIT: Once = Once::new();

fn global_string<'a>() -> &'a Mutex<String> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        #[allow(static_mut_refs)]
        unsafe {
            *STD_ONCE_COUNTER.borrow_mut() = Some(Mutex::new(String::from("Uninitialized")));
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    #[allow(static_mut_refs)]
    unsafe { STD_ONCE_COUNTER.as_ref().unwrap() }
}


#[no_mangle]
pub extern "C" fn get_last_error_length() -> usize {
    global_string().lock().unwrap().len()
}

pub(crate) fn set_last_error_message(message: String) {
    *global_string().lock().unwrap() = message;
}


#[no_mangle]
pub extern "C" fn get_last_error(buf: *mut c_char, buf_len: usize) -> usize {
    use std::ptr;
    if buf.is_null() || buf_len == 0 {
        return 0;
    }

    let text = global_string().lock().unwrap().clone();
    let message_bytes = text.as_bytes();

    // Copy as many bytes as will fit (leaving 1 byte for the null terminator)
    let copy_len = message_bytes.len().min(buf_len - 1);

    unsafe {
        ptr::copy_nonoverlapping(message_bytes.as_ptr(), buf as *mut u8, copy_len);
        *buf.add(copy_len) = 0; // null-terminate
    }
    clear_last_error();
    copy_len
}


pub(crate) fn clear_last_error() {
    set_last_error_message(String::from("unset error"));
}


pub(crate) fn ahk_str_to_string(ahk_str: AHKWstr) -> Result<String, i64> {
    if ahk_str.is_null() {
        return Err(-1);
    }
    let mut length = 0usize;
    unsafe {
        while *ahk_str.add(length) != 0 {
            length += 1;
        }
    }

    let slice = unsafe { std::slice::from_raw_parts(ahk_str, length) };
    Ok(String::from_utf16_lossy(slice))
}


pub(crate) fn string_into_ahk_buff(s: String, out_buff: AHKStringBuffer, buff_len: usize) {
    let ret_bytes = s.as_bytes();
    let copy_len = ret_bytes.len().min(buff_len - 1);
    unsafe {
        ptr::copy_nonoverlapping(ret_bytes.as_ptr(), out_buff as *mut u8, copy_len);
        *out_buff.add(copy_len) = 0;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        println!("{}", *global_string().lock().unwrap());
        set_last_error_message("new".to_string());
        println!("{}", *global_string().lock().unwrap());

    }
}