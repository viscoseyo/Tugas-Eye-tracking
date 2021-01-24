#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Stereo Correspondence
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CUDA_StereoBM, super::CUDA_StereoBeliefPropagation, super::CUDA_StereoConstantSpaceBP, super::CUDA_DisparityBilateralFilter };
}

/// Creates DisparityBilateralFilter object.
/// 
/// ## Parameters
/// * ndisp: Number of disparities.
/// * radius: Filter radius.
/// * iters: Number of iterations.
/// 
/// ## C++ default parameters
/// * ndisp: 64
/// * radius: 3
/// * iters: 1
pub fn create_disparity_bilateral_filter(ndisp: i32, radius: i32, iters: i32) -> Result<core::Ptr::<dyn crate::cudastereo::CUDA_DisparityBilateralFilter>> {
	unsafe { sys::cv_cuda_createDisparityBilateralFilter_int_int_int(ndisp, radius, iters) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudastereo::CUDA_DisparityBilateralFilter>::opencv_from_extern(r) } )
}

/// Creates StereoBM object.
/// 
/// ## Parameters
/// * numDisparities: the disparity search range. For each pixel algorithm will find the best
/// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
/// shifted by changing the minimum disparity.
/// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
/// (as the block is centered at the current pixel). Larger block size implies smoother, though less
/// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
/// chance for algorithm to find a wrong correspondence.
/// 
/// ## C++ default parameters
/// * num_disparities: 64
/// * block_size: 19
pub fn create_stereo_bm(num_disparities: i32, block_size: i32) -> Result<core::Ptr::<dyn crate::cudastereo::CUDA_StereoBM>> {
	unsafe { sys::cv_cuda_createStereoBM_int_int(num_disparities, block_size) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudastereo::CUDA_StereoBM>::opencv_from_extern(r) } )
}

/// Creates StereoBeliefPropagation object.
/// 
/// ## Parameters
/// * ndisp: Number of disparities.
/// * iters: Number of BP iterations on each level.
/// * levels: Number of levels.
/// * msg_type: Type for messages. CV_16SC1 and CV_32FC1 types are supported.
/// 
/// ## C++ default parameters
/// * ndisp: 64
/// * iters: 5
/// * levels: 5
/// * msg_type: CV_32F
pub fn create_stereo_belief_propagation(ndisp: i32, iters: i32, levels: i32, msg_type: i32) -> Result<core::Ptr::<dyn crate::cudastereo::CUDA_StereoBeliefPropagation>> {
	unsafe { sys::cv_cuda_createStereoBeliefPropagation_int_int_int_int(ndisp, iters, levels, msg_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudastereo::CUDA_StereoBeliefPropagation>::opencv_from_extern(r) } )
}

/// Creates StereoConstantSpaceBP object.
/// 
/// ## Parameters
/// * ndisp: Number of disparities.
/// * iters: Number of BP iterations on each level.
/// * levels: Number of levels.
/// * nr_plane: Number of disparity levels on the first level.
/// * msg_type: Type for messages. CV_16SC1 and CV_32FC1 types are supported.
/// 
/// ## C++ default parameters
/// * ndisp: 128
/// * iters: 8
/// * levels: 4
/// * nr_plane: 4
/// * msg_type: CV_32F
pub fn create_stereo_constant_space_bp(ndisp: i32, iters: i32, levels: i32, nr_plane: i32, msg_type: i32) -> Result<core::Ptr::<dyn crate::cudastereo::CUDA_StereoConstantSpaceBP>> {
	unsafe { sys::cv_cuda_createStereoConstantSpaceBP_int_int_int_int_int(ndisp, iters, levels, nr_plane, msg_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudastereo::CUDA_StereoConstantSpaceBP>::opencv_from_extern(r) } )
}

