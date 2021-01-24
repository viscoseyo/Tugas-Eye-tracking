#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Additional photo processing algorithms
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::WhiteBalancer, super::SimpleWB, super::GrayworldWB, super::LearningBasedWB };
}

/// Execute only first step of the algorithm
pub const BM3D_STEP1: i32 = 1;
/// Execute only second step of the algorithm
pub const BM3D_STEP2: i32 = 2;
/// Execute all steps of the algorithm
pub const BM3D_STEPALL: i32 = 0;
/// Un-normalized Haar transform
pub const HAAR: i32 = 0;
/// This algorithm searches for dominant correspondences (transformations) of
/// image patches and tries to seamlessly fill-in the area to be inpainted using this
/// transformations
pub const INPAINT_SHIFTMAP: i32 = 0;
/// BM3D algorithm steps
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bm3dSteps {
	/// Execute all steps of the algorithm
	BM3D_STEPALL = 0,
	/// Execute only first step of the algorithm
	BM3D_STEP1 = 1,
	/// Execute only second step of the algorithm
	BM3D_STEP2 = 2,
}

opencv_type_enum! { crate::xphoto::Bm3dSteps }

/// various inpainting algorithms
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InpaintTypes {
	/// This algorithm searches for dominant correspondences (transformations) of
	/// image patches and tries to seamlessly fill-in the area to be inpainted using this
	/// transformations
	INPAINT_SHIFTMAP = 0,
}

opencv_type_enum! { crate::xphoto::InpaintTypes }

/// BM3D transform types
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TransformTypes {
	/// Un-normalized Haar transform
	HAAR = 0,
}

opencv_type_enum! { crate::xphoto::TransformTypes }

/// Implements an efficient fixed-point approximation for applying channel gains, which is
///    the last step of multiple white balance algorithms.
/// 
/// ## Parameters
/// * src: Input three-channel image in the BGR color space (either CV_8UC3 or CV_16UC3)
/// * dst: Output image of the same size and type as src.
/// * gainB: gain for the B channel
/// * gainG: gain for the G channel
/// * gainR: gain for the R channel
pub fn apply_channel_gains(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, gain_b: f32, gain_g: f32, gain_r: f32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_xphoto_applyChannelGains_const__InputArrayR_const__OutputArrayR_float_float_float(src.as_raw__InputArray(), dst.as_raw__OutputArray(), gain_b, gain_g, gain_r) }.into_result()
}

/// Performs image denoising using the Block-Matching and 3D-filtering algorithm
/// <http://www.cs.tut.fi/~foi/GCF-BM3D/BM3D_TIP_2007.pdf> with several computational
/// optimizations. Noise expected to be a gaussian white noise.
/// 
/// ## Parameters
/// * src: Input 8-bit or 16-bit 1-channel image.
/// * dstStep1: Output image of the first step of BM3D with the same size and type as src.
/// * dstStep2: Output image of the second step of BM3D with the same size and type as src.
/// * h: Parameter regulating filter strength. Big h value perfectly removes noise but also
/// removes image details, smaller h value preserves details but also preserves some noise.
/// * templateWindowSize: Size in pixels of the template patch that is used for block-matching.
/// Should be power of 2.
/// * searchWindowSize: Size in pixels of the window that is used to perform block-matching.
/// Affect performance linearly: greater searchWindowsSize - greater denoising time.
/// Must be larger than templateWindowSize.
/// * blockMatchingStep1: Block matching threshold for the first step of BM3D (hard thresholding),
/// i.e. maximum distance for which two blocks are considered similar.
/// Value expressed in euclidean distance.
/// * blockMatchingStep2: Block matching threshold for the second step of BM3D (Wiener filtering),
/// i.e. maximum distance for which two blocks are considered similar.
/// Value expressed in euclidean distance.
/// * groupSize: Maximum size of the 3D group for collaborative filtering.
/// * slidingStep: Sliding step to process every next reference block.
/// * beta: Kaiser window parameter that affects the sidelobe attenuation of the transform of the
/// window. Kaiser window is used in order to reduce border effects. To prevent usage of the window,
/// set beta to zero.
/// * normType: Norm used to calculate distance between blocks. L2 is slower than L1
/// but yields more accurate results.
/// * step: Step of BM3D to be executed. Possible variants are: step 1, step 2, both steps.
/// * transformType: Type of the orthogonal transform used in collaborative filtering step.
/// Currently only Haar transform is supported.
/// 
/// This function expected to be applied to grayscale images. Advanced usage of this function
/// can be manual denoising of colored image in different colorspaces.
/// ## See also
/// fastNlMeansDenoising
/// 
/// ## C++ default parameters
/// * h: 1
/// * template_window_size: 4
/// * search_window_size: 16
/// * block_matching_step1: 2500
/// * block_matching_step2: 400
/// * group_size: 8
/// * sliding_step: 1
/// * beta: 2.0f
/// * norm_type: cv::NORM_L2
/// * step: cv::xphoto::BM3D_STEPALL
/// * transform_type: cv::xphoto::HAAR
pub fn bm3d_denoising(src: &dyn core::ToInputArray, dst_step1: &mut dyn core::ToInputOutputArray, dst_step2: &mut dyn core::ToOutputArray, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst_step1);
	output_array_arg!(dst_step2);
	unsafe { sys::cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src.as_raw__InputArray(), dst_step1.as_raw__InputOutputArray(), dst_step2.as_raw__OutputArray(), h, template_window_size, search_window_size, block_matching_step1, block_matching_step2, group_size, sliding_step, beta, norm_type, step, transform_type) }.into_result()
}

