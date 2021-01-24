#include "ocvrs_common.hpp"
#include <opencv2/phase_unwrapping.hpp>
#include "phase_unwrapping_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*> cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* parameters) {
		try {
			cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping> ret = cv::phase_unwrapping::HistogramPhaseUnwrapping::create(*parameters);
			return Ok(new cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(cv::phase_unwrapping::HistogramPhaseUnwrapping* instance, const cv::_OutputArray* reliabilityMap) {
		try {
			instance->getInverseReliabilityMap(*reliabilityMap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropWidth_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->width;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropWidth_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->width = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropHeight_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->height;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropHeight_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->height = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropHistThresh_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			float ret = instance->histThresh;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropHistThresh_float(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, float val) {
		try {
			instance->histThresh = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropNbrOfSmallBins_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->nbrOfSmallBins;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropNbrOfSmallBins_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->nbrOfSmallBins = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_getPropNbrOfLargeBins_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->nbrOfLargeBins;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setPropNbrOfLargeBins_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->nbrOfLargeBins = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_HistogramPhaseUnwrapping_Params_delete(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		delete instance;
	}
	Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() {
		try {
			cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* ret = new cv::phase_unwrapping::HistogramPhaseUnwrapping::Params();
			return Ok<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>))
	}
	
	Result_void cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::phase_unwrapping::PhaseUnwrapping* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, const cv::_InputArray* shadowMask) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *shadowMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
