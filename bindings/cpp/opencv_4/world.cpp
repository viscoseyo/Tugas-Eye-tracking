#include "ocvrs_common.hpp"
#include <opencv2/world.hpp>
#include "world_types.hpp"

extern "C" {
	Result<bool> cv_initAll() {
		try {
			bool ret = cv::initAll();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
}
