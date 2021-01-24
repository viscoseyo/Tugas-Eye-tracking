template struct Result<bool>;
template struct Result<const std::vector<cv::Ptr<cv::linemod::Modality>>*>;
template struct Result<const std::vector<cv::linemod::Template>*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::linemod::ColorGradient>*>;
template struct Result<cv::Ptr<cv::linemod::DepthNormal>*>;
template struct Result<cv::Ptr<cv::linemod::Detector>*>;
template struct Result<cv::Ptr<cv::linemod::Modality>*>;
template struct Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>;
template struct Result<cv::Ptr<cv::rgbd::DepthCleaner>*>;
template struct Result<cv::Ptr<cv::rgbd::ICPOdometry>*>;
template struct Result<cv::Ptr<cv::rgbd::OdometryFrame>*>;
template struct Result<cv::Ptr<cv::rgbd::Odometry>*>;
template struct Result<cv::Ptr<cv::rgbd::RgbdFrame>*>;
template struct Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>;
template struct Result<cv::Ptr<cv::rgbd::RgbdNormals>*>;
template struct Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>;
template struct Result<cv::Ptr<cv::rgbd::RgbdPlane>*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::linemod::ColorGradient*>;
template struct Result<cv::linemod::DepthNormal*>;
template struct Result<cv::linemod::Detector*>;
template struct Result<cv::linemod::Feature>;
template struct Result<cv::linemod::Match*>;
template struct Result<cv::linemod::Template*>;
template struct Result<cv::rgbd::DepthCleaner*>;
template struct Result<cv::rgbd::ICPOdometry*>;
template struct Result<cv::rgbd::OdometryFrame*>;
template struct Result<cv::rgbd::RgbdFrame*>;
template struct Result<cv::rgbd::RgbdICPOdometry*>;
template struct Result<cv::rgbd::RgbdNormals*>;
template struct Result<cv::rgbd::RgbdOdometry*>;
template struct Result<cv::rgbd::RgbdPlane*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::String>*>;
template struct Result<std::vector<cv::linemod::Feature>*>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::rgbd::DepthCleaner>* cv_PtrOfDepthCleaner_new(cv::rgbd::DepthCleaner* val) {
		return new cv::Ptr<cv::rgbd::DepthCleaner>(val);
	}
	
	void cv_PtrOfDepthCleaner_delete(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		delete instance;
	}

	const cv::rgbd::DepthCleaner* cv_PtrOfDepthCleaner_get_inner_ptr(const cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		return instance->get();
	}

	cv::rgbd::DepthCleaner* cv_PtrOfDepthCleaner_get_inner_ptr_mut(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::ICPOdometry>* cv_PtrOfICPOdometry_new(cv::rgbd::ICPOdometry* val) {
		return new cv::Ptr<cv::rgbd::ICPOdometry>(val);
	}
	
	void cv_PtrOfICPOdometry_delete(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::ICPOdometry* cv_PtrOfICPOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::ICPOdometry* cv_PtrOfICPOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::linemod::ColorGradient>* cv_PtrOfLinemod_ColorGradient_new(cv::linemod::ColorGradient* val) {
		return new cv::Ptr<cv::linemod::ColorGradient>(val);
	}
	
	void cv_PtrOfLinemod_ColorGradient_delete(cv::Ptr<cv::linemod::ColorGradient>* instance) {
		delete instance;
	}

	const cv::linemod::ColorGradient* cv_PtrOfLinemod_ColorGradient_get_inner_ptr(const cv::Ptr<cv::linemod::ColorGradient>* instance) {
		return instance->get();
	}

	cv::linemod::ColorGradient* cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut(cv::Ptr<cv::linemod::ColorGradient>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::linemod::DepthNormal>* cv_PtrOfLinemod_DepthNormal_new(cv::linemod::DepthNormal* val) {
		return new cv::Ptr<cv::linemod::DepthNormal>(val);
	}
	
	void cv_PtrOfLinemod_DepthNormal_delete(cv::Ptr<cv::linemod::DepthNormal>* instance) {
		delete instance;
	}

	const cv::linemod::DepthNormal* cv_PtrOfLinemod_DepthNormal_get_inner_ptr(const cv::Ptr<cv::linemod::DepthNormal>* instance) {
		return instance->get();
	}

	cv::linemod::DepthNormal* cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut(cv::Ptr<cv::linemod::DepthNormal>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::linemod::Detector>* cv_PtrOfLinemod_Detector_new(cv::linemod::Detector* val) {
		return new cv::Ptr<cv::linemod::Detector>(val);
	}
	
	void cv_PtrOfLinemod_Detector_delete(cv::Ptr<cv::linemod::Detector>* instance) {
		delete instance;
	}

	const cv::linemod::Detector* cv_PtrOfLinemod_Detector_get_inner_ptr(const cv::Ptr<cv::linemod::Detector>* instance) {
		return instance->get();
	}

	cv::linemod::Detector* cv_PtrOfLinemod_Detector_get_inner_ptr_mut(cv::Ptr<cv::linemod::Detector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLinemod_Modality_delete(cv::Ptr<cv::linemod::Modality>* instance) {
		delete instance;
	}

	const cv::linemod::Modality* cv_PtrOfLinemod_Modality_get_inner_ptr(const cv::Ptr<cv::linemod::Modality>* instance) {
		return instance->get();
	}

	cv::linemod::Modality* cv_PtrOfLinemod_Modality_get_inner_ptr_mut(cv::Ptr<cv::linemod::Modality>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLinemod_QuantizedPyramid_delete(cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		delete instance;
	}

	const cv::linemod::QuantizedPyramid* cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr(const cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		return instance->get();
	}

	cv::linemod::QuantizedPyramid* cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut(cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfOdometry_delete(cv::Ptr<cv::rgbd::Odometry>* instance) {
		delete instance;
	}

	const cv::rgbd::Odometry* cv_PtrOfOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::Odometry>* instance) {
		return instance->get();
	}

	cv::rgbd::Odometry* cv_PtrOfOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::Odometry>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::OdometryFrame>* cv_PtrOfOdometryFrame_new(cv::rgbd::OdometryFrame* val) {
		return new cv::Ptr<cv::rgbd::OdometryFrame>(val);
	}
	
	void cv_PtrOfOdometryFrame_delete(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		delete instance;
	}

	const cv::rgbd::OdometryFrame* cv_PtrOfOdometryFrame_get_inner_ptr(const cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		return instance->get();
	}

	cv::rgbd::OdometryFrame* cv_PtrOfOdometryFrame_get_inner_ptr_mut(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrOfRgbdFrame_new(cv::rgbd::RgbdFrame* val) {
		return new cv::Ptr<cv::rgbd::RgbdFrame>(val);
	}
	
	void cv_PtrOfRgbdFrame_delete(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdFrame* cv_PtrOfRgbdFrame_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdFrame* cv_PtrOfRgbdFrame_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::RgbdICPOdometry>* cv_PtrOfRgbdICPOdometry_new(cv::rgbd::RgbdICPOdometry* val) {
		return new cv::Ptr<cv::rgbd::RgbdICPOdometry>(val);
	}
	
	void cv_PtrOfRgbdICPOdometry_delete(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdICPOdometry* cv_PtrOfRgbdICPOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdICPOdometry* cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::RgbdNormals>* cv_PtrOfRgbdNormals_new(cv::rgbd::RgbdNormals* val) {
		return new cv::Ptr<cv::rgbd::RgbdNormals>(val);
	}
	
	void cv_PtrOfRgbdNormals_delete(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdNormals* cv_PtrOfRgbdNormals_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdNormals* cv_PtrOfRgbdNormals_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::RgbdOdometry>* cv_PtrOfRgbdOdometry_new(cv::rgbd::RgbdOdometry* val) {
		return new cv::Ptr<cv::rgbd::RgbdOdometry>(val);
	}
	
	void cv_PtrOfRgbdOdometry_delete(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdOdometry* cv_PtrOfRgbdOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdOdometry* cv_PtrOfRgbdOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::rgbd::RgbdPlane>* cv_PtrOfRgbdPlane_new(cv::rgbd::RgbdPlane* val) {
		return new cv::Ptr<cv::rgbd::RgbdPlane>(val);
	}
	
	void cv_PtrOfRgbdPlane_delete(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdPlane* cv_PtrOfRgbdPlane_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdPlane* cv_PtrOfRgbdPlane_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfLinemod_Feature_delete(std::vector<cv::linemod::Feature>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Feature>* cv_VectorOfLinemod_Feature_new() {
		return new std::vector<cv::linemod::Feature>();
	}

	size_t cv_VectorOfLinemod_Feature_len(const std::vector<cv::linemod::Feature>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Feature_is_empty(const std::vector<cv::linemod::Feature>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Feature_capacity(const std::vector<cv::linemod::Feature>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfLinemod_Feature_shrink_to_fit(std::vector<cv::linemod::Feature>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Feature_reserve(std::vector<cv::linemod::Feature>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Feature_remove(std::vector<cv::linemod::Feature>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Feature_swap(std::vector<cv::linemod::Feature>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Feature_clear(std::vector<cv::linemod::Feature>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Feature_push(std::vector<cv::linemod::Feature>* instance, cv::linemod::Feature* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Feature_insert(std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::linemod::Feature> cv_VectorOfLinemod_Feature_get(const std::vector<cv::linemod::Feature>* instance, size_t index) {
		return Ok<cv::linemod::Feature>((*instance)[index]);
	}

	void cv_VectorOfLinemod_Feature_set(std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* val) {
		(*instance)[index] = *val;
	}

	const cv::linemod::Feature* cv_VectorOfLinemod_Feature_data(const std::vector<cv::linemod::Feature>* instance) {
		return instance->data();
	}
	
	cv::linemod::Feature* cv_VectorOfLinemod_Feature_data_mut(std::vector<cv::linemod::Feature>* instance) {
		return instance->data();
	}
	
		std::vector<cv::linemod::Feature>* cv_VectorOfLinemod_Feature_clone(const std::vector<cv::linemod::Feature>* instance) {
			return new std::vector<cv::linemod::Feature>(*instance);
		}
	
}


extern "C" {
	void cv_VectorOfLinemod_Match_delete(std::vector<cv::linemod::Match>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Match>* cv_VectorOfLinemod_Match_new() {
		return new std::vector<cv::linemod::Match>();
	}

	size_t cv_VectorOfLinemod_Match_len(const std::vector<cv::linemod::Match>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Match_is_empty(const std::vector<cv::linemod::Match>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Match_capacity(const std::vector<cv::linemod::Match>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfLinemod_Match_shrink_to_fit(std::vector<cv::linemod::Match>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Match_reserve(std::vector<cv::linemod::Match>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Match_remove(std::vector<cv::linemod::Match>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Match_swap(std::vector<cv::linemod::Match>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Match_clear(std::vector<cv::linemod::Match>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Match_push(std::vector<cv::linemod::Match>* instance, cv::linemod::Match* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Match_insert(std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::linemod::Match*> cv_VectorOfLinemod_Match_get(const std::vector<cv::linemod::Match>* instance, size_t index) {
		return Ok<cv::linemod::Match*>(new cv::linemod::Match((*instance)[index]));
	}

	void cv_VectorOfLinemod_Match_set(std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfLinemod_Template_delete(std::vector<cv::linemod::Template>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Template>* cv_VectorOfLinemod_Template_new() {
		return new std::vector<cv::linemod::Template>();
	}

	size_t cv_VectorOfLinemod_Template_len(const std::vector<cv::linemod::Template>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Template_is_empty(const std::vector<cv::linemod::Template>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Template_capacity(const std::vector<cv::linemod::Template>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfLinemod_Template_shrink_to_fit(std::vector<cv::linemod::Template>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Template_reserve(std::vector<cv::linemod::Template>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Template_remove(std::vector<cv::linemod::Template>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Template_swap(std::vector<cv::linemod::Template>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Template_clear(std::vector<cv::linemod::Template>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Template_push(std::vector<cv::linemod::Template>* instance, cv::linemod::Template* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Template_insert(std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::linemod::Template*> cv_VectorOfLinemod_Template_get(const std::vector<cv::linemod::Template>* instance, size_t index) {
		return Ok<cv::linemod::Template*>(new cv::linemod::Template((*instance)[index]));
	}

	void cv_VectorOfLinemod_Template_set(std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPtrOfLinemod_Modality_delete(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::linemod::Modality>>* cv_VectorOfPtrOfLinemod_Modality_new() {
		return new std::vector<cv::Ptr<cv::linemod::Modality>>();
	}

	size_t cv_VectorOfPtrOfLinemod_Modality_len(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfLinemod_Modality_is_empty(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfLinemod_Modality_capacity(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfLinemod_Modality_reserve(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfLinemod_Modality_remove(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfLinemod_Modality_swap(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfLinemod_Modality_clear(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfLinemod_Modality_push(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, cv::Ptr<cv::linemod::Modality>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfLinemod_Modality_insert(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::linemod::Modality>*> cv_VectorOfPtrOfLinemod_Modality_get(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::linemod::Modality>*>(new cv::Ptr<cv::linemod::Modality>((*instance)[index]));
	}

	void cv_VectorOfPtrOfLinemod_Modality_set(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>* val) {
		(*instance)[index] = *val;
	}

}


