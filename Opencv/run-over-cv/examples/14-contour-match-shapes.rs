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

    // 形状匹配, 检测两个形状之间的相似度, 值越小, 越相似
    //
    // 形状匹配是通过图像矩(imgproc::hu_moments)来实现的
    // https://en.wikipedia.org/wiki/Image_moment#Rotation_invariant_moments
    let match_val = imgproc::match_shapes(
        &cnt,
        &cnt,
        imgproc::ShapeMatchModes::CONTOURS_MATCH_I1.into(),
        0.0,
    )?;
    println!("match_val: {}", match_val);

    Ok(())
}
