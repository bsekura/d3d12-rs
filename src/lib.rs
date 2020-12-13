#![allow(non_camel_case_types)]
#![allow(bad_style, overflowing_literals, unused_macros, deprecated)]
#![recursion_limit = "2563"]
#![no_std]

/// Hack for exported macros
#[doc(hidden)]
pub extern crate core as _core;

#[macro_use]
mod macros;
pub mod shared;
pub mod um;
pub mod vc;

pub mod ctypes {
    #[cfg(feature = "std")]
    pub use std::os::raw::c_void;
    #[cfg(not(feature = "std"))]
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}
// This trait should be implemented for all COM interfaces
pub trait Interface {
    // Returns the IID of the Interface
    fn uuidof() -> shared::guiddef::GUID;
}
// This trait should be implemented for all COM classes
pub trait Class {
    // Returns the CLSID of the Class
    fn uuidof() -> shared::guiddef::GUID;
}
