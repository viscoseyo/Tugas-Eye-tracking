use std::ffi::c_void;

pub trait Boxed: Sized {
	/// Wrap the specified raw pointer
	unsafe fn from_raw(ptr: *mut c_void) -> Self;

	/// Return an the underlying raw pointer while consuming this wrapper.
	///
	/// This will *not* free object referenced by this pointer so you can use this pointer indefinitely. Be sure
	/// to free it (by e.g. calling `from_raw()` with the same wrapper type) to avoid leaking memory.
	fn into_raw(self) -> *mut c_void;

	/// Return the underlying raw pointer.
	///
	/// You can use this pointer as long as the original object lives. Be careful not to double-free it.
	fn as_raw(&self) -> *const c_void;

	/// Return the underlying mutable raw pointer
	///
	/// You can use this pointer as long as the original object lives. Be careful not to double-free it. Note
	/// that ownership is still retained in the original object. Use `into_raw()` if you want to transfer
	/// ownership to another wrapper.
	fn as_raw_mut(&mut self) -> *mut c_void;
}

#[macro_export]
macro_rules! opencv_type_boxed {
	($type: ty) => {
		impl $crate::traits::Boxed for $type {
			#[inline]
			unsafe fn from_raw(ptr: *mut std::ffi::c_void) -> Self {
				Self { ptr }
			}

			#[inline]
			fn into_raw(self) -> *mut std::ffi::c_void {
				std::mem::ManuallyDrop::new(self).ptr
			}

			#[inline]
			fn as_raw(&self) -> *const std::ffi::c_void {
				self.ptr
			}

			#[inline]
			fn as_raw_mut(&mut self) -> *mut std::ffi::c_void {
				self.ptr
			}
		}

		impl $crate::traits::OpenCVType<'_> for $type {
			type Arg = Self;
			type ExternReceive = *mut std::ffi::c_void;
			type ExternContainer = Self;

			#[inline]
			fn opencv_into_extern_container(self) -> $crate::Result<Self::ExternContainer> {
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

		impl $crate::traits::OpenCVTypeArg<'_> for $type {
			type ExternContainer = Self;

			#[inline]
			fn opencv_into_extern_container(self) -> $crate::Result<Self::ExternContainer> {
				Ok(self)
			}

			#[inline]
			fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
				self
			}
		}

		impl $crate::traits::OpenCVTypeExternContainer for $type {
			type ExternSend = *const std::ffi::c_void;
			type ExternSendMut = *mut std::ffi::c_void;

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
	};
}
