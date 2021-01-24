#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Phase Unwrapping API
//! 
//! Two-dimensional phase unwrapping is found in different applications like terrain elevation estimation
//! in synthetic aperture radar (SAR), field mapping in magnetic resonance imaging or as a way of finding
//! corresponding pixels in structured light reconstruction with sinusoidal patterns.
//! 
//! Given a phase map, wrapped between [-pi; pi], phase unwrapping aims at finding the "true" phase map
//! by adding the right number of 2*pi to each pixel.
//! 
//! The problem is straightforward for perfect wrapped phase map, but real data are usually not noise-free.
//! Among the different algorithms that were developed, quality-guided phase unwrapping methods are fast
//! and efficient. They follow a path that unwraps high quality pixels first,
//! avoiding error propagation from the start.
//! 
//! In this module, a quality-guided phase unwrapping is implemented following the approach described in [histogramUnwrapping](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_histogramUnwrapping) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PhaseUnwrapping, super::HistogramPhaseUnwrapping_ParamsTrait, super::HistogramPhaseUnwrapping };
}

/// Class implementing two-dimensional phase unwrapping based on [histogramUnwrapping](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_histogramUnwrapping)
/// This algorithm belongs to the quality-guided phase unwrapping methods.
/// First, it computes a reliability map from second differences between a pixel and its eight neighbours.
/// Reliability values lie between 0 and 16*pi*pi. Then, this reliability map is used to compute
/// the reliabilities of "edges". An edge is an entity defined by two pixels that are connected
/// horizontally or vertically. Its reliability is found by adding the the reliabilities of the
/// two pixels connected through it. Edges are sorted in a histogram based on their reliability values.
/// This histogram is then used to unwrap pixels, starting from the highest quality pixel.
/// 
/// The wrapped phase map and the unwrapped result are stored in CV_32FC1 Mat.
pub trait HistogramPhaseUnwrapping: crate::phase_unwrapping::PhaseUnwrapping {
	fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void;
	fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void;

	/// Get the reliability map computed from the wrapped phase map.
	/// 
	/// ## Parameters
	/// * reliabilityMap: Image where the reliability map is stored.
	fn get_inverse_reliability_map(&mut self, reliability_map: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(reliability_map);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(self.as_raw_mut_HistogramPhaseUnwrapping(), reliability_map.as_raw__OutputArray()) }.into_result()
	}
	
}

impl dyn HistogramPhaseUnwrapping + '_ {
	/// Constructor
	/// 
	/// ## Parameters
	/// * parameters: HistogramPhaseUnwrapping parameters HistogramPhaseUnwrapping::Params: width,height of the phase map and histogram characteristics.
	/// 
	/// ## C++ default parameters
	/// * parameters: HistogramPhaseUnwrapping::Params()
	pub fn create(parameters: &crate::phase_unwrapping::HistogramPhaseUnwrapping_Params) -> Result<core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>> {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(parameters.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>::opencv_from_extern(r) } )
	}
	
}
/// Parameters of phaseUnwrapping constructor.
/// 
/// ## Parameters
/// * width: Phase map width.
/// * height: Phase map height.
/// * histThresh: Bins in the histogram are not of equal size. Default value is 3*pi*pi. The one before "histThresh" value are smaller.
/// * nbrOfSmallBins: Number of bins between 0 and "histThresh". Default value is 10.
/// * nbrOfLargeBins: Number of bins between "histThresh" and 32*pi*pi (highest edge reliability value). Default value is 5.
pub trait HistogramPhaseUnwrapping_ParamsTrait {
	fn as_raw_HistogramPhaseUnwrapping_Params(&self) -> *const c_void;
	fn as_raw_mut_HistogramPhaseUnwrapping_Params(&mut self) -> *mut c_void;

	fn width(&self) -> i32 {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropWidth_const(self.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().expect("Infallible function failed: width")
	}
	
	fn set_width(&mut self, val: i32) -> () {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropWidth_int(self.as_raw_mut_HistogramPhaseUnwrapping_Params(), val) }.into_result().expect("Infallible function failed: set_width")
	}
	