/// Colors a disparity image.
/// 
/// ## Parameters
/// * src_disp: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. If 16-bit signed format is used, the values are assumed to have no
/// fractional bits.
/// * dst_disp: Output disparity image. It has the same size as src_disp. The type is CV_8UC4
/// in BGRA format (alpha = 255).
/// * ndisp: Number of disparities.
/// * stream: Stream for the asynchronous version.
/// 
/// This function draws a colored disparity map by converting disparity values from [0..ndisp) interval
/// first to HSV color space (where different disparity values correspond to different hues) and then
/// converting the pixels to RGB for visualization.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn draw_color_disp(src_disp: &dyn core::ToInputArray, dst_disp: &mut dyn core::ToOutputArray, ndisp: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src_disp);
	output_array_arg!(dst_disp);
	unsafe { sys::cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int_StreamR(src_disp.as_raw__InputArray(), dst_disp.as_raw__OutputArray(), ndisp, stream.as_raw_mut_Stream()) }.into_result()
}

/// Reprojects a disparity image to 3D space.
/// 
/// ## Parameters
/// * disp: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. If 16-bit signed format is used, the values are assumed to have no
/// fractional bits.
/// * xyzw: Output 3- or 4-channel floating-point image of the same size as disp . Each element of
/// xyzw(x,y) contains 3D coordinates (x,y,z) or (x,y,z,1) of the point (x,y) , computed from the
/// disparity map.
/// * Q: ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) perspective transformation matrix that can be obtained via stereoRectify .
/// * dst_cn: The number of channels for output image. Can be 3 or 4.
/// * stream: Stream for the asynchronous version.
/// ## See also
/// reprojectImageTo3D
/// 
/// ## C++ default parameters
/// * dst_cn: 4
/// * stream: Stream::Null()
pub fn reproject_image_to_3d(disp: &dyn core::ToInputArray, xyzw: &mut dyn core::ToOutputArray, q: &dyn core::ToInputArray, dst_cn: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(disp);
	output_array_arg!(xyzw);
	input_array_arg!(q);
	unsafe { sys::cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(disp.as_raw__InputArray(), xyzw.as_raw__OutputArray(), q.as_raw__InputArray(), dst_cn, stream.as_raw_mut_Stream()) }.into_result()
}

/// Class refining a disparity map using joint bilateral filtering. :
/// 
/// The class implements [Yang2010](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Yang2010) algorithm.
pub trait CUDA_DisparityBilateralFilter: core::AlgorithmTrait {
	fn as_raw_CUDA_DisparityBilateralFilter(&self) -> *const c_void;
	fn as_raw_mut_CUDA_DisparityBilateralFilter(&mut self) -> *mut c_void;

	/// Refines a disparity map using joint bilateral filtering.
	/// 
	/// ## Parameters
	/// * disparity: Input disparity map. CV_8UC1 and CV_16SC1 types are supported.
	/// * image: Input image. CV_8UC1 and CV_8UC3 types are supported.
	/// * dst: Destination disparity map. It has the same size and type as disparity .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn apply(&mut self, disparity: &dyn core::ToInputArray, image: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(disparity);
		input_array_arg!(image);
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_DisparityBilateralFilter(), disparity.as_raw__InputArray(), image.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn get_num_disparities(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getNumDisparities_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_num_disparities(&mut self, num_disparities: i32) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setNumDisparities_int(self.as_raw_mut_CUDA_DisparityBilateralFilter(), num_disparities) }.into_result()
	}
	
	fn get_radius(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getRadius_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_radius(&mut self, radius: i32) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setRadius_int(self.as_raw_mut_CUDA_DisparityBilateralFilter(), radius) }.into_result()
	}
	
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getNumIters_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_num_iters(&mut self, iters: i32) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setNumIters_int(self.as_raw_mut_CUDA_DisparityBilateralFilter(), iters) }.into_result()
	}
	
	/// truncation of data continuity
	fn get_edge_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getEdgeThreshold_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_edge_threshold(&mut self, edge_threshold: f64) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setEdgeThreshold_double(self.as_raw_mut_CUDA_DisparityBilateralFilter(), edge_threshold) }.into_result()
	}
	
	/// truncation of disparity continuity
	fn get_max_disc_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getMaxDiscThreshold_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_max_disc_threshold(&mut self, max_disc_threshold: f64) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setMaxDiscThreshold_double(self.as_raw_mut_CUDA_DisparityBilateralFilter(), max_disc_threshold) }.into_result()
	}
	
	/// filter range sigma
	fn get_sigma_range(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_getSigmaRange_const(self.as_raw_CUDA_DisparityBilateralFilter()) }.into_result()
	}
	
	fn set_sigma_range(&mut self, sigma_range: f64) -> Result<()> {
		unsafe { sys::cv_cuda_DisparityBilateralFilter_setSigmaRange_double(self.as_raw_mut_CUDA_DisparityBilateralFilter(), sigma_range) }.into_result()
	}
	
}

