
#[cfg(feature = "contrib")]
mod aruco_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBoard = core::Ptr::<crate::aruco::Board>;
	
	ptr_extern! { crate::aruco::Board,
		cv_PtrOfBoard_delete, cv_PtrOfBoard_get_inner_ptr, cv_PtrOfBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::Board, cv_PtrOfBoard_new }
	
	impl PtrOfBoard {
		#[inline] pub fn as_raw_PtrOfBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCharucoBoard = core::Ptr::<crate::aruco::CharucoBoard>;
	
	ptr_extern! { crate::aruco::CharucoBoard,
		cv_PtrOfCharucoBoard_delete, cv_PtrOfCharucoBoard_get_inner_ptr, cv_PtrOfCharucoBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::CharucoBoard, cv_PtrOfCharucoBoard_new }
	
	impl PtrOfCharucoBoard {
		#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfCharucoBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::CharucoBoardTrait for PtrOfCharucoBoard {
		#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectorParameters = core::Ptr::<crate::aruco::DetectorParameters>;
	
	ptr_extern! { crate::aruco::DetectorParameters,
		cv_PtrOfDetectorParameters_delete, cv_PtrOfDetectorParameters_get_inner_ptr, cv_PtrOfDetectorParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::DetectorParameters, cv_PtrOfDetectorParameters_new }
	
	impl PtrOfDetectorParameters {
		#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DetectorParametersTrait for PtrOfDetectorParameters {
		#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDictionary = core::Ptr::<crate::aruco::Dictionary>;
	
	ptr_extern! { crate::aruco::Dictionary,
		cv_PtrOfDictionary_delete, cv_PtrOfDictionary_get_inner_ptr, cv_PtrOfDictionary_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::Dictionary, cv_PtrOfDictionary_new }
	
	impl PtrOfDictionary {
		#[inline] pub fn as_raw_PtrOfDictionary(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DictionaryTrait for PtrOfDictionary {
		#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGridBoard = core::Ptr::<crate::aruco::GridBoard>;
	
	ptr_extern! { crate::aruco::GridBoard,
		cv_PtrOfGridBoard_delete, cv_PtrOfGridBoard_get_inner_ptr, cv_PtrOfGridBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::GridBoard, cv_PtrOfGridBoard_new }
	
	impl PtrOfGridBoard {
		#[inline] pub fn as_raw_PtrOfGridBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfGridBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::GridBoardTrait for PtrOfGridBoard {
		#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use aruco_types::*;

#[cfg(feature = "contrib")]
mod bgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorCNT = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorCNT>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorCNT,
		cv_PtrOfBackgroundSubtractorCNT_delete, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorCNT {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorCNT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorCNT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorCNT for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_BackgroundSubtractorCNT(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorCNT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGMG = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorGMG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGMG,
		cv_PtrOfBackgroundSubtractorGMG_delete, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorGMG {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGMG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGSOC = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorGSOC>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGSOC,
		cv_PtrOfBackgroundSubtractorGSOC_delete, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorGSOC {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGSOC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGSOC for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_BackgroundSubtractorGSOC(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorLSBP = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorLSBP>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorLSBP,
		cv_PtrOfBackgroundSubtractorLSBP_delete, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorLSBP {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorLSBP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorLSBP for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_BackgroundSubtractorLSBP(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorMOG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorMOG,
		cv_PtrOfBackgroundSubtractorMOG_delete, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorMOG {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSyntheticSequenceGenerator = core::Ptr::<crate::bgsegm::SyntheticSequenceGenerator>;
	
	ptr_extern! { crate::bgsegm::SyntheticSequenceGenerator,
		cv_PtrOfSyntheticSequenceGenerator_delete, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::bgsegm::SyntheticSequenceGenerator, cv_PtrOfSyntheticSequenceGenerator_new }
	
	impl PtrOfSyntheticSequenceGenerator {
		#[inline] pub fn as_raw_PtrOfSyntheticSequenceGenerator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSyntheticSequenceGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::SyntheticSequenceGeneratorTrait for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_SyntheticSequenceGenerator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SyntheticSequenceGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use bgsegm_types::*;

#[cfg(feature = "contrib")]
mod bioinspired_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfRetina = core::Ptr::<dyn crate::bioinspired::Retina>;
	
	ptr_extern! { dyn crate::bioinspired::Retina,
		cv_PtrOfRetina_delete, cv_PtrOfRetina_get_inner_ptr, cv_PtrOfRetina_get_inner_ptr_mut
	}
	
	impl PtrOfRetina {
		#[inline] pub fn as_raw_PtrOfRetina(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetina(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetina {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::Retina for PtrOfRetina {
		#[inline] fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRetinaFastToneMapping = core::Ptr::<dyn crate::bioinspired::RetinaFastToneMapping>;
	
	ptr_extern! { dyn crate::bioinspired::RetinaFastToneMapping,
		cv_PtrOfRetinaFastToneMapping_delete, cv_PtrOfRetinaFastToneMapping_get_inner_ptr, cv_PtrOfRetinaFastToneMapping_get_inner_ptr_mut
	}
	
	impl PtrOfRetinaFastToneMapping {
		#[inline] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransientAreasSegmentationModule = core::Ptr::<dyn crate::bioinspired::TransientAreasSegmentationModule>;
	
	ptr_extern! { dyn crate::bioinspired::TransientAreasSegmentationModule,
		cv_PtrOfTransientAreasSegmentationModule_delete, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr_mut
	}
	
	impl PtrOfTransientAreasSegmentationModule {
		#[inline] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use bioinspired_types::*;

mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

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
	
	pub type PtrOfStereoMatcher = core::Ptr::<dyn crate::calib3d::StereoMatcher>;
	
	ptr_extern! { dyn crate::calib3d::StereoMatcher,
		cv_PtrOfStereoMatcher_delete, cv_PtrOfStereoMatcher_get_inner_ptr, cv_PtrOfStereoMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfStereoMatcher {
		#[inline] pub fn as_raw_PtrOfStereoMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoMatcher {
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
	
	pub type VectorOfPoint3f = core::Vector::<core::Point3f>;
	
	impl VectorOfPoint3f {
		pub fn as_raw_VectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3f, *const c_void, *mut c_void,
		cv_VectorOfPoint3f_new, cv_VectorOfPoint3f_delete,
		cv_VectorOfPoint3f_len, cv_VectorOfPoint3f_is_empty,
		cv_VectorOfPoint3f_capacity, cv_VectorOfPoint3f_shrink_to_fit,
		cv_VectorOfPoint3f_reserve, cv_VectorOfPoint3f_remove,
		cv_VectorOfPoint3f_swap, cv_VectorOfPoint3f_clear,
		cv_VectorOfPoint3f_get, cv_VectorOfPoint3f_set,
		cv_VectorOfPoint3f_push, cv_VectorOfPoint3f_insert,
	}
	vector_copy_non_bool! { core::Point3f, *const c_void, *mut c_void,
		cv_VectorOfPoint3f_data, cv_VectorOfPoint3f_data_mut,
		cv_VectorOfPoint3f_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Point3f> {}
	
	impl core::ToInputArray for VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3f {
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
	
	pub type VectorOfVec2i = core::Vector::<core::Vec2i>;
	
	impl VectorOfVec2i {
		pub fn as_raw_VectorOfVec2i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2i, *const c_void, *mut c_void,
		cv_VectorOfVec2i_new, cv_VectorOfVec2i_delete,
		cv_VectorOfVec2i_len, cv_VectorOfVec2i_is_empty,
		cv_VectorOfVec2i_capacity, cv_VectorOfVec2i_shrink_to_fit,
		cv_VectorOfVec2i_reserve, cv_VectorOfVec2i_remove,
		cv_VectorOfVec2i_swap, cv_VectorOfVec2i_clear,
		cv_VectorOfVec2i_get, cv_VectorOfVec2i_set,
		cv_VectorOfVec2i_push, cv_VectorOfVec2i_insert,
	}
	vector_copy_non_bool! { core::Vec2i, *const c_void, *mut c_void,
		cv_VectorOfVec2i_data, cv_VectorOfVec2i_data_mut,
		cv_VectorOfVec2i_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec2i> {}
	
	impl core::ToInputArray for VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_input_array(self.as_raw_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec2i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_output_array(self.as_raw_mut_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec3d = core::Vector::<core::Vec3d>;
	
	impl VectorOfVec3d {
		pub fn as_raw_VectorOfVec3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3d, *const c_void, *mut c_void,
		cv_VectorOfVec3d_new, cv_VectorOfVec3d_delete,
		cv_VectorOfVec3d_len, cv_VectorOfVec3d_is_empty,
		cv_VectorOfVec3d_capacity, cv_VectorOfVec3d_shrink_to_fit,
		cv_VectorOfVec3d_reserve, cv_VectorOfVec3d_remove,
		cv_VectorOfVec3d_swap, cv_VectorOfVec3d_clear,
		cv_VectorOfVec3d_get, cv_VectorOfVec3d_set,
		cv_VectorOfVec3d_push, cv_VectorOfVec3d_insert,
	}
	vector_copy_non_bool! { core::Vec3d, *const c_void, *mut c_void,
		cv_VectorOfVec3d_data, cv_VectorOfVec3d_data_mut,
		cv_VectorOfVec3d_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec3d> {}
	
	impl core::ToInputArray for VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec3f = core::Vector::<core::Vec3f>;
	
	impl VectorOfVec3f {
		pub fn as_raw_VectorOfVec3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3f, *const c_void, *mut c_void,
		cv_VectorOfVec3f_new, cv_VectorOfVec3f_delete,
		cv_VectorOfVec3f_len, cv_VectorOfVec3f_is_empty,
		cv_VectorOfVec3f_capacity, cv_VectorOfVec3f_shrink_to_fit,
		cv_VectorOfVec3f_reserve, cv_VectorOfVec3f_remove,
		cv_VectorOfVec3f_swap, cv_VectorOfVec3f_clear,
		cv_VectorOfVec3f_get, cv_VectorOfVec3f_set,
		cv_VectorOfVec3f_push, cv_VectorOfVec3f_insert,
	}
	vector_copy_non_bool! { core::Vec3f, *const c_void, *mut c_void,
		cv_VectorOfVec3f_data, cv_VectorOfVec3f_data_mut,
		cv_VectorOfVec3f_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec3f> {}
	
	impl core::ToInputArray for VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_mut_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec3i = core::Vector::<core::Vec3i>;
	
	impl VectorOfVec3i {
		pub fn as_raw_VectorOfVec3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3i, *const c_void, *mut c_void,
		cv_VectorOfVec3i_new, cv_VectorOfVec3i_delete,
		cv_VectorOfVec3i_len, cv_VectorOfVec3i_is_empty,
		cv_VectorOfVec3i_capacity, cv_VectorOfVec3i_shrink_to_fit,
		cv_VectorOfVec3i_reserve, cv_VectorOfVec3i_remove,
		cv_VectorOfVec3i_swap, cv_VectorOfVec3i_clear,
		cv_VectorOfVec3i_get, cv_VectorOfVec3i_set,
		cv_VectorOfVec3i_push, cv_VectorOfVec3i_insert,
	}
	vector_copy_non_bool! { core::Vec3i, *const c_void, *mut c_void,
		cv_VectorOfVec3i_data, cv_VectorOfVec3i_data_mut,
		cv_VectorOfVec3i_clone,
	}
	
	unsafe impl Send for core::Vector::<core::Vec3i> {}
	
	impl core::ToInputArray for VectorOfVec3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3i_input_array(self.as_raw_VectorOfVec3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3i_output_array(self.as_raw_mut_VectorOfVec3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3i_input_output_array(self.as_raw_mut_VectorOfVec3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
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
	
	pub type VectorOfVectorOfPoint3f = core::Vector::<core::Vector::<core::Point3f>>;
	
	impl VectorOfVectorOfPoint3f {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3f_new, cv_VectorOfVectorOfPoint3f_delete,
		cv_VectorOfVectorOfPoint3f_len, cv_VectorOfVectorOfPoint3f_is_empty,
		cv_VectorOfVectorOfPoint3f_capacity, cv_VectorOfVectorOfPoint3f_shrink_to_fit,
		cv_VectorOfVectorOfPoint3f_reserve, cv_VectorOfVectorOfPoint3f_remove,
		cv_VectorOfVectorOfPoint3f_swap, cv_VectorOfVectorOfPoint3f_clear,
		cv_VectorOfVectorOfPoint3f_get, cv_VectorOfVectorOfPoint3f_set,
		cv_VectorOfVectorOfPoint3f_push, cv_VectorOfVectorOfPoint3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Point3f> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3f {
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
	
	pub type VectorOfVectorOfRect = core::Vector::<core::Vector::<core::Rect>>;
	
	impl VectorOfVectorOfRect {
		pub fn as_raw_VectorOfVectorOfRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Rect>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfRect_new, cv_VectorOfVectorOfRect_delete,
		cv_VectorOfVectorOfRect_len, cv_VectorOfVectorOfRect_is_empty,
		cv_VectorOfVectorOfRect_capacity, cv_VectorOfVectorOfRect_shrink_to_fit,
		cv_VectorOfVectorOfRect_reserve, cv_VectorOfVectorOfRect_remove,
		cv_VectorOfVectorOfRect_swap, cv_VectorOfVectorOfRect_clear,
		cv_VectorOfVectorOfRect_get, cv_VectorOfVectorOfRect_set,
		cv_VectorOfVectorOfRect_push, cv_VectorOfVectorOfRect_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Rect> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Rect>> {}
	
	impl core::ToInputArray for VectorOfVectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_input_array(self.as_raw_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_output_array(self.as_raw_mut_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_input_output_array(self.as_raw_mut_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfVec2i = core::Vector::<core::Vector::<core::Vec2i>>;
	
	impl VectorOfVectorOfVec2i {
		pub fn as_raw_VectorOfVectorOfVec2i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Vec2i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfVec2i_new, cv_VectorOfVectorOfVec2i_delete,
		cv_VectorOfVectorOfVec2i_len, cv_VectorOfVectorOfVec2i_is_empty,
		cv_VectorOfVectorOfVec2i_capacity, cv_VectorOfVectorOfVec2i_shrink_to_fit,
		cv_VectorOfVectorOfVec2i_reserve, cv_VectorOfVectorOfVec2i_remove,
		cv_VectorOfVectorOfVec2i_swap, cv_VectorOfVectorOfVec2i_clear,
		cv_VectorOfVectorOfVec2i_get, cv_VectorOfVectorOfVec2i_set,
		cv_VectorOfVectorOfVec2i_push, cv_VectorOfVectorOfVec2i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<core::Vec2i> }
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Vec2i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_mut_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfbool = core::Vector::<core::Vector::<bool>>;
	
	impl VectorOfVectorOfbool {
		pub fn as_raw_VectorOfVectorOfbool(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfbool(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<bool>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfbool_new, cv_VectorOfVectorOfbool_delete,
		cv_VectorOfVectorOfbool_len, cv_VectorOfVectorOfbool_is_empty,
		cv_VectorOfVectorOfbool_capacity, cv_VectorOfVectorOfbool_shrink_to_fit,
		cv_VectorOfVectorOfbool_reserve, cv_VectorOfVectorOfbool_remove,
		cv_VectorOfVectorOfbool_swap, cv_VectorOfVectorOfbool_clear,
		cv_VectorOfVectorOfbool_get, cv_VectorOfVectorOfbool_set,
		cv_VectorOfVectorOfbool_push, cv_VectorOfVectorOfbool_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<bool> }
	
	unsafe impl Send for core::Vector::<core::Vector::<bool>> {}
	
	pub type VectorOfVectorOff64 = core::Vector::<core::Vector::<f64>>;
	
	impl VectorOfVectorOff64 {
		pub fn as_raw_VectorOfVectorOff64(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<f64>, *const c_void, *mut c_void,
		cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
		cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
		cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_shrink_to_fit,
		cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
		cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
		cv_VectorOfVectorOff64_get, cv_VectorOfVectorOff64_set,
		cv_VectorOfVectorOff64_push, cv_VectorOfVectorOff64_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<f64> }
	
	unsafe impl Send for core::Vector::<core::Vector::<f64>> {}
	
	impl core::ToInputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
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

#[cfg(feature = "contrib")]
mod cudaarithm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfConvolution = core::Ptr::<dyn crate::cudaarithm::Convolution>;
	
	ptr_extern! { dyn crate::cudaarithm::Convolution,
		cv_PtrOfConvolution_delete, cv_PtrOfConvolution_get_inner_ptr, cv_PtrOfConvolution_get_inner_ptr_mut
	}
	
	impl PtrOfConvolution {
		#[inline] pub fn as_raw_PtrOfConvolution(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfConvolution {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaarithm::Convolution for PtrOfConvolution {
		#[inline] fn as_raw_Convolution(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Convolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDFT = core::Ptr::<dyn crate::cudaarithm::DFT>;
	
	ptr_extern! { dyn crate::cudaarithm::DFT,
		cv_PtrOfDFT_delete, cv_PtrOfDFT_get_inner_ptr, cv_PtrOfDFT_get_inner_ptr_mut
	}
	
	impl PtrOfDFT {
		#[inline] pub fn as_raw_PtrOfDFT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDFT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaarithm::DFT for PtrOfDFT {
		#[inline] fn as_raw_DFT(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLookUpTable = core::Ptr::<dyn crate::cudaarithm::LookUpTable>;
	
	ptr_extern! { dyn crate::cudaarithm::LookUpTable,
		cv_PtrOfLookUpTable_delete, cv_PtrOfLookUpTable_get_inner_ptr, cv_PtrOfLookUpTable_get_inner_ptr_mut
	}
	
	impl PtrOfLookUpTable {
		#[inline] pub fn as_raw_PtrOfLookUpTable(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLookUpTable(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLookUpTable {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaarithm::LookUpTable for PtrOfLookUpTable {
		#[inline] fn as_raw_LookUpTable(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudaarithm_types::*;

#[cfg(feature = "contrib")]
mod cudabgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BackgroundSubtractorMOG = core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>;
	
	ptr_extern! { dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG,
		cv_PtrOfCUDA_BackgroundSubtractorMOG_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_BackgroundSubtractorMOG2 = core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2,
		cv_PtrOfCUDA_BackgroundSubtractorMOG2_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2 for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudabgsegm_types::*;

#[cfg(feature = "contrib")]
mod cudacodec_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfEncoderCallBack = core::Ptr::<dyn crate::cudacodec::EncoderCallBack>;
	
	ptr_extern! { dyn crate::cudacodec::EncoderCallBack,
		cv_PtrOfEncoderCallBack_delete, cv_PtrOfEncoderCallBack_get_inner_ptr, cv_PtrOfEncoderCallBack_get_inner_ptr_mut
	}
	
	impl PtrOfEncoderCallBack {
		#[inline] pub fn as_raw_PtrOfEncoderCallBack(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEncoderCallBack(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::EncoderCallBack for PtrOfEncoderCallBack {
		#[inline] fn as_raw_EncoderCallBack(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EncoderCallBack(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRawVideoSource = core::Ptr::<dyn crate::cudacodec::RawVideoSource>;
	
	ptr_extern! { dyn crate::cudacodec::RawVideoSource,
		cv_PtrOfRawVideoSource_delete, cv_PtrOfRawVideoSource_get_inner_ptr, cv_PtrOfRawVideoSource_get_inner_ptr_mut
	}
	
	impl PtrOfRawVideoSource {
		#[inline] pub fn as_raw_PtrOfRawVideoSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRawVideoSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::RawVideoSource for PtrOfRawVideoSource {
		#[inline] fn as_raw_RawVideoSource(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RawVideoSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoReader = core::Ptr::<dyn crate::cudacodec::VideoReader>;
	
	ptr_extern! { dyn crate::cudacodec::VideoReader,
		cv_PtrOfVideoReader_delete, cv_PtrOfVideoReader_get_inner_ptr, cv_PtrOfVideoReader_get_inner_ptr_mut
	}
	
	impl PtrOfVideoReader {
		#[inline] pub fn as_raw_PtrOfVideoReader(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoReader(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoReader for PtrOfVideoReader {
		#[inline] fn as_raw_VideoReader(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_VideoReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoWriter = core::Ptr::<dyn crate::cudacodec::VideoWriter>;
	
	ptr_extern! { dyn crate::cudacodec::VideoWriter,
		cv_PtrOfVideoWriter_delete, cv_PtrOfVideoWriter_get_inner_ptr, cv_PtrOfVideoWriter_get_inner_ptr_mut
	}
	
	impl PtrOfVideoWriter {
		#[inline] pub fn as_raw_PtrOfVideoWriter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoWriter for PtrOfVideoWriter {
		#[inline] fn as_raw_VideoWriter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudacodec_types::*;

#[cfg(feature = "contrib")]
mod cudafeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DescriptorMatcher = core::Ptr::<dyn crate::cudafeatures2d::CUDA_DescriptorMatcher>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_DescriptorMatcher,
		cv_PtrOfCUDA_DescriptorMatcher_delete, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfCUDA_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcher for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FastFeatureDetector = core::Ptr::<dyn crate::cudafeatures2d::CUDA_FastFeatureDetector>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_FastFeatureDetector,
		cv_PtrOfCUDA_FastFeatureDetector_delete, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_FastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetector for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetector for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsync for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_ORB = core::Ptr::<dyn crate::cudafeatures2d::CUDA_ORB>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_ORB,
		cv_PtrOfCUDA_ORB_delete, cv_PtrOfCUDA_ORB_get_inner_ptr, cv_PtrOfCUDA_ORB_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_ORB {
		#[inline] pub fn as_raw_PtrOfCUDA_ORB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_ORB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfCUDA_ORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::ORB for PtrOfCUDA_ORB {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsync for PtrOfCUDA_ORB {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORB for PtrOfCUDA_ORB {
		#[inline] fn as_raw_CUDA_ORB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudafeatures2d_types::*;

#[cfg(feature = "contrib")]
mod cudafilters_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFilter = core::Ptr::<dyn crate::cudafilters::Filter>;
	
	ptr_extern! { dyn crate::cudafilters::Filter,
		cv_PtrOfFilter_delete, cv_PtrOfFilter_get_inner_ptr, cv_PtrOfFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFilter {
		#[inline] pub fn as_raw_PtrOfFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafilters::Filter for PtrOfFilter {
		#[inline] fn as_raw_Filter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Filter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudafilters_types::*;

#[cfg(feature = "contrib")]
mod cudaimgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_CLAHE = core::Ptr::<dyn crate::cudaimgproc::CUDA_CLAHE>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CLAHE,
		cv_PtrOfCUDA_CLAHE_delete, cv_PtrOfCUDA_CLAHE_get_inner_ptr, cv_PtrOfCUDA_CLAHE_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CLAHE {
		#[inline] pub fn as_raw_PtrOfCUDA_CLAHE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHE for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CLAHE for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_CUDA_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CannyEdgeDetector = core::Ptr::<dyn crate::cudaimgproc::CUDA_CannyEdgeDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CannyEdgeDetector,
		cv_PtrOfCUDA_CannyEdgeDetector_delete, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CannyEdgeDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_CannyEdgeDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CannyEdgeDetector for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornernessCriteria = core::Ptr::<dyn crate::cudaimgproc::CUDA_CornernessCriteria>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CornernessCriteria,
		cv_PtrOfCUDA_CornernessCriteria_delete, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CornernessCriteria {
		#[inline] pub fn as_raw_PtrOfCUDA_CornernessCriteria(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornernessCriteria(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornernessCriteria for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornersDetector = core::Ptr::<dyn crate::cudaimgproc::CUDA_CornersDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CornersDetector,
		cv_PtrOfCUDA_CornersDetector_delete, cv_PtrOfCUDA_CornersDetector_get_inner_ptr, cv_PtrOfCUDA_CornersDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CornersDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_CornersDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornersDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornersDetector for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_CUDA_CornersDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughCirclesDetector = core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughCirclesDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughCirclesDetector,
		cv_PtrOfCUDA_HoughCirclesDetector_delete, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughCirclesDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughCirclesDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughCirclesDetector for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughLinesDetector = core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughLinesDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughLinesDetector,
		cv_PtrOfCUDA_HoughLinesDetector_delete, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughLinesDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughLinesDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughLinesDetector for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughSegmentDetector = core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughSegmentDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughSegmentDetector,
		cv_PtrOfCUDA_HoughSegmentDetector_delete, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughSegmentDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughSegmentDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughSegmentDetector for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_TemplateMatching = core::Ptr::<dyn crate::cudaimgproc::CUDA_TemplateMatching>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_TemplateMatching,
		cv_PtrOfCUDA_TemplateMatching_delete, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_TemplateMatching {
		#[inline] pub fn as_raw_PtrOfCUDA_TemplateMatching(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_TemplateMatching(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_TemplateMatching for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudaimgproc_types::*;

#[cfg(feature = "contrib")]
mod cudaobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCascadeClassifier = core::Ptr::<dyn crate::cudaobjdetect::CascadeClassifier>;
	
	ptr_extern! { dyn crate::cudaobjdetect::CascadeClassifier,
		cv_PtrOfCascadeClassifier_delete, cv_PtrOfCascadeClassifier_get_inner_ptr, cv_PtrOfCascadeClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfCascadeClassifier {
		#[inline] pub fn as_raw_PtrOfCascadeClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CascadeClassifier for PtrOfCascadeClassifier {
		#[inline] fn as_raw_CascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHOG = core::Ptr::<dyn crate::cudaobjdetect::HOG>;
	
	ptr_extern! { dyn crate::cudaobjdetect::HOG,
		cv_PtrOfHOG_delete, cv_PtrOfHOG_get_inner_ptr, cv_PtrOfHOG_get_inner_ptr_mut
	}
	
	impl PtrOfHOG {
		#[inline] pub fn as_raw_PtrOfHOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::HOG for PtrOfHOG {
		#[inline] fn as_raw_HOG(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudaobjdetect_types::*;

#[cfg(feature = "contrib")]
mod cudaoptflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BroxOpticalFlow = core::Ptr::<dyn crate::cudaoptflow::CUDA_BroxOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_BroxOpticalFlow,
		cv_PtrOfCUDA_BroxOpticalFlow_delete, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BroxOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlow for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_DensePyrLKOpticalFlow = core::Ptr::<dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow,
		cv_PtrOfCUDA_DensePyrLKOpticalFlow_delete, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FarnebackOpticalFlow = core::Ptr::<dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow,
		cv_PtrOfCUDA_FarnebackOpticalFlow_delete, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlow for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_OpticalFlowDual_TVL1 = core::Ptr::<dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1,
		cv_PtrOfCUDA_OpticalFlowDual_TVL1_delete, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] pub fn as_raw_PtrOfCUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1 for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_SparsePyrLKOpticalFlow = core::Ptr::<dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow,
		cv_PtrOfCUDA_SparsePyrLKOpticalFlow_delete, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlow for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudaoptflow_types::*;

#[cfg(feature = "contrib")]
mod cudastereo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DisparityBilateralFilter = core::Ptr::<dyn crate::cudastereo::CUDA_DisparityBilateralFilter>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_DisparityBilateralFilter,
		cv_PtrOfCUDA_DisparityBilateralFilter_delete, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DisparityBilateralFilter {
		#[inline] pub fn as_raw_PtrOfCUDA_DisparityBilateralFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_DisparityBilateralFilter for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_CUDA_DisparityBilateralFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBM = core::Ptr::<dyn crate::cudastereo::CUDA_StereoBM>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoBM,
		cv_PtrOfCUDA_StereoBM_delete, cv_PtrOfCUDA_StereoBM_get_inner_ptr, cv_PtrOfCUDA_StereoBM_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoBM {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBM for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBM for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_CUDA_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBeliefPropagation = core::Ptr::<dyn crate::cudastereo::CUDA_StereoBeliefPropagation>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoBeliefPropagation,
		cv_PtrOfCUDA_StereoBeliefPropagation_delete, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoBeliefPropagation {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBeliefPropagation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagation for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoConstantSpaceBP = core::Ptr::<dyn crate::cudastereo::CUDA_StereoConstantSpaceBP>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoConstantSpaceBP,
		cv_PtrOfCUDA_StereoConstantSpaceBP_delete, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagation for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoConstantSpaceBP for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_CUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use cudastereo_types::*;

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
	
	pub type PtrOfAccumLayer = core::Ptr::<crate::dnn::AccumLayer>;
	
	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrOfAccumLayer_delete, cv_PtrOfAccumLayer_get_inner_ptr, cv_PtrOfAccumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrOfAccumLayer_new }
	
	impl PtrOfAccumLayer {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAccumLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::AccumLayerTrait for PtrOfAccumLayer {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAccumLayer {
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
	
	pub type PtrOfCorrelationLayer = core::Ptr::<crate::dnn::CorrelationLayer>;
	
	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrOfCorrelationLayer_delete, cv_PtrOfCorrelationLayer_get_inner_ptr, cv_PtrOfCorrelationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrOfCorrelationLayer_new }
	
	impl PtrOfCorrelationLayer {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::CorrelationLayerTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDataAugmentationLayer = core::Ptr::<crate::dnn::DataAugmentationLayer>;
	
	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrOfDataAugmentationLayer_delete, cv_PtrOfDataAugmentationLayer_get_inner_ptr, cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrOfDataAugmentationLayer_new }
	
	impl PtrOfDataAugmentationLayer {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::DataAugmentationLayerTrait for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfDataAugmentationLayer {
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
	
	pub type PtrOfFlowWarpLayer = core::Ptr::<crate::dnn::FlowWarpLayer>;
	
	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrOfFlowWarpLayer_delete, cv_PtrOfFlowWarpLayer_get_inner_ptr, cv_PtrOfFlowWarpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrOfFlowWarpLayer_new }
	
	impl PtrOfFlowWarpLayer {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::FlowWarpLayerTrait for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfFlowWarpLayer {
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
	
	pub type PtrOfMishLayer = core::Ptr::<dyn crate::dnn::MishLayer>;
	
	ptr_extern! { dyn crate::dnn::MishLayer,
		cv_PtrOfMishLayer_delete, cv_PtrOfMishLayer_get_inner_ptr, cv_PtrOfMishLayer_get_inner_ptr_mut
	}
	
	impl PtrOfMishLayer {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfMishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::MishLayer for PtrOfMishLayer {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
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
	
	pub type PtrOfSwishLayer = core::Ptr::<dyn crate::dnn::SwishLayer>;
	
	ptr_extern! { dyn crate::dnn::SwishLayer,
		cv_PtrOfSwishLayer_delete, cv_PtrOfSwishLayer_get_inner_ptr, cv_PtrOfSwishLayer_get_inner_ptr_mut
	}
	
	impl PtrOfSwishLayer {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayer for PtrOfSwishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SwishLayer for PtrOfSwishLayer {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
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
	
	pub type VectorOfPtrOfBackendNode = core::Vector::<core::Ptr::<crate::dnn::BackendNode>>;
	
	impl VectorOfPtrOfBackendNode {
		pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr::<crate::dnn::BackendNode>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfBackendNode_new, cv_VectorOfPtrOfBackendNode_delete,
		cv_VectorOfPtrOfBackendNode_len, cv_VectorOfPtrOfBackendNode_is_empty,
		cv_VectorOfPtrOfBackendNode_capacity, cv_VectorOfPtrOfBackendNode_shrink_to_fit,
		cv_VectorOfPtrOfBackendNode_reserve, cv_VectorOfPtrOfBackendNode_remove,
		cv_VectorOfPtrOfBackendNode_swap, cv_VectorOfPtrOfBackendNode_clear,
		cv_VectorOfPtrOfBackendNode_get, cv_VectorOfPtrOfBackendNode_set,
		cv_VectorOfPtrOfBackendNode_push, cv_VectorOfPtrOfBackendNode_insert,
	}
	vector_non_copy_or_bool! { core::Ptr::<crate::dnn::BackendNode> }
	
	unsafe impl Send for core::Vector::<core::Ptr::<crate::dnn::BackendNode>> {}
	
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

#[cfg(feature = "contrib")]
mod dpm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDPMDetector = core::Ptr::<dyn crate::dpm::DPMDetector>;
	
	ptr_extern! { dyn crate::dpm::DPMDetector,
		cv_PtrOfDPMDetector_delete, cv_PtrOfDPMDetector_get_inner_ptr, cv_PtrOfDPMDetector_get_inner_ptr_mut
	}
	
	impl PtrOfDPMDetector {
		#[inline] pub fn as_raw_PtrOfDPMDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dpm::DPMDetector for PtrOfDPMDetector {
		#[inline] fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDPMDetector_ObjectDetection = core::Vector::<crate::dpm::DPMDetector_ObjectDetection>;
	
	impl VectorOfDPMDetector_ObjectDetection {
		pub fn as_raw_VectorOfDPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dpm::DPMDetector_ObjectDetection, *const c_void, *mut c_void,
		cv_VectorOfDPMDetector_ObjectDetection_new, cv_VectorOfDPMDetector_ObjectDetection_delete,
		cv_VectorOfDPMDetector_ObjectDetection_len, cv_VectorOfDPMDetector_ObjectDetection_is_empty,
		cv_VectorOfDPMDetector_ObjectDetection_capacity, cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit,
		cv_VectorOfDPMDetector_ObjectDetection_reserve, cv_VectorOfDPMDetector_ObjectDetection_remove,
		cv_VectorOfDPMDetector_ObjectDetection_swap, cv_VectorOfDPMDetector_ObjectDetection_clear,
		cv_VectorOfDPMDetector_ObjectDetection_get, cv_VectorOfDPMDetector_ObjectDetection_set,
		cv_VectorOfDPMDetector_ObjectDetection_push, cv_VectorOfDPMDetector_ObjectDetection_insert,
	}
	vector_non_copy_or_bool! { crate::dpm::DPMDetector_ObjectDetection }
	
	unsafe impl Send for core::Vector::<crate::dpm::DPMDetector_ObjectDetection> {}
	
}
#[cfg(feature = "contrib")]
pub use dpm_types::*;

#[cfg(feature = "contrib")]
mod face_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBIF = core::Ptr::<dyn crate::face::BIF>;
	
	ptr_extern! { dyn crate::face::BIF,
		cv_PtrOfBIF_delete, cv_PtrOfBIF_get_inner_ptr, cv_PtrOfBIF_get_inner_ptr_mut
	}
	
	impl PtrOfBIF {
		#[inline] pub fn as_raw_PtrOfBIF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBIF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBIF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BIF for PtrOfBIF {
		#[inline] fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEigenFaceRecognizer = core::Ptr::<dyn crate::face::EigenFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::EigenFaceRecognizer,
		cv_PtrOfEigenFaceRecognizer_delete, cv_PtrOfEigenFaceRecognizer_get_inner_ptr, cv_PtrOfEigenFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfEigenFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfEigenFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEigenFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::EigenFaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemark = core::Ptr::<dyn crate::face::Facemark>;
	
	ptr_extern! { dyn crate::face::Facemark,
		cv_PtrOfFacemark_delete, cv_PtrOfFacemark_get_inner_ptr, cv_PtrOfFacemark_get_inner_ptr_mut
	}
	
	impl PtrOfFacemark {
		#[inline] pub fn as_raw_PtrOfFacemark(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemark {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::Facemark for PtrOfFacemark {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkAAM = core::Ptr::<dyn crate::face::FacemarkAAM>;
	
	ptr_extern! { dyn crate::face::FacemarkAAM,
		cv_PtrOfFacemarkAAM_delete, cv_PtrOfFacemarkAAM_get_inner_ptr, cv_PtrOfFacemarkAAM_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkAAM {
		#[inline] pub fn as_raw_PtrOfFacemarkAAM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkAAM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkAAM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkAAM {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkAAM for PtrOfFacemarkAAM {
		#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrain for PtrOfFacemarkAAM {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkKazemi = core::Ptr::<dyn crate::face::FacemarkKazemi>;
	
	ptr_extern! { dyn crate::face::FacemarkKazemi,
		cv_PtrOfFacemarkKazemi_delete, cv_PtrOfFacemarkKazemi_get_inner_ptr, cv_PtrOfFacemarkKazemi_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkKazemi {
		#[inline] pub fn as_raw_PtrOfFacemarkKazemi(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkKazemi(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkKazemi for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkLBF = core::Ptr::<dyn crate::face::FacemarkLBF>;
	
	ptr_extern! { dyn crate::face::FacemarkLBF,
		cv_PtrOfFacemarkLBF_delete, cv_PtrOfFacemarkLBF_get_inner_ptr, cv_PtrOfFacemarkLBF_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkLBF {
		#[inline] pub fn as_raw_PtrOfFacemarkLBF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkLBF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkLBF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkLBF {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkLBF for PtrOfFacemarkLBF {
		#[inline] fn as_raw_FacemarkLBF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrain for PtrOfFacemarkLBF {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFisherFaceRecognizer = core::Ptr::<dyn crate::face::FisherFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::FisherFaceRecognizer,
		cv_PtrOfFisherFaceRecognizer_delete, cv_PtrOfFisherFaceRecognizer_get_inner_ptr, cv_PtrOfFisherFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfFisherFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfFisherFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisherFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FisherFaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLBPHFaceRecognizer = core::Ptr::<dyn crate::face::LBPHFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::LBPHFaceRecognizer,
		cv_PtrOfLBPHFaceRecognizer_delete, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfLBPHFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMACE = core::Ptr::<dyn crate::face::MACE>;
	
	ptr_extern! { dyn crate::face::MACE,
		cv_PtrOfMACE_delete, cv_PtrOfMACE_get_inner_ptr, cv_PtrOfMACE_get_inner_ptr_mut
	}
	
	impl PtrOfMACE {
		#[inline] pub fn as_raw_PtrOfMACE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMACE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMACE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::MACE for PtrOfMACE {
		#[inline] fn as_raw_MACE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPredictCollector = core::Ptr::<dyn crate::face::PredictCollector>;
	
	ptr_extern! { dyn crate::face::PredictCollector,
		cv_PtrOfPredictCollector_delete, cv_PtrOfPredictCollector_get_inner_ptr, cv_PtrOfPredictCollector_get_inner_ptr_mut
	}
	
	impl PtrOfPredictCollector {
		#[inline] pub fn as_raw_PtrOfPredictCollector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollector for PtrOfPredictCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStandardCollector = core::Ptr::<crate::face::StandardCollector>;
	
	ptr_extern! { crate::face::StandardCollector,
		cv_PtrOfStandardCollector_delete, cv_PtrOfStandardCollector_get_inner_ptr, cv_PtrOfStandardCollector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::face::StandardCollector, cv_PtrOfStandardCollector_new }
	
	impl PtrOfStandardCollector {
		#[inline] pub fn as_raw_PtrOfStandardCollector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollector for PtrOfStandardCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::StandardCollectorTrait for PtrOfStandardCollector {
		#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfFacemarkAAM_Config = core::Vector::<crate::face::FacemarkAAM_Config>;
	
	impl VectorOfFacemarkAAM_Config {
		pub fn as_raw_VectorOfFacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Config, *const c_void, *mut c_void,
		cv_VectorOfFacemarkAAM_Config_new, cv_VectorOfFacemarkAAM_Config_delete,
		cv_VectorOfFacemarkAAM_Config_len, cv_VectorOfFacemarkAAM_Config_is_empty,
		cv_VectorOfFacemarkAAM_Config_capacity, cv_VectorOfFacemarkAAM_Config_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Config_reserve, cv_VectorOfFacemarkAAM_Config_remove,
		cv_VectorOfFacemarkAAM_Config_swap, cv_VectorOfFacemarkAAM_Config_clear,
		cv_VectorOfFacemarkAAM_Config_get, cv_VectorOfFacemarkAAM_Config_set,
		cv_VectorOfFacemarkAAM_Config_push, cv_VectorOfFacemarkAAM_Config_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Config }
	
	unsafe impl Send for core::Vector::<crate::face::FacemarkAAM_Config> {}
	
	pub type VectorOfFacemarkAAM_Model_Texture = core::Vector::<crate::face::FacemarkAAM_Model_Texture>;
	
	impl VectorOfFacemarkAAM_Model_Texture {
		pub fn as_raw_VectorOfFacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Model_Texture, *const c_void, *mut c_void,
		cv_VectorOfFacemarkAAM_Model_Texture_new, cv_VectorOfFacemarkAAM_Model_Texture_delete,
		cv_VectorOfFacemarkAAM_Model_Texture_len, cv_VectorOfFacemarkAAM_Model_Texture_is_empty,
		cv_VectorOfFacemarkAAM_Model_Texture_capacity, cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Model_Texture_reserve, cv_VectorOfFacemarkAAM_Model_Texture_remove,
		cv_VectorOfFacemarkAAM_Model_Texture_swap, cv_VectorOfFacemarkAAM_Model_Texture_clear,
		cv_VectorOfFacemarkAAM_Model_Texture_get, cv_VectorOfFacemarkAAM_Model_Texture_set,
		cv_VectorOfFacemarkAAM_Model_Texture_push, cv_VectorOfFacemarkAAM_Model_Texture_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Model_Texture }
	
	unsafe impl Send for core::Vector::<crate::face::FacemarkAAM_Model_Texture> {}
	
}
#[cfg(feature = "contrib")]
pub use face_types::*;

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
	
	pub type PtrOfAffineFeature = core::Ptr::<dyn crate::features2d::AffineFeature>;
	
	ptr_extern! { dyn crate::features2d::AffineFeature,
		cv_PtrOfAffineFeature_delete, cv_PtrOfAffineFeature_get_inner_ptr, cv_PtrOfAffineFeature_get_inner_ptr_mut
	}
	
	impl PtrOfAffineFeature {
		#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AffineFeature for PtrOfAffineFeature {
		#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineFeature {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAffineFeature {
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
	
	pub type PtrOfSIFT = core::Ptr::<crate::features2d::SIFT>;
	
	ptr_extern! { crate::features2d::SIFT,
		cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr, cv_PtrOfSIFT_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SIFT, cv_PtrOfSIFT_new }
	
	impl PtrOfSIFT {
		#[inline] pub fn as_raw_PtrOfSIFT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSIFT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSIFT {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::SIFTTrait for PtrOfSIFT {
		#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
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
	
}
pub use flann_types::*;

#[cfg(feature = "contrib")]
mod freetype_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFreeType2 = core::Ptr::<dyn crate::freetype::FreeType2>;
	
	ptr_extern! { dyn crate::freetype::FreeType2,
		cv_PtrOfFreeType2_delete, cv_PtrOfFreeType2_get_inner_ptr, cv_PtrOfFreeType2_get_inner_ptr_mut
	}
	
	impl PtrOfFreeType2 {
		#[inline] pub fn as_raw_PtrOfFreeType2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFreeType2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::freetype::FreeType2 for PtrOfFreeType2 {
		#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use freetype_types::*;

#[cfg(feature = "contrib")]
mod hdf_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHDF5 = core::Ptr::<dyn crate::hdf::HDF5>;
	
	ptr_extern! { dyn crate::hdf::HDF5,
		cv_PtrOfHDF5_delete, cv_PtrOfHDF5_get_inner_ptr, cv_PtrOfHDF5_get_inner_ptr_mut
	}
	
	impl PtrOfHDF5 {
		#[inline] pub fn as_raw_PtrOfHDF5(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHDF5(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::hdf::HDF5 for PtrOfHDF5 {
		#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use hdf_types::*;

#[cfg(feature = "contrib")]
mod hfs_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHfsSegment = core::Ptr::<dyn crate::hfs::HfsSegment>;
	
	ptr_extern! { dyn crate::hfs::HfsSegment,
		cv_PtrOfHfsSegment_delete, cv_PtrOfHfsSegment_get_inner_ptr, cv_PtrOfHfsSegment_get_inner_ptr_mut
	}
	
	impl PtrOfHfsSegment {
		#[inline] pub fn as_raw_PtrOfHfsSegment(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHfsSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHfsSegment {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::hfs::HfsSegment for PtrOfHfsSegment {
		#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use hfs_types::*;

#[cfg(feature = "contrib")]
mod img_hash_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAverageHash = core::Ptr::<crate::img_hash::AverageHash>;
	
	ptr_extern! { crate::img_hash::AverageHash,
		cv_PtrOfAverageHash_delete, cv_PtrOfAverageHash_get_inner_ptr, cv_PtrOfAverageHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::AverageHash, cv_PtrOfAverageHash_new }
	
	impl PtrOfAverageHash {
		#[inline] pub fn as_raw_PtrOfAverageHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAverageHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::AverageHashTrait for PtrOfAverageHash {
		#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfAverageHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBlockMeanHash = core::Ptr::<crate::img_hash::BlockMeanHash>;
	
	ptr_extern! { crate::img_hash::BlockMeanHash,
		cv_PtrOfBlockMeanHash_delete, cv_PtrOfBlockMeanHash_get_inner_ptr, cv_PtrOfBlockMeanHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::BlockMeanHash, cv_PtrOfBlockMeanHash_new }
	
	impl PtrOfBlockMeanHash {
		#[inline] pub fn as_raw_PtrOfBlockMeanHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::BlockMeanHashTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfColorMomentHash = core::Ptr::<crate::img_hash::ColorMomentHash>;
	
	ptr_extern! { crate::img_hash::ColorMomentHash,
		cv_PtrOfColorMomentHash_delete, cv_PtrOfColorMomentHash_get_inner_ptr, cv_PtrOfColorMomentHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::ColorMomentHash, cv_PtrOfColorMomentHash_new }
	
	impl PtrOfColorMomentHash {
		#[inline] pub fn as_raw_PtrOfColorMomentHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ColorMomentHashTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMarrHildrethHash = core::Ptr::<crate::img_hash::MarrHildrethHash>;
	
	ptr_extern! { crate::img_hash::MarrHildrethHash,
		cv_PtrOfMarrHildrethHash_delete, cv_PtrOfMarrHildrethHash_get_inner_ptr, cv_PtrOfMarrHildrethHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::MarrHildrethHash, cv_PtrOfMarrHildrethHash_new }
	
	impl PtrOfMarrHildrethHash {
		#[inline] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::MarrHildrethHashTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPHash = core::Ptr::<crate::img_hash::PHash>;
	
	ptr_extern! { crate::img_hash::PHash,
		cv_PtrOfPHash_delete, cv_PtrOfPHash_get_inner_ptr, cv_PtrOfPHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::PHash, cv_PtrOfPHash_new }
	
	impl PtrOfPHash {
		#[inline] pub fn as_raw_PtrOfPHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfPHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::PHashTrait for PtrOfPHash {
		#[inline] fn as_raw_PHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRadialVarianceHash = core::Ptr::<crate::img_hash::RadialVarianceHash>;
	
	ptr_extern! { crate::img_hash::RadialVarianceHash,
		cv_PtrOfRadialVarianceHash_delete, cv_PtrOfRadialVarianceHash_get_inner_ptr, cv_PtrOfRadialVarianceHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::RadialVarianceHash, cv_PtrOfRadialVarianceHash_new }
	
	impl PtrOfRadialVarianceHash {
		#[inline] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::RadialVarianceHashTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use img_hash_types::*;

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

#[cfg(feature = "contrib")]
mod line_descriptor_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBinaryDescriptor = core::Ptr::<crate::line_descriptor::BinaryDescriptor>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptor,
		cv_PtrOfBinaryDescriptor_delete, cv_PtrOfBinaryDescriptor_get_inner_ptr, cv_PtrOfBinaryDescriptor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptor, cv_PtrOfBinaryDescriptor_new }
	
	impl PtrOfBinaryDescriptor {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTrait for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBinaryDescriptorMatcher = core::Ptr::<crate::line_descriptor::BinaryDescriptorMatcher>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
		cv_PtrOfBinaryDescriptorMatcher_delete, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptorMatcher, cv_PtrOfBinaryDescriptorMatcher_new }
	
	impl PtrOfBinaryDescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTrait for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSDDetector = core::Ptr::<crate::line_descriptor::LSDDetector>;
	
	ptr_extern! { crate::line_descriptor::LSDDetector,
		cv_PtrOfLSDDetector_delete, cv_PtrOfLSDDetector_get_inner_ptr, cv_PtrOfLSDDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::LSDDetector, cv_PtrOfLSDDetector_new }
	
	impl PtrOfLSDDetector {
		#[inline] pub fn as_raw_PtrOfLSDDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLSDDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::LSDDetectorTrait for PtrOfLSDDetector {
		#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfKeyLine = core::Vector::<crate::line_descriptor::KeyLine>;
	
	impl VectorOfKeyLine {
		pub fn as_raw_VectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
		cv_VectorOfKeyLine_new, cv_VectorOfKeyLine_delete,
		cv_VectorOfKeyLine_len, cv_VectorOfKeyLine_is_empty,
		cv_VectorOfKeyLine_capacity, cv_VectorOfKeyLine_shrink_to_fit,
		cv_VectorOfKeyLine_reserve, cv_VectorOfKeyLine_remove,
		cv_VectorOfKeyLine_swap, cv_VectorOfKeyLine_clear,
		cv_VectorOfKeyLine_get, cv_VectorOfKeyLine_set,
		cv_VectorOfKeyLine_push, cv_VectorOfKeyLine_insert,
	}
	vector_copy_non_bool! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
		cv_VectorOfKeyLine_data, cv_VectorOfKeyLine_data_mut,
		cv_VectorOfKeyLine_clone,
	}
	
	unsafe impl Send for core::Vector::<crate::line_descriptor::KeyLine> {}
	
	pub type VectorOfVectorOfKeyLine = core::Vector::<core::Vector::<crate::line_descriptor::KeyLine>>;
	
	impl VectorOfVectorOfKeyLine {
		pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<crate::line_descriptor::KeyLine>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyLine_new, cv_VectorOfVectorOfKeyLine_delete,
		cv_VectorOfVectorOfKeyLine_len, cv_VectorOfVectorOfKeyLine_is_empty,
		cv_VectorOfVectorOfKeyLine_capacity, cv_VectorOfVectorOfKeyLine_shrink_to_fit,
		cv_VectorOfVectorOfKeyLine_reserve, cv_VectorOfVectorOfKeyLine_remove,
		cv_VectorOfVectorOfKeyLine_swap, cv_VectorOfVectorOfKeyLine_clear,
		cv_VectorOfVectorOfKeyLine_get, cv_VectorOfVectorOfKeyLine_set,
		cv_VectorOfVectorOfKeyLine_push, cv_VectorOfVectorOfKeyLine_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector::<crate::line_descriptor::KeyLine> }
	
	unsafe impl Send for core::Vector::<core::Vector::<crate::line_descriptor::KeyLine>> {}
	
}
#[cfg(feature = "contrib")]
pub use line_descriptor_types::*;

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

#[cfg(feature = "contrib")]
mod optflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDISOpticalFlow = core::Ptr::<dyn crate::optflow::DISOpticalFlow>;
	
	ptr_extern! { dyn crate::optflow::DISOpticalFlow,
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
	
	impl crate::video::DenseOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::optflow::DISOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTrainingSamples = core::Ptr::<crate::optflow::GPCTrainingSamples>;
	
	ptr_extern! { crate::optflow::GPCTrainingSamples,
		cv_PtrOfGPCTrainingSamples_delete, cv_PtrOfGPCTrainingSamples_get_inner_ptr, cv_PtrOfGPCTrainingSamples_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTrainingSamples, cv_PtrOfGPCTrainingSamples_new }
	
	impl PtrOfGPCTrainingSamples {
		#[inline] pub fn as_raw_PtrOfGPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::GPCTrainingSamplesTrait for PtrOfGPCTrainingSamples {
		#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTree = core::Ptr::<crate::optflow::GPCTree>;
	
	ptr_extern! { crate::optflow::GPCTree,
		cv_PtrOfGPCTree_delete, cv_PtrOfGPCTree_get_inner_ptr, cv_PtrOfGPCTree_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTree, cv_PtrOfGPCTree_new }
	
	impl PtrOfGPCTree {
		#[inline] pub fn as_raw_PtrOfGPCTree(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGPCTree {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::optflow::GPCTreeTrait for PtrOfGPCTree {
		#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCAPrior = core::Ptr::<crate::optflow::PCAPrior>;
	
	ptr_extern! { crate::optflow::PCAPrior,
		cv_PtrOfPCAPrior_delete, cv_PtrOfPCAPrior_get_inner_ptr, cv_PtrOfPCAPrior_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::PCAPrior, cv_PtrOfPCAPrior_new }
	
	impl PtrOfPCAPrior {
		#[inline] pub fn as_raw_PtrOfPCAPrior(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::PCAPriorTrait for PtrOfPCAPrior {
		#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVariationalRefinement = core::Ptr::<dyn crate::optflow::VariationalRefinement>;
	
	ptr_extern! { dyn crate::optflow::VariationalRefinement,
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
	
	impl crate::optflow::VariationalRefinement for PtrOfVariationalRefinement {
		#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfGPCPatchDescriptor = core::Vector::<crate::optflow::GPCPatchDescriptor>;
	
	impl VectorOfGPCPatchDescriptor {
		pub fn as_raw_VectorOfGPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfGPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::optflow::GPCPatchDescriptor, *const c_void, *mut c_void,
		cv_VectorOfGPCPatchDescriptor_new, cv_VectorOfGPCPatchDescriptor_delete,
		cv_VectorOfGPCPatchDescriptor_len, cv_VectorOfGPCPatchDescriptor_is_empty,
		cv_VectorOfGPCPatchDescriptor_capacity, cv_VectorOfGPCPatchDescriptor_shrink_to_fit,
		cv_VectorOfGPCPatchDescriptor_reserve, cv_VectorOfGPCPatchDescriptor_remove,
		cv_VectorOfGPCPatchDescriptor_swap, cv_VectorOfGPCPatchDescriptor_clear,
		cv_VectorOfGPCPatchDescriptor_get, cv_VectorOfGPCPatchDescriptor_set,
		cv_VectorOfGPCPatchDescriptor_push, cv_VectorOfGPCPatchDescriptor_insert,
	}
	vector_non_copy_or_bool! { crate::optflow::GPCPatchDescriptor }
	
	unsafe impl Send for core::Vector::<crate::optflow::GPCPatchDescriptor> {}
	
}
#[cfg(feature = "contrib")]
pub use optflow_types::*;

#[cfg(feature = "contrib")]
mod ovis_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWindowScene = core::Ptr::<dyn crate::ovis::WindowScene>;
	
	ptr_extern! { dyn crate::ovis::WindowScene,
		cv_PtrOfWindowScene_delete, cv_PtrOfWindowScene_get_inner_ptr, cv_PtrOfWindowScene_get_inner_ptr_mut
	}
	
	impl PtrOfWindowScene {
		#[inline] pub fn as_raw_PtrOfWindowScene(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWindowScene(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ovis::WindowScene for PtrOfWindowScene {
		#[inline] fn as_raw_WindowScene(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WindowScene(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use ovis_types::*;

#[cfg(feature = "contrib")]
mod phase_unwrapping_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHistogramPhaseUnwrapping = core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>;
	
	ptr_extern! { dyn crate::phase_unwrapping::HistogramPhaseUnwrapping,
		cv_PtrOfHistogramPhaseUnwrapping_delete, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr_mut
	}
	
	impl PtrOfHistogramPhaseUnwrapping {
		#[inline] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use phase_unwrapping_types::*;

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

#[cfg(feature = "contrib")]
mod plot_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPlot2d = core::Ptr::<dyn crate::plot::Plot2d>;
	
	ptr_extern! { dyn crate::plot::Plot2d,
		cv_PtrOfPlot2d_delete, cv_PtrOfPlot2d_get_inner_ptr, cv_PtrOfPlot2d_get_inner_ptr_mut
	}
	
	impl PtrOfPlot2d {
		#[inline] pub fn as_raw_PtrOfPlot2d(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPlot2d {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::plot::Plot2d for PtrOfPlot2d {
		#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use plot_types::*;

#[cfg(feature = "contrib")]
mod rgbd_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDepthCleaner = core::Ptr::<crate::rgbd::DepthCleaner>;
	
	ptr_extern! { crate::rgbd::DepthCleaner,
		cv_PtrOfDepthCleaner_delete, cv_PtrOfDepthCleaner_get_inner_ptr, cv_PtrOfDepthCleaner_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::DepthCleaner, cv_PtrOfDepthCleaner_new }
	
	impl PtrOfDepthCleaner {
		#[inline] pub fn as_raw_PtrOfDepthCleaner(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDepthCleaner {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::DepthCleanerTrait for PtrOfDepthCleaner {
		#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfICPOdometry = core::Ptr::<crate::rgbd::ICPOdometry>;
	
	ptr_extern! { crate::rgbd::ICPOdometry,
		cv_PtrOfICPOdometry_delete, cv_PtrOfICPOdometry_get_inner_ptr, cv_PtrOfICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::ICPOdometry, cv_PtrOfICPOdometry_new }
	
	impl PtrOfICPOdometry {
		#[inline] pub fn as_raw_PtrOfICPOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfICPOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::ICPOdometryTrait for PtrOfICPOdometry {
		#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Odometry for PtrOfICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_ColorGradient = core::Ptr::<crate::rgbd::Linemod_ColorGradient>;
	
	ptr_extern! { crate::rgbd::Linemod_ColorGradient,
		cv_PtrOfLinemod_ColorGradient_delete, cv_PtrOfLinemod_ColorGradient_get_inner_ptr, cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_ColorGradient, cv_PtrOfLinemod_ColorGradient_new }
	
	impl PtrOfLinemod_ColorGradient {
		#[inline] pub fn as_raw_PtrOfLinemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ColorGradientTrait for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_DepthNormal = core::Ptr::<crate::rgbd::Linemod_DepthNormal>;
	
	ptr_extern! { crate::rgbd::Linemod_DepthNormal,
		cv_PtrOfLinemod_DepthNormal_delete, cv_PtrOfLinemod_DepthNormal_get_inner_ptr, cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_DepthNormal, cv_PtrOfLinemod_DepthNormal_new }
	
	impl PtrOfLinemod_DepthNormal {
		#[inline] pub fn as_raw_PtrOfLinemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DepthNormalTrait for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Detector = core::Ptr::<crate::rgbd::Linemod_Detector>;
	
	ptr_extern! { crate::rgbd::Linemod_Detector,
		cv_PtrOfLinemod_Detector_delete, cv_PtrOfLinemod_Detector_get_inner_ptr, cv_PtrOfLinemod_Detector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_Detector, cv_PtrOfLinemod_Detector_new }
	
	impl PtrOfLinemod_Detector {
		#[inline] pub fn as_raw_PtrOfLinemod_Detector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DetectorTrait for PtrOfLinemod_Detector {
		#[inline] fn as_raw_Linemod_Detector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Modality = core::Ptr::<dyn crate::rgbd::Linemod_Modality>;
	
	ptr_extern! { dyn crate::rgbd::Linemod_Modality,
		cv_PtrOfLinemod_Modality_delete, cv_PtrOfLinemod_Modality_get_inner_ptr, cv_PtrOfLinemod_Modality_get_inner_ptr_mut
	}
	
	impl PtrOfLinemod_Modality {
		#[inline] pub fn as_raw_PtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_Modality {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_QuantizedPyramid = core::Ptr::<dyn crate::rgbd::Linemod_QuantizedPyramid>;
	
	ptr_extern! { dyn crate::rgbd::Linemod_QuantizedPyramid,
		cv_PtrOfLinemod_QuantizedPyramid_delete, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut
	}
	
	impl PtrOfLinemod_QuantizedPyramid {
		#[inline] pub fn as_raw_PtrOfLinemod_QuantizedPyramid(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_QuantizedPyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_QuantizedPyramid for PtrOfLinemod_QuantizedPyramid {
		#[inline] fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometry = core::Ptr::<dyn crate::rgbd::Odometry>;
	
	ptr_extern! { dyn crate::rgbd::Odometry,
		cv_PtrOfOdometry_delete, cv_PtrOfOdometry_get_inner_ptr, cv_PtrOfOdometry_get_inner_ptr_mut
	}
	
	impl PtrOfOdometry {
		#[inline] pub fn as_raw_PtrOfOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Odometry for PtrOfOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometryFrame = core::Ptr::<crate::rgbd::OdometryFrame>;
	
	ptr_extern! { crate::rgbd::OdometryFrame,
		cv_PtrOfOdometryFrame_delete, cv_PtrOfOdometryFrame_get_inner_ptr, cv_PtrOfOdometryFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::OdometryFrame, cv_PtrOfOdometryFrame_new }
	
	impl PtrOfOdometryFrame {
		#[inline] pub fn as_raw_PtrOfOdometryFrame(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryFrameTrait for PtrOfOdometryFrame {
		#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for PtrOfOdometryFrame {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdFrame = core::Ptr::<crate::rgbd::RgbdFrame>;
	
	ptr_extern! { crate::rgbd::RgbdFrame,
		cv_PtrOfRgbdFrame_delete, cv_PtrOfRgbdFrame_get_inner_ptr, cv_PtrOfRgbdFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdFrame, cv_PtrOfRgbdFrame_new }
	
	impl PtrOfRgbdFrame {
		#[inline] pub fn as_raw_PtrOfRgbdFrame(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for PtrOfRgbdFrame {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdICPOdometry = core::Ptr::<crate::rgbd::RgbdICPOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdICPOdometry,
		cv_PtrOfRgbdICPOdometry_delete, cv_PtrOfRgbdICPOdometry_get_inner_ptr, cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdICPOdometry, cv_PtrOfRgbdICPOdometry_new }
	
	impl PtrOfRgbdICPOdometry {
		#[inline] pub fn as_raw_PtrOfRgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Odometry for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTrait for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdNormals = core::Ptr::<crate::rgbd::RgbdNormals>;
	
	ptr_extern! { crate::rgbd::RgbdNormals,
		cv_PtrOfRgbdNormals_delete, cv_PtrOfRgbdNormals_get_inner_ptr, cv_PtrOfRgbdNormals_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdNormals, cv_PtrOfRgbdNormals_new }
	
	impl PtrOfRgbdNormals {
		#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdNormals {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdNormalsTrait for PtrOfRgbdNormals {
		#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdOdometry = core::Ptr::<crate::rgbd::RgbdOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdOdometry,
		cv_PtrOfRgbdOdometry_delete, cv_PtrOfRgbdOdometry_get_inner_ptr, cv_PtrOfRgbdOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdOdometry, cv_PtrOfRgbdOdometry_new }
	
	impl PtrOfRgbdOdometry {
		#[inline] pub fn as_raw_PtrOfRgbdOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Odometry for PtrOfRgbdOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdOdometryTrait for PtrOfRgbdOdometry {
		#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdPlane = core::Ptr::<crate::rgbd::RgbdPlane>;
	
	ptr_extern! { crate::rgbd::RgbdPlane,
		cv_PtrOfRgbdPlane_delete, cv_PtrOfRgbdPlane_get_inner_ptr, cv_PtrOfRgbdPlane_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdPlane, cv_PtrOfRgbdPlane_new }
	
	impl PtrOfRgbdPlane {
		#[inline] pub fn as_raw_PtrOfRgbdPlane(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdPlane {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdPlaneTrait for PtrOfRgbdPlane {
		#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfLinemod_Feature = core::Vector::<crate::rgbd::Linemod_Feature>;
	
	impl VectorOfLinemod_Feature {
		pub fn as_raw_VectorOfLinemod_Feature(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Feature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Feature_new, cv_VectorOfLinemod_Feature_delete,
		cv_VectorOfLinemod_Feature_len, cv_VectorOfLinemod_Feature_is_empty,
		cv_VectorOfLinemod_Feature_capacity, cv_VectorOfLinemod_Feature_shrink_to_fit,
		cv_VectorOfLinemod_Feature_reserve, cv_VectorOfLinemod_Feature_remove,
		cv_VectorOfLinemod_Feature_swap, cv_VectorOfLinemod_Feature_clear,
		cv_VectorOfLinemod_Feature_get, cv_VectorOfLinemod_Feature_set,
		cv_VectorOfLinemod_Feature_push, cv_VectorOfLinemod_Feature_insert,
	}
	vector_copy_non_bool! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Feature_data, cv_VectorOfLinemod_Feature_data_mut,
		cv_VectorOfLinemod_Feature_clone,
	}
	
	unsafe impl Send for core::Vector::<crate::rgbd::Linemod_Feature> {}
	
	pub type VectorOfLinemod_Match = core::Vector::<crate::rgbd::Linemod_Match>;
	
	impl VectorOfLinemod_Match {
		pub fn as_raw_VectorOfLinemod_Match(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Match, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Match_new, cv_VectorOfLinemod_Match_delete,
		cv_VectorOfLinemod_Match_len, cv_VectorOfLinemod_Match_is_empty,
		cv_VectorOfLinemod_Match_capacity, cv_VectorOfLinemod_Match_shrink_to_fit,
		cv_VectorOfLinemod_Match_reserve, cv_VectorOfLinemod_Match_remove,
		cv_VectorOfLinemod_Match_swap, cv_VectorOfLinemod_Match_clear,
		cv_VectorOfLinemod_Match_get, cv_VectorOfLinemod_Match_set,
		cv_VectorOfLinemod_Match_push, cv_VectorOfLinemod_Match_insert,
	}
	vector_non_copy_or_bool! { crate::rgbd::Linemod_Match }
	
	unsafe impl Send for core::Vector::<crate::rgbd::Linemod_Match> {}
	
	pub type VectorOfLinemod_Template = core::Vector::<crate::rgbd::Linemod_Template>;
	
	impl VectorOfLinemod_Template {
		pub fn as_raw_VectorOfLinemod_Template(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Template, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Template_new, cv_VectorOfLinemod_Template_delete,
		cv_VectorOfLinemod_Template_len, cv_VectorOfLinemod_Template_is_empty,
		cv_VectorOfLinemod_Template_capacity, cv_VectorOfLinemod_Template_shrink_to_fit,
		cv_VectorOfLinemod_Template_reserve, cv_VectorOfLinemod_Template_remove,
		cv_VectorOfLinemod_Template_swap, cv_VectorOfLinemod_Template_clear,
		cv_VectorOfLinemod_Template_get, cv_VectorOfLinemod_Template_set,
		cv_VectorOfLinemod_Template_push, cv_VectorOfLinemod_Template_insert,
	}
	vector_non_copy_or_bool! { crate::rgbd::Linemod_Template }
	
	unsafe impl Send for core::Vector::<crate::rgbd::Linemod_Template> {}
	
	pub type VectorOfPtrOfLinemod_Modality = core::Vector::<core::Ptr::<dyn crate::rgbd::Linemod_Modality>>;
	
	impl VectorOfPtrOfLinemod_Modality {
		pub fn as_raw_VectorOfPtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr::<dyn crate::rgbd::Linemod_Modality>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfLinemod_Modality_new, cv_VectorOfPtrOfLinemod_Modality_delete,
		cv_VectorOfPtrOfLinemod_Modality_len, cv_VectorOfPtrOfLinemod_Modality_is_empty,
		cv_VectorOfPtrOfLinemod_Modality_capacity, cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit,
		cv_VectorOfPtrOfLinemod_Modality_reserve, cv_VectorOfPtrOfLinemod_Modality_remove,
		cv_VectorOfPtrOfLinemod_Modality_swap, cv_VectorOfPtrOfLinemod_Modality_clear,
		cv_VectorOfPtrOfLinemod_Modality_get, cv_VectorOfPtrOfLinemod_Modality_set,
		cv_VectorOfPtrOfLinemod_Modality_push, cv_VectorOfPtrOfLinemod_Modality_insert,
	}
	vector_non_copy_or_bool! { core::Ptr::<dyn crate::rgbd::Linemod_Modality> }
	
	unsafe impl Send for core::Vector::<core::Ptr::<dyn crate::rgbd::Linemod_Modality>> {}
	
}
#[cfg(feature = "contrib")]
pub use rgbd_types::*;

#[cfg(feature = "contrib")]
mod saliency_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfMotionSaliencyBinWangApr2014 = core::Ptr::<crate::saliency::MotionSaliencyBinWangApr2014>;
	
	ptr_extern! { crate::saliency::MotionSaliencyBinWangApr2014,
		cv_PtrOfMotionSaliencyBinWangApr2014_delete, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::MotionSaliencyBinWangApr2014, cv_PtrOfMotionSaliencyBinWangApr2014_new }
	
	impl PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] pub fn as_raw_PtrOfMotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliency for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliencyBinWangApr2014Trait for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::Saliency for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfObjectnessBING = core::Ptr::<crate::saliency::ObjectnessBING>;
	
	ptr_extern! { crate::saliency::ObjectnessBING,
		cv_PtrOfObjectnessBING_delete, cv_PtrOfObjectnessBING_get_inner_ptr, cv_PtrOfObjectnessBING_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::ObjectnessBING, cv_PtrOfObjectnessBING_new }
	
	impl PtrOfObjectnessBING {
		#[inline] pub fn as_raw_PtrOfObjectnessBING(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfObjectnessBING {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::Objectness for PtrOfObjectnessBING {
		#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::ObjectnessBINGTrait for PtrOfObjectnessBING {
		#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::Saliency for PtrOfObjectnessBING {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencyFineGrained = core::Ptr::<crate::saliency::StaticSaliencyFineGrained>;
	
	ptr_extern! { crate::saliency::StaticSaliencyFineGrained,
		cv_PtrOfStaticSaliencyFineGrained_delete, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencyFineGrained, cv_PtrOfStaticSaliencyFineGrained_new }
	
	impl PtrOfStaticSaliencyFineGrained {
		#[inline] pub fn as_raw_PtrOfStaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::Saliency for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliency for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyFineGrainedTrait for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencySpectralResidual = core::Ptr::<crate::saliency::StaticSaliencySpectralResidual>;
	
	ptr_extern! { crate::saliency::StaticSaliencySpectralResidual,
		cv_PtrOfStaticSaliencySpectralResidual_delete, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencySpectralResidual, cv_PtrOfStaticSaliencySpectralResidual_new }
	
	impl PtrOfStaticSaliencySpectralResidual {
		#[inline] pub fn as_raw_PtrOfStaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::Saliency for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliency for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencySpectralResidualTrait for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use saliency_types::*;

#[cfg(feature = "contrib")]
mod sfm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSFMLibmvEuclideanReconstruction = core::Ptr::<dyn crate::sfm::SFMLibmvEuclideanReconstruction>;
	
	ptr_extern! { dyn crate::sfm::SFMLibmvEuclideanReconstruction,
		cv_PtrOfSFMLibmvEuclideanReconstruction_delete, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr_mut
	}
	
	impl PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::sfm::BaseSFM for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstruction for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use sfm_types::*;

mod shape_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineTransformer = core::Ptr::<dyn crate::shape::AffineTransformer>;
	
	ptr_extern! { dyn crate::shape::AffineTransformer,
		cv_PtrOfAffineTransformer_delete, cv_PtrOfAffineTransformer_get_inner_ptr, cv_PtrOfAffineTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfAffineTransformer {
		#[inline] pub fn as_raw_PtrOfAffineTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
		#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHausdorffDistanceExtractor = core::Ptr::<dyn crate::shape::HausdorffDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::HausdorffDistanceExtractor,
		cv_PtrOfHausdorffDistanceExtractor_delete, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfHausdorffDistanceExtractor {
		#[inline] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHistogramCostExtractor = core::Ptr::<dyn crate::shape::HistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::HistogramCostExtractor,
		cv_PtrOfHistogramCostExtractor_delete, cv_PtrOfHistogramCostExtractor_get_inner_ptr, cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfHistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeContextDistanceExtractor = core::Ptr::<dyn crate::shape::ShapeContextDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::ShapeContextDistanceExtractor,
		cv_PtrOfShapeContextDistanceExtractor_delete, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfShapeContextDistanceExtractor {
		#[inline] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeTransformer = core::Ptr::<dyn crate::shape::ShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ShapeTransformer,
		cv_PtrOfShapeTransformer_delete, cv_PtrOfShapeTransformer_get_inner_ptr, cv_PtrOfShapeTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfShapeTransformer {
		#[inline] pub fn as_raw_PtrOfShapeTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThinPlateSplineShapeTransformer = core::Ptr::<dyn crate::shape::ThinPlateSplineShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ThinPlateSplineShapeTransformer,
		cv_PtrOfThinPlateSplineShapeTransformer_delete, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfThinPlateSplineShapeTransformer {
		#[inline] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use shape_types::*;

mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

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
	
	pub type PtrOfDetail_FeaturesFinder = core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesFinder,
		cv_PtrOfDetail_FeaturesFinder_delete, cv_PtrOfDetail_FeaturesFinder_get_inner_ptr, cv_PtrOfDetail_FeaturesFinder_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_FeaturesFinder {
		#[inline] pub fn as_raw_PtrOfDetail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesFinder for PtrOfDetail_FeaturesFinder {
		#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
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

#[cfg(feature = "contrib")]
mod structured_light_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayCodePattern = core::Ptr::<dyn crate::structured_light::GrayCodePattern>;
	
	ptr_extern! { dyn crate::structured_light::GrayCodePattern,
		cv_PtrOfGrayCodePattern_delete, cv_PtrOfGrayCodePattern_get_inner_ptr, cv_PtrOfGrayCodePattern_get_inner_ptr_mut
	}
	
	impl PtrOfGrayCodePattern {
		#[inline] pub fn as_raw_PtrOfGrayCodePattern(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayCodePattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
		#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern = core::Ptr::<dyn crate::structured_light::SinusoidalPattern>;
	
	ptr_extern! { dyn crate::structured_light::SinusoidalPattern,
		cv_PtrOfSinusoidalPattern_delete, cv_PtrOfSinusoidalPattern_get_inner_ptr, cv_PtrOfSinusoidalPattern_get_inner_ptr_mut
	}
	
	impl PtrOfSinusoidalPattern {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern_Params = core::Ptr::<crate::structured_light::SinusoidalPattern_Params>;
	
	ptr_extern! { crate::structured_light::SinusoidalPattern_Params,
		cv_PtrOfSinusoidalPattern_Params_delete, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::structured_light::SinusoidalPattern_Params, cv_PtrOfSinusoidalPattern_Params_new }
	
	impl PtrOfSinusoidalPattern_Params {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern_Params(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTrait for PtrOfSinusoidalPattern_Params {
		#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use structured_light_types::*;

mod superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSuperres_BroxOpticalFlow = core::Ptr::<dyn crate::superres::Superres_BroxOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_BroxOpticalFlow,
		cv_PtrOfSuperres_BroxOpticalFlow_delete, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_BroxOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlow for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DenseOpticalFlowExt = core::Ptr::<dyn crate::superres::Superres_DenseOpticalFlowExt>;
	
	ptr_extern! { dyn crate::superres::Superres_DenseOpticalFlowExt,
		cv_PtrOfSuperres_DenseOpticalFlowExt_delete, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DualTVL1OpticalFlow = core::Ptr::<dyn crate::superres::Superres_DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_DualTVL1OpticalFlow,
		cv_PtrOfSuperres_DualTVL1OpticalFlow_delete, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlow for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FarnebackOpticalFlow = core::Ptr::<dyn crate::superres::Superres_FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_FarnebackOpticalFlow,
		cv_PtrOfSuperres_FarnebackOpticalFlow_delete, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlow for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FrameSource = core::Ptr::<dyn crate::superres::Superres_FrameSource>;
	
	ptr_extern! { dyn crate::superres::Superres_FrameSource,
		cv_PtrOfSuperres_FrameSource_delete, cv_PtrOfSuperres_FrameSource_get_inner_ptr, cv_PtrOfSuperres_FrameSource_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_FrameSource {
		#[inline] pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_FrameSource {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_PyrLKOpticalFlow = core::Ptr::<dyn crate::superres::Superres_PyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_PyrLKOpticalFlow,
		cv_PtrOfSuperres_PyrLKOpticalFlow_delete, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlow for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_SuperResolution = core::Ptr::<dyn crate::superres::Superres_SuperResolution>;
	
	ptr_extern! { dyn crate::superres::Superres_SuperResolution,
		cv_PtrOfSuperres_SuperResolution_delete, cv_PtrOfSuperres_SuperResolution_get_inner_ptr, cv_PtrOfSuperres_SuperResolution_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_SuperResolution {
		#[inline] pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_SuperResolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_SuperResolution for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Superres_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use superres_types::*;

#[cfg(feature = "contrib")]
mod surface_matching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPose3D = core::Ptr::<crate::surface_matching::Pose3D>;
	
	ptr_extern! { crate::surface_matching::Pose3D,
		cv_PtrOfPose3D_delete, cv_PtrOfPose3D_get_inner_ptr, cv_PtrOfPose3D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::surface_matching::Pose3D, cv_PtrOfPose3D_new }
	
	impl PtrOfPose3D {
		#[inline] pub fn as_raw_PtrOfPose3D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPose3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::Pose3DTrait for PtrOfPose3D {
		#[inline] fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoseCluster3D = core::Ptr::<crate::surface_matching::PoseCluster3D>;
	
	ptr_extern! { crate::surface_matching::PoseCluster3D,
		cv_PtrOfPoseCluster3D_delete, cv_PtrOfPoseCluster3D_get_inner_ptr, cv_PtrOfPoseCluster3D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::surface_matching::PoseCluster3D, cv_PtrOfPoseCluster3D_new }
	
	impl PtrOfPoseCluster3D {
		#[inline] pub fn as_raw_PtrOfPoseCluster3D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::PoseCluster3DTrait for PtrOfPoseCluster3D {
		#[inline] fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPose3DPtr = core::Vector::<crate::surface_matching::Pose3DPtr>;
	
	impl VectorOfPose3DPtr {
		pub fn as_raw_VectorOfPose3DPtr(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPose3DPtr(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::surface_matching::Pose3DPtr, *const c_void, *mut c_void,
		cv_VectorOfPose3DPtr_new, cv_VectorOfPose3DPtr_delete,
		cv_VectorOfPose3DPtr_len, cv_VectorOfPose3DPtr_is_empty,
		cv_VectorOfPose3DPtr_capacity, cv_VectorOfPose3DPtr_shrink_to_fit,
		cv_VectorOfPose3DPtr_reserve, cv_VectorOfPose3DPtr_remove,
		cv_VectorOfPose3DPtr_swap, cv_VectorOfPose3DPtr_clear,
		cv_VectorOfPose3DPtr_get, cv_VectorOfPose3DPtr_set,
		cv_VectorOfPose3DPtr_push, cv_VectorOfPose3DPtr_insert,
	}
	vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }
	
	unsafe impl Send for core::Vector::<crate::surface_matching::Pose3DPtr> {}
	
}
#[cfg(feature = "contrib")]
pub use surface_matching_types::*;

#[cfg(feature = "contrib")]
mod text_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfERFilter = core::Ptr::<dyn crate::text::ERFilter>;
	
	ptr_extern! { dyn crate::text::ERFilter,
		cv_PtrOfERFilter_delete, cv_PtrOfERFilter_get_inner_ptr, cv_PtrOfERFilter_get_inner_ptr_mut
	}
	
	impl PtrOfERFilter {
		#[inline] pub fn as_raw_PtrOfERFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfERFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::ERFilter for PtrOfERFilter {
		#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfERFilter_Callback = core::Ptr::<dyn crate::text::ERFilter_Callback>;
	
	ptr_extern! { dyn crate::text::ERFilter_Callback,
		cv_PtrOfERFilter_Callback_delete, cv_PtrOfERFilter_Callback_get_inner_ptr, cv_PtrOfERFilter_Callback_get_inner_ptr_mut
	}
	
	impl PtrOfERFilter_Callback {
		#[inline] pub fn as_raw_PtrOfERFilter_Callback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilter_Callback for PtrOfERFilter_Callback {
		#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder = core::Ptr::<crate::text::OCRBeamSearchDecoder>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder,
		cv_PtrOfOCRBeamSearchDecoder_delete, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder, cv_PtrOfOCRBeamSearchDecoder_new }
	
	impl PtrOfOCRBeamSearchDecoder {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder_ClassifierCallback = core::Ptr::<crate::text::OCRBeamSearchDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
		cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder_ClassifierCallback, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_new }
	
	impl PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder = core::Ptr::<crate::text::OCRHMMDecoder>;
	
	ptr_extern! { crate::text::OCRHMMDecoder,
		cv_PtrOfOCRHMMDecoder_delete, cv_PtrOfOCRHMMDecoder_get_inner_ptr, cv_PtrOfOCRHMMDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder, cv_PtrOfOCRHMMDecoder_new }
	
	impl PtrOfOCRHMMDecoder {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoderTrait for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder_ClassifierCallback = core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
		cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder_ClassifierCallback, cv_PtrOfOCRHMMDecoder_ClassifierCallback_new }
	
	impl PtrOfOCRHMMDecoder_ClassifierCallback {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for PtrOfOCRHMMDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHolisticWordRecognizer = core::Ptr::<dyn crate::text::OCRHolisticWordRecognizer>;
	
	ptr_extern! { dyn crate::text::OCRHolisticWordRecognizer,
		cv_PtrOfOCRHolisticWordRecognizer_delete, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfOCRHolisticWordRecognizer {
		#[inline] pub fn as_raw_PtrOfOCRHolisticWordRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRHolisticWordRecognizer for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRTesseract = core::Ptr::<dyn crate::text::OCRTesseract>;
	
	ptr_extern! { dyn crate::text::OCRTesseract,
		cv_PtrOfOCRTesseract_delete, cv_PtrOfOCRTesseract_get_inner_ptr, cv_PtrOfOCRTesseract_get_inner_ptr_mut
	}
	
	impl PtrOfOCRTesseract {
		#[inline] pub fn as_raw_PtrOfOCRTesseract(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRTesseract {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRTesseract for PtrOfOCRTesseract {
		#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTextDetectorCNN = core::Ptr::<dyn crate::text::TextDetectorCNN>;
	
	ptr_extern! { dyn crate::text::TextDetectorCNN,
		cv_PtrOfTextDetectorCNN_delete, cv_PtrOfTextDetectorCNN_get_inner_ptr, cv_PtrOfTextDetectorCNN_get_inner_ptr_mut
	}
	
	impl PtrOfTextDetectorCNN {
		#[inline] pub fn as_raw_PtrOfTextDetectorCNN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTextDetectorCNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::TextDetector for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::TextDetectorCNN for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfERStat = core::Vector::<crate::text::ERStat>;
	
	impl VectorOfERStat {
		pub fn as_raw_VectorOfERStat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::text::ERStat, *const c_void, *mut c_void,
		cv_VectorOfERStat_new, cv_VectorOfERStat_delete,
		cv_VectorOfERStat_len, cv_VectorOfERStat_is_empty,
		cv_VectorOfERStat_capacity, cv_VectorOfERStat_shrink_to_fit,
		cv_VectorOfERStat_reserve, cv_VectorOfERStat_remove,
		cv_VectorOfERStat_swap, cv_VectorOfERStat_clear,
		cv_VectorOfERStat_get, cv_VectorOfERStat_set,
		cv_VectorOfERStat_push, cv_VectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { crate::text::ERStat }
	
	unsafe impl Send for core::Vector::<crate::text::ERStat> {}
	
	pub type VectorOfVectorOfERStat = core::Vector::<core::Vector::<crate::text::ERStat>>;
	
	impl VectorOfVectorOfERStat {
		pub fn as_raw_VectorOfVectorOfERStat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<crate::text::ERStat>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfERStat_new, cv_VectorOfVectorOfERStat_delete,
		cv_VectorOfVectorOfERStat_len, cv_VectorOfVectorOfERStat_is_empty,
		cv_VectorOfVectorOfERStat_capacity, cv_VectorOfVectorOfERStat_shrink_to_fit,
		cv_VectorOfVectorOfERStat_reserve, cv_VectorOfVectorOfERStat_remove,
		cv_VectorOfVectorOfERStat_swap, cv_VectorOfVectorOfERStat_clear,
		cv_VectorOfVectorOfERStat_get, cv_VectorOfVectorOfERStat_set,
		cv_VectorOfVectorOfERStat_push, cv_VectorOfVectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { core::Vector::<crate::text::ERStat> }
	
	unsafe impl Send for core::Vector::<core::Vector::<crate::text::ERStat>> {}
	
}
#[cfg(feature = "contrib")]
pub use text_types::*;

#[cfg(feature = "contrib")]
mod tracking_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfMultiTracker = core::Ptr::<crate::tracking::MultiTracker>;
	
	ptr_extern! { crate::tracking::MultiTracker,
		cv_PtrOfMultiTracker_delete, cv_PtrOfMultiTracker_get_inner_ptr, cv_PtrOfMultiTracker_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::tracking::MultiTracker, cv_PtrOfMultiTracker_new }
	
	impl PtrOfMultiTracker {
		#[inline] pub fn as_raw_PtrOfMultiTracker(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMultiTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMultiTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::MultiTrackerTrait for PtrOfMultiTracker {
		#[inline] fn as_raw_MultiTracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTracker = core::Ptr::<dyn crate::tracking::Tracker>;
	
	ptr_extern! { dyn crate::tracking::Tracker,
		cv_PtrOfTracker_delete, cv_PtrOfTracker_get_inner_ptr, cv_PtrOfTracker_get_inner_ptr_mut
	}
	
	impl PtrOfTracker {
		#[inline] pub fn as_raw_PtrOfTracker(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTracker {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerBoosting = core::Ptr::<dyn crate::tracking::TrackerBoosting>;
	
	ptr_extern! { dyn crate::tracking::TrackerBoosting,
		cv_PtrOfTrackerBoosting_delete, cv_PtrOfTrackerBoosting_get_inner_ptr, cv_PtrOfTrackerBoosting_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerBoosting {
		#[inline] pub fn as_raw_PtrOfTrackerBoosting(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerBoosting {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerBoosting {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerBoosting for PtrOfTrackerBoosting {
		#[inline] fn as_raw_TrackerBoosting(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerBoosting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerCSRT = core::Ptr::<dyn crate::tracking::TrackerCSRT>;
	
	ptr_extern! { dyn crate::tracking::TrackerCSRT,
		cv_PtrOfTrackerCSRT_delete, cv_PtrOfTrackerCSRT_get_inner_ptr, cv_PtrOfTrackerCSRT_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerCSRT {
		#[inline] pub fn as_raw_PtrOfTrackerCSRT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerCSRT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerCSRT {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerCSRT for PtrOfTrackerCSRT {
		#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerFeature = core::Ptr::<dyn crate::tracking::TrackerFeature>;
	
	ptr_extern! { dyn crate::tracking::TrackerFeature,
		cv_PtrOfTrackerFeature_delete, cv_PtrOfTrackerFeature_get_inner_ptr, cv_PtrOfTrackerFeature_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerFeature {
		#[inline] pub fn as_raw_PtrOfTrackerFeature(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerFeature for PtrOfTrackerFeature {
		#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerGOTURN = core::Ptr::<dyn crate::tracking::TrackerGOTURN>;
	
	ptr_extern! { dyn crate::tracking::TrackerGOTURN,
		cv_PtrOfTrackerGOTURN_delete, cv_PtrOfTrackerGOTURN_get_inner_ptr, cv_PtrOfTrackerGOTURN_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerGOTURN {
		#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerGOTURN for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerKCF = core::Ptr::<dyn crate::tracking::TrackerKCF>;
	
	ptr_extern! { dyn crate::tracking::TrackerKCF,
		cv_PtrOfTrackerKCF_delete, cv_PtrOfTrackerKCF_get_inner_ptr, cv_PtrOfTrackerKCF_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerKCF {
		#[inline] pub fn as_raw_PtrOfTrackerKCF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerKCF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerKCF {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerKCF for PtrOfTrackerKCF {
		#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMIL = core::Ptr::<dyn crate::tracking::TrackerMIL>;
	
	ptr_extern! { dyn crate::tracking::TrackerMIL,
		cv_PtrOfTrackerMIL_delete, cv_PtrOfTrackerMIL_get_inner_ptr, cv_PtrOfTrackerMIL_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerMIL {
		#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerMIL {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerMIL {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerMIL for PtrOfTrackerMIL {
		#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMOSSE = core::Ptr::<dyn crate::tracking::TrackerMOSSE>;
	
	ptr_extern! { dyn crate::tracking::TrackerMOSSE,
		cv_PtrOfTrackerMOSSE_delete, cv_PtrOfTrackerMOSSE_get_inner_ptr, cv_PtrOfTrackerMOSSE_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerMOSSE {
		#[inline] pub fn as_raw_PtrOfTrackerMOSSE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMOSSE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerMOSSE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerMOSSE {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerMOSSE for PtrOfTrackerMOSSE {
		#[inline] fn as_raw_TrackerMOSSE(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerMOSSE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMedianFlow = core::Ptr::<dyn crate::tracking::TrackerMedianFlow>;
	
	ptr_extern! { dyn crate::tracking::TrackerMedianFlow,
		cv_PtrOfTrackerMedianFlow_delete, cv_PtrOfTrackerMedianFlow_get_inner_ptr, cv_PtrOfTrackerMedianFlow_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerMedianFlow {
		#[inline] pub fn as_raw_PtrOfTrackerMedianFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMedianFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerMedianFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerMedianFlow {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerMedianFlow for PtrOfTrackerMedianFlow {
		#[inline] fn as_raw_TrackerMedianFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerMedianFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerSamplerAlgorithm = core::Ptr::<dyn crate::tracking::TrackerSamplerAlgorithm>;
	
	ptr_extern! { dyn crate::tracking::TrackerSamplerAlgorithm,
		cv_PtrOfTrackerSamplerAlgorithm_delete, cv_PtrOfTrackerSamplerAlgorithm_get_inner_ptr, cv_PtrOfTrackerSamplerAlgorithm_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerSamplerAlgorithm {
		#[inline] pub fn as_raw_PtrOfTrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerSamplerAlgorithm for PtrOfTrackerSamplerAlgorithm {
		#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerStateEstimator = core::Ptr::<dyn crate::tracking::TrackerStateEstimator>;
	
	ptr_extern! { dyn crate::tracking::TrackerStateEstimator,
		cv_PtrOfTrackerStateEstimator_delete, cv_PtrOfTrackerStateEstimator_get_inner_ptr, cv_PtrOfTrackerStateEstimator_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerStateEstimator {
		#[inline] pub fn as_raw_PtrOfTrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerStateEstimator for PtrOfTrackerStateEstimator {
		#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerTLD = core::Ptr::<dyn crate::tracking::TrackerTLD>;
	
	ptr_extern! { dyn crate::tracking::TrackerTLD,
		cv_PtrOfTrackerTLD_delete, cv_PtrOfTrackerTLD_get_inner_ptr, cv_PtrOfTrackerTLD_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerTLD {
		#[inline] pub fn as_raw_PtrOfTrackerTLD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTrackerTLD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::Tracker for PtrOfTrackerTLD {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::tracking::TrackerTLD for PtrOfTrackerTLD {
		#[inline] fn as_raw_TrackerTLD(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerTLD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerTargetState = core::Ptr::<crate::tracking::TrackerTargetState>;
	
	ptr_extern! { crate::tracking::TrackerTargetState,
		cv_PtrOfTrackerTargetState_delete, cv_PtrOfTrackerTargetState_get_inner_ptr, cv_PtrOfTrackerTargetState_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::tracking::TrackerTargetState, cv_PtrOfTrackerTargetState_new }
	
	impl PtrOfTrackerTargetState {
		#[inline] pub fn as_raw_PtrOfTrackerTargetState(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerTargetStateTrait for PtrOfTrackerTargetState {
		#[inline] fn as_raw_TrackerTargetState(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPtrOfTracker = core::Vector::<core::Ptr::<dyn crate::tracking::Tracker>>;
	
	impl VectorOfPtrOfTracker {
		pub fn as_raw_VectorOfPtrOfTracker(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr::<dyn crate::tracking::Tracker>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfTracker_new, cv_VectorOfPtrOfTracker_delete,
		cv_VectorOfPtrOfTracker_len, cv_VectorOfPtrOfTracker_is_empty,
		cv_VectorOfPtrOfTracker_capacity, cv_VectorOfPtrOfTracker_shrink_to_fit,
		cv_VectorOfPtrOfTracker_reserve, cv_VectorOfPtrOfTracker_remove,
		cv_VectorOfPtrOfTracker_swap, cv_VectorOfPtrOfTracker_clear,
		cv_VectorOfPtrOfTracker_get, cv_VectorOfPtrOfTracker_set,
		cv_VectorOfPtrOfTracker_push, cv_VectorOfPtrOfTracker_insert,
	}
	vector_non_copy_or_bool! { core::Ptr::<dyn crate::tracking::Tracker> }
	
	unsafe impl Send for core::Vector::<core::Ptr::<dyn crate::tracking::Tracker>> {}
	
}
#[cfg(feature = "contrib")]
pub use tracking_types::*;

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
	
	pub type PtrOfDenseOpticalFlow = core::Ptr::<dyn crate::video::DenseOpticalFlow>;
	
	ptr_extern! { dyn crate::video::DenseOpticalFlow,
		cv_PtrOfDenseOpticalFlow_delete, cv_PtrOfDenseOpticalFlow_get_inner_ptr, cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDenseOpticalFlow {
		#[inline] pub fn as_raw_PtrOfDenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDualTVL1OpticalFlow = core::Ptr::<dyn crate::video::DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::video::DualTVL1OpticalFlow,
		cv_PtrOfDualTVL1OpticalFlow_delete, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDualTVL1OpticalFlow {
		#[inline] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
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

mod videostab_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDeblurerBase = core::Ptr::<dyn crate::videostab::DeblurerBase>;
	
	ptr_extern! { dyn crate::videostab::DeblurerBase,
		cv_PtrOfDeblurerBase_delete, cv_PtrOfDeblurerBase_get_inner_ptr, cv_PtrOfDeblurerBase_get_inner_ptr_mut
	}
	
	impl PtrOfDeblurerBase {
		#[inline] pub fn as_raw_PtrOfDeblurerBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIDenseOptFlowEstimator = core::Ptr::<dyn crate::videostab::IDenseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::IDenseOptFlowEstimator,
		cv_PtrOfIDenseOptFlowEstimator_delete, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl PtrOfIDenseOptFlowEstimator {
		#[inline] pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimator for PtrOfIDenseOptFlowEstimator {
		#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIFrameSource = core::Ptr::<dyn crate::videostab::IFrameSource>;
	
	ptr_extern! { dyn crate::videostab::IFrameSource,
		cv_PtrOfIFrameSource_delete, cv_PtrOfIFrameSource_get_inner_ptr, cv_PtrOfIFrameSource_get_inner_ptr_mut
	}
	
	impl PtrOfIFrameSource {
		#[inline] pub fn as_raw_PtrOfIFrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfIFrameSource {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfILog = core::Ptr::<dyn crate::videostab::ILog>;
	
	ptr_extern! { dyn crate::videostab::ILog,
		cv_PtrOfILog_delete, cv_PtrOfILog_get_inner_ptr, cv_PtrOfILog_get_inner_ptr_mut
	}
	
	impl PtrOfILog {
		#[inline] pub fn as_raw_PtrOfILog(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfILog(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ILog for PtrOfILog {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIMotionStabilizer = core::Ptr::<dyn crate::videostab::IMotionStabilizer>;
	
	ptr_extern! { dyn crate::videostab::IMotionStabilizer,
		cv_PtrOfIMotionStabilizer_delete, cv_PtrOfIMotionStabilizer_get_inner_ptr, cv_PtrOfIMotionStabilizer_get_inner_ptr_mut
	}
	
	impl PtrOfIMotionStabilizer {
		#[inline] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIOutlierRejector = core::Ptr::<dyn crate::videostab::IOutlierRejector>;
	
	ptr_extern! { dyn crate::videostab::IOutlierRejector,
		cv_PtrOfIOutlierRejector_delete, cv_PtrOfIOutlierRejector_get_inner_ptr, cv_PtrOfIOutlierRejector_get_inner_ptr_mut
	}
	
	impl PtrOfIOutlierRejector {
		#[inline] pub fn as_raw_PtrOfIOutlierRejector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejector for PtrOfIOutlierRejector {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfISparseOptFlowEstimator = core::Ptr::<dyn crate::videostab::ISparseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::ISparseOptFlowEstimator,
		cv_PtrOfISparseOptFlowEstimator_delete, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl PtrOfISparseOptFlowEstimator {
		#[inline] pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfISparseOptFlowEstimator {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfImageMotionEstimatorBase = core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::ImageMotionEstimatorBase,
		cv_PtrOfImageMotionEstimatorBase_delete, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl PtrOfImageMotionEstimatorBase {
		#[inline] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpainterBase = core::Ptr::<dyn crate::videostab::InpainterBase>;
	
	ptr_extern! { dyn crate::videostab::InpainterBase,
		cv_PtrOfInpainterBase_delete, cv_PtrOfInpainterBase_get_inner_ptr, cv_PtrOfInpainterBase_get_inner_ptr_mut
	}
	
	impl PtrOfInpainterBase {
		#[inline] pub fn as_raw_PtrOfInpainterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfInpainterBase {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionEstimatorBase = core::Ptr::<dyn crate::videostab::MotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::MotionEstimatorBase,
		cv_PtrOfMotionEstimatorBase_delete, cv_PtrOfMotionEstimatorBase_get_inner_ptr, cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl PtrOfMotionEstimatorBase {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionFilterBase = core::Ptr::<dyn crate::videostab::MotionFilterBase>;
	
	ptr_extern! { dyn crate::videostab::MotionFilterBase,
		cv_PtrOfMotionFilterBase_delete, cv_PtrOfMotionFilterBase_get_inner_ptr, cv_PtrOfMotionFilterBase_get_inner_ptr_mut
	}
	
	impl PtrOfMotionFilterBase {
		#[inline] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
		#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfWobbleSuppressorBase = core::Ptr::<dyn crate::videostab::WobbleSuppressorBase>;
	
	ptr_extern! { dyn crate::videostab::WobbleSuppressorBase,
		cv_PtrOfWobbleSuppressorBase_delete, cv_PtrOfWobbleSuppressorBase_get_inner_ptr, cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut
	}
	
	impl PtrOfWobbleSuppressorBase {
		#[inline] pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfWobbleSuppressorBase {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use videostab_types::*;

#[cfg(feature = "contrib")]
mod xfeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineFeature2D = core::Ptr::<dyn crate::xfeatures2d::AffineFeature2D>;
	
	ptr_extern! { dyn crate::xfeatures2d::AffineFeature2D,
		cv_PtrOfAffineFeature2D_delete, cv_PtrOfAffineFeature2D_get_inner_ptr, cv_PtrOfAffineFeature2D_get_inner_ptr_mut
	}
	
	impl PtrOfAffineFeature2D {
		#[inline] pub fn as_raw_PtrOfAffineFeature2D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineFeature2D {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAffineFeature2D {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::AffineFeature2D for PtrOfAffineFeature2D {
		#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoostDesc = core::Ptr::<dyn crate::xfeatures2d::BoostDesc>;
	
	ptr_extern! { dyn crate::xfeatures2d::BoostDesc,
		cv_PtrOfBoostDesc_delete, cv_PtrOfBoostDesc_get_inner_ptr, cv_PtrOfBoostDesc_get_inner_ptr_mut
	}
	
	impl PtrOfBoostDesc {
		#[inline] pub fn as_raw_PtrOfBoostDesc(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoostDesc {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBoostDesc {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BoostDesc for PtrOfBoostDesc {
		#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBriefDescriptorExtractor = core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>;
	
	ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
		cv_PtrOfBriefDescriptorExtractor_delete, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::BriefDescriptorExtractor, cv_PtrOfBriefDescriptorExtractor_new }
	
	impl PtrOfBriefDescriptorExtractor {
		#[inline] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDAISY = core::Ptr::<dyn crate::xfeatures2d::DAISY>;
	
	ptr_extern! { dyn crate::xfeatures2d::DAISY,
		cv_PtrOfDAISY_delete, cv_PtrOfDAISY_get_inner_ptr, cv_PtrOfDAISY_get_inner_ptr_mut
	}
	
	impl PtrOfDAISY {
		#[inline] pub fn as_raw_PtrOfDAISY(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDAISY(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDAISY {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfDAISY {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::DAISY for PtrOfDAISY {
		#[inline] fn as_raw_DAISY(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFREAK = core::Ptr::<crate::xfeatures2d::FREAK>;
	
	ptr_extern! { crate::xfeatures2d::FREAK,
		cv_PtrOfFREAK_delete, cv_PtrOfFREAK_get_inner_ptr, cv_PtrOfFREAK_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::FREAK, cv_PtrOfFREAK_new }
	
	impl PtrOfFREAK {
		#[inline] pub fn as_raw_PtrOfFREAK(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFREAK {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFREAK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::FREAKTrait for PtrOfFREAK {
		#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHarrisLaplaceFeatureDetector = core::Ptr::<crate::xfeatures2d::HarrisLaplaceFeatureDetector>;
	
	ptr_extern! { crate::xfeatures2d::HarrisLaplaceFeatureDetector,
		cv_PtrOfHarrisLaplaceFeatureDetector_delete, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::HarrisLaplaceFeatureDetector, cv_PtrOfHarrisLaplaceFeatureDetector_new }
	
	impl PtrOfHarrisLaplaceFeatureDetector {
		#[inline] pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLATCH = core::Ptr::<crate::xfeatures2d::LATCH>;
	
	ptr_extern! { crate::xfeatures2d::LATCH,
		cv_PtrOfLATCH_delete, cv_PtrOfLATCH_get_inner_ptr, cv_PtrOfLATCH_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::LATCH, cv_PtrOfLATCH_new }
	
	impl PtrOfLATCH {
		#[inline] pub fn as_raw_PtrOfLATCH(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLATCH {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLATCH {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LATCHTrait for PtrOfLATCH {
		#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLUCID = core::Ptr::<crate::xfeatures2d::LUCID>;
	
	ptr_extern! { crate::xfeatures2d::LUCID,
		cv_PtrOfLUCID_delete, cv_PtrOfLUCID_get_inner_ptr, cv_PtrOfLUCID_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::LUCID, cv_PtrOfLUCID_new }
	
	impl PtrOfLUCID {
		#[inline] pub fn as_raw_PtrOfLUCID(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLUCID {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLUCID {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LUCIDTrait for PtrOfLUCID {
		#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMSDDetector = core::Ptr::<crate::xfeatures2d::MSDDetector>;
	
	ptr_extern! { crate::xfeatures2d::MSDDetector,
		cv_PtrOfMSDDetector_delete, cv_PtrOfMSDDetector_get_inner_ptr, cv_PtrOfMSDDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::MSDDetector, cv_PtrOfMSDDetector_new }
	
	impl PtrOfMSDDetector {
		#[inline] pub fn as_raw_PtrOfMSDDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignatures = core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignatures,
		cv_PtrOfPCTSignatures_delete, cv_PtrOfPCTSignatures_get_inner_ptr, cv_PtrOfPCTSignatures_get_inner_ptr_mut
	}
	
	impl PtrOfPCTSignatures {
		#[inline] pub fn as_raw_PtrOfPCTSignatures(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignatures {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
		#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignaturesSQFD = core::Ptr::<dyn crate::xfeatures2d::PCTSignaturesSQFD>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignaturesSQFD,
		cv_PtrOfPCTSignaturesSQFD_delete, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr_mut
	}
	
	impl PtrOfPCTSignaturesSQFD {
		#[inline] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSURF = core::Ptr::<dyn crate::xfeatures2d::SURF>;
	
	ptr_extern! { dyn crate::xfeatures2d::SURF,
		cv_PtrOfSURF_delete, cv_PtrOfSURF_get_inner_ptr, cv_PtrOfSURF_get_inner_ptr_mut
	}
	
	impl PtrOfSURF {
		#[inline] pub fn as_raw_PtrOfSURF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSURF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSURF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSURF {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURF for PtrOfSURF {
		#[inline] fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStarDetector = core::Ptr::<crate::xfeatures2d::StarDetector>;
	
	ptr_extern! { crate::xfeatures2d::StarDetector,
		cv_PtrOfStarDetector_delete, cv_PtrOfStarDetector_get_inner_ptr, cv_PtrOfStarDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::StarDetector, cv_PtrOfStarDetector_new }
	
	impl PtrOfStarDetector {
		#[inline] pub fn as_raw_PtrOfStarDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStarDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfStarDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::StarDetectorTrait for PtrOfStarDetector {
		#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVGG = core::Ptr::<dyn crate::xfeatures2d::VGG>;
	
	ptr_extern! { dyn crate::xfeatures2d::VGG,
		cv_PtrOfVGG_delete, cv_PtrOfVGG_get_inner_ptr, cv_PtrOfVGG_get_inner_ptr_mut
	}
	
	impl PtrOfVGG {
		#[inline] pub fn as_raw_PtrOfVGG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVGG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfVGG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfVGG {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::VGG for PtrOfVGG {
		#[inline] fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfElliptic_KeyPoint = core::Vector::<crate::xfeatures2d::Elliptic_KeyPoint>;
	
	impl VectorOfElliptic_KeyPoint {
		pub fn as_raw_VectorOfElliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfElliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::xfeatures2d::Elliptic_KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfElliptic_KeyPoint_new, cv_VectorOfElliptic_KeyPoint_delete,
		cv_VectorOfElliptic_KeyPoint_len, cv_VectorOfElliptic_KeyPoint_is_empty,
		cv_VectorOfElliptic_KeyPoint_capacity, cv_VectorOfElliptic_KeyPoint_shrink_to_fit,
		cv_VectorOfElliptic_KeyPoint_reserve, cv_VectorOfElliptic_KeyPoint_remove,
		cv_VectorOfElliptic_KeyPoint_swap, cv_VectorOfElliptic_KeyPoint_clear,
		cv_VectorOfElliptic_KeyPoint_get, cv_VectorOfElliptic_KeyPoint_set,
		cv_VectorOfElliptic_KeyPoint_push, cv_VectorOfElliptic_KeyPoint_insert,
	}
	vector_non_copy_or_bool! { crate::xfeatures2d::Elliptic_KeyPoint }
	
	unsafe impl Send for core::Vector::<crate::xfeatures2d::Elliptic_KeyPoint> {}
	
}
#[cfg(feature = "contrib")]
pub use xfeatures2d_types::*;

#[cfg(feature = "contrib")]
mod ximgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAdaptiveManifoldFilter = core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>;
	
	ptr_extern! { dyn crate::ximgproc::AdaptiveManifoldFilter,
		cv_PtrOfAdaptiveManifoldFilter_delete, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr_mut
	}
	
	impl PtrOfAdaptiveManifoldFilter {
		#[inline] pub fn as_raw_PtrOfAdaptiveManifoldFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAdaptiveManifoldFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::AdaptiveManifoldFilter for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfContourFitting = core::Ptr::<crate::ximgproc::ContourFitting>;
	
	ptr_extern! { crate::ximgproc::ContourFitting,
		cv_PtrOfContourFitting_delete, cv_PtrOfContourFitting_get_inner_ptr, cv_PtrOfContourFitting_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ximgproc::ContourFitting, cv_PtrOfContourFitting_new }
	
	impl PtrOfContourFitting {
		#[inline] pub fn as_raw_PtrOfContourFitting(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfContourFitting(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfContourFitting {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::ContourFittingTrait for PtrOfContourFitting {
		#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTFilter = core::Ptr::<dyn crate::ximgproc::DTFilter>;
	
	ptr_extern! { dyn crate::ximgproc::DTFilter,
		cv_PtrOfDTFilter_delete, cv_PtrOfDTFilter_get_inner_ptr, cv_PtrOfDTFilter_get_inner_ptr_mut
	}
	
	impl PtrOfDTFilter {
		#[inline] pub fn as_raw_PtrOfDTFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDTFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::DTFilter for PtrOfDTFilter {
		#[inline] fn as_raw_DTFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DTFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDisparityWLSFilter = core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>;
	
	ptr_extern! { dyn crate::ximgproc::DisparityWLSFilter,
		cv_PtrOfDisparityWLSFilter_delete, cv_PtrOfDisparityWLSFilter_get_inner_ptr, cv_PtrOfDisparityWLSFilter_get_inner_ptr_mut
	}
	
	impl PtrOfDisparityWLSFilter {
		#[inline] pub fn as_raw_PtrOfDisparityWLSFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDisparityWLSFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityFilter for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityWLSFilter for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_DisparityWLSFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeAwareInterpolator = core::Ptr::<dyn crate::ximgproc::EdgeAwareInterpolator>;
	
	ptr_extern! { dyn crate::ximgproc::EdgeAwareInterpolator,
		cv_PtrOfEdgeAwareInterpolator_delete, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr_mut
	}
	
	impl PtrOfEdgeAwareInterpolator {
		#[inline] pub fn as_raw_PtrOfEdgeAwareInterpolator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeAwareInterpolator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeAwareInterpolator for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolator for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeBoxes = core::Ptr::<dyn crate::ximgproc::EdgeBoxes>;
	
	ptr_extern! { dyn crate::ximgproc::EdgeBoxes,
		cv_PtrOfEdgeBoxes_delete, cv_PtrOfEdgeBoxes_get_inner_ptr, cv_PtrOfEdgeBoxes_get_inner_ptr_mut
	}
	
	impl PtrOfEdgeBoxes {
		#[inline] pub fn as_raw_PtrOfEdgeBoxes(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeBoxes(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEdgeBoxes {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeBoxes for PtrOfEdgeBoxes {
		#[inline] fn as_raw_EdgeBoxes(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastBilateralSolverFilter = core::Ptr::<dyn crate::ximgproc::FastBilateralSolverFilter>;
	
	ptr_extern! { dyn crate::ximgproc::FastBilateralSolverFilter,
		cv_PtrOfFastBilateralSolverFilter_delete, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFastBilateralSolverFilter {
		#[inline] pub fn as_raw_PtrOfFastBilateralSolverFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastBilateralSolverFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::FastBilateralSolverFilter for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastGlobalSmootherFilter = core::Ptr::<dyn crate::ximgproc::FastGlobalSmootherFilter>;
	
	ptr_extern! { dyn crate::ximgproc::FastGlobalSmootherFilter,
		cv_PtrOfFastGlobalSmootherFilter_delete, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFastGlobalSmootherFilter {
		#[inline] pub fn as_raw_PtrOfFastGlobalSmootherFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastGlobalSmootherFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::FastGlobalSmootherFilter for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastLineDetector = core::Ptr::<dyn crate::ximgproc::FastLineDetector>;
	
	ptr_extern! { dyn crate::ximgproc::FastLineDetector,
		cv_PtrOfFastLineDetector_delete, cv_PtrOfFastLineDetector_get_inner_ptr, cv_PtrOfFastLineDetector_get_inner_ptr_mut
	}
	
	impl PtrOfFastLineDetector {
		#[inline] pub fn as_raw_PtrOfFastLineDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastLineDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastLineDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::FastLineDetector for PtrOfFastLineDetector {
		#[inline] fn as_raw_FastLineDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGraphSegmentation = core::Ptr::<dyn crate::ximgproc::GraphSegmentation>;
	
	ptr_extern! { dyn crate::ximgproc::GraphSegmentation,
		cv_PtrOfGraphSegmentation_delete, cv_PtrOfGraphSegmentation_get_inner_ptr, cv_PtrOfGraphSegmentation_get_inner_ptr_mut
	}
	
	impl PtrOfGraphSegmentation {
		#[inline] pub fn as_raw_PtrOfGraphSegmentation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGraphSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGraphSegmentation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::GraphSegmentation for PtrOfGraphSegmentation {
		#[inline] fn as_raw_GraphSegmentation(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGuidedFilter = core::Ptr::<dyn crate::ximgproc::GuidedFilter>;
	
	ptr_extern! { dyn crate::ximgproc::GuidedFilter,
		cv_PtrOfGuidedFilter_delete, cv_PtrOfGuidedFilter_get_inner_ptr, cv_PtrOfGuidedFilter_get_inner_ptr_mut
	}
	
	impl PtrOfGuidedFilter {
		#[inline] pub fn as_raw_PtrOfGuidedFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGuidedFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGuidedFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::GuidedFilter for PtrOfGuidedFilter {
		#[inline] fn as_raw_GuidedFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRFFeatureGetter = core::Ptr::<dyn crate::ximgproc::RFFeatureGetter>;
	
	ptr_extern! { dyn crate::ximgproc::RFFeatureGetter,
		cv_PtrOfRFFeatureGetter_delete, cv_PtrOfRFFeatureGetter_get_inner_ptr, cv_PtrOfRFFeatureGetter_get_inner_ptr_mut
	}
	
	impl PtrOfRFFeatureGetter {
		#[inline] pub fn as_raw_PtrOfRFFeatureGetter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRFFeatureGetter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::RFFeatureGetter for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_RFFeatureGetter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRidgeDetectionFilter = core::Ptr::<dyn crate::ximgproc::RidgeDetectionFilter>;
	
	ptr_extern! { dyn crate::ximgproc::RidgeDetectionFilter,
		cv_PtrOfRidgeDetectionFilter_delete, cv_PtrOfRidgeDetectionFilter_get_inner_ptr, cv_PtrOfRidgeDetectionFilter_get_inner_ptr_mut
	}
	
	impl PtrOfRidgeDetectionFilter {
		#[inline] pub fn as_raw_PtrOfRidgeDetectionFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRidgeDetectionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::RidgeDetectionFilter for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_RidgeDetectionFilter(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentation = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentation>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentation,
		cv_PtrOfSelectiveSearchSegmentation_delete, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentation {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentation for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategy = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategy,
		cv_PtrOfSelectiveSearchSegmentationStrategy_delete, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyColor = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor,
		cv_PtrOfSelectiveSearchSegmentationStrategyColor_delete, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyColor for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyFill = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill,
		cv_PtrOfSelectiveSearchSegmentationStrategyFill_delete, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyFill for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyMultiple = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple,
		cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_delete, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategySize = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize,
		cv_PtrOfSelectiveSearchSegmentationStrategySize_delete, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategySize for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyTexture = core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture,
		cv_PtrOfSelectiveSearchSegmentationStrategyTexture_delete, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTexture for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStructuredEdgeDetection = core::Ptr::<dyn crate::ximgproc::StructuredEdgeDetection>;
	
	ptr_extern! { dyn crate::ximgproc::StructuredEdgeDetection,
		cv_PtrOfStructuredEdgeDetection_delete, cv_PtrOfStructuredEdgeDetection_get_inner_ptr, cv_PtrOfStructuredEdgeDetection_get_inner_ptr_mut
	}
	
	impl PtrOfStructuredEdgeDetection {
		#[inline] pub fn as_raw_PtrOfStructuredEdgeDetection(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStructuredEdgeDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::StructuredEdgeDetection for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_StructuredEdgeDetection(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelLSC = core::Ptr::<dyn crate::ximgproc::SuperpixelLSC>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelLSC,
		cv_PtrOfSuperpixelLSC_delete, cv_PtrOfSuperpixelLSC_get_inner_ptr, cv_PtrOfSuperpixelLSC_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelLSC {
		#[inline] pub fn as_raw_PtrOfSuperpixelLSC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelLSC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelLSC for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_SuperpixelLSC(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSEEDS = core::Ptr::<dyn crate::ximgproc::SuperpixelSEEDS>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelSEEDS,
		cv_PtrOfSuperpixelSEEDS_delete, cv_PtrOfSuperpixelSEEDS_get_inner_ptr, cv_PtrOfSuperpixelSEEDS_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelSEEDS {
		#[inline] pub fn as_raw_PtrOfSuperpixelSEEDS(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSEEDS(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSEEDS for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_SuperpixelSEEDS(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSLIC = core::Ptr::<dyn crate::ximgproc::SuperpixelSLIC>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelSLIC,
		cv_PtrOfSuperpixelSLIC_delete, cv_PtrOfSuperpixelSLIC_get_inner_ptr, cv_PtrOfSuperpixelSLIC_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelSLIC {
		#[inline] pub fn as_raw_PtrOfSuperpixelSLIC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSLIC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSLIC for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_SuperpixelSLIC(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use ximgproc_types::*;

#[cfg(feature = "contrib")]
mod xobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWBDetector = core::Ptr::<dyn crate::xobjdetect::WBDetector>;
	
	ptr_extern! { dyn crate::xobjdetect::WBDetector,
		cv_PtrOfWBDetector_delete, cv_PtrOfWBDetector_get_inner_ptr, cv_PtrOfWBDetector_get_inner_ptr_mut
	}
	
	impl PtrOfWBDetector {
		#[inline] pub fn as_raw_PtrOfWBDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
		#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use xobjdetect_types::*;

#[cfg(feature = "contrib")]
mod xphoto_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayworldWB = core::Ptr::<dyn crate::xphoto::GrayworldWB>;
	
	ptr_extern! { dyn crate::xphoto::GrayworldWB,
		cv_PtrOfGrayworldWB_delete, cv_PtrOfGrayworldWB_get_inner_ptr, cv_PtrOfGrayworldWB_get_inner_ptr_mut
	}
	
	impl PtrOfGrayworldWB {
		#[inline] pub fn as_raw_PtrOfGrayworldWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayworldWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
		#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLearningBasedWB = core::Ptr::<dyn crate::xphoto::LearningBasedWB>;
	
	ptr_extern! { dyn crate::xphoto::LearningBasedWB,
		cv_PtrOfLearningBasedWB_delete, cv_PtrOfLearningBasedWB_get_inner_ptr, cv_PtrOfLearningBasedWB_get_inner_ptr_mut
	}
	
	impl PtrOfLearningBasedWB {
		#[inline] pub fn as_raw_PtrOfLearningBasedWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLearningBasedWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
		#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleWB = core::Ptr::<dyn crate::xphoto::SimpleWB>;
	
	ptr_extern! { dyn crate::xphoto::SimpleWB,
		cv_PtrOfSimpleWB_delete, cv_PtrOfSimpleWB_get_inner_ptr, cv_PtrOfSimpleWB_get_inner_ptr_mut
	}
	
	impl PtrOfSimpleWB {
		#[inline] pub fn as_raw_PtrOfSimpleWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
		#[inline] fn as_raw_SimpleWB(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDurand = core::Ptr::<dyn crate::xphoto::TonemapDurand>;
	
	ptr_extern! { dyn crate::xphoto::TonemapDurand,
		cv_PtrOfTonemapDurand_delete, cv_PtrOfTonemapDurand_get_inner_ptr, cv_PtrOfTonemapDurand_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapDurand {
		#[inline] pub fn as_raw_PtrOfTonemapDurand(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDurand {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDurand {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::TonemapDurand for PtrOfTonemapDurand {
		#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
		#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use xphoto_types::*;

pub use crate::manual::types::*;
