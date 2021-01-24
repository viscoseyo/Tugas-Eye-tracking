use std::{
	ffi::c_void,
	marker::PhantomData,
};

use crate::{
	Result,
	traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

pub struct AbstractRefMut<'r, T: ?Sized> {
	ptr: *mut c_void,
	_d: PhantomData<&'r mut T>,
}

impl<T: ?Sized> Boxed for AbstractRefMut<'_, T> {
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		self.ptr
	}

	#[inline]
	fn as_raw(&self) -> *const c_void {
		self.ptr
	}

	#[inline]
	fn as_raw_mut(&mut self) -> *mut c_void {
		self.ptr
	}
}

impl<'r, T: ?Sized> OpenCVType<'r> for AbstractRefMut<'r, T> {
	type Arg = Self;
	type ExternReceive = *mut c_void;
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer> {
		Ok(self)
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self::from_raw(s)
	}
}

impl<'r, T: ?Sized> OpenCVTypeArg<'r> for AbstractRefMut<'r, T> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer> {
		Ok(self)
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T: ?Sized> OpenCVTypeExternContainer for AbstractRefMut<'_, T> {
	type ExternSend = *const c_void;
	type ExternSendMut = *mut c_void;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_raw()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.as_raw_mut()
	}

	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}
