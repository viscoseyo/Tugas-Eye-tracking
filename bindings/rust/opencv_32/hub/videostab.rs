#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Video Stabilization
//! 
//! The video stabilization module contains a set of functions and classes that can be used to solve the
//! problem of video stabilization. There are a few methods implemented, most of them are described in
//! the papers [OF06](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_OF06) and [G11](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_G11) . However, there are some extensions and deviations from the original
//! paper methods.
//! 
//! ### References
//! 
//!  1. "Full-Frame Video Stabilization with Motion Inpainting"
//!      Yasuyuki Matsushita, Eyal Ofek, Weina Ge, Xiaoou Tang, Senior Member, and Heung-Yeung Shum
//!  2. "Auto-Directed Video Stabilization with Robust L1 Optimal Camera Paths"
//!      Matthias Grundmann, Vivek Kwatra, Irfan Essa
//!          # Global Motion Estimation
//! 
//! The video stabilization module contains a set of functions and classes for global motion estimation
//! between point clouds or between images. In the last case features are extracted and matched
//! internally. For the sake of convenience the motion estimation functions are wrapped into classes.
//! Both the functions and the classes are available.
//! 
//!          # Fast Marching Method
//! 
//! The Fast Marching Method [Telea04](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Telea04) is used in of the video stabilization routines to do motion and
//! color inpainting. The method is implemented is a flexible way and it's made public for other users.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ISparseOptFlowEstimator, super::IDenseOptFlowEstimator, super::PyrLkOptFlowEstimatorBaseTrait, super::SparsePyrLkOptFlowEstimatorTrait, super::RansacParamsTrait, super::IOutlierRejector, super::NullOutlierRejectorTrait, super::TranslationBasedLocalOutlierRejectorTrait, super::MotionEstimatorBase, super::MotionEstimatorRansacL2Trait, super::MotionEstimatorL1Trait, super::ImageMotionEstimatorBase, super::FromFileMotionReaderTrait, super::ToFileMotionWriterTrait, super::KeypointBasedMotionEstimatorTrait, super::IMotionStabilizer, super::MotionStabilizationPipelineTrait, super::MotionFilterBase, super::GaussianMotionFilterTrait, super::LpMotionStabilizerTrait, super::IFrameSource, super::NullFrameSourceTrait, super::VideoFileSourceTrait, super::ILog, super::NullLogTrait, super::LogToStdoutTrait, super::FastMarchingMethodTrait, super::InpainterBase, super::NullInpainterTrait, super::InpaintingPipelineTrait, super::ConsistentMosaicInpainterTrait, super::MotionInpainterTrait, super::ColorAverageInpainterTrait, super::ColorInpainterTrait, super::DeblurerBase, super::NullDeblurerTrait, super::WeightingDeblurerTrait, super::WobbleSuppressorBase, super::NullWobbleSuppressorTrait, super::MoreAccurateMotionWobbleSuppressorBase, super::MoreAccurateMotionWobbleSuppressorTrait, super::StabilizerBase, super::OnePassStabilizerTrait, super::TwoPassStabilizerTrait };
}

pub const MM_AFFINE: i32 = 5;
pub const MM_HOMOGRAPHY: i32 = 6;
pub const MM_RIGID: i32 = 3;
pub const MM_ROTATION: i32 = 2;
pub const MM_SIMILARITY: i32 = 4;
pub const MM_TRANSLATION: i32 = 0;
pub const MM_TRANSLATION_AND_SCALE: i32 = 1;
pub const MM_UNKNOWN: i32 = 7;
/// Describes motion model between two point clouds.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MotionModel {
	MM_TRANSLATION = 0,
	MM_TRANSLATION_AND_SCALE = 1,
	MM_ROTATION = 2,
	MM_RIGID = 3,
	MM_SIMILARITY = 4,
	MM_AFFINE = 5,
	MM_HOMOGRAPHY = 6,
	MM_UNKNOWN = 7,
}

opencv_type_enum! { crate::videostab::MotionModel }

pub fn calc_blurriness(frame: &core::Mat) -> Result<f32> {
	unsafe { sys::cv_videostab_calcBlurriness_const_MatR(frame.as_raw_Mat()) }.into_result()
}

pub fn calc_flow_mask(flow_x: &core::Mat, flow_y: &core::Mat, errors: &core::Mat, max_error: f32, mask0: &core::Mat, mask1: &core::Mat, flow_mask: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat(), max_error, mask0.as_raw_Mat(), mask1.as_raw_Mat(), flow_mask.as_raw_mut_Mat()) }.into_result()
}

pub fn complete_frame_according_to_flow(flow_mask: &core::Mat, flow_x: &core::Mat, flow_y: &core::Mat, frame1: &core::Mat, mask1: &core::Mat, dist_thresh: f32, frame0: &mut core::Mat, mask0: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(flow_mask.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), frame1.as_raw_Mat(), mask1.as_raw_Mat(), dist_thresh, frame0.as_raw_mut_Mat(), mask0.as_raw_mut_Mat()) }.into_result()
}

