/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
/*
 * Copyright 2024 Saso Kiselkov. All rights reserved.
 */

pub trait SafePointers<T> {
	fn as_ref_safe(self) -> Option<&'static T>;
	fn as_ref_unwrap<'a>(self) -> &'a T;
}

impl<T> SafePointers<T> for *const T {
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	#[inline]
	fn as_ref_safe(self) -> Option<&'static T> {
		unsafe { self.as_ref() }
	}
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	#[inline]
	fn as_ref_unwrap<'a>(self) -> &'a T {
		unsafe {
			self.as_ref()
			    .expect("Attempted to access a NULL pointer")
		}
	}
}

pub trait SafePointersMut<T> {
	fn as_mut_safe<'a>(self) -> Option<&'a mut T>;
	fn as_mut_unwrap<'a>(self) -> &'a mut T;
}

impl<T> SafePointersMut<T> for *mut T {
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	#[inline]
	fn as_mut_safe<'a>(self) -> Option<&'a mut T> {
		unsafe { self.as_mut() }
	}
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	#[inline]
	fn as_mut_unwrap<'a>(self) -> &'a mut T {
		unsafe {
			self.as_mut()
			    .expect("Attempted to access a NULL pointer")
		}
	}
}

pub trait SafePointerSlices<T> {
	fn as_slice<'a>(self, len: usize) -> &'a [T];
}

impl<T> SafePointerSlices<T> for *const T {
	fn as_slice<'a>(self, len: usize) -> &'a [T] {
		assert!(!self.is_null() || len == 0);
		let ptr = if !self.is_null() {
			self
		} else {
			std::ptr::NonNull::dangling().as_ptr()
		};
		unsafe {
			std::slice::from_raw_parts(ptr, len)
		}
	}
}