/// Performs image denoising using the Block-Matching and 3D-filtering algorithm
/// <http://www.cs.tut.fi/~foi/GCF-BM3D/BM3D_TIP_2007.pdf> with several computational
/// optimizations. Noise expected to be a gaussian white noise.
/// 
/// ## Parameters
/// * src: Input 8-bit or 16-bit 1-channel image.
/// * dst: Output image with the same size and type as src.
/// * h: Parameter regulating filter strength. Big h value perfectly removes noise but also
/// removes image details, smaller h value preserves details but also preserves some noise.
/// * templateWindowSize: Size in pixels of the template patch that is used for block-matching.
/// Should be power of 2.
/// * searchWindowSize: Size in pixels of the window that is used to perform block-matching.
/// Affect performance linearly: greater searchWindowsSize - greater denoising time.
/// Must be larger than templateWindowSize.
/// * blockMatchingStep1: Block matching threshold for the first step of BM3D (hard thresholding),
/// i.e. maximum distance for which two blocks are considered similar.
/// Value expressed in euclidean distance.
/// * blockMatchingStep2: Block matching threshold for the second step of BM3D (Wiener filtering),
/// i.e. maximum distance for which two blocks are considered similar.
/// Value expressed in euclidean distance.
/// * groupSize: Maximum size of the 3D group for collaborative filtering.
/// * slidingStep: Sliding step to process every next reference block.
/// * beta: Kaiser window parameter that affects the sidelobe attenuation of the transform of the
/// window. Kaiser window is used in order to reduce border effects. To prevent usage of the window,
/// set beta to zero.
/// * normType: Norm used to calculate distance between blocks. L2 is slower than L1
/// but yields more accurate results.
/// * step: Step of BM3D to be executed. Allowed are only BM3D_STEP1 and BM3D_STEPALL.
/// BM3D_STEP2 is not allowed as it requires basic estimate to be present.
/// * transformType: Type of the orthogonal transform used in collaborative filtering step.
/// Currently only Haar transform is supported.
/// 
/// This function expected to be applied to grayscale images. Advanced usage of this function
/// can be manual denoising of colored image in different colorspaces.
/// ## See also
/// fastNlMeansDenoising
/// 
/// ## C++ default parameters
/// * h: 1
/// * template_window_size: 4
/// * search_window_size: 16
/// * block_matching_step1: 2500
/// * block_matching_step2: 400
/// * group_size: 8
/// * sliding_step: 1
/// * beta: 2.0f
/// * norm_type: cv::NORM_L2
/// * step: cv::xphoto::BM3D_STEPALL
/// * transform_type: cv::xphoto::HAAR
pub fn bm3d_denoising_1(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h, template_window_size, search_window_size, block_matching_step1, block_matching_step2, group_size, sliding_step, beta, norm_type, step, transform_type) }.into_result()
}

