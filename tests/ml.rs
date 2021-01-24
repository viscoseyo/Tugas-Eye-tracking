use opencv::{
    core::{Scalar, Size},
    ml,
    prelude::*,
    Result,
    types::PtrOfKNearest,
};

#[test]
fn knn() -> Result<()> {
    let mut knn: PtrOfKNearest = KNearest::create()?;
    assert!(StatModel::empty(&knn)?);
    let samp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(1.))?;
    let resp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(2.))?;
    knn.train(&samp, ml::ROW_SAMPLE, &resp)?;
    let mut resp = Mat::default()?;
    let mut neigh = Mat::default()?;
    let mut dist = Mat::default()?;
    knn.find_nearest(&samp, 3, &mut resp, &mut neigh, &mut dist)?;
    assert_eq!(2., *resp.at_2d::<f32>(0, 0)?);
    assert_eq!(Size::new(1, 1), resp.size()?);
    let width = if cfg!(feature = "opencv-32") { 3 } else { 1 };
    assert_eq!(2., *neigh.at_2d::<f32>(0, 0)?);
    assert_eq!(Size::new(width, 1), neigh.size()?);
    assert_eq!(0., *dist.at_2d::<f32>(0, 0)?);
    assert_eq!(Size::new(width, 1), dist.size()?);
    Ok(())
}
