use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut img = imgcodecs::imread("assets/sun-well.png", imgcodecs::IMREAD_COLOR)?;

    let mut img_gray = Mat::default();
    imgproc::cvt_color_def(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY)?;

    // ! 轮廓多边形逼近
    // 采用的是Douglas-Peucker算法
    // 核心就是不断找多边形最远的点加入形成新的多边形, 直到最短距离小于指定的精度

    let mut thresh = Mat::default();
    imgproc::threshold(
        &img_gray,                                         // 输入图片
        &mut thresh,                                       // 输出图片
        127.0,                                             // 阈值
        255.0,                                             // 最大值
        imgproc::THRESH_BINARY_INV + imgproc::THRESH_OTSU, // 类型
    )?;
    // 寻找二值化图中的轮廓
    let mut contours: Vector<Vector<core::Point>> = Vector::new();
    imgproc::find_contours_def(
        &thresh,                      // 输入图片
        &mut contours,                // 输出图片
        imgproc::RETR_TREE, // 轮廓的查找方法, https://docs.opencv.org/4.0.0/d3/dc0/group__imgproc__shape.html#ga819779b9857cc2f8601e6526a3a5bc71
        imgproc::CHAIN_APPROX_SIMPLE, // 轮廓近似方法, https://docs.opencv.org/4.0.0/d3/dc0/group__imgproc__shape.html#ga4303f45752694956374734a03c54d5ff
    )?;
    let contour = contours.get(1)?;
    let mut approx_curve: Vector<core::Point> = Vector::new();
    imgproc::approx_poly_dp(&contour, &mut approx_curve, 5.0, true)?;

    let contour_approx: Vector<Vector<core::Point>> = Vector::from_elem(approx_curve, 1);
    imgproc::draw_contours(
        &mut img,                                // 输入图片
        &contour_approx,                         // 轮廓
        -1,                                      // 轮廓索引, -1 表示绘制所有轮廓
        core::Scalar::new(0.0, 255.0, 0.0, 0.0), // 颜色
        2,
        imgproc::LINE_8,
        &core::no_array(),
        imgproc::INTER_MAX,
        core::Point::default(),
    )?;

    imgcodecs::imwrite("assets/output/sun-contour-approx.png", &img, &Vector::new())?;

    Ok(())
}
