#![allow(unused)]
#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::io::{stdout, Write};
use std::slice;

use libc::{c_char, c_void, size_t};

pub struct Curl {
    _handle: *mut curl_sys::CURL,
}

impl Default for Curl{
    fn default() -> Self {
        Self::new()
    }
}

impl Curl {
    pub fn new() -> Curl {
        todo!()
    }

    pub fn url(&mut self, url: &str) -> Result<(), curl_sys::CURLcode> {
        todo!()
    }

    pub fn perform(&mut self) -> Result<(), curl_sys::CURLcode> {
        todo!()
    }

    pub fn set_write_cb(&mut self) -> Result<(), curl_sys::CURLcode> {
        todo!()
    }

    fn check_rc(&mut self, rc: curl_sys::CURLcode) -> Result<(), curl_sys::CURLcode> {
        if rc == curl_sys::CURLE_OK {
            return Ok(());
        }
        Err(rc)
    }
}

impl Drop for Curl {
    fn drop(&mut self) {
        todo!()
    }
}

extern "C" fn write_callback(
    ptr: *mut c_char,
    size: size_t,
    nmemb: size_t,
    _userdata: *mut c_void,
) -> size_t {
    unsafe {
        let data = slice::from_raw_parts(ptr as *const u8, size * nmemb);
        stdout().write_all(data).unwrap();
        data.len()
    }
}

fn main() {
    let mut curl = Curl::new();
    curl.url("https://worldtimeapi.org/api/timezone/Europe/Vienna").unwrap();
    curl.perform().unwrap();
}
