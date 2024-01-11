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
		String::from(unsafe { std::ffi::CStr::from_ptr(ptr) }
		    .to_str()
		    .expect("Invalid UTF-8 data in string"))
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

impl CStringInHelpers<*const c_char> for &'static str {
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	fn from_c(ptr: *const c_char) -> &'static str {
		assert!(!ptr.is_null());
		unsafe {std::ffi::CStr::from_ptr(ptr)}
		    .to_str()
		    .expect("Invalid UTF-8 data in string")
	}
}
impl CStringInHelpers<&[c_char]> for &'static str {
	fn from_c(arr: &[c_char]) -> &'static str {
		Self::from_c(arr.as_ptr())
	}
}
fixed_c_array_impl!(&'static str, 1);
fixed_c_array_impl!(&'static str, 2);
fixed_c_array_impl!(&'static str, 3);
fixed_c_array_impl!(&'static str, 4);
fixed_c_array_impl!(&'static str, 5);
fixed_c_array_impl!(&'static str, 6);
fixed_c_array_impl!(&'static str, 7);
fixed_c_array_impl!(&'static str, 8);
fixed_c_array_impl!(&'static str, 9);
fixed_c_array_impl!(&'static str, 10);
fixed_c_array_impl!(&'static str, 11);
fixed_c_array_impl!(&'static str, 12);
fixed_c_array_impl!(&'static str, 13);
fixed_c_array_impl!(&'static str, 14);
fixed_c_array_impl!(&'static str, 15);
fixed_c_array_impl!(&'static str, 16);

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Str2C(Option<std::ffi::CString>);

impl Str2C {
	pub unsafe fn as_ptr(&self) -> *const c_char {
		match &self.0 {
		    Some(s) => s.as_ptr(),
		    None => std::ptr::null(),
		}
	}
}

impl From<&str> for Str2C {
	fn from(value: &str) -> Self {
		Self(Some(std::ffi::CString::new(value)
		    .expect("Cannot convert to C string")))
	}
}

impl From<String> for Str2C {
	fn from(value: String) -> Self {
		Self(Some(std::ffi::CString::new(value.as_str())
		    .expect("Cannot convert to C string")))
	}
}

impl From<&String> for Str2C {
	fn from(value: &String) -> Self {
		Self(Some(std::ffi::CString::new(value.as_str())
		    .expect("Cannot convert to C string")))
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

#[macro_export]
macro_rules! lit2c {
    ($lit:literal) => {
	std::ffi::CStr::from_ptr(concat!($lit, "\0")
	    .as_ptr() as *const c_char)
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
