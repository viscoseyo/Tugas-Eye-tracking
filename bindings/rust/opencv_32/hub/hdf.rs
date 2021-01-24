#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Hierarchical Data Format I/O routines
//! 
//! This module provides storage routines for Hierarchical Data Format objects.
//!    # Hierarchical Data Format version 5
//! 
//! Hierarchical Data Format version 5
//! --------------------------------------------------------
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::HDF5 };
}

pub const HDF5_H5_GETCHUNKDIMS: i32 = 102;
pub const HDF5_H5_GETDIMS: i32 = 100;
pub const HDF5_H5_GETMAXDIMS: i32 = 101;
pub const HDF5_H5_NONE: i32 = -1;
pub const HDF5_H5_UNLIMITED: i32 = -1;
/// Open or create hdf5 file
/// ## Parameters
/// * HDF5Filename: specify the HDF5 filename.
/// 
/// Returns pointer to the hdf5 object class
/// 
/// 
/// Note: If hdf5 file does not exist it will be created. Any operations except dscreate() functions on object
/// will be thread safe. Multiple datasets can be created inside single hdf5 file, and can be accessed
/// from same hdf5 object from multiple instances as long read or write operations are done over
/// non-overlapping regions of dataset. Single hdf5 file also can be opened by multiple instances,
/// reads and writes can be instantiated at the same time as long non-overlapping regions are involved. Object
/// is released using close().
/// 
/// - Example below open and then release the file.
/// ```ignore
///   // open / autocreate hdf5 file
///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
///   // ...
///   // release
///   h5io->close();
/// ```
/// 
/// 
/// ![Visualization of 10x10 CV_64FC2 (Hilbert matrix) using HDFView tool](https://docs.opencv.org/3.2.0/hdfview_demo.gif)
/// 
/// - Text dump (3x3 Hilbert matrix) of hdf5 dataset using **h5dump** tool:
/// ```ignore
/// $ h5dump test.h5
/// HDF5 "test.h5" {
/// GROUP "/" {
///    DATASET "hilbert" {
///       DATATYPE  H5T_ARRAY { [2] H5T_IEEE_F64LE }
///       DATASPACE  SIMPLE { ( 3, 3 ) / ( 3, 3 ) }
///       DATA {
///       (0,0): [ 1, -1 ], [ 0.5, -0.5 ], [ 0.333333, -0.333333 ],
///       (1,0): [ 0.5, -0.5 ], [ 0.333333, -0.333333 ], [ 0.25, -0.25 ],
///       (2,0): [ 0.333333, -0.333333 ], [ 0.25, -0.25 ], [ 0.2, -0.2 ]
///       }
///    }
/// }
/// }
/// ```
/// 
pub fn open(hdf5_filename: &str) -> Result<core::Ptr::<dyn crate::hdf::HDF5>> {
	extern_container_arg!(mut hdf5_filename);
	unsafe { sys::cv_hdf_open_String(hdf5_filename.opencv_as_extern_mut()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::hdf::HDF5>::opencv_from_extern(r) } )
}

/// Hierarchical Data Format version 5 interface.
/// 
/// Notice that module is compiled only when hdf5 is correctly installed.
pub trait HDF5 {
	fn as_raw_HDF5(&self) -> *const c_void;
	fn as_raw_mut_HDF5(&mut self) -> *mut c_void;

	/// Close and release hdf5 object.
	fn close(&mut self) -> Result<()> {
		unsafe { sys::cv_hdf_HDF5_close(self.as_raw_mut_HDF5()) }.into_result()
	}
	
