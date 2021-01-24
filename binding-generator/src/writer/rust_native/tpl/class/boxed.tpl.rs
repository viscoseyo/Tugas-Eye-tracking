{{doc_comment}}
{{debug}}
pub struct {{rust_local}} {
	ptr: {{rust_extern_mut}}
}

opencv_type_boxed! { {{rust_local}} }

impl Drop for {{rust_local}} {
	fn drop(&mut self) {
		extern "C" { fn cv_{{rust_local}}_delete(instance: {{rust_extern_mut}}); }
		unsafe { cv_{{rust_local}}_delete(self.as_raw_mut_{{rust_local}}()) };
	}
}

impl {{rust_local}} {
	#[inline] pub fn as_raw_{{rust_local}}(&self) -> {{rust_extern_const}} { self.as_raw() }
	#[inline] pub fn as_raw_mut_{{rust_local}}(&mut self) -> {{rust_extern_mut}} { self.as_raw_mut() }
}

unsafe impl Send for {{rust_local}} {}

{{bases}}
{{impl}}
{{impls}}