/// Creates an instance of GrayworldWB
pub fn create_grayworld_wb() -> Result<core::Ptr::<dyn crate::xphoto::GrayworldWB>> {
	unsafe { sys::cv_xphoto_createGrayworldWB() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xphoto::GrayworldWB>::opencv_from_extern(r) } )
}

/// Creates an instance of LearningBasedWB
/// 
/// ## Parameters
/// * path_to_model: Path to a .yml file with the model. If not specified, the default model is used
/// 
/// ## C++ default parameters
/// * path_to_model: String()
pub fn create_learning_based_wb(path_to_model: &str) -> Result<core::Ptr::<dyn crate::xphoto::LearningBasedWB>> {
	extern_container_arg!(path_to_model);
	unsafe { sys::cv_xphoto_createLearningBasedWB_const_StringR(path_to_model.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xphoto::LearningBasedWB>::opencv_from_extern(r) } )
}

/// Creates an instance of SimpleWB
pub fn create_simple_wb() -> Result<core::Ptr::<dyn crate::xphoto::SimpleWB>> {
	unsafe { sys::cv_xphoto_createSimpleWB() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xphoto::SimpleWB>::opencv_from_extern(r) } )
}

/// The function implements simple dct-based denoising
/// 
/// <http://www.ipol.im/pub/art/2011/ys-dct/>.
/// ## Parameters
/// * src: source image
/// * dst: destination image
/// * sigma: expected noise standard deviation
/// * psize: size of block side where dct is computed
/// ## See also
/// fastNlMeansDenoising
/// 
/// ## C++ default parameters
/// * psize: 16
pub fn dct_denoising(src: &core::Mat, dst: &mut core::Mat, sigma: f64, psize: i32) -> Result<()> {
	unsafe { sys::cv_xphoto_dctDenoising_const_MatR_MatR_const_double_const_int(src.as_raw_Mat(), dst.as_raw_mut_Mat(), sigma, psize) }.into_result()
}

/// The function implements different single-image inpainting algorithms.
/// 
/// See the original paper [He2012](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_He2012) for details.
/// 
/// ## Parameters
/// * src: source image, it could be of any type and any number of channels from 1 to 4. In case of
/// 3- and 4-channels images the function expect them in CIELab colorspace or similar one, where first
/// color component shows intensity, while second and third shows colors. Nonetheless you can try any
/// colorspaces.
/// * mask: mask (CV_8UC1), where non-zero pixels indicate valid image area, while zero pixels
/// indicate area to be inpainted
/// * dst: destination image
/// * algorithmType: see xphoto::InpaintTypes
pub fn inpaint(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, algorithm_type: i32) -> Result<()> {
	unsafe { sys::cv_xphoto_inpaint_const_MatR_const_MatR_MatR_const_int(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_mut_Mat(), algorithm_type) }.into_result()
}

/// Gray-world white balance algorithm
/// 
/// This algorithm scales the values of pixels based on a
/// gray-world assumption which states that the average of all channels
/// should result in a gray image.
/// 
/// It adds a modification which thresholds pixels based on their
/// saturation value and only uses pixels below the provided threshold in
/// finding average pixel values.
/// 
/// Saturation is calculated using the following for a 3-channel RGB image per
/// pixel I and is in the range [0, 1]:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Ctexttt%7BSaturation%7D%20%5BI%5D%20%3D%20%5Cfrac%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%20%2D%20%5Ctextrm%7Bmin%7D%28R%2CG%2CB%29%0A%7D%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%7D%20)
/// 
/// A threshold of 1 means that all pixels are used to white-balance, while a
/// threshold of 0 means no pixels are used. Lower thresholds are useful in
/// white-balancing saturated images.
/// 
/// Currently supports images of type @ref CV_8UC3 and @ref CV_16UC3.
pub trait GrayworldWB: crate::xphoto::WhiteBalancer {
	fn as_raw_GrayworldWB(&self) -> *const c_void;
	fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void;