/// Class computing stereo correspondence (disparity map) using the block matching algorithm. :
/// ## See also
/// StereoBM
pub trait CUDA_StereoBM: crate::calib3d::StereoBM {
	fn as_raw_CUDA_StereoBM(&self) -> *const c_void;
	fn as_raw_mut_CUDA_StereoBM(&mut self) -> *mut c_void;

	fn compute(&mut self, left: &dyn core::ToInputArray, right: &dyn core::ToInputArray, disparity: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(left);
		input_array_arg!(right);
		output_array_arg!(disparity);
		unsafe { sys::cv_cuda_StereoBM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_StereoBM(), left.as_raw__InputArray(), right.as_raw__InputArray(), disparity.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}

/// Class computing stereo correspondence using the belief propagation algorithm. :
/// 
/// The class implements algorithm described in [Felzenszwalb2006](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Felzenszwalb2006) . It can compute own data cost
/// (using a truncated linear model) or use a user-provided data cost.
/// 
/// 
/// Note:
///    StereoBeliefPropagation requires a lot of memory for message storage:
/// 
///    ![block formula](https://latex.codecogs.com/png.latex?width%20%5C%5F%20step%20%20%5Ccdot%20height%20%20%5Ccdot%20ndisp%20%20%5Ccdot%204%20%20%5Ccdot%20%281%20%2B%200%2E25%29)
/// 
///    and for data cost storage:
/// 
///    ![block formula](https://latex.codecogs.com/png.latex?width%5C%5Fstep%20%5Ccdot%20height%20%5Ccdot%20ndisp%20%5Ccdot%20%281%20%2B%200%2E25%20%2B%200%2E0625%20%2B%20%20%5Cdotsm%20%2B%20%5Cfrac%7B1%7D%7B4%5E%7Blevels%7D%7D%29)
/// 
///    width_step is the number of bytes in a line including padding.
/// 
/// StereoBeliefPropagation uses a truncated linear model for the data cost and discontinuity terms:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?DataCost%20%3D%20data%20%5C%5F%20weight%20%20%5Ccdot%20%5Cmin%20%28%20%5Clvert%20Img%5FLeft%28x%2Cy%29%2DImg%5FRight%28x%2Dd%2Cy%29%20%20%5Crvert%20%2C%20max%20%5C%5F%20data%20%5C%5F%20term%29)
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?DiscTerm%20%3D%20%20%5Cmin%20%28disc%20%5C%5F%20single%20%5C%5F%20jump%20%20%5Ccdot%20%5Clvert%20f%5F1%2Df%5F2%20%20%5Crvert%20%2C%20max%20%5C%5F%20disc%20%5C%5F%20term%29)
/// 
/// For more details, see [Felzenszwalb2006](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Felzenszwalb2006) .
/// 
/// By default, StereoBeliefPropagation uses floating-point arithmetics and the CV_32FC1 type for
/// messages. But it can also use fixed-point arithmetics and the CV_16SC1 message type for better
/// performance. To avoid an overflow in this case, the parameters must satisfy the following
/// requirement:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?10%20%20%5Ccdot%202%5E%7Blevels%2D1%7D%20%20%5Ccdot%20max%20%5C%5F%20data%20%5C%5F%20term%20%3C%20SHRT%20%5C%5F%20MAX)
/// ## See also
/// StereoMatcher
pub trait CUDA_StereoBeliefPropagation: crate::calib3d::StereoMatcher {
	fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void;
	fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void;

