use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img_color = imgcodecs::imread("assets/sun-well.png", imgcodecs::IMREAD_COLOR)?;

    let mut img_gray = Mat::default();
    imgproc::cvt_color_def(&img_color, &mut img_gray, imgproc::COLOR_BGR2GRAY)?;

    // ! 轮廓特征
    let mut img_thresh = Mat::default();
    imgproc::threshold(
        &img_gray,                                         // 输入图片
        &mut img_thresh,                                   // 输出图片
        127.0,                                             // 阈值
        255.0,                                             // 最大值
        imgproc::THRESH_BINARY_INV + imgproc::THRESH_OTSU, // 类型
    )?;
    // 寻找二值化图中的轮廓
    let mut contours: Vector<Vector<core::Point>> = Vector::new();
    imgproc::find_contours_def(
        &img_thresh,        // 输入图片
        &mut contours,      // 输出图片
        imgproc::RETR_TREE, // 深度
        imgproc::CHAIN_APPROX_SIMPLE,
    )?;

    let cnt = contours.get(1)?;
    // 轮廓面积
    let area = imgproc::contour_area_def(&cnt)?;
    println!("area: {}", area);

    // 轮廓周长
    let perimeter = imgproc::arc_length(&cnt, true)?;
    println!("perimeter: {}", perimeter);

    // 图像矩 - 各类几何特征
    // https://en.wikipedia.org/wiki/Image_moment
    let m = imgproc::moments(&cnt, true)?;
    println!("m: {:?}", m);

    let mut hu_var = [0.0; 7];
    imgproc::hu_moments(m, &mut hu_var)?;
    println!("hu_var: {:?}", hu_var);

    Ok(())
}
