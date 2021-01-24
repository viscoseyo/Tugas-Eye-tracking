pub mod calib3d;
pub mod core;
pub mod dnn;
pub mod features2d;
pub mod flann;
pub mod highgui;
pub mod imgcodecs;
pub mod imgproc;
pub mod ml;
pub mod objdetect;
pub mod photo;
pub mod stitching;
pub mod video;
pub mod videoio;
#[cfg(feature = "contrib")]
pub mod world;
pub mod types;
#[doc(hidden)]
pub mod sys;
pub mod hub_prelude {
	pub use super::calib3d::prelude::*;
	pub use super::core::prelude::*;
	pub use super::dnn::prelude::*;
	pub use super::features2d::prelude::*;
	pub use super::flann::prelude::*;
	pub use super::highgui::prelude::*;
	pub use super::imgcodecs::prelude::*;
	pub use super::imgproc::prelude::*;
	pub use super::ml::prelude::*;
	pub use super::objdetect::prelude::*;
	pub use super::photo::prelude::*;
	pub use super::stitching::prelude::*;
	pub use super::video::prelude::*;
	pub use super::videoio::prelude::*;
	#[cfg(feature = "contrib")]
	pub use super::world::prelude::*;
}