	/// Enables the stereo correspondence operator that finds the disparity for the specified data cost.
	/// 
	/// ## Parameters
	/// * data: User-specified data cost, a matrix of msg_type type and
	/// Size(\<image columns\>\*ndisp, \<image rows\>) size.
	/// * disparity: Output disparity map. If disparity is empty, the output type is CV_16SC1 .
	/// Otherwise, the type is retained. In 16-bit signed format, the disparity values do not have
	/// fractional bits.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Overloaded parameters
	fn compute(&mut self, left: &dyn core::ToInputArray, right: &dyn core::ToInputArray, disparity: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(left);
		input_array_arg!(right);
		output_array_arg!(disparity);
		unsafe { sys::cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_StereoBeliefPropagation(), left.as_raw__InputArray(), right.as_raw__InputArray(), disparity.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Enables the stereo correspondence operator that finds the disparity for the specified data cost.
	/// 
	/// ## Parameters
	/// * data: User-specified data cost, a matrix of msg_type type and
	/// Size(\<image columns\>\*ndisp, \<image rows\>) size.
	/// * disparity: Output disparity map. If disparity is empty, the output type is CV_16SC1 .
	/// Otherwise, the type is retained. In 16-bit signed format, the disparity values do not have
	/// fractional bits.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn compute_1(&mut self, data: &dyn core::ToInputArray, disparity: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(data);
		output_array_arg!(disparity);
		unsafe { sys::cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_StereoBeliefPropagation(), data.as_raw__InputArray(), disparity.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// number of BP iterations on each level
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getNumIters_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_num_iters(&mut self, iters: i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setNumIters_int(self.as_raw_mut_CUDA_StereoBeliefPropagation(), iters) }.into_result()
	}
	
	/// number of levels
	fn get_num_levels(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getNumLevels_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_num_levels(&mut self, levels: i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setNumLevels_int(self.as_raw_mut_CUDA_StereoBeliefPropagation(), levels) }.into_result()
	}
	
	/// truncation of data cost
	fn get_max_data_term(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getMaxDataTerm_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_max_data_term(&mut self, max_data_term: f64) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setMaxDataTerm_double(self.as_raw_mut_CUDA_StereoBeliefPropagation(), max_data_term) }.into_result()
	}
	
	/// data weight
	fn get_data_weight(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getDataWeight_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_data_weight(&mut self, data_weight: f64) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setDataWeight_double(self.as_raw_mut_CUDA_StereoBeliefPropagation(), data_weight) }.into_result()
	}
	
	/// truncation of discontinuity cost
	fn get_max_disc_term(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getMaxDiscTerm_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_max_disc_term(&mut self, max_disc_term: f64) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setMaxDiscTerm_double(self.as_raw_mut_CUDA_StereoBeliefPropagation(), max_disc_term) }.into_result()
	}
	
	/// discontinuity single jump
	fn get_disc_single_jump(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getDiscSingleJump_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_disc_single_jump(&mut self, disc_single_jump: f64) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setDiscSingleJump_double(self.as_raw_mut_CUDA_StereoBeliefPropagation(), disc_single_jump) }.into_result()
	}
	
	/// type for messages (CV_16SC1 or CV_32FC1)
	fn get_msg_type(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_getMsgType_const(self.as_raw_CUDA_StereoBeliefPropagation()) }.into_result()
	}
	
	fn set_msg_type(&mut self, msg_type: i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_setMsgType_int(self.as_raw_mut_CUDA_StereoBeliefPropagation(), msg_type) }.into_result()
	}
	
}