	/// Maximum saturation for a pixel to be included in the
	///    gray-world assumption
	/// ## See also
	/// setSaturationThreshold
	fn get_saturation_threshold(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_GrayworldWB_getSaturationThreshold_const(self.as_raw_GrayworldWB()) }.into_result()
	}
	
	/// Maximum saturation for a pixel to be included in the
	///    gray-world assumption
	/// ## See also
	/// setSaturationThreshold getSaturationThreshold
	fn set_saturation_threshold(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_GrayworldWB_setSaturationThreshold_float(self.as_raw_mut_GrayworldWB(), val) }.into_result()
	}
	
}

/// More sophisticated learning-based automatic white balance algorithm.
/// 
/// As @ref GrayworldWB, this algorithm works by applying different gains to the input
/// image channels, but their computation is a bit more involved compared to the
/// simple gray-world assumption. More details about the algorithm can be found in
/// [Cheng2015](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Cheng2015) .
/// 
/// To mask out saturated pixels this function uses only pixels that satisfy the
/// following condition:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%7D%7B%5Ctexttt%7Brange%5Fmax%5Fval%7D%7D%20%3C%20%5Ctexttt%7Bsaturation%5Fthresh%7D%20)
/// 
/// Currently supports images of type @ref CV_8UC3 and @ref CV_16UC3.
pub trait LearningBasedWB: crate::xphoto::WhiteBalancer {
	fn as_raw_LearningBasedWB(&self) -> *const c_void;
	fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void;

	/// Implements the feature extraction part of the algorithm.
	/// 
	/// In accordance with [Cheng2015](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Cheng2015) , computes the following features for the input image:
	/// 1. Chromaticity of an average (R,G,B) tuple
	/// 2. Chromaticity of the brightest (R,G,B) tuple (while ignoring saturated pixels)
	/// 3. Chromaticity of the dominant (R,G,B) tuple (the one that has the highest value in the RGB histogram)
	/// 4. Mode of the chromaticity palette, that is constructed by taking 300 most common colors according to
	///    the RGB histogram and projecting them on the chromaticity plane. Mode is the most high-density point
	///    of the palette, which is computed by a straightforward fixed-bandwidth kernel density estimator with
	///    a Epanechnikov kernel function.
	/// 
	/// ## Parameters
	/// * src: Input three-channel image (BGR color space is assumed).
	/// * dst: An array of four (r,g) chromaticity tuples corresponding to the features listed above.
	fn extract_simple_features(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_LearningBasedWB(), src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// Maximum possible value of the input image (e.g. 255 for 8 bit images,
	///            4095 for 12 bit images)
	/// ## See also
	/// setRangeMaxVal
	fn get_range_max_val(&self) -> Result<i32> {
		unsafe { sys::cv_xphoto_LearningBasedWB_getRangeMaxVal_const(self.as_raw_LearningBasedWB()) }.into_result()
	}
	
	/// Maximum possible value of the input image (e.g. 255 for 8 bit images,
	///            4095 for 12 bit images)
	/// ## See also
	/// setRangeMaxVal getRangeMaxVal
	fn set_range_max_val(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_xphoto_LearningBasedWB_setRangeMaxVal_int(self.as_raw_mut_LearningBasedWB(), val) }.into_result()
	}
	
