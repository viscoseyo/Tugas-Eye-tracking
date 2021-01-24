#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

pub fn init_all() -> Result<bool> {
	unsafe { sys::cv_initAll() }.into_result()
}