impl dyn CUDA_StereoBeliefPropagation + '_ {
	/// Uses a heuristic method to compute the recommended parameters ( ndisp, iters and levels ) for the
	/// specified image size ( width and height ).
	pub fn estimate_recommended_params(width: i32, height: i32, ndisp: &mut i32, iters: &mut i32, levels: &mut i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoBeliefPropagation_estimateRecommendedParams_int_int_intR_intR_intR(width, height, ndisp, iters, levels) }.into_result()
	}
	
}
/// Class computing stereo correspondence using the constant space belief propagation algorithm. :
/// 
/// The class implements algorithm described in [Yang2010](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Yang2010) . StereoConstantSpaceBP supports both local
/// minimum and global minimum data cost initialization algorithms. For more details, see the paper
/// mentioned above. By default, a local algorithm is used. To enable a global algorithm, set
/// use_local_init_data_cost to false .
/// 
/// StereoConstantSpaceBP uses a truncated linear model for the data cost and discontinuity terms:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?DataCost%20%3D%20data%20%5C%5F%20weight%20%20%5Ccdot%20%5Cmin%20%28%20%5Clvert%20I%5F2%2DI%5F1%20%20%5Crvert%20%2C%20max%20%5C%5F%20data%20%5C%5F%20term%29)
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?DiscTerm%20%3D%20%20%5Cmin%20%28disc%20%5C%5F%20single%20%5C%5F%20jump%20%20%5Ccdot%20%5Clvert%20f%5F1%2Df%5F2%20%20%5Crvert%20%2C%20max%20%5C%5F%20disc%20%5C%5F%20term%29)
/// 
/// For more details, see [Yang2010](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Yang2010) .
/// 
/// By default, StereoConstantSpaceBP uses floating-point arithmetics and the CV_32FC1 type for
/// messages. But it can also use fixed-point arithmetics and the CV_16SC1 message type for better
/// performance. To avoid an overflow in this case, the parameters must satisfy the following
/// requirement:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?10%20%20%5Ccdot%202%5E%7Blevels%2D1%7D%20%20%5Ccdot%20max%20%5C%5F%20data%20%5C%5F%20term%20%3C%20SHRT%20%5C%5F%20MAX)
pub trait CUDA_StereoConstantSpaceBP: crate::cudastereo::CUDA_StereoBeliefPropagation {
	fn as_raw_CUDA_StereoConstantSpaceBP(&self) -> *const c_void;
	fn as_raw_mut_CUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void;

	/// number of active disparity on the first level
	fn get_nr_plane(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_StereoConstantSpaceBP_getNrPlane_const(self.as_raw_CUDA_StereoConstantSpaceBP()) }.into_result()
	}
	
	fn set_nr_plane(&mut self, nr_plane: i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoConstantSpaceBP_setNrPlane_int(self.as_raw_mut_CUDA_StereoConstantSpaceBP(), nr_plane) }.into_result()
	}
	
	fn get_use_local_init_data_cost(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_StereoConstantSpaceBP_getUseLocalInitDataCost_const(self.as_raw_CUDA_StereoConstantSpaceBP()) }.into_result()
	}
	
	fn set_use_local_init_data_cost(&mut self, use_local_init_data_cost: bool) -> Result<()> {
		unsafe { sys::cv_cuda_StereoConstantSpaceBP_setUseLocalInitDataCost_bool(self.as_raw_mut_CUDA_StereoConstantSpaceBP(), use_local_init_data_cost) }.into_result()
	}
	
}

impl dyn CUDA_StereoConstantSpaceBP + '_ {
	/// Uses a heuristic method to compute parameters (ndisp, iters, levelsand nrplane) for the specified
	/// image size (widthand height).
	pub fn estimate_recommended_params(width: i32, height: i32, ndisp: &mut i32, iters: &mut i32, levels: &mut i32, nr_plane: &mut i32) -> Result<()> {
		unsafe { sys::cv_cuda_StereoConstantSpaceBP_estimateRecommendedParams_int_int_intR_intR_intR_intR(width, height, ndisp, iters, levels, nr_plane) }.into_result()
	}
	
}