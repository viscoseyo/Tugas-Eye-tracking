use std::{
	borrow::Borrow,
	ffi::c_void,
	fmt,
	iter::FromIterator,
	marker::PhantomData,
	mem::ManuallyDrop,
	slice,
};

pub use iter::{VectorIterator, VectorRefIterator};
pub use vector_extern::{VectorElement, VectorExtern, VectorExternCopyNonBool};

use crate::{
	platform_types::size_t,
	Result,
	traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

mod vector_extern;
mod iter;

/// Wrapper for C++ [std::vector](https://en.cppreference.com/w/cpp/container/vector)
pub struct Vector<T: VectorElement> where Self: VectorExtern<T> {
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T: VectorElement> Vector<T> where Self: VectorExtern<T> {
	/// Create a new Vector
	pub fn new() -> Self {
		unsafe { Self::from_raw(Self::extern_new()) }
	}

	/// Create a Vector with pre-defined capacity
	pub fn with_capacity(capacity: size_t) -> Self {
		let mut out = Self::new();
		out.reserve(capacity);
		out
	}

	/// Create a Vector from iterator
	pub fn from_iter<'a>(s: impl IntoIterator<Item=<T as OpenCVType<'a>>::Arg>) -> Self {
		let mut out = Self::new();
		out.extend(s);
		out
	}

	/// Return Vector length
	pub fn len(&self) -> size_t {
		unsafe { self.extern_len() }
	}

	/// Return true if Vector is empty
	pub fn is_empty(&self) -> bool {
		unsafe { self.extern_is_empty() }
	}

	/// Return Vector current capacity
	pub fn capacity(&self) -> size_t {
		unsafe { self.extern_capacity() }
	}

	/// Free extra capacity
	pub fn shrink_to_fit(&mut self) {
		unsafe { self.extern_shrink_to_fit() }
	}

	/// Reserve capacity for `additional` new elements
	pub fn reserve(&mut self, additional: size_t) {
		unsafe { self.extern_reserve(additional) }
	}

	/// Remove all elements
	pub fn clear(&mut self) {
		unsafe { self.extern_clear() }
	}

	/// Remove the element at the specified `index`
	pub fn remove(&mut self, index: size_t) -> Result<()> {
		vector_index_check(index, self.len())?;
		unsafe { self.extern_remove(index) }
		Ok(())
	}

	/// Swap 2 elements in the Vector
	pub fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
		let len = self.len();
		vector_index_check(index1, len)?;
		vector_index_check(index2, len)?;
		if index1 != index2 {
			unsafe { self.extern_swap(index1, index2) }
		}
		Ok(())
	}

	/// Add new element
	pub fn push(&mut self, val: <T as OpenCVType>::Arg) {
		let val = val.opencv_into_extern_container_nofail();
		unsafe { self.extern_push(val.opencv_as_extern()) }
	}

	pub(crate) fn push_owned(&mut self, val: T) {
		let val = val.opencv_into_extern_container_nofail();
		unsafe { self.extern_push_owned(val.opencv_as_extern()) }
	}

	/// Insert a new element at the specified `index`
	pub fn insert(&mut self, index: size_t, val: <T as OpenCVType>::Arg) -> Result<()> {
		vector_index_check(index, self.len() + 1)?;
		let val = val.opencv_into_extern_container()?;
		unsafe { self.extern_insert(index, val.opencv_as_extern()) }
		Ok(())
	}

	/// Set element at the specified `index`
	pub fn set(&mut self, index: size_t, val: <T as OpenCVType>::Arg) -> Result<()> {
		vector_index_check(index, self.len())?;
		let val = val.opencv_into_extern_container()?;
		unsafe { self.extern_set(index, val.opencv_as_extern()) }
		Ok(())
	}

	/// Same as `set()` but without bounds checking
	pub unsafe fn set_unchecked(&mut self, index: size_t, val: <T as OpenCVType>::Arg) {
		let val = val.opencv_into_extern_container_nofail();
		self.extern_set(index, val.opencv_as_extern())
	}

	/// Get element at the specified `index`
	pub fn get(&self, index: size_t) -> Result<T> {
		vector_index_check(index, self.len())?;
		unsafe { self.extern_get(index) }
			.into_result()
			.map(|s| unsafe { T::opencv_from_extern(s) } )
	}

	/// Same as `get()` but without bounds checking
	pub unsafe fn get_unchecked(&self, index: size_t) -> T {
		self.extern_get(index)
			.into_result()
			.map(|s| T::opencv_from_extern(s) )
			.unwrap() // fixme, make it return value directly
	}

	pub fn iter(&self) -> VectorRefIterator<T> {
		VectorRefIterator::new(self)
	}

	/// Return slice to the elements of the array.
	///
	/// This method is only available for OpenCV types that are Copy, with the exception of bool
	/// because bool is handled in a special way on the C++ side.
	pub fn as_slice(&self) -> &[T] where Self: VectorExternCopyNonBool<T> {
		unsafe {
			slice::from_raw_parts(self.extern_data(), self.len())
		}
	}

	/// Return mutable slice to the elements of the array.
	///
	/// This method is only available for OpenCV types that are Copy, with the exception of bool
	/// because bool is handled in a special way on the C++ side.
	pub fn as_mut_slice(&mut self) -> &mut [T] where Self: VectorExternCopyNonBool<T> {
		unsafe {
			slice::from_raw_parts_mut(self.extern_data_mut(), self.len())
		}
	}

	pub fn to_vec(&self) -> Vec<T> {
		T::opencv_vector_to_vec(self)
	}
}