	/// Create a group.
	/// ## Parameters
	/// * grlabel: specify the hdf5 group label.
	/// 
	/// Create a hdf5 group.
	/// 
	/// 
	/// Note: Groups are useful for better organise multiple datasets. It is possible to create subgroups within any group.
	/// Existence of a particular group can be checked using hlexists(). In case of subgroups label would be e.g: 'Group1/SubGroup1'
	/// where SubGroup1 is within the root group Group1.
	/// 
	/// - In this example Group1 will have one subgrup labeled SubGroup1:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create Group1 if does not exists
	///   if ( ! h5io->hlexists( "Group1" ) )
	///    h5io->grcreate( "Group1" );
	///   else
	///    printf("Group1 already created, skipping\n" );
	///   // create SubGroup1 if does not exists
	///   if ( ! h5io->hlexists( "Group1/SubGroup1" ) )
	///    h5io->grcreate( "Group1/SubGroup1" );
	///   else
	///    printf("SubGroup1 already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: When a dataset is created with dscreate() or kpcreate() it can be created right within a group by specifying
	/// full path within the label, in our example would be: 'Group1/SubGroup1/MyDataSet'. It is not thread safe.
	fn grcreate(&mut self, grlabel: &str) -> Result<()> {
		extern_container_arg!(mut grlabel);
		unsafe { sys::cv_hdf_HDF5_grcreate_String(self.as_raw_mut_HDF5(), grlabel.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Check if label exists or not.
	/// ## Parameters
	/// * label: specify the hdf5 dataset label.
	/// 
	/// Returns **true** if dataset exists, and **false** if does not.
	/// 
	/// 
	/// Note: Checks if dataset, group or other object type (hdf5 link) exists under the label name. It is thread safe.
	fn hlexists(&self, label: &str) -> Result<bool> {
		extern_container_arg!(mut label);
		unsafe { sys::cv_hdf_HDF5_hlexists_const_String(self.as_raw_HDF5(), label.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dscreate(&self, rows: i32, cols: i32, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_String(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dscreate_1(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_String_const_int(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern_mut(), compresslevel) }.into_result()
	}
	
	fn dscreate_2(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector::<i32>) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_String_const_int_const_vector_int_R(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern_mut(), compresslevel, dims_chunks.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Create and allocate storage for two dimensional single or multi channel dataset.
	/// ## Parameters
	/// * rows: declare amount of rows
	/// * cols: declare amount of cols
	/// * type: type to be used
	/// * dslabel: specify the hdf5 dataset label, any existing dataset with the same label will be overwritten.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is default and means no compression.
	/// * dims_chunks: each array member specify chunking sizes to be used for block i/o,
	///        by default NULL means none at all.
	/// 
	/// 
	/// Note: If the dataset already exists an exception will be thrown.
	/// 
	/// - Existence of the dataset can be checked using hlexists(), see in this example:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 100x50 CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert" ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed booth at read or write time especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified default one will be invoked by the size of **whole** dataset
	/// as single big chunk of data.
	/// 
	/// - See example of level 9 compression using internal default chunking:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert", 9 ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert", 9 );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: A value of H5_UNLIMITED for **rows** or **cols** or booth means **unlimited** data on the specified dimension,
	/// thus is possible to expand anytime such dataset on row, col or booth directions. Presence of H5_UNLIMITED on any
	/// dimension **require** to define custom chunking. No default chunking will be defined in unlimited scenario since
	/// default size on that dimension will be zero, and will grow once dataset is written. Writing into dataset that have
	/// H5_UNLIMITED on some of its dimension requires dsinsert() that allow growth on unlimited dimension instead of dswrite()
	/// that allows to write only in predefined data space.
	/// 
	/// - Example below shows no compression but unlimited dimension on cols using 100x100 internal chunking:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   int chunks[2] = { 100, 100 };
	///   h5io->dscreate( 100, cv::hdf::HDF5::H5_UNLIMITED, CV_64FC2, "hilbert", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: It is **not** thread safe, it must be called only once at dataset creation otherwise exception will occur.
	/// Multiple datasets inside single hdf5 file is allowed.
	fn dscreate_3(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &i32) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_String_const_int_const_intX(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern_mut(), compresslevel, dims_chunks) }.into_result()
	}
	
	fn dscreate_4(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_String(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dscreate_5(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_String_const_int(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern_mut(), compresslevel) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * compresslevel: HDF5::H5_NONE
	/// * dims_chunks: vector<int>()
	fn dscreate_6(&self, sizes: &core::Vector::<i32>, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector::<i32>) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_vector_int_R_const_int_String_const_int_const_vector_int_R(self.as_raw_HDF5(), sizes.as_raw_VectorOfi32(), typ, dslabel.opencv_as_extern_mut(), compresslevel, dims_chunks.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Create and allocate storage for n-dimensional dataset, single or mutichannel type.
	/// ## Parameters
	/// * n_dims: declare number of dimensions
	/// * sizes: array containing sizes for each dimensions
	/// * type: type to be used
	/// * dslabel: specify the hdf5 dataset label, any existing dataset with the same label will be overwritten.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is default and means no compression.
	/// * dims_chunks: each array member specify chunking sizes to be used for block i/o,
	///        by default NULL means none at all.
	/// 
	/// Note: If the dataset already exists an exception will be thrown. Existence of the dataset can be checked
	/// using hlexists().
	/// 
	/// - See example below that creates a 6 dimensional storage space:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 6 dimensional CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "nddata" ) )
	///    int n_dims = 5;
	///    int dsdims[n_dims] = { 100, 100, 20, 10, 5, 5 };
	///    h5io->dscreate( n_dims, sizes, CV_64FC2, "nddata" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed booth at read or write time especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified default one will be invoked by the size of **whole** dataset
	/// as single big chunk of data.
	/// 
	/// - See example of level 0 compression (shallow) using chunking against first
	/// dimension, thus storage will consists by 100 chunks of data:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 6 dimensional CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "nddata" ) )
	///    int n_dims = 5;
	///    int dsdims[n_dims] = { 100, 100, 20, 10, 5, 5 };
	///    int chunks[n_dims] = {   1, 100, 20, 10, 5, 5 };
	///    h5io->dscreate( n_dims, dsdims, CV_64FC2, "nddata", 0, chunks );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// 
	/// Note: A value of H5_UNLIMITED inside the **sizes** array means **unlimited** data on that dimension, thus is
	/// possible to expand anytime such dataset on those unlimited directions. Presence of H5_UNLIMITED on any dimension
	/// **require** to define custom chunking. No default chunking will be defined in unlimited scenario since default size
	/// on that dimension will be zero, and will grow once dataset is written. Writing into dataset that have H5_UNLIMITED on
	/// some of its dimension requires dsinsert() instead of dswrite() that allow growth on unlimited dimension instead of
	/// dswrite() that allows to write only in predefined data space.
	/// 
	/// - Example below shows a 3 dimensional dataset using no compression with all unlimited sizes and one unit chunking:
	/// ```ignore
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   int n_dims = 3;
	///   int chunks[n_dims] = { 1, 1, 1 };
	///   int dsdims[n_dims] = { cv::hdf::HDF5::H5_UNLIMITED, cv::hdf::HDF5::H5_UNLIMITED, cv::hdf::HDF5::H5_UNLIMITED };
	///   h5io->dscreate( n_dims, dsdims, CV_64FC2, "nddata", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	fn dscreate_7(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &i32) -> Result<()> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_String_const_int_const_intX(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern_mut(), compresslevel, dims_chunks) }.into_result()
	}
	
	/// Fetch dataset sizes
	/// ## Parameters
	/// * dslabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, and dataset maximum dimensions on H5_GETMAXDIMS.
	/// 
	/// Returns vector object containing sizes of dataset on each dimensions.
	/// 
	/// 
	/// Note: Resulting vector size will match the amount of dataset dimensions. By default H5_GETDIMS will return
	/// actual dataset dimensions. Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match
	/// actual dataset dimension but can hold H5_UNLIMITED value if dataset was prepared in **unlimited** mode on
	/// some of its dimension. It can be useful to check existing dataset dimensions before overwrite it as whole or subset.
	/// Trying to write with oversized source data into dataset target will thrown exception. The H5_GETCHUNKDIMS will
	/// return the dimension of chunk if dataset was created with chunking options otherwise returned vector size
	/// will be zero.
	/// 
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	fn dsgetsize(&self, dslabel: &str, dims_flag: i32) -> Result<core::Vector::<i32>> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsgetsize_const_String_int(self.as_raw_HDF5(), dslabel.opencv_as_extern_mut(), dims_flag) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	/// Fetch dataset type
	/// ## Parameters
	/// * dslabel: specify the hdf5 dataset label to be checked.
	/// 
	/// Returns the stored matrix type. This is an identifier compatible with the CvMat type system,
	/// like e.g. CV_16SC5 (16-bit signed 5-channel array), and so on.
	/// 
	/// 
	/// Note: Result can be parsed with CV_MAT_CN() to obtain amount of channels and CV_MAT_DEPTH() to obtain native cvdata type.
	/// It is thread safe.
	fn dsgettype(&self, dslabel: &str) -> Result<i32> {
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsgettype_const_String(self.as_raw_HDF5(), dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dswrite(&self, array: &dyn core::ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_String(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dswrite_1(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_String_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	fn dswrite_2(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &core::Vector::<i32>, dims_counts: &core::Vector::<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_String_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Write or overwrite a Mat object into specified dataset of hdf5 file.
	/// ## Parameters
	/// * Array: specify Mat data array to be written.
	/// * dslabel: specify the target hdf5 dataset label.
	/// * dims_offset: each array member specify the offset location
	///        over dataset's each dimensions from where InputArray will be (over)written into dataset.
	/// * dims_counts: each array member specify the amount of data over dataset's
	///        each dimensions from InputArray that will be written into dataset.
	/// 
	/// Writes Mat object into targeted dataset.
	/// 
	/// 
	/// Note: If dataset is not created and does not exist it will be created **automatically**. Only Mat is supported and
	/// it must to be **continuous**. It is thread safe but it is recommended that writes to happen over separate non overlapping
	/// regions. Multiple datasets can be written inside single hdf5 file.
	/// 
	/// - Example below writes a 100x100 CV_64FC2 matrix into a dataset. No dataset precreation required. If routine
	/// is called multiple times dataset will be just overwritten:
	/// ```ignore
	///   // dual channel hilbert matrix
	///   cv::Mat H(100, 100, CV_64FC2);
	///   for(int i = 0; i < H.rows; i++)
	///    for(int j = 0; j < H.cols; j++)
	///    {
	///        H.at<cv::Vec2d>(i,j)[0] =  1./(i+j+1);
	///        H.at<cv::Vec2d>(i,j)[1] = -1./(i+j+1);
	///        count++;
	///    }
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // write / overwrite dataset
	///   h5io->dswrite( H, "hilbert" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// - Example below writes a smaller 50x100 matrix into 100x100 compressed space optimised by two 50x100 chunks.
	/// Matrix is written twice into first half (0->50) and second half (50->100) of data space using offset.
	/// ```ignore
	///   // dual channel hilbert matrix
	///   cv::Mat H(50, 100, CV_64FC2);
	///   for(int i = 0; i < H.rows; i++)
	///    for(int j = 0; j < H.cols; j++)
	///    {
	///        H.at<cv::Vec2d>(i,j)[0] =  1./(i+j+1);
	///        H.at<cv::Vec2d>(i,j)[1] = -1./(i+j+1);
	///        count++;
	///    }
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // optimise dataset by two chunks
	///   int chunks[2] = { 50, 100 };
	///   // create 100x100 CV_64FC2 compressed space
	///   h5io->dscreate( 100, 100, CV_64FC2, "hilbert", 9, chunks );
	///   // write into first half
	///   int offset1[2] = { 0, 0 };
	///   h5io->dswrite( H, "hilbert", offset1 );
	///   // write into second half
	///   int offset2[2] = { 50, 0 };
	///   h5io->dswrite( H, "hilbert", offset2 );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	fn dswrite_3(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_String_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset, dims_counts) }.into_result()
	}
	
	fn dsinsert(&self, array: &dyn core::ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_String(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dsinsert_1(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_String_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	fn dsinsert_2(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &core::Vector::<i32>, dims_counts: &core::Vector::<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_String_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Insert or overwrite a Mat object into specified dataset and autoexpand dataset size if **unlimited** property allows.
	/// ## Parameters
	/// * Array: specify Mat data array to be written.
	/// * dslabel: specify the target hdf5 dataset label.
	/// * dims_offset: each array member specify the offset location
	///        over dataset's each dimensions from where InputArray will be (over)written into dataset.
	/// * dims_counts: each array member specify the amount of data over dataset's
	///        each dimensions from InputArray that will be written into dataset.
	/// 
	/// Writes Mat object into targeted dataset and **autoexpand** dataset dimension if allowed.
	/// 
	/// 
	/// Note: Unlike dswrite(), datasets are **not** created **automatically**. Only Mat is supported and it must to be **continuous**.
	/// If dsinsert() happen over outer regions of dataset dimensions and on that dimension of dataset is in **unlimited** mode then
	/// dataset is expanded, otherwise exception is thrown. To create datasets with **unlimited** property on specific or more
	/// dimensions see dscreate() and the optional H5_UNLIMITED flag at creation time. It is not thread safe over same dataset
	/// but multiple datasets can be merged inside single hdf5 file.
	/// 
	/// - Example below creates **unlimited** rows x 100 cols and expand rows 5 times with dsinsert() using single 100x100 CV_64FC2
	/// over the dataset. Final size will have 5x100 rows and 100 cols, reflecting H matrix five times over row's span. Chunks size is
	/// 100x100 just optimized against the H matrix size having compression disabled. If routine is called multiple times dataset will be
	/// just overwritten:
	/// ```ignore
	///   // dual channel hilbert matrix
	///   cv::Mat H(50, 100, CV_64FC2);
	///   for(int i = 0; i < H.rows; i++)
	///    for(int j = 0; j < H.cols; j++)
	///    {
	///        H.at<cv::Vec2d>(i,j)[0] =  1./(i+j+1);
	///        H.at<cv::Vec2d>(i,j)[1] = -1./(i+j+1);
	///        count++;
	///    }
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // optimise dataset by chunks
	///   int chunks[2] = { 100, 100 };
	///   // create Unlimited x 100 CV_64FC2 space
	///   h5io->dscreate( cv::hdf::HDF5::H5_UNLIMITED, 100, CV_64FC2, "hilbert", cv::hdf::HDF5::H5_NONE, chunks );
	///   // write into first half
	///   int offset[2] = { 0, 0 };
	///   for ( int t = 0; t < 5; t++ )
	///   {
	///    offset[0] += 100 * t;
	///    h5io->dsinsert( H, "hilbert", offset );
	///   }
	///   // release
	///   h5io->close();
	/// ```
	/// 
	fn dsinsert_3(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_String_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern_mut(), dims_offset, dims_counts) }.into_result()
	}
	
	fn dsread(&self, array: &mut dyn core::ToOutputArray, dslabel: &str) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_String(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern_mut()) }.into_result()
	}
	
	fn dsread_1(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_String_const_intX(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern_mut(), dims_offset) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	fn dsread_2(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &core::Vector::<i32>, dims_counts: &core::Vector::<i32>) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_String_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern_mut(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Read specific dataset from hdf5 file into Mat object.
	/// ## Parameters
	/// * Array: Mat container where data reads will be returned.
	/// * dslabel: specify the source hdf5 dataset label.
	/// * dims_offset: each array member specify the offset location over
	///        each dimensions from where dataset starts to read into OutputArray.
	/// * dims_counts: each array member specify the amount over dataset's each
	///        dimensions of dataset to read into OutputArray.
	/// 
	/// Reads out Mat object reflecting the stored dataset.
	/// 
	/// 
	/// Note: If hdf5 file does not exist an exception will be thrown. Use hlexists() to check dataset presence.
	/// It is thread safe.
	/// 
	/// - Example below reads a dataset:
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank Mat container
	///   cv::Mat H;
	///   // read hibert dataset
	///   h5io->read( H, "hilbert" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// - Example below perform read of 3x5 submatrix from second row and third element.
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank Mat container
	///   cv::Mat H;
	///   int offset[2] = { 1, 2 };
	///   int counts[2] = { 3, 5 };
	///   // read hibert dataset
	///   h5io->read( H, "hilbert", offset, counts );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	fn dsread_3(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(mut dslabel);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_String_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern_mut(), dims_offset, dims_counts) }.into_result()
	}
	
	/// Fetch keypoint dataset size
	/// ## Parameters
	/// * kplabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, and dataset maximum dimensions on H5_GETMAXDIMS.
	/// 
	/// Returns size of keypoints dataset.
	/// 
	/// 
	/// Note: Resulting size will match the amount of keypoints. By default H5_GETDIMS will return actual dataset dimension.
	/// Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match actual dataset dimension but can hold
	/// H5_UNLIMITED value if dataset was prepared in **unlimited** mode. It can be useful to check existing dataset dimension
	/// before overwrite it as whole or subset. Trying to write with oversized source data into dataset target will thrown
	/// exception. The H5_GETCHUNKDIMS will return the dimension of chunk if dataset was created with chunking options otherwise
	/// returned vector size will be zero.
	/// 
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	fn kpgetsize(&self, kplabel: &str, dims_flag: i32) -> Result<i32> {
		extern_container_arg!(mut kplabel);
		unsafe { sys::cv_hdf_HDF5_kpgetsize_const_String_int(self.as_raw_HDF5(), kplabel.opencv_as_extern_mut(), dims_flag) }.into_result()
	}
	
	/// Create and allocate special storage for cv::KeyPoint dataset.
	/// ## Parameters
	/// * size: declare fixed number of KeyPoints
	/// * kplabel: specify the hdf5 dataset label, any existing dataset with the same label will be overwritten.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is default and means no compression.
	/// * chunks: each array member specify chunking sizes to be used for block i/o,
	///        H5_NONE is default and means no compression.
	/// 
	/// Note: If the dataset already exists an exception will be thrown. Existence of the dataset can be checked
	/// using hlexists().
	/// 
	/// - See example below that creates space for 100 keypoints in the dataset:
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( 100, "keypoints" );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	/// 
	/// 
	/// 
	/// Note: A value of H5_UNLIMITED for **size** means **unlimited** keypoints, thus is possible to expand anytime such
	/// dataset by adding or inserting. Presence of H5_UNLIMITED **require** to define custom chunking. No default chunking
	/// will be defined in unlimited scenario since default size on that dimension will be zero, and will grow once dataset
	/// is written. Writing into dataset that have H5_UNLIMITED on some of its dimension requires kpinsert() that allow
	/// growth on unlimited dimension instead of kpwrite() that allows to write only in predefined data space.
	/// 
	/// - See example below that creates unlimited space for keypoints chunking size of 100 but no compression:
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", cv::hdf::HDF5::H5_NONE, 100 );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * compresslevel: H5_NONE
	/// * chunks: H5_NONE
	fn kpcreate(&self, size: i32, kplabel: &str, compresslevel: i32, chunks: i32) -> Result<()> {
		extern_container_arg!(mut kplabel);
		unsafe { sys::cv_hdf_HDF5_kpcreate_const_const_int_String_const_int_const_int(self.as_raw_HDF5(), size, kplabel.opencv_as_extern_mut(), compresslevel, chunks) }.into_result()
	}
	
	/// Write or overwrite list of KeyPoint into specified dataset of hdf5 file.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	/// 
	/// Writes vector<KeyPoint> object into targeted dataset.
	/// 
	/// 
	/// Note: If dataset is not created and does not exist it will be created **automatically**. It is thread safe but
	/// it is recommended that writes to happen over separate non overlapping regions. Multiple datasets can be written
	/// inside single hdf5 file.
	/// 
	/// - Example below writes a 100 keypoints into a dataset. No dataset precreation required. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```ignore
	///   // generate 100 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 100; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // write / overwrite dataset
	///   h5io->kpwrite( keypoints, "keypoints" );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// - Example below uses smaller set of 50 keypoints and writes into compressed space of 100 keypoints optimised by 10 chunks.
	/// Same keypoint set is written three times, first into first half (0->50) and at second half (50->75) then into remaining slots
	/// (75->99) of data space using offset and count parameters to settle the window for write access.If routine is called multiple times
	/// dataset will be just overwritten:
	/// ```ignore
	///   // generate 50 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 50; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create maximum compressed space of size 100 with chunk size 10
	///   h5io->kpcreate( 100, "keypoints", 9, 10 );
	///   // write into first half
	///   h5io->kpwrite( keypoints, "keypoints", 0 );
	///   // write first 25 keypoints into second half
	///   h5io->kpwrite( keypoints, "keypoints", 50, 25 );
	///   // write first 25 keypoints into remained space of second half
	///   h5io->kpwrite( keypoints, "keypoints", 75, 25 );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	fn kpwrite(&self, keypoints: core::Vector::<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(mut kplabel);
		unsafe { sys::cv_hdf_HDF5_kpwrite_const_const_vector_KeyPoint__String_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern_mut(), offset, counts) }.into_result()
	}
	
	/// Insert or overwrite list of KeyPoint into specified dataset and autoexpand dataset size if **unlimited** property allows.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	/// 
	/// Writes vector<KeyPoint> object into targeted dataset and **autoexpand** dataset dimension if allowed.
	/// 
	/// 
	/// Note: Unlike kpwrite(), datasets are **not** created **automatically**. If dsinsert() happen over outer region of dataset
	/// and dataset has been created in **unlimited** mode then dataset is expanded, otherwise exception is thrown. To create datasets
	/// with **unlimited** property see kpcreate() and the optional H5_UNLIMITED flag at creation time. It is not thread safe over same
	/// dataset but multiple datasets can be merged inside single hdf5 file.
	/// 
	/// - Example below creates **unlimited** space for keypoints storage, and inserts a list of 10 keypoints ten times into that space.
	/// Final dataset will have 100 keypoints. Chunks size is 10 just optimized against list of keypoints. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```ignore
	///   // generate 10 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 10; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create unlimited size space with chunk size of 10
	///   h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", -1, 10 );
	///   // insert 10 times same 10 keypoints
	///   for(int i = 0; i < 10; i++)
	///    h5io->kpinsert( keypoints, "keypoints", i * 10 );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	fn kpinsert(&self, keypoints: core::Vector::<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(mut kplabel);
		unsafe { sys::cv_hdf_HDF5_kpinsert_const_const_vector_KeyPoint__String_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern_mut(), offset, counts) }.into_result()
	}
	
	/// Read specific keypoint dataset from hdf5 file into vector<KeyPoint> object.
	/// ## Parameters
	/// * keypoints: vector<KeyPoint> container where data reads will be returned.
	/// * kplabel: specify the source hdf5 dataset label.
	/// * offset: specify the offset location over dataset from where read starts.
	/// * counts: specify the amount of keypoints from dataset to read.
	/// 
	/// Reads out vector<KeyPoint> object reflecting the stored dataset.
	/// 
	/// 
	/// Note: If hdf5 file does not exist an exception will be thrown. Use hlexists() to check dataset presence.
	/// It is thread safe.
	/// 
	/// - Example below reads a dataset containing keypoints starting with second entry:
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1 );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// - Example below perform read of 3 keypoints from second entry.
	/// ```ignore
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read three keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1, 3 );
	///   // release
	///   h5io->close();
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	fn kpread(&self, keypoints: &mut core::Vector::<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(mut kplabel);
		unsafe { sys::cv_hdf_HDF5_kpread_const_vector_KeyPoint_R_String_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_mut_VectorOfKeyPoint(), kplabel.opencv_as_extern_mut(), offset, counts) }.into_result()
	}
	
}
