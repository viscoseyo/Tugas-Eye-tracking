use opencv::core::{Point2d, Size, Size2d, Size2f, Size2i};

#[test]
fn size_add() {
    let src = Size::new(0, 0);

    let res = Size::new(50, 50);
    {
        let src = src.clone();
        let out = src + Size::new(50, 50);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out += Size::new(50, 50);
        assert_eq!(out, res);
    }
}

#[test]
fn size_sub() {
    let src = Size::new(50, 50);

    let res = Size::new(25, 25);
    {
        let src = src.clone();
        let out = src - Size::new(25, 25);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out -= Size::new(25, 25);
        assert_eq!(out, res);
    }
}

#[test]
fn size_mul() {
    let src = Size2f::new(50., 50.);

    let res = Size2f::new(100., 100.);
    {
        let src = src.clone();
        let out = src * 2.;
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out *= 2.;
        assert_eq!(out, res);
    }
}

#[test]
fn size_div() {
    let src = Size::new(50, 50);

    let res = Size::new(25, 25);
    {
        let src = src.clone();
        let out = src / 2;
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out /= 2;
        assert_eq!(out, res);
    }
}

#[test]
fn size_methods() {
    let mut size = Size2d::new(10., 20.);
    assert!(!size.empty());
    assert_eq!(200., size.area());
    size.width = 0.;
    assert!(size.empty());
}

#[test]
fn size_conv() {
    let sizef = Size2d::new(1.2, 2.3);
    assert_eq!(Size2i::new(1, 2), sizef.to::<i32>().unwrap());
    assert_eq!(Size2f::new(1.2, 2.3), sizef.to::<f32>().unwrap());
    let sizei = Size2i::new(1, 2);
    assert_eq!(Size2f::new(1., 2.), sizei.to::<f32>().unwrap());

    assert_eq!(Size2d::new(10., 20.), Size2d::from_point(Point2d::new(10., 20.)));
}