impl<T: VectorElement> Default for Vector<T> where Self: VectorExtern<T> {
	#[inline]
	fn default() -> Vector<T> {
		Vector::new()
	}
}

impl<T: VectorElement> From<Vector<T>> for Vec<T> where Vector<T>: VectorExtern<T> {
	#[inline]
	fn from(from: Vector<T>) -> Self {
		from.to_vec()
	}
}

impl<T: VectorElement> From<Vec<<T as OpenCVType<'_>>::Arg>> for Vector<T> where Vector<T>: VectorExtern<T> {
	#[inline]
	fn from(from: Vec<<T as OpenCVType<'_>>::Arg>) -> Self {
		Self::from_iter(from)
	}
}

impl<'a, T: VectorElement> FromIterator<<T as OpenCVType<'a>>::Arg> for Vector<T> where Self: VectorExtern<T> {
	#[inline]
	fn from_iter<I: IntoIterator<Item=<T as OpenCVType<'a>>::Arg>>(s: I) -> Vector<T> {
		Self::from_iter(s)
	}
}

impl<T: VectorElement> AsRef<[T]> for Vector<T> where Self: VectorExtern<T> + VectorExternCopyNonBool<T> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: VectorElement> Borrow<[T]> for Vector<T> where Self: VectorExtern<T> + VectorExternCopyNonBool<T> {
	#[inline]
	fn borrow(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: VectorElement + fmt::Debug> fmt::Debug for Vector<T> where Self: VectorExtern<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

impl<T: VectorElement> Drop for Vector<T> where Self: VectorExtern<T> {
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}

impl<'a, T: VectorElement> Extend<<T as OpenCVType<'a>>::Arg> for Vector<T> where Self: VectorExtern<T> {
	fn extend<I: IntoIterator<Item=<T as OpenCVType<'a>>::Arg>>(&mut self, s: I) {
		let s = s.into_iter();
		let (lo, hi) = s.size_hint();
		self.reserve(hi.unwrap_or(lo));
		s.for_each(|elem| {
			self.push(elem);
		});
	}
}

impl<T: VectorElement> Boxed for Vector<T> where Self: VectorExtern<T> {
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		ManuallyDrop::new(self).ptr
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

impl<T: VectorElement> OpenCVType<'_> for Vector<T> where Self: VectorExtern<T> {
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

impl<T: VectorElement> OpenCVTypeArg<'_> for Vector<T> where Self: VectorExtern<T> {
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

impl<T: VectorElement> OpenCVTypeExternContainer for Vector<T> where Self: VectorExtern<T> {
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

#[inline(always)]
fn vector_index_check(index: size_t, len: size_t) -> crate::Result<()> {
	if index >= len {
		Err(crate::Error::new(crate::core::StsOutOfRange, format!("Index: {} out of bounds: 0..{}", index, len)))
	} else {
		Ok(())
	}
}
