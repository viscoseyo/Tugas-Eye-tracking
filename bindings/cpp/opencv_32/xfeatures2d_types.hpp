template struct Result<bool>;
template struct Result<cv::KeyPoint>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::DAISY>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::FREAK>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::LATCH>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::LUCID>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::SIFT>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::SURF>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>;
template struct Result<cv::Ptr<cv::xfeatures2d::VGG>*>;
template struct Result<cv::cuda::GpuMat*>;
template struct Result<cv::cuda::SURF_CUDA*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<int>*>;
extern "C" {
	cv::Ptr<cv::xfeatures2d::BoostDesc>* cv_PtrOfBoostDesc_new(cv::xfeatures2d::BoostDesc* val) {
		return new cv::Ptr<cv::xfeatures2d::BoostDesc>(val);
	}
	
	void cv_PtrOfBoostDesc_delete(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::BoostDesc* cv_PtrOfBoostDesc_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::BoostDesc* cv_PtrOfBoostDesc_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* cv_PtrOfBriefDescriptorExtractor_new(cv::xfeatures2d::BriefDescriptorExtractor* val) {
		return new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(val);
	}
	
	void cv_PtrOfBriefDescriptorExtractor_delete(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrOfBriefDescriptorExtractor_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDAISY_delete(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::DAISY* cv_PtrOfDAISY_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::DAISY* cv_PtrOfDAISY_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::FREAK>* cv_PtrOfFREAK_new(cv::xfeatures2d::FREAK* val) {
		return new cv::Ptr<cv::xfeatures2d::FREAK>(val);
	}
	
	void cv_PtrOfFREAK_delete(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::FREAK* cv_PtrOfFREAK_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::FREAK* cv_PtrOfFREAK_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::LATCH>* cv_PtrOfLATCH_new(cv::xfeatures2d::LATCH* val) {
		return new cv::Ptr<cv::xfeatures2d::LATCH>(val);
	}
	
	void cv_PtrOfLATCH_delete(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::LATCH* cv_PtrOfLATCH_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::LATCH* cv_PtrOfLATCH_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::LUCID>* cv_PtrOfLUCID_new(cv::xfeatures2d::LUCID* val) {
		return new cv::Ptr<cv::xfeatures2d::LUCID>(val);
	}
	
	void cv_PtrOfLUCID_delete(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::LUCID* cv_PtrOfLUCID_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::LUCID* cv_PtrOfLUCID_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::MSDDetector>* cv_PtrOfMSDDetector_new(cv::xfeatures2d::MSDDetector* val) {
		return new cv::Ptr<cv::xfeatures2d::MSDDetector>(val);
	}
	
	void cv_PtrOfMSDDetector_delete(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::MSDDetector* cv_PtrOfMSDDetector_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::MSDDetector* cv_PtrOfMSDDetector_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfPCTSignatures_delete(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::PCTSignatures* cv_PtrOfPCTSignatures_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::PCTSignatures* cv_PtrOfPCTSignatures_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfPCTSignaturesSQFD_delete(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrOfPCTSignaturesSQFD_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrOfPCTSignaturesSQFD_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::SIFT>* cv_PtrOfSIFT_new(cv::xfeatures2d::SIFT* val) {
		return new cv::Ptr<cv::xfeatures2d::SIFT>(val);
	}
	
	void cv_PtrOfSIFT_delete(cv::Ptr<cv::xfeatures2d::SIFT>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::SIFT* cv_PtrOfSIFT_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::SIFT>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::SIFT* cv_PtrOfSIFT_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::SIFT>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfSURF_delete(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::SURF* cv_PtrOfSURF_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::SURF* cv_PtrOfSURF_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::xfeatures2d::StarDetector>* cv_PtrOfStarDetector_new(cv::xfeatures2d::StarDetector* val) {
		return new cv::Ptr<cv::xfeatures2d::StarDetector>(val);
	}
	
	void cv_PtrOfStarDetector_delete(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::StarDetector* cv_PtrOfStarDetector_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::StarDetector* cv_PtrOfStarDetector_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfVGG_delete(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::VGG* cv_PtrOfVGG_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::VGG* cv_PtrOfVGG_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		return instance->get();
	}
}

