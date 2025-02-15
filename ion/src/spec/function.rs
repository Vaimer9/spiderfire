/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::ptr;

use mozjs::jsapi::{JSFunctionSpec, JSNativeWrapper, JSPropertySpec_Name};

use crate::flags::PropertyFlags;

/// Creates a [JSFunctionSpec] with the given native function, number of arguments and flags.
pub const fn create_function_spec(name: &'static str, func: JSNativeWrapper, nargs: u16, flags: PropertyFlags) -> JSFunctionSpec {
	JSFunctionSpec {
		name: JSPropertySpec_Name { string_: name.as_ptr() as *const i8 },
		call: func,
		nargs,
		flags: flags.bits(),
		selfHostedName: ptr::null_mut(),
	}
}

#[cfg(feature = "macros")]
#[macro_export(local_inner_macros)]
macro_rules! function_spec {
	($function:expr, $name:expr, $nargs:expr, $flags:expr) => {
		$crate::spec::create_function_spec(
			::std::concat!($name, "\0"),
			::mozjs::jsapi::JSNativeWrapper {
				op: Some($function),
				info: ::std::ptr::null_mut(),
			},
			$nargs,
			$flags,
		)
	};
	($function:expr, $name:expr, $nargs:expr) => {
		function_spec!($function, $name, $nargs, $crate::flags::PropertyFlags::CONSTANT_ENUMERATED)
	};
	($function:expr, $nargs:expr) => {
		function_spec!($function, ::std::stringify!($function), $nargs)
	};
}