pub fn ensure_inclusion_constraint(m: &core::Mat, size: core::Size, trim_ratio: f32) -> Result<core::Mat> {
	unsafe { sys::cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(m.as_raw_Mat(), size.opencv_as_extern(), trim_ratio) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Estimates best global motion between two 2D point clouds in the least-squares sense.
/// 
/// 
/// Note: Works in-place and changes input point arrays.
/// 
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model (up to MM_AFFINE).
/// * rmse: Final root-mean-square error.
/// ## Returns
/// 3x3 2D transformation matrix (32F).
/// 
/// ## C++ default parameters
/// * model: MM_AFFINE
/// * rmse: 0
pub fn estimate_global_motion_least_squares(points0: &mut dyn core::ToInputOutputArray, points1: &mut dyn core::ToInputOutputArray, model: i32, rmse: &mut f32) -> Result<core::Mat> {
	input_output_array_arg!(points0);
	input_output_array_arg!(points1);
	unsafe { sys::cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(points0.as_raw__InputOutputArray(), points1.as_raw__InputOutputArray(), model, rmse) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Estimates best global motion between two 2D point clouds robustly (using RANSAC method).
/// 
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model. See cv::videostab::MotionModel.
/// * params: RANSAC method parameters. See videostab::RansacParams.
/// * rmse: Final root-mean-square error.
/// * ninliers: Final number of inliers.
/// 
/// ## C++ default parameters
/// * model: MM_AFFINE
/// * params: RansacParams::default2dMotion(MM_AFFINE)
/// * rmse: 0
/// * ninliers: 0
pub fn estimate_global_motion_ransac(points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, model: i32, params: &crate::videostab::RansacParams, rmse: &mut f32, ninliers: &mut i32) -> Result<core::Mat> {
	input_array_arg!(points0);
	input_array_arg!(points1);
	unsafe { sys::cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(points0.as_raw__InputArray(), points1.as_raw__InputArray(), model, params.as_raw_RansacParams(), rmse, ninliers) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

pub fn estimate_optimal_trim_ratio(m: &core::Mat, size: core::Size) -> Result<f32> {
	unsafe { sys::cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(m.as_raw_Mat(), size.opencv_as_extern()) }.into_result()
}

/// Computes motion between two frames assuming that all the intermediate motions are known.
/// 
/// ## Parameters
/// * from: Source frame index.
/// * to: Destination frame index.
/// * motions: Pair-wise motions. motions[i] denotes motion from the frame i to the frame i+1
/// ## Returns
/// Motion from the frame from to the frame to.
pub fn get_motion(from: i32, to: i32, motions: &core::Vector::<core::Mat>) -> Result<core::Mat> {
	unsafe { sys::cv_videostab_getMotion_int_int_const_vector_Mat_R(from, to, motions.as_raw_VectorOfMat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

pub trait ColorAverageInpainterTrait: crate::videostab::InpainterBase {
	fn as_raw_ColorAverageInpainter(&self) -> *const c_void;
	fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void;

	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ColorAverageInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct ColorAverageInpainter {
	ptr: *mut c_void
}

opencv_type_boxed! { ColorAverageInpainter }

impl Drop for ColorAverageInpainter {
	fn drop(&mut self) {
		extern "C" { fn cv_ColorAverageInpainter_delete(instance: *mut c_void); }
		unsafe { cv_ColorAverageInpainter_delete(self.as_raw_mut_ColorAverageInpainter()) };
	}
}

impl ColorAverageInpainter {
	#[inline] pub fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ColorAverageInpainter {}

impl crate::videostab::ColorAverageInpainterTrait for ColorAverageInpainter {
	#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpainterBase for ColorAverageInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ColorAverageInpainter {
}

pub trait ColorInpainterTrait: crate::videostab::InpainterBase {
	fn as_raw_ColorInpainter(&self) -> *const c_void;
	fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void;

	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ColorInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct ColorInpainter {
	ptr: *mut c_void
}

opencv_type_boxed! { ColorInpainter }

impl Drop for ColorInpainter {
	fn drop(&mut self) {
		extern "C" { fn cv_ColorInpainter_delete(instance: *mut c_void); }
		unsafe { cv_ColorInpainter_delete(self.as_raw_mut_ColorInpainter()) };
	}
}

impl ColorInpainter {
	#[inline] pub fn as_raw_ColorInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ColorInpainter {}

impl crate::videostab::ColorInpainterTrait for ColorInpainter {
	#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpainterBase for ColorInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ColorInpainter {
	/// ## C++ default parameters
	/// * method: INPAINT_TELEA
	/// * radius: 2.
	pub fn new(method: i32, radius: f64) -> Result<crate::videostab::ColorInpainter> {
		unsafe { sys::cv_videostab_ColorInpainter_ColorInpainter_int_double(method, radius) }.into_result().map(|r| unsafe { crate::videostab::ColorInpainter::opencv_from_extern(r) } )
	}
	
}

pub trait ConsistentMosaicInpainterTrait: crate::videostab::InpainterBase {
	fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void;
	fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void;

	fn set_stdev_thresh(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(self.as_raw_mut_ConsistentMosaicInpainter(), val) }.into_result()
	}
	
	fn stdev_thresh(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(self.as_raw_ConsistentMosaicInpainter()) }.into_result()
	}
	
	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ConsistentMosaicInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct ConsistentMosaicInpainter {
	ptr: *mut c_void
}

opencv_type_boxed! { ConsistentMosaicInpainter }

impl Drop for ConsistentMosaicInpainter {
	fn drop(&mut self) {
		extern "C" { fn cv_ConsistentMosaicInpainter_delete(instance: *mut c_void); }
		unsafe { cv_ConsistentMosaicInpainter_delete(self.as_raw_mut_ConsistentMosaicInpainter()) };
	}
}

impl ConsistentMosaicInpainter {
	#[inline] pub fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ConsistentMosaicInpainter {}

impl crate::videostab::ConsistentMosaicInpainterTrait for ConsistentMosaicInpainter {
	#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpainterBase for ConsistentMosaicInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ConsistentMosaicInpainter {
	pub fn default() -> Result<crate::videostab::ConsistentMosaicInpainter> {
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter() }.into_result().map(|r| unsafe { crate::videostab::ConsistentMosaicInpainter::opencv_from_extern(r) } )
	}
	
}

pub trait DeblurerBase {
	fn as_raw_DeblurerBase(&self) -> *const c_void;
	fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void;

	fn set_radius(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_DeblurerBase_setRadius_int(self.as_raw_mut_DeblurerBase(), val) }.into_result()
	}
	
	fn radius(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_DeblurerBase_radius_const(self.as_raw_DeblurerBase()) }.into_result()
	}
	
	fn deblur(&mut self, idx: i32, frame: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_DeblurerBase_deblur_int_MatR(self.as_raw_mut_DeblurerBase(), idx, frame.as_raw_mut_Mat()) }.into_result()
	}
	
	fn set_frames(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_DeblurerBase_setFrames_const_vector_Mat_R(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn frames(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_DeblurerBase_frames_const(self.as_raw_DeblurerBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_DeblurerBase_setMotions_const_vector_Mat_R(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn motions(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_DeblurerBase_motions_const(self.as_raw_DeblurerBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_blurriness_rates(&mut self, val: &core::Vector::<f32>) -> Result<()> {
		unsafe { sys::cv_videostab_DeblurerBase_setBlurrinessRates_const_vector_float_R(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOff32()) }.into_result()
	}
	
	fn blurriness_rates(&self) -> Result<core::Vector::<f32>> {
		unsafe { sys::cv_videostab_DeblurerBase_blurrinessRates_const(self.as_raw_DeblurerBase()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
}

/// Describes the Fast Marching Method implementation.
/// 
/// See http://iwi.eldoc.ub.rug.nl/FILES/root/2004/JGraphToolsTelea/2004JGraphToolsTelea.pdf
pub trait FastMarchingMethodTrait {
	fn as_raw_FastMarchingMethod(&self) -> *const c_void;
	fn as_raw_mut_FastMarchingMethod(&mut self) -> *mut c_void;

	/// ## Returns
	/// Distance map that's created during working of the method.
	fn distance_map(&self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_FastMarchingMethod_distanceMap_const(self.as_raw_FastMarchingMethod()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes the Fast Marching Method implementation.
/// 
/// See http://iwi.eldoc.ub.rug.nl/FILES/root/2004/JGraphToolsTelea/2004JGraphToolsTelea.pdf
pub struct FastMarchingMethod {
	ptr: *mut c_void
}

opencv_type_boxed! { FastMarchingMethod }

impl Drop for FastMarchingMethod {
	fn drop(&mut self) {
		extern "C" { fn cv_FastMarchingMethod_delete(instance: *mut c_void); }
		unsafe { cv_FastMarchingMethod_delete(self.as_raw_mut_FastMarchingMethod()) };
	}
}

impl FastMarchingMethod {
	#[inline] pub fn as_raw_FastMarchingMethod(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FastMarchingMethod(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FastMarchingMethod {}

impl crate::videostab::FastMarchingMethodTrait for FastMarchingMethod {
	#[inline] fn as_raw_FastMarchingMethod(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FastMarchingMethod(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FastMarchingMethod {
	pub fn default() -> Result<crate::videostab::FastMarchingMethod> {
		unsafe { sys::cv_videostab_FastMarchingMethod_FastMarchingMethod() }.into_result().map(|r| unsafe { crate::videostab::FastMarchingMethod::opencv_from_extern(r) } )
	}
	
}

pub trait FromFileMotionReaderTrait: crate::videostab::ImageMotionEstimatorBase {
	fn as_raw_FromFileMotionReader(&self) -> *const c_void;
	fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_FromFileMotionReader(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub struct FromFileMotionReader {
	ptr: *mut c_void
}

opencv_type_boxed! { FromFileMotionReader }

impl Drop for FromFileMotionReader {
	fn drop(&mut self) {
		extern "C" { fn cv_FromFileMotionReader_delete(instance: *mut c_void); }
		unsafe { cv_FromFileMotionReader_delete(self.as_raw_mut_FromFileMotionReader()) };
	}
}

impl FromFileMotionReader {
	#[inline] pub fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FromFileMotionReader {}

impl crate::videostab::FromFileMotionReaderTrait for FromFileMotionReader {
	#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBase for FromFileMotionReader {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FromFileMotionReader {
	pub fn new(path: &str) -> Result<crate::videostab::FromFileMotionReader> {
		extern_container_arg!(path);
		unsafe { sys::cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(path.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::videostab::FromFileMotionReader::opencv_from_extern(r) } )
	}
	
}

pub trait GaussianMotionFilterTrait: crate::videostab::MotionFilterBase {
	fn as_raw_GaussianMotionFilter(&self) -> *const c_void;
	fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * stdev: -1.f
	fn set_params(&mut self, radius: i32, stdev: f32) -> Result<()> {
		unsafe { sys::cv_videostab_GaussianMotionFilter_setParams_int_float(self.as_raw_mut_GaussianMotionFilter(), radius, stdev) }.into_result()
	}
	
	fn radius(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_GaussianMotionFilter_radius_const(self.as_raw_GaussianMotionFilter()) }.into_result()
	}
	
	fn stdev(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_GaussianMotionFilter_stdev_const(self.as_raw_GaussianMotionFilter()) }.into_result()
	}
	
}

pub struct GaussianMotionFilter {
	ptr: *mut c_void
}

opencv_type_boxed! { GaussianMotionFilter }

impl Drop for GaussianMotionFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_GaussianMotionFilter_delete(instance: *mut c_void); }
		unsafe { cv_GaussianMotionFilter_delete(self.as_raw_mut_GaussianMotionFilter()) };
	}
}

impl GaussianMotionFilter {
	#[inline] pub fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GaussianMotionFilter {}

impl crate::videostab::GaussianMotionFilterTrait for GaussianMotionFilter {
	#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::IMotionStabilizer for GaussianMotionFilter {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionFilterBase for GaussianMotionFilter {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GaussianMotionFilter {
	/// ## C++ default parameters
	/// * radius: 15
	/// * stdev: -1.f
	pub fn new(radius: i32, stdev: f32) -> Result<crate::videostab::GaussianMotionFilter> {
		unsafe { sys::cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(radius, stdev) }.into_result().map(|r| unsafe { crate::videostab::GaussianMotionFilter::opencv_from_extern(r) } )
	}
	
}

pub trait IDenseOptFlowEstimator {
	fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void;
	fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void;

	fn run(&mut self, frame0: &dyn core::ToInputArray, frame1: &dyn core::ToInputArray, flow_x: &mut dyn core::ToInputOutputArray, flow_y: &mut dyn core::ToInputOutputArray, errors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_output_array_arg!(flow_x);
		input_output_array_arg!(flow_y);
		output_array_arg!(errors);
		unsafe { sys::cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(self.as_raw_mut_IDenseOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow_x.as_raw__InputOutputArray(), flow_y.as_raw__InputOutputArray(), errors.as_raw__OutputArray()) }.into_result()
	}
	
}

pub trait IFrameSource {
	fn as_raw_IFrameSource(&self) -> *const c_void;
	fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void;

	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_videostab_IFrameSource_reset(self.as_raw_mut_IFrameSource()) }.into_result()
	}
	
	fn next_frame(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_IFrameSource_nextFrame(self.as_raw_mut_IFrameSource()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub trait ILog {
	fn as_raw_ILog(&self) -> *const c_void;
	fn as_raw_mut_ILog(&mut self) -> *mut c_void;

	fn print(&mut self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		unsafe { sys::cv_videostab_ILog_print_const_charX(self.as_raw_mut_ILog(), format.opencv_as_extern()) }.into_result()
	}
	
}

pub trait IMotionStabilizer {
	fn as_raw_IMotionStabilizer(&self) -> *const c_void;
	fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void;

}

pub trait IOutlierRejector {
	fn as_raw_IOutlierRejector(&self) -> *const c_void;
	fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void;

	fn process(&mut self, frame_size: core::Size, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		unsafe { sys::cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_IOutlierRejector(), frame_size.opencv_as_extern(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray()) }.into_result()
	}
	
}

pub trait ISparseOptFlowEstimator {
	fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void;
	fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void;

	fn run(&mut self, frame0: &dyn core::ToInputArray, frame1: &dyn core::ToInputArray, points0: &dyn core::ToInputArray, points1: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, errors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_array_arg!(points0);
		input_output_array_arg!(points1);
		output_array_arg!(status);
		output_array_arg!(errors);
		unsafe { sys::cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_ISparseOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), points0.as_raw__InputArray(), points1.as_raw__InputOutputArray(), status.as_raw__OutputArray(), errors.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Base class for global 2D motion estimation methods which take frames as input.
pub trait ImageMotionEstimatorBase {
	fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void;
	fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void;

	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(self.as_raw_mut_ImageMotionEstimatorBase(), val) }.into_result()
	}
	
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_motionModel_const(self.as_raw_ImageMotionEstimatorBase()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_ImageMotionEstimatorBase(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub trait InpainterBase {
	fn as_raw_InpainterBase(&self) -> *const c_void;
	fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void;

	fn set_radius(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setRadius_int(self.as_raw_mut_InpainterBase(), val) }.into_result()
	}
	
	fn radius(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_InpainterBase_radius_const(self.as_raw_InpainterBase()) }.into_result()
	}
	
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setMotionModel_MotionModel(self.as_raw_mut_InpainterBase(), val) }.into_result()
	}
	
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_InpainterBase_motionModel_const(self.as_raw_InpainterBase()) }.into_result()
	}
	
	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_inpaint_int_MatR_MatR(self.as_raw_mut_InpainterBase(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
	fn set_frames(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setFrames_const_vector_Mat_R(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn frames(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_InpainterBase_frames_const(self.as_raw_InpainterBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setMotions_const_vector_Mat_R(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn motions(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_InpainterBase_motions_const(self.as_raw_InpainterBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_stabilized_frames(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setStabilizedFrames_const_vector_Mat_R(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn stabilized_frames(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_InpainterBase_stabilizedFrames_const(self.as_raw_InpainterBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_stabilization_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpainterBase_setStabilizationMotions_const_vector_Mat_R(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn stabilization_motions(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_InpainterBase_stabilizationMotions_const(self.as_raw_InpainterBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
}

pub trait InpaintingPipelineTrait: crate::videostab::InpainterBase {
	fn as_raw_InpaintingPipeline(&self) -> *const c_void;
	fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void;

	fn push_back(&mut self, mut inpainter: core::Ptr::<dyn crate::videostab::InpainterBase>) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_pushBack_Ptr_InpainterBase_(self.as_raw_mut_InpaintingPipeline(), inpainter.as_raw_mut_PtrOfInpainterBase()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_videostab_InpaintingPipeline_empty_const(self.as_raw_InpaintingPipeline()) }.into_result()
	}
	
	fn set_radius(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setRadius_int(self.as_raw_mut_InpaintingPipeline(), val) }.into_result()
	}
	
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(self.as_raw_mut_InpaintingPipeline(), val) }.into_result()
	}
	
	fn set_frames(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setFrames_const_vector_Mat_R(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn set_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setMotions_const_vector_Mat_R(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn set_stabilized_frames(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vector_Mat_R(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn set_stabilization_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vector_Mat_R(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(self.as_raw_mut_InpaintingPipeline(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct InpaintingPipeline {
	ptr: *mut c_void
}

opencv_type_boxed! { InpaintingPipeline }

impl Drop for InpaintingPipeline {
	fn drop(&mut self) {
		extern "C" { fn cv_InpaintingPipeline_delete(instance: *mut c_void); }
		unsafe { cv_InpaintingPipeline_delete(self.as_raw_mut_InpaintingPipeline()) };
	}
}

impl InpaintingPipeline {
	#[inline] pub fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for InpaintingPipeline {}

impl crate::videostab::InpainterBase for InpaintingPipeline {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpaintingPipelineTrait for InpaintingPipeline {
	#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl InpaintingPipeline {
}

/// Describes a global 2D motion estimation method which uses keypoints detection and optical flow for
/// matching.
pub trait KeypointBasedMotionEstimatorTrait: crate::videostab::ImageMotionEstimatorBase {
	fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void;
	fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void;

	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(self.as_raw_mut_KeypointBasedMotionEstimator(), val) }.into_result()
	}
	
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_motionModel_const(self.as_raw_KeypointBasedMotionEstimator()) }.into_result()
	}
	
	fn set_detector(&mut self, mut val: core::Ptr::<crate::features2d::Feature2D>) -> Result<()> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setDetector_Ptr_Feature2D_(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfFeature2D()) }.into_result()
	}
	
	fn detector(&self) -> Result<core::Ptr::<crate::features2d::Feature2D>> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_detector_const(self.as_raw_KeypointBasedMotionEstimator()) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(r) } )
	}
	
	fn set_optical_flow_estimator(&mut self, mut val: core::Ptr::<dyn crate::videostab::ISparseOptFlowEstimator>) -> Result<()> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_Ptr_ISparseOptFlowEstimator_(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfISparseOptFlowEstimator()) }.into_result()
	}
	
	fn optical_flow_estimator(&self) -> Result<core::Ptr::<dyn crate::videostab::ISparseOptFlowEstimator>> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(self.as_raw_KeypointBasedMotionEstimator()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::ISparseOptFlowEstimator>::opencv_from_extern(r) } )
	}
	
	fn set_outlier_rejector(&mut self, mut val: core::Ptr::<dyn crate::videostab::IOutlierRejector>) -> Result<()> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_Ptr_IOutlierRejector_(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfIOutlierRejector()) }.into_result()
	}
	
	fn outlier_rejector(&self) -> Result<core::Ptr::<dyn crate::videostab::IOutlierRejector>> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(self.as_raw_KeypointBasedMotionEstimator()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::IOutlierRejector>::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * ok: 0
	fn estimate_mat(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_KeypointBasedMotionEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes a global 2D motion estimation method which uses keypoints detection and optical flow for
/// matching.
pub struct KeypointBasedMotionEstimator {
	ptr: *mut c_void
}

opencv_type_boxed! { KeypointBasedMotionEstimator }

impl Drop for KeypointBasedMotionEstimator {
	fn drop(&mut self) {
		extern "C" { fn cv_KeypointBasedMotionEstimator_delete(instance: *mut c_void); }
		unsafe { cv_KeypointBasedMotionEstimator_delete(self.as_raw_mut_KeypointBasedMotionEstimator()) };
	}
}

impl KeypointBasedMotionEstimator {
	#[inline] pub fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for KeypointBasedMotionEstimator {}

impl crate::videostab::ImageMotionEstimatorBase for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTrait for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeypointBasedMotionEstimator {
	pub fn new(mut estimator: core::Ptr::<dyn crate::videostab::MotionEstimatorBase>) -> Result<crate::videostab::KeypointBasedMotionEstimator> {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_Ptr_MotionEstimatorBase_(estimator.as_raw_mut_PtrOfMotionEstimatorBase()) }.into_result().map(|r| unsafe { crate::videostab::KeypointBasedMotionEstimator::opencv_from_extern(r) } )
	}
	
}

pub trait LogToStdoutTrait: crate::videostab::ILog {
	fn as_raw_LogToStdout(&self) -> *const c_void;
	fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void;

	fn print(&mut self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		unsafe { sys::cv_videostab_LogToStdout_print_const_charX(self.as_raw_mut_LogToStdout(), format.opencv_as_extern()) }.into_result()
	}
	
}

pub struct LogToStdout {
	ptr: *mut c_void
}

opencv_type_boxed! { LogToStdout }

impl Drop for LogToStdout {
	fn drop(&mut self) {
		extern "C" { fn cv_LogToStdout_delete(instance: *mut c_void); }
		unsafe { cv_LogToStdout_delete(self.as_raw_mut_LogToStdout()) };
	}
}

impl LogToStdout {
	#[inline] pub fn as_raw_LogToStdout(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LogToStdout {}

impl crate::videostab::ILog for LogToStdout {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::LogToStdoutTrait for LogToStdout {
	#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LogToStdout {
}

pub trait LpMotionStabilizerTrait: crate::videostab::IMotionStabilizer {
	fn as_raw_LpMotionStabilizer(&self) -> *const c_void;
	fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void;

	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_motionModel_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_frame_size(&mut self, val: core::Size) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setFrameSize_Size(self.as_raw_mut_LpMotionStabilizer(), val.opencv_as_extern()) }.into_result()
	}
	
	fn frame_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_frameSize_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setTrimRatio_float(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn trim_ratio(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_trimRatio_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_weight1(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight1_float(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn weight1(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight1_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_weight2(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight2_float(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn weight2(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight2_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_weight3(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight3_float(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn weight3(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight3_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
	fn set_weight4(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight4_float(self.as_raw_mut_LpMotionStabilizer(), val) }.into_result()
	}
	
	fn weight4(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight4_const(self.as_raw_LpMotionStabilizer()) }.into_result()
	}
	
}

pub struct LpMotionStabilizer {
	ptr: *mut c_void
}

opencv_type_boxed! { LpMotionStabilizer }

impl Drop for LpMotionStabilizer {
	fn drop(&mut self) {
		extern "C" { fn cv_LpMotionStabilizer_delete(instance: *mut c_void); }
		unsafe { cv_LpMotionStabilizer_delete(self.as_raw_mut_LpMotionStabilizer()) };
	}
}

impl LpMotionStabilizer {
	#[inline] pub fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LpMotionStabilizer {}

impl crate::videostab::IMotionStabilizer for LpMotionStabilizer {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::LpMotionStabilizerTrait for LpMotionStabilizer {
	#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LpMotionStabilizer {
	/// ## C++ default parameters
	/// * model: MM_SIMILARITY
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::LpMotionStabilizer> {
		unsafe { sys::cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(model) }.into_result().map(|r| unsafe { crate::videostab::LpMotionStabilizer::opencv_from_extern(r) } )
	}
	
}

pub trait MoreAccurateMotionWobbleSuppressorTrait: crate::videostab::MoreAccurateMotionWobbleSuppressorBase {
	fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void;
	fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void;

	fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(self.as_raw_mut_MoreAccurateMotionWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct MoreAccurateMotionWobbleSuppressor {
	ptr: *mut c_void
}

opencv_type_boxed! { MoreAccurateMotionWobbleSuppressor }

impl Drop for MoreAccurateMotionWobbleSuppressor {
	fn drop(&mut self) {
		extern "C" { fn cv_MoreAccurateMotionWobbleSuppressor_delete(instance: *mut c_void); }
		unsafe { cv_MoreAccurateMotionWobbleSuppressor_delete(self.as_raw_mut_MoreAccurateMotionWobbleSuppressor()) };
	}
}

impl MoreAccurateMotionWobbleSuppressor {
	#[inline] pub fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MoreAccurateMotionWobbleSuppressor {}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorTrait for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MoreAccurateMotionWobbleSuppressor {
}

pub trait MoreAccurateMotionWobbleSuppressorBase: crate::videostab::WobbleSuppressorBase {
	fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void;
	fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void;

	fn set_period(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(), val) }.into_result()
	}
	
	fn period(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(self.as_raw_MoreAccurateMotionWobbleSuppressorBase()) }.into_result()
	}
	
}

/// Base class for all global motion estimation methods.
pub trait MotionEstimatorBase {
	fn as_raw_MotionEstimatorBase(&self) -> *const c_void;
	fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void;

	/// Sets motion model.
	/// 
	/// ## Parameters
	/// * val: Motion model. See cv::videostab::MotionModel.
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(self.as_raw_mut_MotionEstimatorBase(), val) }.into_result()
	}
	
	/// ## Returns
	/// Motion model. See cv::videostab::MotionModel.
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_MotionEstimatorBase_motionModel_const(self.as_raw_MotionEstimatorBase()) }.into_result()
	}
	
	/// Estimates global motion between two 2D point clouds.
	/// 
	/// ## Parameters
	/// * points0: Source set of 2D points (32F).
	/// * points1: Destination set of 2D points (32F).
	/// * ok: Indicates whether motion was estimated successfully.
	/// ## Returns
	/// 3x3 2D transformation matrix (32F).
	/// 
	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		unsafe { sys::cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorBase(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes a global 2D motion estimation method which minimizes L1 error.
/// 
/// 
/// Note: To be able to use this method you must build OpenCV with CLP library support. :
pub trait MotionEstimatorL1Trait: crate::videostab::MotionEstimatorBase {
	fn as_raw_MotionEstimatorL1(&self) -> *const c_void;
	fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		unsafe { sys::cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorL1(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes a global 2D motion estimation method which minimizes L1 error.
/// 
/// 
/// Note: To be able to use this method you must build OpenCV with CLP library support. :
pub struct MotionEstimatorL1 {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionEstimatorL1 }

impl Drop for MotionEstimatorL1 {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionEstimatorL1_delete(instance: *mut c_void); }
		unsafe { cv_MotionEstimatorL1_delete(self.as_raw_mut_MotionEstimatorL1()) };
	}
}

impl MotionEstimatorL1 {
	#[inline] pub fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MotionEstimatorL1 {}

impl crate::videostab::MotionEstimatorBase for MotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorL1Trait for MotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionEstimatorL1 {
	/// ## C++ default parameters
	/// * model: MM_AFFINE
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::MotionEstimatorL1> {
		unsafe { sys::cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(model) }.into_result().map(|r| unsafe { crate::videostab::MotionEstimatorL1::opencv_from_extern(r) } )
	}
	
}

/// Describes a robust RANSAC-based global 2D motion estimation method which minimizes L2 error.
pub trait MotionEstimatorRansacL2Trait: crate::videostab::MotionEstimatorBase {
	fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void;
	fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void;

	fn set_ransac_params(&mut self, val: &crate::videostab::RansacParams) -> Result<()> {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(self.as_raw_mut_MotionEstimatorRansacL2(), val.as_raw_RansacParams()) }.into_result()
	}
	
	fn ransac_params(&self) -> Result<crate::videostab::RansacParams> {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_ransacParams_const(self.as_raw_MotionEstimatorRansacL2()) }.into_result().map(|r| unsafe { crate::videostab::RansacParams::opencv_from_extern(r) } )
	}
	
	fn set_min_inlier_ratio(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(self.as_raw_mut_MotionEstimatorRansacL2(), val) }.into_result()
	}
	
	fn min_inlier_ratio(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(self.as_raw_MotionEstimatorRansacL2()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorRansacL2(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes a robust RANSAC-based global 2D motion estimation method which minimizes L2 error.
pub struct MotionEstimatorRansacL2 {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionEstimatorRansacL2 }

impl Drop for MotionEstimatorRansacL2 {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionEstimatorRansacL2_delete(instance: *mut c_void); }
		unsafe { cv_MotionEstimatorRansacL2_delete(self.as_raw_mut_MotionEstimatorRansacL2()) };
	}
}

impl MotionEstimatorRansacL2 {
	#[inline] pub fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MotionEstimatorRansacL2 {}

impl crate::videostab::MotionEstimatorBase for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorRansacL2Trait for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionEstimatorRansacL2 {
	/// ## C++ default parameters
	/// * model: MM_AFFINE
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::MotionEstimatorRansacL2> {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(model) }.into_result().map(|r| unsafe { crate::videostab::MotionEstimatorRansacL2::opencv_from_extern(r) } )
	}
	
}

pub trait MotionFilterBase: crate::videostab::IMotionStabilizer {
	fn as_raw_MotionFilterBase(&self) -> *const c_void;
	fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void;

}

pub trait MotionInpainterTrait: crate::videostab::InpainterBase {
	fn as_raw_MotionInpainter(&self) -> *const c_void;
	fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void;

	fn set_opt_flow_estimator(&mut self, mut val: core::Ptr::<dyn crate::videostab::IDenseOptFlowEstimator>) -> Result<()> {
		unsafe { sys::cv_videostab_MotionInpainter_setOptFlowEstimator_Ptr_IDenseOptFlowEstimator_(self.as_raw_mut_MotionInpainter(), val.as_raw_mut_PtrOfIDenseOptFlowEstimator()) }.into_result()
	}
	
	fn opt_flow_estimator(&self) -> Result<core::Ptr::<dyn crate::videostab::IDenseOptFlowEstimator>> {
		unsafe { sys::cv_videostab_MotionInpainter_optFlowEstimator_const(self.as_raw_MotionInpainter()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::IDenseOptFlowEstimator>::opencv_from_extern(r) } )
	}
	
	fn set_flow_error_threshold(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_MotionInpainter_setFlowErrorThreshold_float(self.as_raw_mut_MotionInpainter(), val) }.into_result()
	}
	
	fn flow_error_threshold(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_MotionInpainter_flowErrorThreshold_const(self.as_raw_MotionInpainter()) }.into_result()
	}
	
	fn set_dist_threshold(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_MotionInpainter_setDistThreshold_float(self.as_raw_mut_MotionInpainter(), val) }.into_result()
	}
	
	fn dist_thresh(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_MotionInpainter_distThresh_const(self.as_raw_MotionInpainter()) }.into_result()
	}
	
	fn set_border_mode(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_MotionInpainter_setBorderMode_int(self.as_raw_mut_MotionInpainter(), val) }.into_result()
	}
	
	fn border_mode(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_MotionInpainter_borderMode_const(self.as_raw_MotionInpainter()) }.into_result()
	}
	
	fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_MotionInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct MotionInpainter {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionInpainter }

impl Drop for MotionInpainter {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionInpainter_delete(instance: *mut c_void); }
		unsafe { cv_MotionInpainter_delete(self.as_raw_mut_MotionInpainter()) };
	}
}

impl MotionInpainter {
	#[inline] pub fn as_raw_MotionInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MotionInpainter {}

impl crate::videostab::InpainterBase for MotionInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionInpainterTrait for MotionInpainter {
	#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionInpainter {
	pub fn default() -> Result<crate::videostab::MotionInpainter> {
		unsafe { sys::cv_videostab_MotionInpainter_MotionInpainter() }.into_result().map(|r| unsafe { crate::videostab::MotionInpainter::opencv_from_extern(r) } )
	}
	
}

pub trait MotionStabilizationPipelineTrait: crate::videostab::IMotionStabilizer {
	fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void;
	fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void;

	fn push_back(&mut self, mut stabilizer: core::Ptr::<dyn crate::videostab::IMotionStabilizer>) -> Result<()> {
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_pushBack_Ptr_IMotionStabilizer_(self.as_raw_mut_MotionStabilizationPipeline(), stabilizer.as_raw_mut_PtrOfIMotionStabilizer()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_empty_const(self.as_raw_MotionStabilizationPipeline()) }.into_result()
	}
	
}

pub struct MotionStabilizationPipeline {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionStabilizationPipeline }

impl Drop for MotionStabilizationPipeline {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionStabilizationPipeline_delete(instance: *mut c_void); }
		unsafe { cv_MotionStabilizationPipeline_delete(self.as_raw_mut_MotionStabilizationPipeline()) };
	}
}

impl MotionStabilizationPipeline {
	#[inline] pub fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MotionStabilizationPipeline {}

impl crate::videostab::IMotionStabilizer for MotionStabilizationPipeline {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionStabilizationPipelineTrait for MotionStabilizationPipeline {
	#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionStabilizationPipeline {
}

pub trait NullDeblurerTrait: crate::videostab::DeblurerBase {
	fn as_raw_NullDeblurer(&self) -> *const c_void;
	fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void;

	fn deblur(&mut self, unnamed: i32, unnamed_1: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_NullDeblurer_deblur_int_MatR(self.as_raw_mut_NullDeblurer(), unnamed, unnamed_1.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct NullDeblurer {
	ptr: *mut c_void
}

opencv_type_boxed! { NullDeblurer }

impl Drop for NullDeblurer {
	fn drop(&mut self) {
		extern "C" { fn cv_NullDeblurer_delete(instance: *mut c_void); }
		unsafe { cv_NullDeblurer_delete(self.as_raw_mut_NullDeblurer()) };
	}
}

impl NullDeblurer {
	#[inline] pub fn as_raw_NullDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullDeblurer {}

impl crate::videostab::DeblurerBase for NullDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullDeblurerTrait for NullDeblurer {
	#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullDeblurer {
}

pub trait NullFrameSourceTrait: crate::videostab::IFrameSource {
	fn as_raw_NullFrameSource(&self) -> *const c_void;
	fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void;

	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_videostab_NullFrameSource_reset(self.as_raw_mut_NullFrameSource()) }.into_result()
	}
	
	fn next_frame(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_NullFrameSource_nextFrame(self.as_raw_mut_NullFrameSource()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub struct NullFrameSource {
	ptr: *mut c_void
}

opencv_type_boxed! { NullFrameSource }

impl Drop for NullFrameSource {
	fn drop(&mut self) {
		extern "C" { fn cv_NullFrameSource_delete(instance: *mut c_void); }
		unsafe { cv_NullFrameSource_delete(self.as_raw_mut_NullFrameSource()) };
	}
}

impl NullFrameSource {
	#[inline] pub fn as_raw_NullFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullFrameSource {}

impl crate::videostab::IFrameSource for NullFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullFrameSourceTrait for NullFrameSource {
	#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullFrameSource {
}

pub trait NullInpainterTrait: crate::videostab::InpainterBase {
	fn as_raw_NullInpainter(&self) -> *const c_void;
	fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void;

	fn inpaint(&mut self, unnamed: i32, unnamed_1: &mut core::Mat, unnamed_2: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_NullInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_NullInpainter(), unnamed, unnamed_1.as_raw_mut_Mat(), unnamed_2.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct NullInpainter {
	ptr: *mut c_void
}

opencv_type_boxed! { NullInpainter }

impl Drop for NullInpainter {
	fn drop(&mut self) {
		extern "C" { fn cv_NullInpainter_delete(instance: *mut c_void); }
		unsafe { cv_NullInpainter_delete(self.as_raw_mut_NullInpainter()) };
	}
}

impl NullInpainter {
	#[inline] pub fn as_raw_NullInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullInpainter {}

impl crate::videostab::InpainterBase for NullInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullInpainterTrait for NullInpainter {
	#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullInpainter {
}

pub trait NullLogTrait: crate::videostab::ILog {
	fn as_raw_NullLog(&self) -> *const c_void;
	fn as_raw_mut_NullLog(&mut self) -> *mut c_void;

	fn print(&mut self, unnamed: &str) -> Result<()> {
		extern_container_arg!(unnamed);
		unsafe { sys::cv_videostab_NullLog_print_const_charX(self.as_raw_mut_NullLog(), unnamed.opencv_as_extern()) }.into_result()
	}
	
}

pub struct NullLog {
	ptr: *mut c_void
}

opencv_type_boxed! { NullLog }

impl Drop for NullLog {
	fn drop(&mut self) {
		extern "C" { fn cv_NullLog_delete(instance: *mut c_void); }
		unsafe { cv_NullLog_delete(self.as_raw_mut_NullLog()) };
	}
}

impl NullLog {
	#[inline] pub fn as_raw_NullLog(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullLog {}

impl crate::videostab::ILog for NullLog {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullLogTrait for NullLog {
	#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullLog {
}

pub trait NullOutlierRejectorTrait: crate::videostab::IOutlierRejector {
	fn as_raw_NullOutlierRejector(&self) -> *const c_void;
	fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void;

	fn process(&mut self, frame_size: core::Size, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		unsafe { sys::cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_NullOutlierRejector(), frame_size.opencv_as_extern(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray()) }.into_result()
	}
	
}

pub struct NullOutlierRejector {
	ptr: *mut c_void
}

opencv_type_boxed! { NullOutlierRejector }

impl Drop for NullOutlierRejector {
	fn drop(&mut self) {
		extern "C" { fn cv_NullOutlierRejector_delete(instance: *mut c_void); }
		unsafe { cv_NullOutlierRejector_delete(self.as_raw_mut_NullOutlierRejector()) };
	}
}

impl NullOutlierRejector {
	#[inline] pub fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullOutlierRejector {}

impl crate::videostab::IOutlierRejector for NullOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullOutlierRejectorTrait for NullOutlierRejector {
	#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullOutlierRejector {
}

pub trait NullWobbleSuppressorTrait: crate::videostab::WobbleSuppressorBase {
	fn as_raw_NullWobbleSuppressor(&self) -> *const c_void;
	fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void;

	fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(self.as_raw_mut_NullWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct NullWobbleSuppressor {
	ptr: *mut c_void
}

opencv_type_boxed! { NullWobbleSuppressor }

impl Drop for NullWobbleSuppressor {
	fn drop(&mut self) {
		extern "C" { fn cv_NullWobbleSuppressor_delete(instance: *mut c_void); }
		unsafe { cv_NullWobbleSuppressor_delete(self.as_raw_mut_NullWobbleSuppressor()) };
	}
}

impl NullWobbleSuppressor {
	#[inline] pub fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NullWobbleSuppressor {}

impl crate::videostab::NullWobbleSuppressorTrait for NullWobbleSuppressor {
	#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBase for NullWobbleSuppressor {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NullWobbleSuppressor {
}

pub trait OnePassStabilizerTrait: crate::videostab::IFrameSource + crate::videostab::StabilizerBase {
	fn as_raw_OnePassStabilizer(&self) -> *const c_void;
	fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void;

	fn set_motion_filter(&mut self, mut val: core::Ptr::<dyn crate::videostab::MotionFilterBase>) -> Result<()> {
		unsafe { sys::cv_videostab_OnePassStabilizer_setMotionFilter_Ptr_MotionFilterBase_(self.as_raw_mut_OnePassStabilizer(), val.as_raw_mut_PtrOfMotionFilterBase()) }.into_result()
	}
	
	fn motion_filter(&self) -> Result<core::Ptr::<dyn crate::videostab::MotionFilterBase>> {
		unsafe { sys::cv_videostab_OnePassStabilizer_motionFilter_const(self.as_raw_OnePassStabilizer()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::MotionFilterBase>::opencv_from_extern(r) } )
	}
	
	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_videostab_OnePassStabilizer_reset(self.as_raw_mut_OnePassStabilizer()) }.into_result()
	}
	
	fn next_frame(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_OnePassStabilizer_nextFrame(self.as_raw_mut_OnePassStabilizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub struct OnePassStabilizer {
	ptr: *mut c_void
}

opencv_type_boxed! { OnePassStabilizer }

impl Drop for OnePassStabilizer {
	fn drop(&mut self) {
		extern "C" { fn cv_OnePassStabilizer_delete(instance: *mut c_void); }
		unsafe { cv_OnePassStabilizer_delete(self.as_raw_mut_OnePassStabilizer()) };
	}
}

impl OnePassStabilizer {
	#[inline] pub fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for OnePassStabilizer {}

impl crate::videostab::IFrameSource for OnePassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::OnePassStabilizerTrait for OnePassStabilizer {
	#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::StabilizerBase for OnePassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OnePassStabilizer {
	pub fn default() -> Result<crate::videostab::OnePassStabilizer> {
		unsafe { sys::cv_videostab_OnePassStabilizer_OnePassStabilizer() }.into_result().map(|r| unsafe { crate::videostab::OnePassStabilizer::opencv_from_extern(r) } )
	}
	
}

pub trait PyrLkOptFlowEstimatorBaseTrait {
	fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void;
	fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void;

	fn set_win_size(&mut self, val: core::Size) -> Result<()> {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(self.as_raw_mut_PyrLkOptFlowEstimatorBase(), val.opencv_as_extern()) }.into_result()
	}
	
	fn win_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(self.as_raw_PyrLkOptFlowEstimatorBase()) }.into_result()
	}
	
	fn set_max_level(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(self.as_raw_mut_PyrLkOptFlowEstimatorBase(), val) }.into_result()
	}
	
	fn max_level(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(self.as_raw_PyrLkOptFlowEstimatorBase()) }.into_result()
	}
	
}

pub struct PyrLkOptFlowEstimatorBase {
	ptr: *mut c_void
}

opencv_type_boxed! { PyrLkOptFlowEstimatorBase }

impl Drop for PyrLkOptFlowEstimatorBase {
	fn drop(&mut self) {
		extern "C" { fn cv_PyrLkOptFlowEstimatorBase_delete(instance: *mut c_void); }
		unsafe { cv_PyrLkOptFlowEstimatorBase_delete(self.as_raw_mut_PyrLkOptFlowEstimatorBase()) };
	}
}

impl PyrLkOptFlowEstimatorBase {
	#[inline] pub fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PyrLkOptFlowEstimatorBase {}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PyrLkOptFlowEstimatorBase {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PyrLkOptFlowEstimatorBase {
	pub fn default() -> Result<crate::videostab::PyrLkOptFlowEstimatorBase> {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase() }.into_result().map(|r| unsafe { crate::videostab::PyrLkOptFlowEstimatorBase::opencv_from_extern(r) } )
	}
	
}

/// Describes RANSAC method parameters.
pub trait RansacParamsTrait {
	fn as_raw_RansacParams(&self) -> *const c_void;
	fn as_raw_mut_RansacParams(&mut self) -> *mut c_void;

	/// subset size
	fn size(&self) -> i32 {
		unsafe { sys::cv_videostab_RansacParams_getPropSize_const(self.as_raw_RansacParams()) }.into_result().expect("Infallible function failed: size")
	}
	
	/// subset size
	fn set_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_videostab_RansacParams_setPropSize_int(self.as_raw_mut_RansacParams(), val) }.into_result().expect("Infallible function failed: set_size")
	}
	
	/// max error to classify as inlier
	fn thresh(&self) -> f32 {
		unsafe { sys::cv_videostab_RansacParams_getPropThresh_const(self.as_raw_RansacParams()) }.into_result().expect("Infallible function failed: thresh")
	}
	
	/// max error to classify as inlier
	fn set_thresh(&mut self, val: f32) -> () {
		unsafe { sys::cv_videostab_RansacParams_setPropThresh_float(self.as_raw_mut_RansacParams(), val) }.into_result().expect("Infallible function failed: set_thresh")
	}
	
	/// max outliers ratio
	fn eps(&self) -> f32 {
		unsafe { sys::cv_videostab_RansacParams_getPropEps_const(self.as_raw_RansacParams()) }.into_result().expect("Infallible function failed: eps")
	}
	
	/// max outliers ratio
	fn set_eps(&mut self, val: f32) -> () {
		unsafe { sys::cv_videostab_RansacParams_setPropEps_float(self.as_raw_mut_RansacParams(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
	/// probability of success
	fn prob(&self) -> f32 {
		unsafe { sys::cv_videostab_RansacParams_getPropProb_const(self.as_raw_RansacParams()) }.into_result().expect("Infallible function failed: prob")
	}
	
	/// probability of success
	fn set_prob(&mut self, val: f32) -> () {
		unsafe { sys::cv_videostab_RansacParams_setPropProb_float(self.as_raw_mut_RansacParams(), val) }.into_result().expect("Infallible function failed: set_prob")
	}
	
	/// ## Returns
	/// Number of iterations that'll be performed by RANSAC method.
	fn niters(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_RansacParams_niters_const(self.as_raw_RansacParams()) }.into_result()
	}
	
}

/// Describes RANSAC method parameters.
pub struct RansacParams {
	ptr: *mut c_void
}

opencv_type_boxed! { RansacParams }

impl Drop for RansacParams {
	fn drop(&mut self) {
		extern "C" { fn cv_RansacParams_delete(instance: *mut c_void); }
		unsafe { cv_RansacParams_delete(self.as_raw_mut_RansacParams()) };
	}
}

impl RansacParams {
	#[inline] pub fn as_raw_RansacParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RansacParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RansacParams {}

impl crate::videostab::RansacParamsTrait for RansacParams {
	#[inline] fn as_raw_RansacParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RansacParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RansacParams {
	pub fn default() -> Result<crate::videostab::RansacParams> {
		unsafe { sys::cv_videostab_RansacParams_RansacParams() }.into_result().map(|r| unsafe { crate::videostab::RansacParams::opencv_from_extern(r) } )
	}
	
	/// Constructor
	/// ## Parameters
	/// * size: Subset size.
	/// * thresh: Maximum re-projection error value to classify as inlier.
	/// * eps: Maximum ratio of incorrect correspondences.
	/// * prob: Required success probability.
	pub fn new(size: i32, thresh: f32, eps: f32, prob: f32) -> Result<crate::videostab::RansacParams> {
		unsafe { sys::cv_videostab_RansacParams_RansacParams_int_float_float_float(size, thresh, eps, prob) }.into_result().map(|r| unsafe { crate::videostab::RansacParams::opencv_from_extern(r) } )
	}
	
	/// ## Parameters
	/// * model: Motion model. See cv::videostab::MotionModel.
	/// ## Returns
	/// Default RANSAC method parameters for the given motion model.
	pub fn default2d_motion(model: crate::videostab::MotionModel) -> Result<crate::videostab::RansacParams> {
		unsafe { sys::cv_videostab_RansacParams_default2dMotion_MotionModel(model) }.into_result().map(|r| unsafe { crate::videostab::RansacParams::opencv_from_extern(r) } )
	}
	
}

pub trait SparsePyrLkOptFlowEstimatorTrait: crate::videostab::ISparseOptFlowEstimator + crate::videostab::PyrLkOptFlowEstimatorBaseTrait {
	fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void;
	fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void;

	fn run(&mut self, frame0: &dyn core::ToInputArray, frame1: &dyn core::ToInputArray, points0: &dyn core::ToInputArray, points1: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, errors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_array_arg!(points0);
		input_output_array_arg!(points1);
		output_array_arg!(status);
		output_array_arg!(errors);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparsePyrLkOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), points0.as_raw__InputArray(), points1.as_raw__InputOutputArray(), status.as_raw__OutputArray(), errors.as_raw__OutputArray()) }.into_result()
	}
	
}

pub struct SparsePyrLkOptFlowEstimator {
	ptr: *mut c_void
}

opencv_type_boxed! { SparsePyrLkOptFlowEstimator }

impl Drop for SparsePyrLkOptFlowEstimator {
	fn drop(&mut self) {
		extern "C" { fn cv_SparsePyrLkOptFlowEstimator_delete(instance: *mut c_void); }
		unsafe { cv_SparsePyrLkOptFlowEstimator_delete(self.as_raw_mut_SparsePyrLkOptFlowEstimator()) };
	}
}

impl SparsePyrLkOptFlowEstimator {
	#[inline] pub fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparsePyrLkOptFlowEstimator {}

impl crate::videostab::ISparseOptFlowEstimator for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparsePyrLkOptFlowEstimator {
}

pub trait StabilizerBase {
	fn as_raw_StabilizerBase(&self) -> *const c_void;
	fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void;

	fn set_log(&mut self, mut ilog: core::Ptr::<dyn crate::videostab::ILog>) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setLog_Ptr_ILog_(self.as_raw_mut_StabilizerBase(), ilog.as_raw_mut_PtrOfILog()) }.into_result()
	}
	
	fn log(&self) -> Result<core::Ptr::<dyn crate::videostab::ILog>> {
		unsafe { sys::cv_videostab_StabilizerBase_log_const(self.as_raw_StabilizerBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::ILog>::opencv_from_extern(r) } )
	}
	
	fn set_radius(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setRadius_int(self.as_raw_mut_StabilizerBase(), val) }.into_result()
	}
	
	fn radius(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_StabilizerBase_radius_const(self.as_raw_StabilizerBase()) }.into_result()
	}
	
	fn set_frame_source(&mut self, mut val: core::Ptr::<dyn crate::videostab::IFrameSource>) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setFrameSource_Ptr_IFrameSource_(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfIFrameSource()) }.into_result()
	}
	
	fn frame_source(&self) -> Result<core::Ptr::<dyn crate::videostab::IFrameSource>> {
		unsafe { sys::cv_videostab_StabilizerBase_frameSource_const(self.as_raw_StabilizerBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::IFrameSource>::opencv_from_extern(r) } )
	}
	
	fn set_motion_estimator(&mut self, mut val: core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfImageMotionEstimatorBase()) }.into_result()
	}
	
	fn motion_estimator(&self) -> Result<core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>> {
		unsafe { sys::cv_videostab_StabilizerBase_motionEstimator_const(self.as_raw_StabilizerBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>::opencv_from_extern(r) } )
	}
	
	fn set_deblurer(&mut self, mut val: core::Ptr::<dyn crate::videostab::DeblurerBase>) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setDeblurer_Ptr_DeblurerBase_(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfDeblurerBase()) }.into_result()
	}
	
	fn deblurrer(&self) -> Result<core::Ptr::<dyn crate::videostab::DeblurerBase>> {
		unsafe { sys::cv_videostab_StabilizerBase_deblurrer_const(self.as_raw_StabilizerBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::DeblurerBase>::opencv_from_extern(r) } )
	}
	
	fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setTrimRatio_float(self.as_raw_mut_StabilizerBase(), val) }.into_result()
	}
	
	fn trim_ratio(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_StabilizerBase_trimRatio_const(self.as_raw_StabilizerBase()) }.into_result()
	}
	
	fn set_correction_for_inclusion(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(self.as_raw_mut_StabilizerBase(), val) }.into_result()
	}
	
	fn do_correction_for_inclusion(&self) -> Result<bool> {
		unsafe { sys::cv_videostab_StabilizerBase_doCorrectionForInclusion_const(self.as_raw_StabilizerBase()) }.into_result()
	}
	
	fn set_border_mode(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setBorderMode_int(self.as_raw_mut_StabilizerBase(), val) }.into_result()
	}
	
	fn border_mode(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_StabilizerBase_borderMode_const(self.as_raw_StabilizerBase()) }.into_result()
	}
	
	fn set_inpainter(&mut self, mut val: core::Ptr::<dyn crate::videostab::InpainterBase>) -> Result<()> {
		unsafe { sys::cv_videostab_StabilizerBase_setInpainter_Ptr_InpainterBase_(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfInpainterBase()) }.into_result()
	}
	
	fn inpainter(&self) -> Result<core::Ptr::<dyn crate::videostab::InpainterBase>> {
		unsafe { sys::cv_videostab_StabilizerBase_inpainter_const(self.as_raw_StabilizerBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::InpainterBase>::opencv_from_extern(r) } )
	}
	
}

pub trait ToFileMotionWriterTrait: crate::videostab::ImageMotionEstimatorBase {
	fn as_raw_ToFileMotionWriter(&self) -> *const c_void;
	fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void;

	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		unsafe { sys::cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(self.as_raw_mut_ToFileMotionWriter(), val) }.into_result()
	}
	
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		unsafe { sys::cv_videostab_ToFileMotionWriter_motionModel_const(self.as_raw_ToFileMotionWriter()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * ok: 0
	fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_ToFileMotionWriter(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub struct ToFileMotionWriter {
	ptr: *mut c_void
}

opencv_type_boxed! { ToFileMotionWriter }

impl Drop for ToFileMotionWriter {
	fn drop(&mut self) {
		extern "C" { fn cv_ToFileMotionWriter_delete(instance: *mut c_void); }
		unsafe { cv_ToFileMotionWriter_delete(self.as_raw_mut_ToFileMotionWriter()) };
	}
}

impl ToFileMotionWriter {
	#[inline] pub fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ToFileMotionWriter {}

impl crate::videostab::ImageMotionEstimatorBase for ToFileMotionWriter {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ToFileMotionWriterTrait for ToFileMotionWriter {
	#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ToFileMotionWriter {
	pub fn new(path: &str, mut estimator: core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>) -> Result<crate::videostab::ToFileMotionWriter> {
		extern_container_arg!(path);
		unsafe { sys::cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_Ptr_ImageMotionEstimatorBase_(path.opencv_as_extern(), estimator.as_raw_mut_PtrOfImageMotionEstimatorBase()) }.into_result().map(|r| unsafe { crate::videostab::ToFileMotionWriter::opencv_from_extern(r) } )
	}
	
}

pub trait TranslationBasedLocalOutlierRejectorTrait: crate::videostab::IOutlierRejector {
	fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void;
	fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void;

	fn set_cell_size(&mut self, val: core::Size) -> Result<()> {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), val.opencv_as_extern()) }.into_result()
	}
	
	fn cell_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(self.as_raw_TranslationBasedLocalOutlierRejector()) }.into_result()
	}
	
	fn set_ransac_params(&mut self, mut val: crate::videostab::RansacParams) -> Result<()> {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), val.as_raw_mut_RansacParams()) }.into_result()
	}
	
	fn ransac_params(&self) -> Result<crate::videostab::RansacParams> {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(self.as_raw_TranslationBasedLocalOutlierRejector()) }.into_result().map(|r| unsafe { crate::videostab::RansacParams::opencv_from_extern(r) } )
	}
	
	fn process(&mut self, frame_size: core::Size, points0: &dyn core::ToInputArray, points1: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), frame_size.opencv_as_extern(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray()) }.into_result()
	}
	
}

pub struct TranslationBasedLocalOutlierRejector {
	ptr: *mut c_void
}

opencv_type_boxed! { TranslationBasedLocalOutlierRejector }

impl Drop for TranslationBasedLocalOutlierRejector {
	fn drop(&mut self) {
		extern "C" { fn cv_TranslationBasedLocalOutlierRejector_delete(instance: *mut c_void); }
		unsafe { cv_TranslationBasedLocalOutlierRejector_delete(self.as_raw_mut_TranslationBasedLocalOutlierRejector()) };
	}
}

impl TranslationBasedLocalOutlierRejector {
	#[inline] pub fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TranslationBasedLocalOutlierRejector {}

impl crate::videostab::IOutlierRejector for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TranslationBasedLocalOutlierRejector {
	pub fn default() -> Result<crate::videostab::TranslationBasedLocalOutlierRejector> {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector() }.into_result().map(|r| unsafe { crate::videostab::TranslationBasedLocalOutlierRejector::opencv_from_extern(r) } )
	}
	
}

pub trait TwoPassStabilizerTrait: crate::videostab::IFrameSource + crate::videostab::StabilizerBase {
	fn as_raw_TwoPassStabilizer(&self) -> *const c_void;
	fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void;

	fn set_motion_stabilizer(&mut self, mut val: core::Ptr::<dyn crate::videostab::IMotionStabilizer>) -> Result<()> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_setMotionStabilizer_Ptr_IMotionStabilizer_(self.as_raw_mut_TwoPassStabilizer(), val.as_raw_mut_PtrOfIMotionStabilizer()) }.into_result()
	}
	
	fn motion_stabilizer(&self) -> Result<core::Ptr::<dyn crate::videostab::IMotionStabilizer>> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_motionStabilizer_const(self.as_raw_TwoPassStabilizer()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::IMotionStabilizer>::opencv_from_extern(r) } )
	}
	
	fn set_wobble_suppressor(&mut self, mut val: core::Ptr::<dyn crate::videostab::WobbleSuppressorBase>) -> Result<()> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_setWobbleSuppressor_Ptr_WobbleSuppressorBase_(self.as_raw_mut_TwoPassStabilizer(), val.as_raw_mut_PtrOfWobbleSuppressorBase()) }.into_result()
	}
	
	fn wobble_suppressor(&self) -> Result<core::Ptr::<dyn crate::videostab::WobbleSuppressorBase>> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(self.as_raw_TwoPassStabilizer()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::WobbleSuppressorBase>::opencv_from_extern(r) } )
	}
	
	fn set_estimate_trim_ratio(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(self.as_raw_mut_TwoPassStabilizer(), val) }.into_result()
	}
	
	fn must_estimate_trima_ratio(&self) -> Result<bool> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(self.as_raw_TwoPassStabilizer()) }.into_result()
	}
	
	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_reset(self.as_raw_mut_TwoPassStabilizer()) }.into_result()
	}
	
	fn next_frame(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_nextFrame(self.as_raw_mut_TwoPassStabilizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub struct TwoPassStabilizer {
	ptr: *mut c_void
}

opencv_type_boxed! { TwoPassStabilizer }

impl Drop for TwoPassStabilizer {
	fn drop(&mut self) {
		extern "C" { fn cv_TwoPassStabilizer_delete(instance: *mut c_void); }
		unsafe { cv_TwoPassStabilizer_delete(self.as_raw_mut_TwoPassStabilizer()) };
	}
}

impl TwoPassStabilizer {
	#[inline] pub fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TwoPassStabilizer {}

impl crate::videostab::IFrameSource for TwoPassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::StabilizerBase for TwoPassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::TwoPassStabilizerTrait for TwoPassStabilizer {
	#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TwoPassStabilizer {
	pub fn default() -> Result<crate::videostab::TwoPassStabilizer> {
		unsafe { sys::cv_videostab_TwoPassStabilizer_TwoPassStabilizer() }.into_result().map(|r| unsafe { crate::videostab::TwoPassStabilizer::opencv_from_extern(r) } )
	}
	
}

pub trait VideoFileSourceTrait: crate::videostab::IFrameSource {
	fn as_raw_VideoFileSource(&self) -> *const c_void;
	fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void;

	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_videostab_VideoFileSource_reset(self.as_raw_mut_VideoFileSource()) }.into_result()
	}
	
	fn next_frame(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_videostab_VideoFileSource_nextFrame(self.as_raw_mut_VideoFileSource()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn width(&mut self) -> Result<i32> {
		unsafe { sys::cv_videostab_VideoFileSource_width(self.as_raw_mut_VideoFileSource()) }.into_result()
	}
	
	fn height(&mut self) -> Result<i32> {
		unsafe { sys::cv_videostab_VideoFileSource_height(self.as_raw_mut_VideoFileSource()) }.into_result()
	}
	
	fn count(&mut self) -> Result<i32> {
		unsafe { sys::cv_videostab_VideoFileSource_count(self.as_raw_mut_VideoFileSource()) }.into_result()
	}
	
	fn fps(&mut self) -> Result<f64> {
		unsafe { sys::cv_videostab_VideoFileSource_fps(self.as_raw_mut_VideoFileSource()) }.into_result()
	}
	
}

pub struct VideoFileSource {
	ptr: *mut c_void
}

opencv_type_boxed! { VideoFileSource }

impl Drop for VideoFileSource {
	fn drop(&mut self) {
		extern "C" { fn cv_VideoFileSource_delete(instance: *mut c_void); }
		unsafe { cv_VideoFileSource_delete(self.as_raw_mut_VideoFileSource()) };
	}
}

impl VideoFileSource {
	#[inline] pub fn as_raw_VideoFileSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for VideoFileSource {}

impl crate::videostab::IFrameSource for VideoFileSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::VideoFileSourceTrait for VideoFileSource {
	#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl VideoFileSource {
	/// ## C++ default parameters
	/// * volatile_frame: false
	pub fn new(path: &str, volatile_frame: bool) -> Result<crate::videostab::VideoFileSource> {
		extern_container_arg!(path);
		unsafe { sys::cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(path.opencv_as_extern(), volatile_frame) }.into_result().map(|r| unsafe { crate::videostab::VideoFileSource::opencv_from_extern(r) } )
	}
	
}

pub trait WeightingDeblurerTrait: crate::videostab::DeblurerBase {
	fn as_raw_WeightingDeblurer(&self) -> *const c_void;
	fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void;

	fn set_sensitivity(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_videostab_WeightingDeblurer_setSensitivity_float(self.as_raw_mut_WeightingDeblurer(), val) }.into_result()
	}
	
	fn sensitivity(&self) -> Result<f32> {
		unsafe { sys::cv_videostab_WeightingDeblurer_sensitivity_const(self.as_raw_WeightingDeblurer()) }.into_result()
	}
	
	fn deblur(&mut self, idx: i32, frame: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_WeightingDeblurer_deblur_int_MatR(self.as_raw_mut_WeightingDeblurer(), idx, frame.as_raw_mut_Mat()) }.into_result()
	}
	
}

pub struct WeightingDeblurer {
	ptr: *mut c_void
}

opencv_type_boxed! { WeightingDeblurer }

impl Drop for WeightingDeblurer {
	fn drop(&mut self) {
		extern "C" { fn cv_WeightingDeblurer_delete(instance: *mut c_void); }
		unsafe { cv_WeightingDeblurer_delete(self.as_raw_mut_WeightingDeblurer()) };
	}
}

impl WeightingDeblurer {
	#[inline] pub fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for WeightingDeblurer {}

impl crate::videostab::DeblurerBase for WeightingDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::WeightingDeblurerTrait for WeightingDeblurer {
	#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WeightingDeblurer {
	pub fn default() -> Result<crate::videostab::WeightingDeblurer> {
		unsafe { sys::cv_videostab_WeightingDeblurer_WeightingDeblurer() }.into_result().map(|r| unsafe { crate::videostab::WeightingDeblurer::opencv_from_extern(r) } )
	}
	
}

pub trait WobbleSuppressorBase {
	fn as_raw_WobbleSuppressorBase(&self) -> *const c_void;
	fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void;

	fn set_motion_estimator(&mut self, mut val: core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_mut_PtrOfImageMotionEstimatorBase()) }.into_result()
	}
	
	fn motion_estimator(&self) -> Result<core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motionEstimator_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>::opencv_from_extern(r) } )
	}
	
	fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(self.as_raw_mut_WobbleSuppressorBase(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat()) }.into_result()
	}
	
	fn set_frame_count(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setFrameCount_int(self.as_raw_mut_WobbleSuppressorBase(), val) }.into_result()
	}
	
	fn frame_count(&self) -> Result<i32> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_frameCount_const(self.as_raw_WobbleSuppressorBase()) }.into_result()
	}
	
	fn set_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions_const_vector_Mat_R(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn motions(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motions_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_motions2(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions2_const_vector_Mat_R(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn motions2(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motions2_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn set_stabilization_motions(&mut self, val: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vector_Mat_R(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn stabilization_motions(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
}