	/// Threshold that is used to determine saturated pixels, i.e. pixels where at least one of the
	///    channels exceeds ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsaturation%5Fthreshold%7D%5Ctimes%5Ctexttt%7Brange%5Fmax%5Fval%7D) are ignored.
	/// ## See also
	/// setSaturationThreshold
	fn get_saturation_threshold(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_LearningBasedWB_getSaturationThreshold_const(self.as_raw_LearningBasedWB()) }.into_result()
	}
	
	/// Threshold that is used to determine saturated pixels, i.e. pixels where at least one of the
	///    channels exceeds ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsaturation%5Fthreshold%7D%5Ctimes%5Ctexttt%7Brange%5Fmax%5Fval%7D) are ignored.
	/// ## See also
	/// setSaturationThreshold getSaturationThreshold
	fn set_saturation_threshold(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_LearningBasedWB_setSaturationThreshold_float(self.as_raw_mut_LearningBasedWB(), val) }.into_result()
	}
	
	/// Defines the size of one dimension of a three-dimensional RGB histogram that is used internally
	///    by the algorithm. It often makes sense to increase the number of bins for images with higher bit depth
	///    (e.g. 256 bins for a 12 bit image).
	/// ## See also
	/// setHistBinNum
	fn get_hist_bin_num(&self) -> Result<i32> {
		unsafe { sys::cv_xphoto_LearningBasedWB_getHistBinNum_const(self.as_raw_LearningBasedWB()) }.into_result()
	}
	
	/// Defines the size of one dimension of a three-dimensional RGB histogram that is used internally
	///    by the algorithm. It often makes sense to increase the number of bins for images with higher bit depth
	///    (e.g. 256 bins for a 12 bit image).
	/// ## See also
	/// setHistBinNum getHistBinNum
	fn set_hist_bin_num(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_xphoto_LearningBasedWB_setHistBinNum_int(self.as_raw_mut_LearningBasedWB(), val) }.into_result()
	}
	
}

/// A simple white balance algorithm that works by independently stretching
/// each of the input image channels to the specified range. For increased robustness
/// it ignores the top and bottom ![inline formula](https://latex.codecogs.com/png.latex?p%5C%25) of pixel values.
pub trait SimpleWB: crate::xphoto::WhiteBalancer {
	fn as_raw_SimpleWB(&self) -> *const c_void;
	fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void;

	/// Input image range minimum value
	/// ## See also
	/// setInputMin
	fn get_input_min(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_SimpleWB_getInputMin_const(self.as_raw_SimpleWB()) }.into_result()
	}
	
	/// Input image range minimum value
	/// ## See also
	/// setInputMin getInputMin
	fn set_input_min(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_SimpleWB_setInputMin_float(self.as_raw_mut_SimpleWB(), val) }.into_result()
	}
	
	/// Input image range maximum value
	/// ## See also
	/// setInputMax
	fn get_input_max(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_SimpleWB_getInputMax_const(self.as_raw_SimpleWB()) }.into_result()
	}
	
	/// Input image range maximum value
	/// ## See also
	/// setInputMax getInputMax
	fn set_input_max(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_SimpleWB_setInputMax_float(self.as_raw_mut_SimpleWB(), val) }.into_result()
	}
	
	/// Output image range minimum value
	/// ## See also
	/// setOutputMin
	fn get_output_min(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_SimpleWB_getOutputMin_const(self.as_raw_SimpleWB()) }.into_result()
	}
	
	/// Output image range minimum value
	/// ## See also
	/// setOutputMin getOutputMin
	fn set_output_min(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_SimpleWB_setOutputMin_float(self.as_raw_mut_SimpleWB(), val) }.into_result()
	}
	
	/// Output image range maximum value
	/// ## See also
	/// setOutputMax
	fn get_output_max(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_SimpleWB_getOutputMax_const(self.as_raw_SimpleWB()) }.into_result()
	}
	
	/// Output image range maximum value
	/// ## See also
	/// setOutputMax getOutputMax
	fn set_output_max(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_SimpleWB_setOutputMax_float(self.as_raw_mut_SimpleWB(), val) }.into_result()
	}
	
	/// Percent of top/bottom values to ignore
	/// ## See also
	/// setP
	fn get_p(&self) -> Result<f32> {
		unsafe { sys::cv_xphoto_SimpleWB_getP_const(self.as_raw_SimpleWB()) }.into_result()
	}
	
	/// Percent of top/bottom values to ignore
	/// ## See also
	/// setP getP
	fn set_p(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_xphoto_SimpleWB_setP_float(self.as_raw_mut_SimpleWB(), val) }.into_result()
	}
	
}

/// The base class for auto white balance algorithms.
pub trait WhiteBalancer: core::AlgorithmTrait {
	fn as_raw_WhiteBalancer(&self) -> *const c_void;
	fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void;

	/// Applies white balancing to the input image
	/// 
	/// ## Parameters
	/// * src: Input image
	/// * dst: White balancing result
	/// ## See also
	/// cvtColor, equalizeHist
	fn balance_white(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_WhiteBalancer(), src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
}
