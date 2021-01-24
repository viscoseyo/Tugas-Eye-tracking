
mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfLMSolver = core::Ptr::<dyn crate::calib3d::LMSolver>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver,
		cv_PtrOfLMSolver_delete, cv_PtrOfLMSolver_get_inner_ptr, cv_PtrOfLMSolver_get_inner_ptr_mut
	}
	
	impl PtrOfLMSolver {
		#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLMSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolver for PtrOfLMSolver {
		#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLMSolver_Callback = core::Ptr::<dyn crate::calib3d::LMSolver_Callback>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver_Callback,
		cv_PtrOfLMSolver_Callback_delete, cv_PtrOfLMSolver_Callback_get_inner_ptr, cv_PtrOfLMSolver_Callback_get_inner_ptr_mut
	}
	
	impl PtrOfLMSolver_Callback {
		#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolver_Callback for PtrOfLMSolver_Callback {
		#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoBM = core::Ptr::<dyn crate::calib3d::StereoBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoBM,
		cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr, cv_PtrOfStereoBM_get_inner_ptr_mut
	}
	
	impl PtrOfStereoBM {
		#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBM for PtrOfStereoBM {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoSGBM = core::Ptr::<dyn crate::calib3d::StereoSGBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoSGBM,
		cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr, cv_PtrOfStereoSGBM_get_inner_ptr_mut
	}
	
	impl PtrOfStereoSGBM {
		#[inline] pub fn as_raw_PtrOfStereoSGBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoSGBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use calib3d_types::*;

mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	impl core::GpuMat_Allocator for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	impl core::MatOp for types::AbstractRefMut<'static, dyn core::MatOp> {
		#[inline] fn as_raw_MatOp(&self) -> *const c_void { self.as_raw() }
		#[inline] fn as_raw_mut_MatOp(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	pub type PtrOfConjGradSolver = core::Ptr::<dyn core::ConjGradSolver>;
	
	ptr_extern! { dyn core::ConjGradSolver,
		cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr, cv_PtrOfConjGradSolver_get_inner_ptr_mut
	}
	
	impl PtrOfConjGradSolver {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfConjGradSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::ConjGradSolver for PtrOfConjGradSolver {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolver for PtrOfConjGradSolver {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDownhillSolver = core::Ptr::<dyn core::DownhillSolver>;
	
	ptr_extern! { dyn core::DownhillSolver,
		cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr, cv_PtrOfDownhillSolver_get_inner_ptr_mut
	}
	
	impl PtrOfDownhillSolver {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDownhillSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::DownhillSolver for PtrOfDownhillSolver {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolver for PtrOfDownhillSolver {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFileStorage = core::Ptr::<core::FileStorage>;
	
	ptr_extern! { core::FileStorage,
		cv_PtrOfFileStorage_delete, cv_PtrOfFileStorage_get_inner_ptr, cv_PtrOfFileStorage_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::FileStorage, cv_PtrOfFileStorage_new }
	
	impl PtrOfFileStorage {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::FileStorageTrait for PtrOfFileStorage {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatted = core::Ptr::<dyn core::Formatted>;
	
	ptr_extern! { dyn core::Formatted,
		cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr, cv_PtrOfFormatted_get_inner_ptr_mut
	}
	
	impl PtrOfFormatted {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::Formatted for PtrOfFormatted {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatter = core::Ptr::<dyn core::Formatter>;
	
	ptr_extern! { dyn core::Formatter,
		cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr, cv_PtrOfFormatter_get_inner_ptr_mut
	}
	
	impl PtrOfFormatter {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::Formatter for PtrOfFormatter {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGpuMat_Allocator = core::Ptr::<dyn core::GpuMat_Allocator>;
	
	ptr_extern! { dyn core::GpuMat_Allocator,
		cv_PtrOfGpuMat_Allocator_delete, cv_PtrOfGpuMat_Allocator_get_inner_ptr, cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut
	}
	
	impl PtrOfGpuMat_Allocator {
		#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::GpuMat_Allocator for PtrOfGpuMat_Allocator {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMinProblemSolver_Function = core::Ptr::<dyn core::MinProblemSolver_Function>;
	
	ptr_extern! { dyn core::MinProblemSolver_Function,
		cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr, cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut
	}
	
	impl PtrOfMinProblemSolver_Function {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::MinProblemSolver_Function for PtrOfMinProblemSolver_Function {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOff32 = core::Ptr::<f32>;
	
	ptr_extern! { f32,
		cv_PtrOff32_delete, cv_PtrOff32_get_inner_ptr, cv_PtrOff32_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { f32, cv_PtrOff32_new }
	
	impl PtrOff32 {
		#[inline] pub fn as_raw_PtrOff32(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	pub type VectorOfDMatch = core::Vector::<core::DMatch>;
	
	impl VectorOfDMatch {
		pub fn as_raw_VectorOfDMatch(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::DMatch, *const c_void, *mut c_void,
		cv_VectorOfDMatch_new, cv_VectorOfDMatch_delete,
		cv_VectorOfDMatch_len, cv_VectorOfDMatch_is_empty,
		cv_VectorOfDMatch_capacity, cv_VectorOfDMatch_shrink_to_fit,
		cv_VectorOfDMatch_reserve, cv_VectorOfDMatch_remove,
		cv_VectorOfDMatch_swap, cv_VectorOfDMatch_clear,
		cv_VectorOfDMatch_get, cv_VectorOfDMatch_set,
		cv_VectorOfDMatch_push, cv_VectorOfDMatch_insert,
	}
	vector_copy_non_bool! { core::DMatch, *const c_void, *mut c_void,
		cv_VectorOfDMatch_data, cv_VectorOfDMatch_data_mut,
		cv_VectorOfDMatch_clone,
	}
	
	unsafe impl Send for core::Vector::<core::DMatch> {}
	
	pub type VectorOfGpuMat = core::Vector::<core::GpuMat>;
	
	impl VectorOfGpuMat {
		pub fn as_raw_VectorOfGpuMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::GpuMat, *const c_void, *mut c_void,
		cv_VectorOfGpuMat_new, cv_VectorOfGpuMat_delete,
		cv_VectorOfGpuMat_len, cv_VectorOfGpuMat_is_empty,
		cv_VectorOfGpuMat_capacity, cv_VectorOfGpuMat_shrink_to_fit,
		cv_VectorOfGpuMat_reserve, cv_VectorOfGpuMat_remove,
		cv_VectorOfGpuMat_swap, cv_VectorOfGpuMat_clear,
		cv_VectorOfGpuMat_get, cv_VectorOfGpuMat_set,
		cv_VectorOfGpuMat_push, cv_VectorOfGpuMat_insert,
	}
	vector_non_copy_or_bool! { clone core::GpuMat }
	
	unsafe impl Send for core::Vector::<core::GpuMat> {}
	
	pub type VectorOfKeyPoint = core::Vector::<core::KeyPoint>;
	
	impl VectorOfKeyPoint {
		pub fn as_raw_VectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfKeyPoint_new, cv_VectorOfKeyPoint_delete,
		cv_VectorOfKeyPoint_len, cv_VectorOfKeyPoint_is_empty,
		cv_VectorOfKeyPoint_capacity, cv_VectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfKeyPoint_reserve, cv_VectorOfKeyPoint_remove,
		cv_VectorOfKeyPoint_swap, cv_VectorOfKeyPoint_clear,
		cv_VectorOfKeyPoint_get, cv_VectorOfKeyPoint_set,
		cv_VectorOfKeyPoint_push, cv_VectorOfKeyPoint_insert,
	}
	vector_copy_non_bool! { core::KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfKeyPoint_data, cv_VectorOfKeyPoint_data_mut,
		cv_VectorOfKeyPoint_clone,
	}
	
	unsafe impl Send for core::Vector::<core::KeyPoint> {}
	
	pub type VectorOfMat = core::Vector::<core::Mat>;
	
	impl VectorOfMat {
		pub fn as_raw_VectorOfMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Mat, *const c_void, *mut c_void,
		cv_VectorOfMat_new, cv_VectorOfMat_delete,
		cv_VectorOfMat_len, cv_VectorOfMat_is_empty,
		cv_VectorOfMat_capacity, cv_VectorOfMat_shrink_to_fit,
		cv_VectorOfMat_reserve, cv_VectorOfMat_remove,
		cv_VectorOfMat_swap, cv_VectorOfMat_clear,
		cv_VectorOfMat_get, cv_VectorOfMat_set,
		cv_VectorOfMat_push, cv_VectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Mat }
	
	unsafe impl Send for core::Vector::<core::Mat> {}
	
	pub type VectorOfPlatformInfo = core::Vector::<core::PlatformInfo>;
	
	impl VectorOfPlatformInfo {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::PlatformInfo, *const c_void, *mut c_void,
		cv_VectorOfPlatformInfo_new, cv_VectorOfPlatformInfo_delete,
		cv_VectorOfPlatformInfo_len, cv_VectorOfPlatformInfo_is_empty,
		cv_VectorOfPlatformInfo_capacity, cv_VectorOfPlatformInfo_shrink_to_fit,
		cv_VectorOfPlatformInfo_reserve, cv_VectorOfPlatformInfo_remove,
		cv_VectorOfPlatformInfo_swap, cv_VectorOfPlatformInfo_clear,
		cv_VectorOfPlatformInfo_get, cv_VectorOfPlatformInfo_set,
		cv_VectorOfPlatformInfo_push, cv_VectorOfPlatformInfo_insert,
	}
	vector_non_copy_or_bool! { core::PlatformInfo }
	
	unsafe impl Send for core::Vector::<core::PlatformInfo> {}
	
	pub type VectorOfPoint = core::Vector::<core::Point>;
	
	impl VectorOfPoint {
		pub fn as_raw_VectorOfPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point, *const c_void, *mut c_void,
		cv_VectorOfPoint_new, cv_VectorOfPoint_delete,
		cv_VectorOfPoint_len, cv_VectorOfPoint_is_empty,
		cv_VectorOfPoint_capacity, cv_VectorOfPoint_shrink_to_fit,
		cv_VectorOfPoint_reserve, cv_VectorOfPoint_remove,
		cv_VectorOfPoint_swap, cv_VectorOfPoint_clear,
		cv_VectorOfPoint_get, cv_VectorOfPoint_set,
		cv_VectorOfPoint_push, cv_VectorOfPoint_insert,
	}
	vector_copy_non_bool! { core::Point, *const c_void, *mut c_void,
		cv_VectorOfPoint_data, cv_VectorOfPoint_data_mut,
		cv_VectorOfPoint_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point> {}
	
	impl core::ToInputArray for VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint2d = core::Vector::<core::Point2d>;
	
	impl VectorOfPoint2d {
		pub fn as_raw_VectorOfPoint2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2d, *const c_void, *mut c_void,
		cv_VectorOfPoint2d_new, cv_VectorOfPoint2d_delete,
		cv_VectorOfPoint2d_len, cv_VectorOfPoint2d_is_empty,
		cv_VectorOfPoint2d_capacity, cv_VectorOfPoint2d_shrink_to_fit,
		cv_VectorOfPoint2d_reserve, cv_VectorOfPoint2d_remove,
		cv_VectorOfPoint2d_swap, cv_VectorOfPoint2d_clear,
		cv_VectorOfPoint2d_get, cv_VectorOfPoint2d_set,
		cv_VectorOfPoint2d_push, cv_VectorOfPoint2d_insert,
	}
	vector_copy_non_bool! { core::Point2d, *const c_void, *mut c_void,
		cv_VectorOfPoint2d_data, cv_VectorOfPoint2d_data_mut,
		cv_VectorOfPoint2d_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point2d> {}
	
	impl core::ToInputArray for VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint2f = core::Vector::<core::Point2f>;
	
	impl VectorOfPoint2f {
		pub fn as_raw_VectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2f, *const c_void, *mut c_void,
		cv_VectorOfPoint2f_new, cv_VectorOfPoint2f_delete,
		cv_VectorOfPoint2f_len, cv_VectorOfPoint2f_is_empty,
		cv_VectorOfPoint2f_capacity, cv_VectorOfPoint2f_shrink_to_fit,
		cv_VectorOfPoint2f_reserve, cv_VectorOfPoint2f_remove,
		cv_VectorOfPoint2f_swap, cv_VectorOfPoint2f_clear,
		cv_VectorOfPoint2f_get, cv_VectorOfPoint2f_set,
		cv_VectorOfPoint2f_push, cv_VectorOfPoint2f_insert,
	}
	vector_copy_non_bool! { core::Point2f, *const c_void, *mut c_void,
		cv_VectorOfPoint2f_data, cv_VectorOfPoint2f_data_mut,
		cv_VectorOfPoint2f_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point2f> {}
	
	impl core::ToInputArray for VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint3d = core::Vector::<core::Point3d>;
	
	impl VectorOfPoint3d {
		pub fn as_raw_VectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3d, *const c_void, *mut c_void,
		cv_VectorOfPoint3d_new, cv_VectorOfPoint3d_delete,
		cv_VectorOfPoint3d_len, cv_VectorOfPoint3d_is_empty,
		cv_VectorOfPoint3d_capacity, cv_VectorOfPoint3d_shrink_to_fit,
		cv_VectorOfPoint3d_reserve, cv_VectorOfPoint3d_remove,
		cv_VectorOfPoint3d_swap, cv_VectorOfPoint3d_clear,
		cv_VectorOfPoint3d_get, cv_VectorOfPoint3d_set,
		cv_VectorOfPoint3d_push, cv_VectorOfPoint3d_insert,
	}
	vector_copy_non_bool! { core::Point3d, *const c_void, *mut c_void,
		cv_VectorOfPoint3d_data, cv_VectorOfPoint3d_data_mut,
		cv_VectorOfPoint3d_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point3d> {}
	
	impl core::ToInputArray for VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_mut_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint3i = core::Vector::<core::Point3i>;
	
	impl VectorOfPoint3i {
		pub fn as_raw_VectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3i, *const c_void, *mut c_void,
		cv_VectorOfPoint3i_new, cv_VectorOfPoint3i_delete,
		cv_VectorOfPoint3i_len, cv_VectorOfPoint3i_is_empty,
		cv_VectorOfPoint3i_capacity, cv_VectorOfPoint3i_shrink_to_fit,
		cv_VectorOfPoint3i_reserve, cv_VectorOfPoint3i_remove,
		cv_VectorOfPoint3i_swap, cv_VectorOfPoint3i_clear,
		cv_VectorOfPoint3i_get, cv_VectorOfPoint3i_set,
		cv_VectorOfPoint3i_push, cv_VectorOfPoint3i_insert,
	}
	vector_copy_non_bool! { core::Point3i, *const c_void, *mut c_void,
		cv_VectorOfPoint3i_data, cv_VectorOfPoint3i_data_mut,
		cv_VectorOfPoint3i_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point3i> {}
	
	impl core::ToInputArray for VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfRange = core::Vector::<core::Range>;
	
	impl VectorOfRange {
		pub fn as_raw_VectorOfRange(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Range, *const c_void, *mut c_void,
		cv_VectorOfRange_new, cv_VectorOfRange_delete,
		cv_VectorOfRange_len, cv_VectorOfRange_is_empty,
		cv_VectorOfRange_capacity, cv_VectorOfRange_shrink_to_fit,
		cv_VectorOfRange_reserve, cv_VectorOfRange_remove,
		cv_VectorOfRange_swap, cv_VectorOfRange_clear,
		cv_VectorOfRange_get, cv_VectorOfRange_set,
		cv_VectorOfRange_push, cv_VectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Range }
	
	unsafe impl Send for core::Vector::<core::Range> {}
	
	pub type VectorOfRect = core::Vector::<core::Rect>;
	
	impl VectorOfRect {
		pub fn as_raw_VectorOfRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect, *const c_void, *mut c_void,
		cv_VectorOfRect_new, cv_VectorOfRect_delete,
		cv_VectorOfRect_len, cv_VectorOfRect_is_empty,
		cv_VectorOfRect_capacity, cv_VectorOfRect_shrink_to_fit,
		cv_VectorOfRect_reserve, cv_VectorOfRect_remove,
		cv_VectorOfRect_swap, cv_VectorOfRect_clear,
		cv_VectorOfRect_get, cv_VectorOfRect_set,
		cv_VectorOfRect_push, cv_VectorOfRect_insert,
	}
	vector_copy_non_bool! { core::Rect, *const c_void, *mut c_void,
		cv_VectorOfRect_data, cv_VectorOfRect_data_mut,
		cv_VectorOfRect_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Rect> {}
	
	impl core::ToInputArray for VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfRect_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfRect_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfRect2d = core::Vector::<core::Rect2d>;
	
	impl VectorOfRect2d {
		pub fn as_raw_VectorOfRect2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect2d, *const c_void, *mut c_void,
		cv_VectorOfRect2d_new, cv_VectorOfRect2d_delete,
		cv_VectorOfRect2d_len, cv_VectorOfRect2d_is_empty,
		cv_VectorOfRect2d_capacity, cv_VectorOfRect2d_shrink_to_fit,
		cv_VectorOfRect2d_reserve, cv_VectorOfRect2d_remove,
		cv_VectorOfRect2d_swap, cv_VectorOfRect2d_clear,
		cv_VectorOfRect2d_get, cv_VectorOfRect2d_set,
		cv_VectorOfRect2d_push, cv_VectorOfRect2d_insert,
	}
	vector_copy_non_bool! { core::Rect2d, *const c_void, *mut c_void,
		cv_VectorOfRect2d_data, cv_VectorOfRect2d_data_mut,
		cv_VectorOfRect2d_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Rect2d> {}
	
	impl core::ToInputArray for VectorOfRect2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect2d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect2d_input_array(self.as_raw_VectorOfRect2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfRect2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfRect2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfRect2d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect2d_output_array(self.as_raw_mut_VectorOfRect2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfRect2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfRect2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfRect2d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect2d_input_output_array(self.as_raw_mut_VectorOfRect2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfRect2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfRotatedRect = core::Vector::<core::RotatedRect>;
	
	impl VectorOfRotatedRect {
		pub fn as_raw_VectorOfRotatedRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::RotatedRect, *const c_void, *mut c_void,
		cv_VectorOfRotatedRect_new, cv_VectorOfRotatedRect_delete,
		cv_VectorOfRotatedRect_len, cv_VectorOfRotatedRect_is_empty,
		cv_VectorOfRotatedRect_capacity, cv_VectorOfRotatedRect_shrink_to_fit,
		cv_VectorOfRotatedRect_reserve, cv_VectorOfRotatedRect_remove,
		cv_VectorOfRotatedRect_swap, cv_VectorOfRotatedRect_clear,
		cv_VectorOfRotatedRect_get, cv_VectorOfRotatedRect_set,
		cv_VectorOfRotatedRect_push, cv_VectorOfRotatedRect_insert,
	}
	vector_non_copy_or_bool! { core::RotatedRect }
	
	unsafe impl Send for core::Vector::<core::RotatedRect> {}
	
	pub type VectorOfScalar = core::Vector::<core::Scalar>;
	
	impl VectorOfScalar {
		pub fn as_raw_VectorOfScalar(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfScalar(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Scalar, *const c_void, *mut c_void,
		cv_VectorOfScalar_new, cv_VectorOfScalar_delete,
		cv_VectorOfScalar_len, cv_VectorOfScalar_is_empty,
		cv_VectorOfScalar_capacity, cv_VectorOfScalar_shrink_to_fit,
		cv_VectorOfScalar_reserve, cv_VectorOfScalar_remove,
		cv_VectorOfScalar_swap, cv_VectorOfScalar_clear,
		cv_VectorOfScalar_get, cv_VectorOfScalar_set,
		cv_VectorOfScalar_push, cv_VectorOfScalar_insert,
	}
	vector_copy_non_bool! { core::Scalar, *const c_void, *mut c_void,
		cv_VectorOfScalar_data, cv_VectorOfScalar_data_mut,
		cv_VectorOfScalar_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Scalar> {}
	
	impl core::ToInputArray for VectorOfScalar {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfScalar_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfScalar_input_array(self.as_raw_VectorOfScalar()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfScalar {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfScalar {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfScalar_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfScalar_output_array(self.as_raw_mut_VectorOfScalar()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfScalar {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfScalar {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfScalar_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfScalar_input_output_array(self.as_raw_mut_VectorOfScalar()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfScalar {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfSize = core::Vector::<core::Size>;
	
	impl VectorOfSize {
		pub fn as_raw_VectorOfSize(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfSize(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Size, *const c_void, *mut c_void,
		cv_VectorOfSize_new, cv_VectorOfSize_delete,
		cv_VectorOfSize_len, cv_VectorOfSize_is_empty,
		cv_VectorOfSize_capacity, cv_VectorOfSize_shrink_to_fit,
		cv_VectorOfSize_reserve, cv_VectorOfSize_remove,
		cv_VectorOfSize_swap, cv_VectorOfSize_clear,
		cv_VectorOfSize_get, cv_VectorOfSize_set,
		cv_VectorOfSize_push, cv_VectorOfSize_insert,
	}
	vector_copy_non_bool! { core::Size, *const c_void, *mut c_void,
		cv_VectorOfSize_data, cv_VectorOfSize_data_mut,
		cv_VectorOfSize_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Size> {}
	
	impl core::ToInputArray for VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfSize_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfSize {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfSize_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfSize {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfSize_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfString = core::Vector::<String>;
	
	impl VectorOfString {
		pub fn as_raw_VectorOfString(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { String, *const c_void, *mut c_void,
		cv_VectorOfString_new, cv_VectorOfString_delete,
		cv_VectorOfString_len, cv_VectorOfString_is_empty,
		cv_VectorOfString_capacity, cv_VectorOfString_shrink_to_fit,
		cv_VectorOfString_reserve, cv_VectorOfString_remove,
		cv_VectorOfString_swap, cv_VectorOfString_clear,
		cv_VectorOfString_get, cv_VectorOfString_set,
		cv_VectorOfString_push, cv_VectorOfString_insert,
	}
	vector_non_copy_or_bool! { String }
	
	unsafe impl Send for core::Vector::<String> {}
	
	pub type VectorOfUMat = core::Vector::<core::UMat>;
	
	impl VectorOfUMat {
		pub fn as_raw_VectorOfUMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfUMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::UMat, *const c_void, *mut c_void,
		cv_VectorOfUMat_new, cv_VectorOfUMat_delete,
		cv_VectorOfUMat_len, cv_VectorOfUMat_is_empty,
		cv_VectorOfUMat_capacity, cv_VectorOfUMat_shrink_to_fit,
		cv_VectorOfUMat_reserve, cv_VectorOfUMat_remove,
		cv_VectorOfUMat_swap, cv_VectorOfUMat_clear,
		cv_VectorOfUMat_get, cv_VectorOfUMat_set,
		cv_VectorOfUMat_push, cv_VectorOfUMat_insert,
	}
	vector_non_copy_or_bool! { clone core::UMat }
	
	unsafe impl Send for core::Vector::<core::UMat> {}
	
	pub type VectorOfVec4f = core::Vector::<core::Vec4f>;
	
	impl VectorOfVec4f {
		pub fn as_raw_VectorOfVec4f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4f, *const c_void, *mut c_void,
		cv_VectorOfVec4f_new, cv_VectorOfVec4f_delete,
		cv_VectorOfVec4f_len, cv_VectorOfVec4f_is_empty,
		cv_VectorOfVec4f_capacity, cv_VectorOfVec4f_shrink_to_fit,
		cv_VectorOfVec4f_reserve, cv_VectorOfVec4f_remove,
		cv_VectorOfVec4f_swap, cv_VectorOfVec4f_clear,
		cv_VectorOfVec4f_get, cv_VectorOfVec4f_set,
		cv_VectorOfVec4f_push, cv_VectorOfVec4f_insert,
	}
	vector_copy_non_bool! { core::Vec4f, *const c_void, *mut c_void,
		cv_VectorOfVec4f_data, cv_VectorOfVec4f_data_mut,
		cv_VectorOfVec4f_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec4f> {}
	
	impl core::ToInputArray for VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec4f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec4i = core::Vector::<core::Vec4i>;
	
	impl VectorOfVec4i {
		pub fn as_raw_VectorOfVec4i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4i, *const c_void, *mut c_void,
		cv_VectorOfVec4i_new, cv_VectorOfVec4i_delete,
		cv_VectorOfVec4i_len, cv_VectorOfVec4i_is_empty,
		cv_VectorOfVec4i_capacity, cv_VectorOfVec4i_shrink_to_fit,
		cv_VectorOfVec4i_reserve, cv_VectorOfVec4i_remove,
		cv_VectorOfVec4i_swap, cv_VectorOfVec4i_clear,
		cv_VectorOfVec4i_get, cv_VectorOfVec4i_set,
		cv_VectorOfVec4i_push, cv_VectorOfVec4i_insert,
	}
	vector_copy_non_bool! { core::Vec4i, *const c_void, *mut c_void,
		cv_VectorOfVec4i_data, cv_VectorOfVec4i_data_mut,
		cv_VectorOfVec4i_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec4i> {}
	
	impl core::ToInputArray for VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_mut_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec4i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_mut_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec6f = core::Vector::<core::Vec6f>;
	
	impl VectorOfVec6f {
		pub fn as_raw_VectorOfVec6f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec6f, *const c_void, *mut c_void,
		cv_VectorOfVec6f_new, cv_VectorOfVec6f_delete,
		cv_VectorOfVec6f_len, cv_VectorOfVec6f_is_empty,
		cv_VectorOfVec6f_capacity, cv_VectorOfVec6f_shrink_to_fit,
		cv_VectorOfVec6f_reserve, cv_VectorOfVec6f_remove,
		cv_VectorOfVec6f_swap, cv_VectorOfVec6f_clear,
		cv_VectorOfVec6f_get, cv_VectorOfVec6f_set,
		cv_VectorOfVec6f_push, cv_VectorOfVec6f_insert,
	}
	vector_copy_non_bool! { core::Vec6f, *const c_void, *mut c_void,
		cv_VectorOfVec6f_data, cv_VectorOfVec6f_data_mut,
		cv_VectorOfVec6f_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec6f> {}
	
	impl core::ToInputArray for VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec6f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec6f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_mut_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec6f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_mut_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfDMatch = core::Vector::<core::Vector::<core::DMatch>>;
	
	impl VectorOfVectorOfDMatch {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::DMatch>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
		cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
		cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_shrink_to_fit,
		cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
		cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
		cv_VectorOfVectorOfDMatch_get, cv_VectorOfVectorOfDMatch_set,
		cv_VectorOfVectorOfDMatch_push, cv_VectorOfVectorOfDMatch_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::DMatch> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::DMatch>> {}
	
	pub type VectorOfVectorOfKeyPoint = core::Vector::<core::Vector::<core::KeyPoint>>;
	
	impl VectorOfVectorOfKeyPoint {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::KeyPoint>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
		cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
		cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
		cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
		cv_VectorOfVectorOfKeyPoint_get, cv_VectorOfVectorOfKeyPoint_set,
		cv_VectorOfVectorOfKeyPoint_push, cv_VectorOfVectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::KeyPoint> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::KeyPoint>> {}
	
	pub type VectorOfVectorOfMat = core::Vector::<core::Vector::<core::Mat>>;
	
	impl VectorOfVectorOfMat {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Mat>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfMat_new, cv_VectorOfVectorOfMat_delete,
		cv_VectorOfVectorOfMat_len, cv_VectorOfVectorOfMat_is_empty,
		cv_VectorOfVectorOfMat_capacity, cv_VectorOfVectorOfMat_shrink_to_fit,
		cv_VectorOfVectorOfMat_reserve, cv_VectorOfVectorOfMat_remove,
		cv_VectorOfVectorOfMat_swap, cv_VectorOfVectorOfMat_clear,
		cv_VectorOfVectorOfMat_get, cv_VectorOfVectorOfMat_set,
		cv_VectorOfVectorOfMat_push, cv_VectorOfVectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Mat> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Mat>> {}
	
	pub type VectorOfVectorOfPoint = core::Vector::<core::Vector::<core::Point>>;
	
	impl VectorOfVectorOfPoint {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
		cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
		cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_shrink_to_fit,
		cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
		cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
		cv_VectorOfVectorOfPoint_get, cv_VectorOfVectorOfPoint_set,
		cv_VectorOfVectorOfPoint_push, cv_VectorOfVectorOfPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Point> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint2f = core::Vector::<core::Vector::<core::Point2f>>;
	
	impl VectorOfVectorOfPoint2f {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point2f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
		cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
		cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_shrink_to_fit,
		cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
		cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
		cv_VectorOfVectorOfPoint2f_get, cv_VectorOfVectorOfPoint2f_set,
		cv_VectorOfVectorOfPoint2f_push, cv_VectorOfVectorOfPoint2f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Point2f> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point2f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint3d = core::Vector::<core::Vector::<core::Point3d>>;
	
	impl VectorOfVectorOfPoint3d {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3d>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
		cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
		cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_shrink_to_fit,
		cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
		cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
		cv_VectorOfVectorOfPoint3d_get, cv_VectorOfVectorOfPoint3d_set,
		cv_VectorOfVectorOfPoint3d_push, cv_VectorOfVectorOfPoint3d_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Point3d> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3d>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint3i = core::Vector::<core::Vector::<core::Point3i>>;
	
	impl VectorOfVectorOfPoint3i {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
		cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
		cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_shrink_to_fit,
		cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
		cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
		cv_VectorOfVectorOfPoint3i_get, cv_VectorOfVectorOfPoint3i_set,
		cv_VectorOfVectorOfPoint3i_push, cv_VectorOfVectorOfPoint3i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Point3i> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfRange = core::Vector::<core::Vector::<core::Range>>;
	
	impl VectorOfVectorOfRange {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Range>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfRange_new, cv_VectorOfVectorOfRange_delete,
		cv_VectorOfVectorOfRange_len, cv_VectorOfVectorOfRange_is_empty,
		cv_VectorOfVectorOfRange_capacity, cv_VectorOfVectorOfRange_shrink_to_fit,
		cv_VectorOfVectorOfRange_reserve, cv_VectorOfVectorOfRange_remove,
		cv_VectorOfVectorOfRange_swap, cv_VectorOfVectorOfRange_clear,
		cv_VectorOfVectorOfRange_get, cv_VectorOfVectorOfRange_set,
		cv_VectorOfVectorOfRange_push, cv_VectorOfVectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Vector::<core::Range> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Range>> {}
	
	pub type VectorOfVectorOfi32 = core::Vector::<core::Vector::<i32>>;
	
	impl VectorOfVectorOfi32 {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<i32>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
		cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
		cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_shrink_to_fit,
		cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
		cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
		cv_VectorOfVectorOfi32_get, cv_VectorOfVectorOfi32_set,
		cv_VectorOfVectorOfi32_push, cv_VectorOfVectorOfi32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<i32> }
	
	unsafe impl Send for core::Vector::<core::Vector::<i32>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfi8 = core::Vector::<core::Vector::<i8>>;
	
	impl VectorOfVectorOfi8 {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<i8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
		cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
		cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_shrink_to_fit,
		cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
		cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
		cv_VectorOfVectorOfi8_get, cv_VectorOfVectorOfi8_set,
		cv_VectorOfVectorOfi8_push, cv_VectorOfVectorOfi8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<i8> }
	
	unsafe impl Send for core::Vector::<core::Vector::<i8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfu8 = core::Vector::<core::Vector::<u8>>;
	
	impl VectorOfVectorOfu8 {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<u8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
		cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
		cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_shrink_to_fit,
		cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
		cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
		cv_VectorOfVectorOfu8_get, cv_VectorOfVectorOfu8_set,
		cv_VectorOfVectorOfu8_push, cv_VectorOfVectorOfu8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<u8> }
	
	unsafe impl Send for core::Vector::<core::Vector::<u8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfbool = core::Vector::<bool>;
	
	impl VectorOfbool {
		pub fn as_raw_VectorOfbool(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { bool, *const c_void, *mut c_void,
		cv_VectorOfbool_new, cv_VectorOfbool_delete,
		cv_VectorOfbool_len, cv_VectorOfbool_is_empty,
		cv_VectorOfbool_capacity, cv_VectorOfbool_shrink_to_fit,
		cv_VectorOfbool_reserve, cv_VectorOfbool_remove,
		cv_VectorOfbool_swap, cv_VectorOfbool_clear,
		cv_VectorOfbool_get, cv_VectorOfbool_set,
		cv_VectorOfbool_push, cv_VectorOfbool_insert,
	}
	vector_non_copy_or_bool! { clone bool }
	
	unsafe impl Send for core::Vector::<bool> {}
	
	pub type VectorOff32 = core::Vector::<f32>;
	
	impl VectorOff32 {
		pub fn as_raw_VectorOff32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { f32, *const c_void, *mut c_void,
		cv_VectorOff32_new, cv_VectorOff32_delete,
		cv_VectorOff32_len, cv_VectorOff32_is_empty,
		cv_VectorOff32_capacity, cv_VectorOff32_shrink_to_fit,
		cv_VectorOff32_reserve, cv_VectorOff32_remove,
		cv_VectorOff32_swap, cv_VectorOff32_clear,
		cv_VectorOff32_get, cv_VectorOff32_set,
		cv_VectorOff32_push, cv_VectorOff32_insert,
	}
	vector_copy_non_bool! { f32, *const c_void, *mut c_void,
		cv_VectorOff32_data, cv_VectorOff32_data_mut,
		cv_VectorOff32_clone,
	}
	
	unsafe impl Send for core::Vector::<f32> {}
	
	impl core::ToInputArray for VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOff64 = core::Vector::<f64>;
	
	impl VectorOff64 {
		pub fn as_raw_VectorOff64(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { f64, *const c_void, *mut c_void,
		cv_VectorOff64_new, cv_VectorOff64_delete,
		cv_VectorOff64_len, cv_VectorOff64_is_empty,
		cv_VectorOff64_capacity, cv_VectorOff64_shrink_to_fit,
		cv_VectorOff64_reserve, cv_VectorOff64_remove,
		cv_VectorOff64_swap, cv_VectorOff64_clear,
		cv_VectorOff64_get, cv_VectorOff64_set,
		cv_VectorOff64_push, cv_VectorOff64_insert,
	}
	vector_copy_non_bool! { f64, *const c_void, *mut c_void,
		cv_VectorOff64_data, cv_VectorOff64_data_mut,
		cv_VectorOff64_clone,
	}
	
	unsafe impl Send for core::Vector::<f64> {}
	
	impl core::ToInputArray for VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff64_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff64_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff64_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfi32 = core::Vector::<i32>;
	
	impl VectorOfi32 {
		pub fn as_raw_VectorOfi32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { i32, *const c_void, *mut c_void,
		cv_VectorOfi32_new, cv_VectorOfi32_delete,
		cv_VectorOfi32_len, cv_VectorOfi32_is_empty,
		cv_VectorOfi32_capacity, cv_VectorOfi32_shrink_to_fit,
		cv_VectorOfi32_reserve, cv_VectorOfi32_remove,
		cv_VectorOfi32_swap, cv_VectorOfi32_clear,
		cv_VectorOfi32_get, cv_VectorOfi32_set,
		cv_VectorOfi32_push, cv_VectorOfi32_insert,
	}
	vector_copy_non_bool! { i32, *const c_void, *mut c_void,
		cv_VectorOfi32_data, cv_VectorOfi32_data_mut,
		cv_VectorOfi32_clone,
	}
	
	unsafe impl Send for core::Vector::<i32> {}
	
	impl core::ToInputArray for VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfi8 = core::Vector::<i8>;
	
	impl VectorOfi8 {
		pub fn as_raw_VectorOfi8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { i8, *const c_void, *mut c_void,
		cv_VectorOfi8_new, cv_VectorOfi8_delete,
		cv_VectorOfi8_len, cv_VectorOfi8_is_empty,
		cv_VectorOfi8_capacity, cv_VectorOfi8_shrink_to_fit,
		cv_VectorOfi8_reserve, cv_VectorOfi8_remove,
		cv_VectorOfi8_swap, cv_VectorOfi8_clear,
		cv_VectorOfi8_get, cv_VectorOfi8_set,
		cv_VectorOfi8_push, cv_VectorOfi8_insert,
	}
	vector_copy_non_bool! { i8, *const c_void, *mut c_void,
		cv_VectorOfi8_data, cv_VectorOfi8_data_mut,
		cv_VectorOfi8_clone,
	}
	
	unsafe impl Send for core::Vector::<i8> {}
	
	impl core::ToInputArray for VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfsize_t = core::Vector::<size_t>;
	
	impl VectorOfsize_t {
		pub fn as_raw_VectorOfsize_t(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { size_t, *const c_void, *mut c_void,
		cv_VectorOfsize_t_new, cv_VectorOfsize_t_delete,
		cv_VectorOfsize_t_len, cv_VectorOfsize_t_is_empty,
		cv_VectorOfsize_t_capacity, cv_VectorOfsize_t_shrink_to_fit,
		cv_VectorOfsize_t_reserve, cv_VectorOfsize_t_remove,
		cv_VectorOfsize_t_swap, cv_VectorOfsize_t_clear,
		cv_VectorOfsize_t_get, cv_VectorOfsize_t_set,
		cv_VectorOfsize_t_push, cv_VectorOfsize_t_insert,
	}
	vector_copy_non_bool! { size_t, *const c_void, *mut c_void,
		cv_VectorOfsize_t_data, cv_VectorOfsize_t_data_mut,
		cv_VectorOfsize_t_clone,
	}
	
	unsafe impl Send for core::Vector::<size_t> {}
	
	pub type VectorOfu8 = core::Vector::<u8>;
	
	impl VectorOfu8 {
		pub fn as_raw_VectorOfu8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { u8, *const c_void, *mut c_void,
		cv_VectorOfu8_new, cv_VectorOfu8_delete,
		cv_VectorOfu8_len, cv_VectorOfu8_is_empty,
		cv_VectorOfu8_capacity, cv_VectorOfu8_shrink_to_fit,
		cv_VectorOfu8_reserve, cv_VectorOfu8_remove,
		cv_VectorOfu8_swap, cv_VectorOfu8_clear,
		cv_VectorOfu8_get, cv_VectorOfu8_set,
		cv_VectorOfu8_push, cv_VectorOfu8_insert,
	}
	vector_copy_non_bool! { u8, *const c_void, *mut c_void,
		cv_VectorOfu8_data, cv_VectorOfu8_data_mut,
		cv_VectorOfu8_clone,
	}
	
	unsafe impl Send for core::Vector::<u8> {}
	
	impl core::ToInputArray for VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfu8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfu8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfu8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
}
pub use core_types::*;

mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAbsLayer = core::Ptr::<dyn crate::dnn::AbsLayer>;
	
	ptr_extern! { dyn crate::dnn::AbsLayer,
		cv_PtrOfAbsLayer_delete, cv_PtrOfAbsLayer_get_inner_ptr, cv_PtrOfAbsLayer_get_inner_ptr_mut
	}
	
	impl PtrOfAbsLayer {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayer for PtrOfAbsLayer {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfAbsLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayer = core::Ptr::<dyn crate::dnn::ActivationLayer>;
	
	ptr_extern! { dyn crate::dnn::ActivationLayer,
		cv_PtrOfActivationLayer_delete, cv_PtrOfActivationLayer_get_inner_ptr, cv_PtrOfActivationLayer_get_inner_ptr_mut
	}
	
	impl PtrOfActivationLayer {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfActivationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfActivationLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfActivationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBNLLLayer = core::Ptr::<dyn crate::dnn::BNLLLayer>;
	
	ptr_extern! { dyn crate::dnn::BNLLLayer,
		cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr, cv_PtrOfBNLLLayer_get_inner_ptr_mut
	}
	
	impl PtrOfBNLLLayer {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfBNLLLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayer for PtrOfBNLLLayer {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendNode = core::Ptr::<crate::dnn::BackendNode>;
	
	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrOfBackendNode_delete, cv_PtrOfBackendNode_get_inner_ptr, cv_PtrOfBackendNode_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrOfBackendNode_new }
	
	impl PtrOfBackendNode {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendNodeTrait for PtrOfBackendNode {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendWrapper = core::Ptr::<dyn crate::dnn::BackendWrapper>;
	
	ptr_extern! { dyn crate::dnn::BackendWrapper,
		cv_PtrOfBackendWrapper_delete, cv_PtrOfBackendWrapper_get_inner_ptr, cv_PtrOfBackendWrapper_get_inner_ptr_mut
	}
	
	impl PtrOfBackendWrapper {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseConvolutionLayer = core::Ptr::<crate::dnn::BaseConvolutionLayer>;
	
	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr, cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrOfBaseConvolutionLayer_new }
	
	impl PtrOfBaseConvolutionLayer {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayer = core::Ptr::<dyn crate::dnn::BatchNormLayer>;
	
	ptr_extern! { dyn crate::dnn::BatchNormLayer,
		cv_PtrOfBatchNormLayer_delete, cv_PtrOfBatchNormLayer_get_inner_ptr, cv_PtrOfBatchNormLayer_get_inner_ptr_mut
	}
	
	impl PtrOfBatchNormLayer {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfBatchNormLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayer for PtrOfBatchNormLayer {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfConcatLayer = core::Ptr::<crate::dnn::ConcatLayer>;
	
	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr, cv_PtrOfConcatLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrOfConcatLayer_new }
	
	impl PtrOfConcatLayer {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayerTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionOutputLayer = core::Ptr::<crate::dnn::DetectionOutputLayer>;
	
	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrOfDetectionOutputLayer_delete, cv_PtrOfDetectionOutputLayer_get_inner_ptr, cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrOfDetectionOutputLayer_new }
	
	impl PtrOfDetectionOutputLayer {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::DetectionOutputLayerTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfELULayer = core::Ptr::<dyn crate::dnn::ELULayer>;
	
	ptr_extern! { dyn crate::dnn::ELULayer,
		cv_PtrOfELULayer_delete, cv_PtrOfELULayer_get_inner_ptr, cv_PtrOfELULayer_get_inner_ptr_mut
	}
	
	impl PtrOfELULayer {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfELULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfELULayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ELULayer for PtrOfELULayer {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfELULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayer = core::Ptr::<crate::dnn::EltwiseLayer>;
	
	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr, cv_PtrOfEltwiseLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrOfEltwiseLayer_new }
	
	impl PtrOfEltwiseLayer {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlattenLayer = core::Ptr::<crate::dnn::FlattenLayer>;
	
	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrOfFlattenLayer_delete, cv_PtrOfFlattenLayer_get_inner_ptr, cv_PtrOfFlattenLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrOfFlattenLayer_new }
	
	impl PtrOfFlattenLayer {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::FlattenLayerTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayer = core::Ptr::<crate::dnn::InnerProductLayer>;
	
	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr, cv_PtrOfInnerProductLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrOfInnerProductLayer_new }
	
	impl PtrOfInnerProductLayer {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLRNLayer = core::Ptr::<crate::dnn::LRNLayer>;
	
	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr, cv_PtrOfLRNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrOfLRNLayer_new }
	
	impl PtrOfLRNLayer {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayerTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSTMLayer = core::Ptr::<dyn crate::dnn::LSTMLayer>;
	
	ptr_extern! { dyn crate::dnn::LSTMLayer,
		cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr, cv_PtrOfLSTMLayer_get_inner_ptr_mut
	}
	
	impl PtrOfLSTMLayer {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLSTMLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLSTMLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLayer = core::Ptr::<crate::dnn::Layer>;
	
	ptr_extern! { crate::dnn::Layer,
		cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr, cv_PtrOfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrOfLayer_new }
	
	impl PtrOfLayer {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMVNLayer = core::Ptr::<crate::dnn::MVNLayer>;
	
	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr, cv_PtrOfMVNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrOfMVNLayer_new }
	
	impl PtrOfMVNLayer {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayerTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMaxUnpoolLayer = core::Ptr::<crate::dnn::MaxUnpoolLayer>;
	
	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrOfMaxUnpoolLayer_delete, cv_PtrOfMaxUnpoolLayer_get_inner_ptr, cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrOfMaxUnpoolLayer_new }
	
	impl PtrOfMaxUnpoolLayer {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalizeBBoxLayer = core::Ptr::<crate::dnn::NormalizeBBoxLayer>;
	
	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrOfNormalizeBBoxLayer_delete, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrOfNormalizeBBoxLayer_new }
	
	impl PtrOfNormalizeBBoxLayer {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPaddingLayer = core::Ptr::<crate::dnn::PaddingLayer>;
	
	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrOfPaddingLayer_delete, cv_PtrOfPaddingLayer_get_inner_ptr, cv_PtrOfPaddingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrOfPaddingLayer_new }
	
	impl PtrOfPaddingLayer {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PaddingLayerTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPermuteLayer = core::Ptr::<crate::dnn::PermuteLayer>;
	
	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrOfPermuteLayer_delete, cv_PtrOfPermuteLayer_get_inner_ptr, cv_PtrOfPermuteLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrOfPermuteLayer_new }
	
	impl PtrOfPermuteLayer {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PermuteLayerTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayer = core::Ptr::<crate::dnn::PoolingLayer>;
	
	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr, cv_PtrOfPoolingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrOfPoolingLayer_new }
	
	impl PtrOfPoolingLayer {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPowerLayer = core::Ptr::<dyn crate::dnn::PowerLayer>;
	
	ptr_extern! { dyn crate::dnn::PowerLayer,
		cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr, cv_PtrOfPowerLayer_get_inner_ptr_mut
	}
	
	impl PtrOfPowerLayer {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfPowerLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayer for PtrOfPowerLayer {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPriorBoxLayer = core::Ptr::<crate::dnn::PriorBoxLayer>;
	
	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrOfPriorBoxLayer_delete, cv_PtrOfPriorBoxLayer_get_inner_ptr, cv_PtrOfPriorBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrOfPriorBoxLayer_new }
	
	impl PtrOfPriorBoxLayer {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PriorBoxLayerTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfProposalLayer = core::Ptr::<crate::dnn::ProposalLayer>;
	
	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrOfProposalLayer_delete, cv_PtrOfProposalLayer_get_inner_ptr, cv_PtrOfProposalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrOfProposalLayer_new }
	
	impl PtrOfProposalLayer {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ProposalLayerTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRNNLayer = core::Ptr::<dyn crate::dnn::RNNLayer>;
	
	ptr_extern! { dyn crate::dnn::RNNLayer,
		cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr, cv_PtrOfRNNLayer_get_inner_ptr_mut
	}
	
	impl PtrOfRNNLayer {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRNNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRNNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayer for PtrOfRNNLayer {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLU6Layer = core::Ptr::<dyn crate::dnn::ReLU6Layer>;
	
	ptr_extern! { dyn crate::dnn::ReLU6Layer,
		cv_PtrOfReLU6Layer_delete, cv_PtrOfReLU6Layer_get_inner_ptr, cv_PtrOfReLU6Layer_get_inner_ptr_mut
	}
	
	impl PtrOfReLU6Layer {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfReLU6Layer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReLU6Layer for PtrOfReLU6Layer {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLULayer = core::Ptr::<dyn crate::dnn::ReLULayer>;
	
	ptr_extern! { dyn crate::dnn::ReLULayer,
		cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr, cv_PtrOfReLULayer_get_inner_ptr_mut
	}
	
	impl PtrOfReLULayer {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfReLULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfReLULayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReLULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayer for PtrOfReLULayer {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRegionLayer = core::Ptr::<crate::dnn::RegionLayer>;
	
	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrOfRegionLayer_delete, cv_PtrOfRegionLayer_get_inner_ptr, cv_PtrOfRegionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrOfRegionLayer_new }
	
	impl PtrOfRegionLayer {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::RegionLayerTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReorgLayer = core::Ptr::<crate::dnn::ReorgLayer>;
	
	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrOfReorgLayer_delete, cv_PtrOfReorgLayer_get_inner_ptr, cv_PtrOfReorgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrOfReorgLayer_new }
	
	impl PtrOfReorgLayer {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReorgLayerTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReshapeLayer = core::Ptr::<crate::dnn::ReshapeLayer>;
	
	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr, cv_PtrOfReshapeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrOfReshapeLayer_new }
	
	impl PtrOfReshapeLayer {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayerTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfResizeLayer = core::Ptr::<crate::dnn::ResizeLayer>;
	
	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrOfResizeLayer_delete, cv_PtrOfResizeLayer_get_inner_ptr, cv_PtrOfResizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrOfResizeLayer_new }
	
	impl PtrOfResizeLayer {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ResizeLayerTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayer = core::Ptr::<crate::dnn::ScaleLayer>;
	
	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrOfScaleLayer_delete, cv_PtrOfScaleLayer_get_inner_ptr, cv_PtrOfScaleLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrOfScaleLayer_new }
	
	impl PtrOfScaleLayer {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSigmoidLayer = core::Ptr::<dyn crate::dnn::SigmoidLayer>;
	
	ptr_extern! { dyn crate::dnn::SigmoidLayer,
		cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr, cv_PtrOfSigmoidLayer_get_inner_ptr_mut
	}
	
	impl PtrOfSigmoidLayer {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfSigmoidLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayer for PtrOfSigmoidLayer {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSliceLayer = core::Ptr::<crate::dnn::SliceLayer>;
	
	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr, cv_PtrOfSliceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrOfSliceLayer_new }
	
	impl PtrOfSliceLayer {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayerTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayer = core::Ptr::<crate::dnn::SoftmaxLayer>;
	
	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr, cv_PtrOfSoftmaxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrOfSoftmaxLayer_new }
	
	impl PtrOfSoftmaxLayer {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSplitLayer = core::Ptr::<crate::dnn::SplitLayer>;
	
	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr, cv_PtrOfSplitLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrOfSplitLayer_new }
	
	impl PtrOfSplitLayer {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayerTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanHLayer = core::Ptr::<dyn crate::dnn::TanHLayer>;
	
	ptr_extern! { dyn crate::dnn::TanHLayer,
		cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr, cv_PtrOfTanHLayer_get_inner_ptr_mut
	}
	
	impl PtrOfTanHLayer {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfTanHLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayer for PtrOfTanHLayer {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfMatShape = core::Vector::<crate::dnn::MatShape>;
	
	impl VectorOfMatShape {
		pub fn as_raw_VectorOfMatShape(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	pub type VectorOfPtrOfBackendWrapper = core::Vector::<core::Ptr::<dyn crate::dnn::BackendWrapper>>;
	
	impl VectorOfPtrOfBackendWrapper {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr::<dyn crate::dnn::BackendWrapper>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfBackendWrapper_new, cv_VectorOfPtrOfBackendWrapper_delete,
		cv_VectorOfPtrOfBackendWrapper_len, cv_VectorOfPtrOfBackendWrapper_is_empty,
		cv_VectorOfPtrOfBackendWrapper_capacity, cv_VectorOfPtrOfBackendWrapper_shrink_to_fit,
		cv_VectorOfPtrOfBackendWrapper_reserve, cv_VectorOfPtrOfBackendWrapper_remove,
		cv_VectorOfPtrOfBackendWrapper_swap, cv_VectorOfPtrOfBackendWrapper_clear,
		cv_VectorOfPtrOfBackendWrapper_get, cv_VectorOfPtrOfBackendWrapper_set,
		cv_VectorOfPtrOfBackendWrapper_push, cv_VectorOfPtrOfBackendWrapper_insert,
	}
	vector_non_copy_or_bool! { core::Ptr::<dyn crate::dnn::BackendWrapper> }
	
	unsafe impl Send for core::Vector::<core::Ptr::<dyn crate::dnn::BackendWrapper>> {}
	
	pub type VectorOfPtrOfLayer = core::Vector::<core::Ptr::<crate::dnn::Layer>>;
	
	impl VectorOfPtrOfLayer {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr::<crate::dnn::Layer>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfLayer_new, cv_VectorOfPtrOfLayer_delete,
		cv_VectorOfPtrOfLayer_len, cv_VectorOfPtrOfLayer_is_empty,
		cv_VectorOfPtrOfLayer_capacity, cv_VectorOfPtrOfLayer_shrink_to_fit,
		cv_VectorOfPtrOfLayer_reserve, cv_VectorOfPtrOfLayer_remove,
		cv_VectorOfPtrOfLayer_swap, cv_VectorOfPtrOfLayer_clear,
		cv_VectorOfPtrOfLayer_get, cv_VectorOfPtrOfLayer_set,
		cv_VectorOfPtrOfLayer_push, cv_VectorOfPtrOfLayer_insert,
	}
	vector_non_copy_or_bool! { core::Ptr::<crate::dnn::Layer> }
	
	unsafe impl Send for core::Vector::<core::Ptr::<crate::dnn::Layer>> {}
	
	pub type VectorOfTarget = core::Vector::<crate::dnn::Target>;
	
	impl VectorOfTarget {
		pub fn as_raw_VectorOfTarget(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Target, *const c_void, *mut c_void,
		cv_VectorOfTarget_new, cv_VectorOfTarget_delete,
		cv_VectorOfTarget_len, cv_VectorOfTarget_is_empty,
		cv_VectorOfTarget_capacity, cv_VectorOfTarget_shrink_to_fit,
		cv_VectorOfTarget_reserve, cv_VectorOfTarget_remove,
		cv_VectorOfTarget_swap, cv_VectorOfTarget_clear,
		cv_VectorOfTarget_get, cv_VectorOfTarget_set,
		cv_VectorOfTarget_push, cv_VectorOfTarget_insert,
	}
	vector_copy_non_bool! { crate::dnn::Target, *const c_void, *mut c_void,
		cv_VectorOfTarget_data, cv_VectorOfTarget_data_mut,
		cv_VectorOfTarget_clone,
	}
	
	unsafe impl Send for core::Vector::<crate::dnn::Target> {}
	
	pub type VectorOfVectorOfMatShape = core::Vector::<core::Vector::<crate::dnn::MatShape>>;
	
	impl VectorOfVectorOfMatShape {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<crate::dnn::MatShape>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfMatShape_new, cv_VectorOfVectorOfMatShape_delete,
		cv_VectorOfVectorOfMatShape_len, cv_VectorOfVectorOfMatShape_is_empty,
		cv_VectorOfVectorOfMatShape_capacity, cv_VectorOfVectorOfMatShape_shrink_to_fit,
		cv_VectorOfVectorOfMatShape_reserve, cv_VectorOfVectorOfMatShape_remove,
		cv_VectorOfVectorOfMatShape_swap, cv_VectorOfVectorOfMatShape_clear,
		cv_VectorOfVectorOfMatShape_get, cv_VectorOfVectorOfMatShape_set,
		cv_VectorOfVectorOfMatShape_push, cv_VectorOfVectorOfMatShape_insert,
	}
	vector_non_copy_or_bool! { core::Vector::<crate::dnn::MatShape> }
	
	unsafe impl Send for core::Vector::<core::Vector::<crate::dnn::MatShape>> {}
	
}
pub use dnn_types::*;

mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAKAZE = core::Ptr::<dyn crate::features2d::AKAZE>;
	
	ptr_extern! { dyn crate::features2d::AKAZE,
		cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr, cv_PtrOfAKAZE_get_inner_ptr_mut
	}
	
	impl PtrOfAKAZE {
		#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZE for PtrOfAKAZE {
		#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAKAZE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAgastFeatureDetector = core::Ptr::<dyn crate::features2d::AgastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::AgastFeatureDetector,
		cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr, cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfAgastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBFMatcher = core::Ptr::<crate::features2d::BFMatcher>;
	
	ptr_extern! { crate::features2d::BFMatcher,
		cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr, cv_PtrOfBFMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrOfBFMatcher_new }
	
	impl PtrOfBFMatcher {
		#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBFMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTrait for PtrOfBFMatcher {
		#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfBFMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBRISK = core::Ptr::<crate::features2d::BRISK>;
	
	ptr_extern! { crate::features2d::BRISK,
		cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr, cv_PtrOfBRISK_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BRISK, cv_PtrOfBRISK_new }
	
	impl PtrOfBRISK {
		#[inline] pub fn as_raw_PtrOfBRISK(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBRISK {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTrait for PtrOfBRISK {
		#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBRISK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDescriptorMatcher = core::Ptr::<dyn crate::features2d::DescriptorMatcher>;
	
	ptr_extern! { dyn crate::features2d::DescriptorMatcher,
		cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr, cv_PtrOfDescriptorMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfDescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastFeatureDetector = core::Ptr::<dyn crate::features2d::FastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::FastFeatureDetector,
		cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr, cv_PtrOfFastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfFastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFeature2D = core::Ptr::<crate::features2d::Feature2D>;
	
	ptr_extern! { crate::features2d::Feature2D,
		cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr, cv_PtrOfFeature2D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrOfFeature2D_new }
	
	impl PtrOfFeature2D {
		#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFeature2D {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFeature2D {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlannBasedMatcher = core::Ptr::<crate::features2d::FlannBasedMatcher>;
	
	ptr_extern! { crate::features2d::FlannBasedMatcher,
		cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr, cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::FlannBasedMatcher, cv_PtrOfFlannBasedMatcher_new }
	
	impl PtrOfFlannBasedMatcher {
		#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGFTTDetector = core::Ptr::<dyn crate::features2d::GFTTDetector>;
	
	ptr_extern! { dyn crate::features2d::GFTTDetector,
		cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr, cv_PtrOfGFTTDetector_get_inner_ptr_mut
	}
	
	impl PtrOfGFTTDetector {
		#[inline] pub fn as_raw_PtrOfGFTTDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGFTTDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
		#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKAZE = core::Ptr::<dyn crate::features2d::KAZE>;
	
	ptr_extern! { dyn crate::features2d::KAZE,
		cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr, cv_PtrOfKAZE_get_inner_ptr_mut
	}
	
	impl PtrOfKAZE {
		#[inline] pub fn as_raw_PtrOfKAZE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfKAZE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfKAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::KAZE for PtrOfKAZE {
		#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMSER = core::Ptr::<dyn crate::features2d::MSER>;
	
	ptr_extern! { dyn crate::features2d::MSER,
		cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr, cv_PtrOfMSER_get_inner_ptr_mut
	}
	
	impl PtrOfMSER {
		#[inline] pub fn as_raw_PtrOfMSER(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSER {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSER {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::MSER for PtrOfMSER {
		#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfORB = core::Ptr::<dyn crate::features2d::ORB>;
	
	ptr_extern! { dyn crate::features2d::ORB,
		cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr, cv_PtrOfORB_get_inner_ptr_mut
	}
	
	impl PtrOfORB {
		#[inline] pub fn as_raw_PtrOfORB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfORB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::ORB for PtrOfORB {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleBlobDetector = core::Ptr::<crate::features2d::SimpleBlobDetector>;
	
	ptr_extern! { crate::features2d::SimpleBlobDetector,
		cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr, cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SimpleBlobDetector, cv_PtrOfSimpleBlobDetector_new }
	
	impl PtrOfSimpleBlobDetector {
		#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use features2d_types::*;

mod flann_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfIndexParams = core::Ptr::<crate::flann::IndexParams>;
	
	ptr_extern! { crate::flann::IndexParams,
		cv_PtrOfIndexParams_delete, cv_PtrOfIndexParams_get_inner_ptr, cv_PtrOfIndexParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::IndexParams, cv_PtrOfIndexParams_new }
	
	impl PtrOfIndexParams {
		#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfIndexParams {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSearchParams = core::Ptr::<crate::flann::SearchParams>;
	
	ptr_extern! { crate::flann::SearchParams,
		cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr, cv_PtrOfSearchParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrOfSearchParams_new }
	
	impl PtrOfSearchParams {
		#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfSearchParams {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::flann::SearchParamsTrait for PtrOfSearchParams {
		#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfFlannIndexType = core::Vector::<crate::flann::FlannIndexType>;
	
	impl VectorOfFlannIndexType {
		pub fn as_raw_VectorOfFlannIndexType(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
		cv_VectorOfFlannIndexType_new, cv_VectorOfFlannIndexType_delete,
		cv_VectorOfFlannIndexType_len, cv_VectorOfFlannIndexType_is_empty,
		cv_VectorOfFlannIndexType_capacity, cv_VectorOfFlannIndexType_shrink_to_fit,
		cv_VectorOfFlannIndexType_reserve, cv_VectorOfFlannIndexType_remove,
		cv_VectorOfFlannIndexType_swap, cv_VectorOfFlannIndexType_clear,
		cv_VectorOfFlannIndexType_get, cv_VectorOfFlannIndexType_set,
		cv_VectorOfFlannIndexType_push, cv_VectorOfFlannIndexType_insert,
	}
	vector_copy_non_bool! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
		cv_VectorOfFlannIndexType_data, cv_VectorOfFlannIndexType_data_mut,
		cv_VectorOfFlannIndexType_clone,
	}
	
	unsafe impl Send for core::Vector::<crate::flann::FlannIndexType> {}
	
}
pub use flann_types::*;

mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCLAHE = core::Ptr::<dyn crate::imgproc::CLAHE>;
	
	ptr_extern! { dyn crate::imgproc::CLAHE,
		cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr, cv_PtrOfCLAHE_get_inner_ptr_mut
	}
	
	impl PtrOfCLAHE {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCLAHE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHE for PtrOfCLAHE {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughBallard = core::Ptr::<dyn crate::imgproc::GeneralizedHoughBallard>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughBallard,
		cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut
	}
	
	impl PtrOfGeneralizedHoughBallard {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughGuil = core::Ptr::<dyn crate::imgproc::GeneralizedHoughGuil>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughGuil,
		cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut
	}
	
	impl PtrOfGeneralizedHoughGuil {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLineSegmentDetector = core::Ptr::<dyn crate::imgproc::LineSegmentDetector>;
	
	ptr_extern! { dyn crate::imgproc::LineSegmentDetector,
		cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr, cv_PtrOfLineSegmentDetector_get_inner_ptr_mut
	}
	
	impl PtrOfLineSegmentDetector {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use imgproc_types::*;

mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfANN_MLP = core::Ptr::<dyn crate::ml::ANN_MLP>;
	
	ptr_extern! { dyn crate::ml::ANN_MLP,
		cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr, cv_PtrOfANN_MLP_get_inner_ptr_mut
	}
	
	impl PtrOfANN_MLP {
		#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfANN_MLP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::ANN_MLP for PtrOfANN_MLP {
		#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfANN_MLP {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoost = core::Ptr::<dyn crate::ml::Boost>;
	
	ptr_extern! { dyn crate::ml::Boost,
		cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr, cv_PtrOfBoost_get_inner_ptr_mut
	}
	
	impl PtrOfBoost {
		#[inline] pub fn as_raw_PtrOfBoost(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoost {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::Boost for PtrOfBoost {
		#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfBoost {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfBoost {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTrees = core::Ptr::<dyn crate::ml::DTrees>;
	
	ptr_extern! { dyn crate::ml::DTrees,
		cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr, cv_PtrOfDTrees_get_inner_ptr_mut
	}
	
	impl PtrOfDTrees {
		#[inline] pub fn as_raw_PtrOfDTrees(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDTrees {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfDTrees {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfDTrees {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEM = core::Ptr::<dyn crate::ml::EM>;
	
	ptr_extern! { dyn crate::ml::EM,
		cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr, cv_PtrOfEM_get_inner_ptr_mut
	}
	
	impl PtrOfEM {
		#[inline] pub fn as_raw_PtrOfEM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::EM for PtrOfEM {
		#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfEM {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKNearest = core::Ptr::<dyn crate::ml::KNearest>;
	
	ptr_extern! { dyn crate::ml::KNearest,
		cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr, cv_PtrOfKNearest_get_inner_ptr_mut
	}
	
	impl PtrOfKNearest {
		#[inline] pub fn as_raw_PtrOfKNearest(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfKNearest {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::KNearest for PtrOfKNearest {
		#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfKNearest {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogisticRegression = core::Ptr::<dyn crate::ml::LogisticRegression>;
	
	ptr_extern! { dyn crate::ml::LogisticRegression,
		cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr, cv_PtrOfLogisticRegression_get_inner_ptr_mut
	}
	
	impl PtrOfLogisticRegression {
		#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLogisticRegression {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
		#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfLogisticRegression {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalBayesClassifier = core::Ptr::<dyn crate::ml::NormalBayesClassifier>;
	
	ptr_extern! { dyn crate::ml::NormalBayesClassifier,
		cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr, cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfNormalBayesClassifier {
		#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParamGrid = core::Ptr::<crate::ml::ParamGrid>;
	
	ptr_extern! { crate::ml::ParamGrid,
		cv_PtrOfParamGrid_delete, cv_PtrOfParamGrid_get_inner_ptr, cv_PtrOfParamGrid_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrOfParamGrid_new }
	
	impl PtrOfParamGrid {
		#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::ParamGridTrait for PtrOfParamGrid {
		#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRTrees = core::Ptr::<dyn crate::ml::RTrees>;
	
	ptr_extern! { dyn crate::ml::RTrees,
		cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr, cv_PtrOfRTrees_get_inner_ptr_mut
	}
	
	impl PtrOfRTrees {
		#[inline] pub fn as_raw_PtrOfRTrees(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRTrees {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfRTrees {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::RTrees for PtrOfRTrees {
		#[inline] fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfRTrees {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM = core::Ptr::<dyn crate::ml::SVM>;
	
	ptr_extern! { dyn crate::ml::SVM,
		cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr, cv_PtrOfSVM_get_inner_ptr_mut
	}
	
	impl PtrOfSVM {
		#[inline] pub fn as_raw_PtrOfSVM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVM for PtrOfSVM {
		#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfSVM {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVMSGD = core::Ptr::<dyn crate::ml::SVMSGD>;
	
	ptr_extern! { dyn crate::ml::SVMSGD,
		cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr, cv_PtrOfSVMSGD_get_inner_ptr_mut
	}
	
	impl PtrOfSVMSGD {
		#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVMSGD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVMSGD for PtrOfSVMSGD {
		#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfSVMSGD {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM_Kernel = core::Ptr::<dyn crate::ml::SVM_Kernel>;
	
	ptr_extern! { dyn crate::ml::SVM_Kernel,
		cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr, cv_PtrOfSVM_Kernel_get_inner_ptr_mut
	}
	
	impl PtrOfSVM_Kernel {
		#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM_Kernel {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVM_Kernel for PtrOfSVM_Kernel {
		#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrainData = core::Ptr::<dyn crate::ml::TrainData>;
	
	ptr_extern! { dyn crate::ml::TrainData,
		cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr, cv_PtrOfTrainData_get_inner_ptr_mut
	}
	
	impl PtrOfTrainData {
		#[inline] pub fn as_raw_PtrOfTrainData(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::TrainData for PtrOfTrainData {
		#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDTrees_Node = core::Vector::<crate::ml::DTrees_Node>;
	
	impl VectorOfDTrees_Node {
		pub fn as_raw_VectorOfDTrees_Node(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Node, *const c_void, *mut c_void,
		cv_VectorOfDTrees_Node_new, cv_VectorOfDTrees_Node_delete,
		cv_VectorOfDTrees_Node_len, cv_VectorOfDTrees_Node_is_empty,
		cv_VectorOfDTrees_Node_capacity, cv_VectorOfDTrees_Node_shrink_to_fit,
		cv_VectorOfDTrees_Node_reserve, cv_VectorOfDTrees_Node_remove,
		cv_VectorOfDTrees_Node_swap, cv_VectorOfDTrees_Node_clear,
		cv_VectorOfDTrees_Node_get, cv_VectorOfDTrees_Node_set,
		cv_VectorOfDTrees_Node_push, cv_VectorOfDTrees_Node_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Node }
	
	unsafe impl Send for core::Vector::<crate::ml::DTrees_Node> {}
	
	pub type VectorOfDTrees_Split = core::Vector::<crate::ml::DTrees_Split>;
	
	impl VectorOfDTrees_Split {
		pub fn as_raw_VectorOfDTrees_Split(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Split, *const c_void, *mut c_void,
		cv_VectorOfDTrees_Split_new, cv_VectorOfDTrees_Split_delete,
		cv_VectorOfDTrees_Split_len, cv_VectorOfDTrees_Split_is_empty,
		cv_VectorOfDTrees_Split_capacity, cv_VectorOfDTrees_Split_shrink_to_fit,
		cv_VectorOfDTrees_Split_reserve, cv_VectorOfDTrees_Split_remove,
		cv_VectorOfDTrees_Split_swap, cv_VectorOfDTrees_Split_clear,
		cv_VectorOfDTrees_Split_get, cv_VectorOfDTrees_Split_set,
		cv_VectorOfDTrees_Split_push, cv_VectorOfDTrees_Split_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Split }
	
	unsafe impl Send for core::Vector::<crate::ml::DTrees_Split> {}
	
}
pub use ml_types::*;

mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBaseCascadeClassifier = core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier,
		cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfBaseCascadeClassifier {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr_mut
	}
	
	impl PtrOfBaseCascadeClassifier_MaskGenerator {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfBaseCascadeClassifier_MaskGenerator {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr::<dyn crate::objdetect::DetectionBasedTracker_IDetector>;
	
	ptr_extern! { dyn crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut
	}
	
	impl PtrOfDetectionBasedTracker_IDetector {
		#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetector for PtrOfDetectionBasedTracker_IDetector {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject>;
	
	impl VectorOfDetectionBasedTracker_ExtObject {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject, *const c_void, *mut c_void,
		cv_VectorOfDetectionBasedTracker_ExtObject_new, cv_VectorOfDetectionBasedTracker_ExtObject_delete,
		cv_VectorOfDetectionBasedTracker_ExtObject_len, cv_VectorOfDetectionBasedTracker_ExtObject_is_empty,
		cv_VectorOfDetectionBasedTracker_ExtObject_capacity, cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_ExtObject_reserve, cv_VectorOfDetectionBasedTracker_ExtObject_remove,
		cv_VectorOfDetectionBasedTracker_ExtObject_swap, cv_VectorOfDetectionBasedTracker_ExtObject_clear,
		cv_VectorOfDetectionBasedTracker_ExtObject_get, cv_VectorOfDetectionBasedTracker_ExtObject_set,
		cv_VectorOfDetectionBasedTracker_ExtObject_push, cv_VectorOfDetectionBasedTracker_ExtObject_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_ExtObject }
	
	unsafe impl Send for core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject> {}
	
	pub type VectorOfDetectionROI = core::Vector::<crate::objdetect::DetectionROI>;
	
	impl VectorOfDetectionROI {
		pub fn as_raw_VectorOfDetectionROI(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionROI, *const c_void, *mut c_void,
		cv_VectorOfDetectionROI_new, cv_VectorOfDetectionROI_delete,
		cv_VectorOfDetectionROI_len, cv_VectorOfDetectionROI_is_empty,
		cv_VectorOfDetectionROI_capacity, cv_VectorOfDetectionROI_shrink_to_fit,
		cv_VectorOfDetectionROI_reserve, cv_VectorOfDetectionROI_remove,
		cv_VectorOfDetectionROI_swap, cv_VectorOfDetectionROI_clear,
		cv_VectorOfDetectionROI_get, cv_VectorOfDetectionROI_set,
		cv_VectorOfDetectionROI_push, cv_VectorOfDetectionROI_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }
	
	unsafe impl Send for core::Vector::<crate::objdetect::DetectionROI> {}
	
}
pub use objdetect_types::*;

mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAlignMTB = core::Ptr::<dyn crate::photo::AlignMTB>;
	
	ptr_extern! { dyn crate::photo::AlignMTB,
		cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr, cv_PtrOfAlignMTB_get_inner_ptr_mut
	}
	
	impl PtrOfAlignMTB {
		#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAlignMTB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignExposures for PtrOfAlignMTB {
		#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignMTB for PtrOfAlignMTB {
		#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateDebevec = core::Ptr::<dyn crate::photo::CalibrateDebevec>;
	
	ptr_extern! { dyn crate::photo::CalibrateDebevec,
		cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr, cv_PtrOfCalibrateDebevec_get_inner_ptr_mut
	}
	
	impl PtrOfCalibrateDebevec {
		#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateRobertson = core::Ptr::<dyn crate::photo::CalibrateRobertson>;
	
	ptr_extern! { dyn crate::photo::CalibrateRobertson,
		cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr, cv_PtrOfCalibrateRobertson_get_inner_ptr_mut
	}
	
	impl PtrOfCalibrateRobertson {
		#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeDebevec = core::Ptr::<dyn crate::photo::MergeDebevec>;
	
	ptr_extern! { dyn crate::photo::MergeDebevec,
		cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr, cv_PtrOfMergeDebevec_get_inner_ptr_mut
	}
	
	impl PtrOfMergeDebevec {
		#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeDebevec {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
		#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeDebevec {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeMertens = core::Ptr::<dyn crate::photo::MergeMertens>;
	
	ptr_extern! { dyn crate::photo::MergeMertens,
		cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr, cv_PtrOfMergeMertens_get_inner_ptr_mut
	}
	
	impl PtrOfMergeMertens {
		#[inline] pub fn as_raw_PtrOfMergeMertens(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeMertens {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeMertens {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeMertens for PtrOfMergeMertens {
		#[inline] fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeRobertson = core::Ptr::<dyn crate::photo::MergeRobertson>;
	
	ptr_extern! { dyn crate::photo::MergeRobertson,
		cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr, cv_PtrOfMergeRobertson_get_inner_ptr_mut
	}
	
	impl PtrOfMergeRobertson {
		#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeRobertson {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeRobertson {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
		#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemap = core::Ptr::<dyn crate::photo::Tonemap>;
	
	ptr_extern! { dyn crate::photo::Tonemap,
		cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr, cv_PtrOfTonemap_get_inner_ptr_mut
	}
	
	impl PtrOfTonemap {
		#[inline] pub fn as_raw_PtrOfTonemap(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemap {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemap {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDrago = core::Ptr::<dyn crate::photo::TonemapDrago>;
	
	ptr_extern! { dyn crate::photo::TonemapDrago,
		cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr, cv_PtrOfTonemapDrago_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapDrago {
		#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDrago {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDrago {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
		#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapMantiuk = core::Ptr::<dyn crate::photo::TonemapMantiuk>;
	
	ptr_extern! { dyn crate::photo::TonemapMantiuk,
		cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr, cv_PtrOfTonemapMantiuk_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapMantiuk {
		#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapReinhard = core::Ptr::<dyn crate::photo::TonemapReinhard>;
	
	ptr_extern! { dyn crate::photo::TonemapReinhard,
		cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr, cv_PtrOfTonemapReinhard_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapReinhard {
		#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapReinhard {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapReinhard {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
		#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use photo_types::*;

mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDetail_BestOf2NearestMatcher = core::Ptr::<crate::stitching::Detail_BestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
		cv_PtrOfDetail_BestOf2NearestMatcher_delete, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrOfDetail_BestOf2NearestMatcher_new }
	
	impl PtrOfDetail_BestOf2NearestMatcher {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_Blender = core::Ptr::<crate::stitching::Detail_Blender>;
	
	ptr_extern! { crate::stitching::Detail_Blender,
		cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr, cv_PtrOfDetail_Blender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrOfDetail_Blender_new }
	
	impl PtrOfDetail_Blender {
		#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_Blender {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_BundleAdjusterBase = core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>;
	
	ptr_extern! { dyn crate::stitching::Detail_BundleAdjusterBase,
		cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_BundleAdjusterBase {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_Estimator = core::Ptr::<dyn crate::stitching::Detail_Estimator>;
	
	ptr_extern! { dyn crate::stitching::Detail_Estimator,
		cv_PtrOfDetail_Estimator_delete, cv_PtrOfDetail_Estimator_get_inner_ptr, cv_PtrOfDetail_Estimator_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_Estimator {
		#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_Estimator {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_ExposureCompensator = core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_ExposureCompensator,
		cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_ExposureCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ExposureCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeaturesMatcher = core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesMatcher,
		cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_FeaturesMatcher {
		#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_FeaturesMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_RotationWarper = core::Ptr::<dyn crate::stitching::Detail_RotationWarper>;
	
	ptr_extern! { dyn crate::stitching::Detail_RotationWarper,
		cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr, cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_RotationWarper {
		#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_RotationWarper for PtrOfDetail_RotationWarper {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_SeamFinder = core::Ptr::<dyn crate::stitching::Detail_SeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_SeamFinder,
		cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr, cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_SeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_SeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStitcher = core::Ptr::<crate::stitching::Stitcher>;
	
	ptr_extern! { crate::stitching::Stitcher,
		cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr, cv_PtrOfStitcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrOfStitcher_new }
	
	impl PtrOfStitcher {
		#[inline] pub fn as_raw_PtrOfStitcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::StitcherTrait for PtrOfStitcher {
		#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfWarperCreator = core::Ptr::<dyn crate::stitching::WarperCreator>;
	
	ptr_extern! { dyn crate::stitching::WarperCreator,
		cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr, cv_PtrOfWarperCreator_get_inner_ptr_mut
	}
	
	impl PtrOfWarperCreator {
		#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfWarperCreator {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetail_CameraParams = core::Vector::<crate::stitching::Detail_CameraParams>;
	
	impl VectorOfDetail_CameraParams {
		pub fn as_raw_VectorOfDetail_CameraParams(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_CameraParams, *const c_void, *mut c_void,
		cv_VectorOfDetail_CameraParams_new, cv_VectorOfDetail_CameraParams_delete,
		cv_VectorOfDetail_CameraParams_len, cv_VectorOfDetail_CameraParams_is_empty,
		cv_VectorOfDetail_CameraParams_capacity, cv_VectorOfDetail_CameraParams_shrink_to_fit,
		cv_VectorOfDetail_CameraParams_reserve, cv_VectorOfDetail_CameraParams_remove,
		cv_VectorOfDetail_CameraParams_swap, cv_VectorOfDetail_CameraParams_clear,
		cv_VectorOfDetail_CameraParams_get, cv_VectorOfDetail_CameraParams_set,
		cv_VectorOfDetail_CameraParams_push, cv_VectorOfDetail_CameraParams_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_CameraParams }
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_CameraParams> {}
	
	pub type VectorOfDetail_ImageFeatures = core::Vector::<crate::stitching::Detail_ImageFeatures>;
	
	impl VectorOfDetail_ImageFeatures {
		pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_ImageFeatures, *const c_void, *mut c_void,
		cv_VectorOfDetail_ImageFeatures_new, cv_VectorOfDetail_ImageFeatures_delete,
		cv_VectorOfDetail_ImageFeatures_len, cv_VectorOfDetail_ImageFeatures_is_empty,
		cv_VectorOfDetail_ImageFeatures_capacity, cv_VectorOfDetail_ImageFeatures_shrink_to_fit,
		cv_VectorOfDetail_ImageFeatures_reserve, cv_VectorOfDetail_ImageFeatures_remove,
		cv_VectorOfDetail_ImageFeatures_swap, cv_VectorOfDetail_ImageFeatures_clear,
		cv_VectorOfDetail_ImageFeatures_get, cv_VectorOfDetail_ImageFeatures_set,
		cv_VectorOfDetail_ImageFeatures_push, cv_VectorOfDetail_ImageFeatures_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_ImageFeatures }
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_ImageFeatures> {}
	
	pub type VectorOfDetail_MatchesInfo = core::Vector::<crate::stitching::Detail_MatchesInfo>;
	
	impl VectorOfDetail_MatchesInfo {
		pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_MatchesInfo, *const c_void, *mut c_void,
		cv_VectorOfDetail_MatchesInfo_new, cv_VectorOfDetail_MatchesInfo_delete,
		cv_VectorOfDetail_MatchesInfo_len, cv_VectorOfDetail_MatchesInfo_is_empty,
		cv_VectorOfDetail_MatchesInfo_capacity, cv_VectorOfDetail_MatchesInfo_shrink_to_fit,
		cv_VectorOfDetail_MatchesInfo_reserve, cv_VectorOfDetail_MatchesInfo_remove,
		cv_VectorOfDetail_MatchesInfo_swap, cv_VectorOfDetail_MatchesInfo_clear,
		cv_VectorOfDetail_MatchesInfo_get, cv_VectorOfDetail_MatchesInfo_set,
		cv_VectorOfDetail_MatchesInfo_push, cv_VectorOfDetail_MatchesInfo_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_MatchesInfo }
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_MatchesInfo> {}
	
}
pub use stitching_types::*;

mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorKNN = core::Ptr::<dyn crate::video::BackgroundSubtractorKNN>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorKNN,
		cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorKNN {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr::<dyn crate::video::BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorMOG2,
		cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorMOG2 {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDISOpticalFlow = core::Ptr::<dyn crate::video::DISOpticalFlow>;
	
	ptr_extern! { dyn crate::video::DISOpticalFlow,
		cv_PtrOfDISOpticalFlow_delete, cv_PtrOfDISOpticalFlow_get_inner_ptr, cv_PtrOfDISOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDISOpticalFlow {
		#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DISOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFarnebackOpticalFlow = core::Ptr::<dyn crate::video::FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::video::FarnebackOpticalFlow,
		cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfFarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr::<dyn crate::video::SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::video::SparsePyrLKOpticalFlow,
		cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSparsePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVariationalRefinement = core::Ptr::<dyn crate::video::VariationalRefinement>;
	
	ptr_extern! { dyn crate::video::VariationalRefinement,
		cv_PtrOfVariationalRefinement_delete, cv_PtrOfVariationalRefinement_get_inner_ptr, cv_PtrOfVariationalRefinement_get_inner_ptr_mut
	}
	
	impl PtrOfVariationalRefinement {
		#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfVariationalRefinement {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfVariationalRefinement {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::VariationalRefinement for PtrOfVariationalRefinement {
		#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use video_types::*;

mod videoio_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfVideoCaptureAPIs = core::Vector::<crate::videoio::VideoCaptureAPIs>;
	
	impl VectorOfVideoCaptureAPIs {
		pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
		cv_VectorOfVideoCaptureAPIs_new, cv_VectorOfVideoCaptureAPIs_delete,
		cv_VectorOfVideoCaptureAPIs_len, cv_VectorOfVideoCaptureAPIs_is_empty,
		cv_VectorOfVideoCaptureAPIs_capacity, cv_VectorOfVideoCaptureAPIs_shrink_to_fit,
		cv_VectorOfVideoCaptureAPIs_reserve, cv_VectorOfVideoCaptureAPIs_remove,
		cv_VectorOfVideoCaptureAPIs_swap, cv_VectorOfVideoCaptureAPIs_clear,
		cv_VectorOfVideoCaptureAPIs_get, cv_VectorOfVideoCaptureAPIs_set,
		cv_VectorOfVideoCaptureAPIs_push, cv_VectorOfVideoCaptureAPIs_insert,
	}
	vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
		cv_VectorOfVideoCaptureAPIs_data, cv_VectorOfVideoCaptureAPIs_data_mut,
		cv_VectorOfVideoCaptureAPIs_clone,
	}
	
	unsafe impl Send for core::Vector::<crate::videoio::VideoCaptureAPIs> {}
	
}
pub use videoio_types::*;

pub use crate::manual::types::*;
