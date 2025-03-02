/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
/*
 * Copyright 2024 Saso Kiselkov. All rights reserved.
 */

use std::ffi::c_char;

pub trait CStringInHelpers<T> {
    fn from_c(ptr: T) -> Self;
}

impl CStringInHelpers<*const c_char> for String {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_c(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        String::from(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_str()
                .expect("Invalid UTF-8 data in string"),
        )
    }
}
impl CStringInHelpers<&[c_char]> for String {
    fn from_c(arr: &[c_char]) -> Self {
        Self::from_c(arr.as_ptr())
    }
}
macro_rules! fixed_c_array_impl {
    ($impl_type:ty, $len:expr) => {
        impl CStringInHelpers<&[c_char; $len]> for $impl_type {
            fn from_c(arr: &[c_char; $len]) -> $impl_type {
                Self::from_c(arr.as_ptr())
            }
        }
    };
}
fixed_c_array_impl!(String, 1);
fixed_c_array_impl!(String, 2);
fixed_c_array_impl!(String, 3);
fixed_c_array_impl!(String, 4);
fixed_c_array_impl!(String, 5);
fixed_c_array_impl!(String, 6);
fixed_c_array_impl!(String, 7);
fixed_c_array_impl!(String, 8);
fixed_c_array_impl!(String, 9);
fixed_c_array_impl!(String, 10);
fixed_c_array_impl!(String, 11);
fixed_c_array_impl!(String, 12);
fixed_c_array_impl!(String, 13);
fixed_c_array_impl!(String, 14);
fixed_c_array_impl!(String, 15);
fixed_c_array_impl!(String, 16);

impl<'a> CStringInHelpers<*const c_char> for &'a str {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_c(ptr: *const c_char) -> &'a str {
        assert!(!ptr.is_null());
        unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_str()
            .expect("Invalid UTF-8 data in string")
    }
}
impl<'a> CStringInHelpers<&'a [c_char]> for &'a str {
    fn from_c(arr: &'a [c_char]) -> &'a str {
        Self::from_c(arr.as_ptr())
    }
}
macro_rules! fixed_c_array2str_impl {
    ($len:expr) => {
        impl<'a> CStringInHelpers<&'a [c_char; $len]> for &'a str {
            fn from_c(arr: &'a [c_char; $len]) -> &'a str {
                Self::from_c(arr.as_ptr())
            }
        }
    };
}
fixed_c_array2str_impl!(1);
fixed_c_array2str_impl!(2);
fixed_c_array2str_impl!(3);
fixed_c_array2str_impl!(4);
fixed_c_array2str_impl!(5);
fixed_c_array2str_impl!(6);
fixed_c_array2str_impl!(7);
fixed_c_array2str_impl!(8);
fixed_c_array2str_impl!(9);
fixed_c_array2str_impl!(10);
fixed_c_array2str_impl!(11);
fixed_c_array2str_impl!(12);
fixed_c_array2str_impl!(13);
fixed_c_array2str_impl!(14);
fixed_c_array2str_impl!(15);
fixed_c_array2str_impl!(16);

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Str2C(Option<std::ffi::CString>);

impl Str2C {
    pub unsafe fn as_ptr(&self) -> *const c_char {
        match &self.0 {
            Some(s) => s.as_ptr(),
            None => std::ptr::null(),
        }
    }
    pub fn from_ref<S: AsRef<str>>(value: S) -> Self {
        Self(Some(
            std::ffi::CString::new(value.as_ref())
                .expect("Cannot convert to C string"),
        ))
    }
}

impl From<&str> for Str2C {
    fn from(value: &str) -> Self {
        Self(Some(
            std::ffi::CString::new(value).expect("Cannot convert to C string"),
        ))
    }
}

impl From<String> for Str2C {
    fn from(value: String) -> Self {
        Self(Some(
            std::ffi::CString::new(value.as_str())
                .expect("Cannot convert to C string"),
        ))
    }
}

impl From<&String> for Str2C {
    fn from(value: &String) -> Self {
        Self(Some(
            std::ffi::CString::new(value.as_str())
                .expect("Cannot convert to C string"),
        ))
    }
}

impl From<Option<&str>> for Str2C {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(s) => Str2C::from(s),
            None => Str2C(None),
        }
    }
}

impl From<Option<String>> for Str2C {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Str2C::from(s),
            None => Str2C(None),
        }
    }
}

impl From<&std::path::Path> for Str2C {
    fn from(value: &std::path::Path) -> Self {
        Self(Some(
            std::ffi::CString::new(value.as_os_str().to_str().unwrap_or_else(
                || {
                    panic!(
                        "Cannot represent path {} as string",
                        value.display()
                    )
                },
            ))
            .unwrap_or_else(|_| {
                panic!("Cannot represent path {} as CString", value.display())
            }),
        ))
    }
}

impl From<&std::path::PathBuf> for Str2C {
    fn from(value: &std::path::PathBuf) -> Self {
        Self::from(value.as_path())
    }
}

#[macro_export]
macro_rules! lit2c {
    ($lit:literal) => {
        std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr() as *const c_char)
            .as_ptr()
    };
}

/*
 * CAUTION: this macro must NEVER EVER be used anywhere other than an
 * argument during a call OUT to a C function. This macro constructs a
 * temporary Str2C object, which gets destroyed as soon as the current
 * call site finishes, so you must NEVER hang onto its output. Rust
 * doesn't have an elegant and concise mechanism to enforce this, so we
 * try to enforce this by making the as_ptr() method of Str2C unsafe.
 */
#[macro_export]
macro_rules! str2c {
    ($s:expr) => {
        $crate::c_str::Str2C::from($s).as_ptr()
    };
}

pub fn strlcpy(out_c: *mut c_char, in_rust: &str, out_c_cap: usize) -> usize {
    use crate::c_ptr::SafePointerSlicesMut;
    let out_c = out_c.as_slice_mut(out_c_cap);
    // if the output is zero-length, bail, we can't do anything here
    if out_c.is_empty() {
        return in_rust.len() + 1;
    }
    // the amount of bytes to copy is the lesser of either the input Rust
    // string slice, or the output buffer cap -1 (to allow for Nul termination).
    let copy_lim = in_rust.len().min(out_c.len() - 1);
    unsafe {
        // copy_from_slice() needs both slices to be of equal length, so reslice
        // the output to the actual copy limit and copy in only as many bytes as
        // we want to copy from the input Rust string.
        use std::mem::transmute;
        out_c[..copy_lim].copy_from_slice(transmute::<&[u8], &[c_char]>(
            &in_rust.as_bytes()[..copy_lim],
        ));
    }
    // make sure the output C string is properly terminated
    out_c[copy_lim] = 0;
    // how much we actually tried to copy
    in_rust.len() + 1
}
