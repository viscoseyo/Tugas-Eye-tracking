#include "ocvrs_common.hpp"
#include <opencv2/fuzzy.hpp>
#include "fuzzy_types.hpp"

extern "C" {
	Result_void cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, const cv::_InputArray* mask) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height) {
		try {
			cv::ft::FT02D_inverseFT(*components, *kernel, *output, width, height);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, const cv::_OutputArray* maskOutput, bool firstStop) {
		try {
			int ret = cv::ft::FT02D_iteration(*matrix, *kernel, *output, *mask, *maskOutput, firstStop);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* kernel, const int chn) {
		try {
			cv::ft::createKernel(*A, *B, *kernel, chn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_createKernel_int_int_const__OutputArrayR_const_int(int function, int radius, const cv::_OutputArray* kernel, const int chn) {
		try {
			cv::ft::createKernel(function, radius, *kernel, chn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* kernel, const cv::_OutputArray* output) {
		try {
			cv::ft::filter(*image, *kernel, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* output, int radius, int function, int algorithm) {
		try {
			cv::ft::inpaint(*image, *mask, *output, radius, function, algorithm);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