	fn height(&self) -> i32 {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropHeight_const(self.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().expect("Infallible function failed: height")
	}
	
	fn set_height(&mut self, val: i32) -> () {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropHeight_int(self.as_raw_mut_HistogramPhaseUnwrapping_Params(), val) }.into_result().expect("Infallible function failed: set_height")
	}
	
	fn hist_thresh(&self) -> f32 {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropHistThresh_const(self.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().expect("Infallible function failed: hist_thresh")
	}
	
	fn set_hist_thresh(&mut self, val: f32) -> () {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropHistThresh_float(self.as_raw_mut_HistogramPhaseUnwrapping_Params(), val) }.into_result().expect("Infallible function failed: set_hist_thresh")
	}
	
	fn nbr_of_small_bins(&self) -> i32 {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropNbrOfSmallBins_const(self.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().expect("Infallible function failed: nbr_of_small_bins")
	}
	
	fn set_nbr_of_small_bins(&mut self, val: i32) -> () {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropNbrOfSmallBins_int(self.as_raw_mut_HistogramPhaseUnwrapping_Params(), val) }.into_result().expect("Infallible function failed: set_nbr_of_small_bins")
	}
	
	fn nbr_of_large_bins(&self) -> i32 {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropNbrOfLargeBins_const(self.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().expect("Infallible function failed: nbr_of_large_bins")
	}
	
	fn set_nbr_of_large_bins(&mut self, val: i32) -> () {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropNbrOfLargeBins_int(self.as_raw_mut_HistogramPhaseUnwrapping_Params(), val) }.into_result().expect("Infallible function failed: set_nbr_of_large_bins")
	}
	
}

/// Parameters of phaseUnwrapping constructor.
/// 
/// ## Parameters
/// * width: Phase map width.
/// * height: Phase map height.
/// * histThresh: Bins in the histogram are not of equal size. Default value is 3*pi*pi. The one before "histThresh" value are smaller.
/// * nbrOfSmallBins: Number of bins between 0 and "histThresh". Default value is 10.
/// * nbrOfLargeBins: Number of bins between "histThresh" and 32*pi*pi (highest edge reliability value). Default value is 5.
pub struct HistogramPhaseUnwrapping_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { HistogramPhaseUnwrapping_Params }

impl Drop for HistogramPhaseUnwrapping_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_HistogramPhaseUnwrapping_Params_delete(instance: *mut c_void); }
		unsafe { cv_HistogramPhaseUnwrapping_Params_delete(self.as_raw_mut_HistogramPhaseUnwrapping_Params()) };
	}
}

impl HistogramPhaseUnwrapping_Params {
	#[inline] pub fn as_raw_HistogramPhaseUnwrapping_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_HistogramPhaseUnwrapping_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for HistogramPhaseUnwrapping_Params {}

impl crate::phase_unwrapping::HistogramPhaseUnwrapping_ParamsTrait for HistogramPhaseUnwrapping_Params {
	#[inline] fn as_raw_HistogramPhaseUnwrapping_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_HistogramPhaseUnwrapping_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HistogramPhaseUnwrapping_Params {
	pub fn default() -> Result<crate::phase_unwrapping::HistogramPhaseUnwrapping_Params> {
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() }.into_result().map(|r| unsafe { crate::phase_unwrapping::HistogramPhaseUnwrapping_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for phase unwrapping.
pub trait PhaseUnwrapping: core::AlgorithmTrait {
	fn as_raw_PhaseUnwrapping(&self) -> *const c_void;
	fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void;

	/// Unwraps a 2D phase map.
	/// 
	/// ## Parameters
	/// * wrappedPhaseMap: The wrapped phase map that needs to be unwrapped.
	/// * unwrappedPhaseMap: The unwrapped phase map.
	/// * shadowMask: Optional parameter used when some pixels do not hold any phase information in the wrapped phase map.
	/// 
	/// ## C++ default parameters
	/// * shadow_mask: noArray()
	fn unwrap_phase_map(&mut self, wrapped_phase_map: &dyn core::ToInputArray, unwrapped_phase_map: &mut dyn core::ToOutputArray, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(wrapped_phase_map);
		output_array_arg!(unwrapped_phase_map);
		input_array_arg!(shadow_mask);
		unsafe { sys::cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_PhaseUnwrapping(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__InputArray()) }.into_result()
	}
	
}
