#![allow(non_camel_case_types)]

use libc::{c_int, c_void};

pub type IOHandle = *mut c_void;

pub type IOCallback_Read =
    unsafe extern "C" fn(ptr: *mut c_void, size: usize, nmemb: usize, handle: IOHandle) -> usize;

pub type IOCallback_Write =
    unsafe extern "C" fn(ptr: *mut c_void, size: usize, nmemb: usize, handle: IOHandle) -> usize;

pub type IOCallback_Seek =
    unsafe extern "C" fn(handle: IOHandle, offset: i64, whence: c_int) -> c_int;

pub type IOCallback_Tell = unsafe extern "C" fn(handle: IOHandle) -> i64;

pub type IOCallback_Eof = unsafe extern "C" fn(handle: IOHandle) -> c_int;

pub type IOCallback_Close = unsafe extern "C" fn(handle: IOHandle) -> c_int;

#[repr(C)]
pub struct IOCallbacks {
    pub read: Option<IOCallback_Read>,
    pub write: Option<IOCallback_Write>,
    pub seek: Option<IOCallback_Seek>,
    pub tell: Option<IOCallback_Tell>,
    pub eof: Option<IOCallback_Eof>,
    pub close: Option<IOCallback_Close>,
}
