template struct Result<bool>;
template struct Result<const std::vector<cv::Mat>*>;
template struct Result<const std::vector<cv::Rect_<double>>*>;
template struct Result<cv::ClfMilBoost*>;
template struct Result<cv::ClfMilBoost::Params*>;
template struct Result<cv::CvHaarEvaluator::FeatureHaar*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Mat_<double>*>;
template struct Result<cv::MultiTracker*>;
template struct Result<cv::MultiTracker_Alt*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Ptr<cv::MultiTracker>*>;
template struct Result<cv::Ptr<cv::TrackerBoosting>*>;
template struct Result<cv::Ptr<cv::TrackerCSRT>*>;
template struct Result<cv::Ptr<cv::TrackerFeature>*>;
template struct Result<cv::Ptr<cv::TrackerGOTURN>*>;
template struct Result<cv::Ptr<cv::TrackerKCF>*>;
template struct Result<cv::Ptr<cv::TrackerMIL>*>;
template struct Result<cv::Ptr<cv::TrackerMOSSE>*>;
template struct Result<cv::Ptr<cv::TrackerMedianFlow>*>;
template struct Result<cv::Ptr<cv::TrackerSamplerAlgorithm>*>;
template struct Result<cv::Ptr<cv::TrackerStateEstimator>*>;
template struct Result<cv::Ptr<cv::TrackerTLD>*>;
template struct Result<cv::Ptr<cv::TrackerTargetState>*>;
template struct Result<cv::Ptr<cv::Tracker>*>;
template struct Result<cv::Rect_<double>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::TrackerBoosting::Params*>;
template struct Result<cv::TrackerCSRT::Params*>;
template struct Result<cv::TrackerFeatureFeature2d*>;
template struct Result<cv::TrackerFeatureHAAR*>;
template struct Result<cv::TrackerFeatureHAAR::Params*>;
template struct Result<cv::TrackerFeatureHOG*>;
template struct Result<cv::TrackerFeatureLBP*>;
template struct Result<cv::TrackerFeatureSet*>;
template struct Result<cv::TrackerGOTURN::Params*>;
template struct Result<cv::TrackerKCF::Params*>;
template struct Result<cv::TrackerMIL::Params*>;
template struct Result<cv::TrackerMedianFlow::Params*>;
template struct Result<cv::TrackerSamplerCSC*>;
template struct Result<cv::TrackerSamplerCSC::Params*>;
template struct Result<cv::TrackerSamplerCS*>;
template struct Result<cv::TrackerSamplerCS::Params*>;
template struct Result<cv::TrackerSamplerPF*>;
template struct Result<cv::TrackerSamplerPF::Params*>;
template struct Result<cv::TrackerSampler*>;
template struct Result<cv::TrackerStateEstimatorAdaBoosting*>;
template struct Result<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*>;
template struct Result<cv::TrackerStateEstimatorMILBoosting*>;
template struct Result<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*>;
template struct Result<cv::TrackerStateEstimatorSVM*>;
template struct Result<cv::TrackerTLD::Params*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Ptr<cv::Tracker>>*>;
template struct Result<std::vector<cv::Rect_<double>>*>;
template struct Result<std::vector<cv::Scalar_<double>>*>;
template struct Result<std::vector<float>*>;
template struct Result<std::vector<int>*>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::MultiTracker>* cv_PtrOfMultiTracker_new(cv::MultiTracker* val) {
		return new cv::Ptr<cv::MultiTracker>(val);
	}
	
	void cv_PtrOfMultiTracker_delete(cv::Ptr<cv::MultiTracker>* instance) {
		delete instance;
	}

	const cv::MultiTracker* cv_PtrOfMultiTracker_get_inner_ptr(const cv::Ptr<cv::MultiTracker>* instance) {
		return instance->get();
	}

	cv::MultiTracker* cv_PtrOfMultiTracker_get_inner_ptr_mut(cv::Ptr<cv::MultiTracker>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTracker_delete(cv::Ptr<cv::Tracker>* instance) {
		delete instance;
	}

	const cv::Tracker* cv_PtrOfTracker_get_inner_ptr(const cv::Ptr<cv::Tracker>* instance) {
		return instance->get();
	}

	cv::Tracker* cv_PtrOfTracker_get_inner_ptr_mut(cv::Ptr<cv::Tracker>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerBoosting_delete(cv::Ptr<cv::TrackerBoosting>* instance) {
		delete instance;
	}

	const cv::TrackerBoosting* cv_PtrOfTrackerBoosting_get_inner_ptr(const cv::Ptr<cv::TrackerBoosting>* instance) {
		return instance->get();
	}

	cv::TrackerBoosting* cv_PtrOfTrackerBoosting_get_inner_ptr_mut(cv::Ptr<cv::TrackerBoosting>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerCSRT_delete(cv::Ptr<cv::TrackerCSRT>* instance) {
		delete instance;
	}

	const cv::TrackerCSRT* cv_PtrOfTrackerCSRT_get_inner_ptr(const cv::Ptr<cv::TrackerCSRT>* instance) {
		return instance->get();
	}

	cv::TrackerCSRT* cv_PtrOfTrackerCSRT_get_inner_ptr_mut(cv::Ptr<cv::TrackerCSRT>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerFeature_delete(cv::Ptr<cv::TrackerFeature>* instance) {
		delete instance;
	}

	const cv::TrackerFeature* cv_PtrOfTrackerFeature_get_inner_ptr(const cv::Ptr<cv::TrackerFeature>* instance) {
		return instance->get();
	}

	cv::TrackerFeature* cv_PtrOfTrackerFeature_get_inner_ptr_mut(cv::Ptr<cv::TrackerFeature>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerGOTURN_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
		delete instance;
	}

	const cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr(const cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}

	cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr_mut(cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerKCF_delete(cv::Ptr<cv::TrackerKCF>* instance) {
		delete instance;
	}

	const cv::TrackerKCF* cv_PtrOfTrackerKCF_get_inner_ptr(const cv::Ptr<cv::TrackerKCF>* instance) {
		return instance->get();
	}

	cv::TrackerKCF* cv_PtrOfTrackerKCF_get_inner_ptr_mut(cv::Ptr<cv::TrackerKCF>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMIL_delete(cv::Ptr<cv::TrackerMIL>* instance) {
		delete instance;
	}

	const cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr(const cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}

	cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr_mut(cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMOSSE_delete(cv::Ptr<cv::TrackerMOSSE>* instance) {
		delete instance;
	}

	const cv::TrackerMOSSE* cv_PtrOfTrackerMOSSE_get_inner_ptr(const cv::Ptr<cv::TrackerMOSSE>* instance) {
		return instance->get();
	}

	cv::TrackerMOSSE* cv_PtrOfTrackerMOSSE_get_inner_ptr_mut(cv::Ptr<cv::TrackerMOSSE>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMedianFlow_delete(cv::Ptr<cv::TrackerMedianFlow>* instance) {
		delete instance;
	}

	const cv::TrackerMedianFlow* cv_PtrOfTrackerMedianFlow_get_inner_ptr(const cv::Ptr<cv::TrackerMedianFlow>* instance) {
		return instance->get();
	}

	cv::TrackerMedianFlow* cv_PtrOfTrackerMedianFlow_get_inner_ptr_mut(cv::Ptr<cv::TrackerMedianFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerSamplerAlgorithm_delete(cv::Ptr<cv::TrackerSamplerAlgorithm>* instance) {
		delete instance;
	}

	const cv::TrackerSamplerAlgorithm* cv_PtrOfTrackerSamplerAlgorithm_get_inner_ptr(const cv::Ptr<cv::TrackerSamplerAlgorithm>* instance) {
		return instance->get();
	}

	cv::TrackerSamplerAlgorithm* cv_PtrOfTrackerSamplerAlgorithm_get_inner_ptr_mut(cv::Ptr<cv::TrackerSamplerAlgorithm>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerStateEstimator_delete(cv::Ptr<cv::TrackerStateEstimator>* instance) {
		delete instance;
	}

	const cv::TrackerStateEstimator* cv_PtrOfTrackerStateEstimator_get_inner_ptr(const cv::Ptr<cv::TrackerStateEstimator>* instance) {
		return instance->get();
	}

	cv::TrackerStateEstimator* cv_PtrOfTrackerStateEstimator_get_inner_ptr_mut(cv::Ptr<cv::TrackerStateEstimator>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerTLD_delete(cv::Ptr<cv::TrackerTLD>* instance) {
		delete instance;
	}

	const cv::TrackerTLD* cv_PtrOfTrackerTLD_get_inner_ptr(const cv::Ptr<cv::TrackerTLD>* instance) {
		return instance->get();
	}

	cv::TrackerTLD* cv_PtrOfTrackerTLD_get_inner_ptr_mut(cv::Ptr<cv::TrackerTLD>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::TrackerTargetState>* cv_PtrOfTrackerTargetState_new(cv::TrackerTargetState* val) {
		return new cv::Ptr<cv::TrackerTargetState>(val);
	}
	
	void cv_PtrOfTrackerTargetState_delete(cv::Ptr<cv::TrackerTargetState>* instance) {
		delete instance;
	}

	const cv::TrackerTargetState* cv_PtrOfTrackerTargetState_get_inner_ptr(const cv::Ptr<cv::TrackerTargetState>* instance) {
		return instance->get();
	}

	cv::TrackerTargetState* cv_PtrOfTrackerTargetState_get_inner_ptr_mut(cv::Ptr<cv::TrackerTargetState>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfPtrOfTracker_delete(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::Tracker>>* cv_VectorOfPtrOfTracker_new() {
		return new std::vector<cv::Ptr<cv::Tracker>>();
	}

	size_t cv_VectorOfPtrOfTracker_len(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfTracker_is_empty(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfTracker_capacity(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfTracker_shrink_to_fit(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfTracker_reserve(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfTracker_remove(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfTracker_swap(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfTracker_clear(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfTracker_push(std::vector<cv::Ptr<cv::Tracker>>* instance, cv::Ptr<cv::Tracker>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfTracker_insert(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, cv::Ptr<cv::Tracker>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::Tracker>*> cv_VectorOfPtrOfTracker_get(const std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::Tracker>*>(new cv::Ptr<cv::Tracker>((*instance)[index]));
	}

	void cv_VectorOfPtrOfTracker_set(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, cv::Ptr<cv::Tracker>* val) {
		(*instance)[index] = *val;
	}

}


